use std::fmt;
use std::ops;

use super::logger;
use super::data;
use super::exceptions;



// Success/Failure identification class.
#[derive(Clone, Debug)]
pub struct EvaluationResult {
    pub success   : bool,
    pub value     : f32,
    pub exception : exceptions::RendererException
}



// Nodes for AST.
#[derive(Clone, Debug)]
pub struct Node {
    pub base  : NodeBase,
    pub range : data::Range
}
#[derive(Clone, Debug)]
pub enum NodeBase {



    // NODE = NODE
    EqualsExpression {
        left  : Box<Node>,
        right : Box<Node>
    },

    // Integer, Float, etc
    Number {
        value : f32
    },
    // Single character
    Variable {
        name  : String
    },

    // NODE (+|-|*|/) NODE
    AdditionOperation {
        left  : Box<Node>,
        right : Box<Node>
    },
    SubtractionOperation {
        left  : Box<Node>,
        right : Box<Node>
    },
    MultiplicationOperation {
        left  : Box<Node>,
        right : Box<Node>
    },
    DivisionOperation {
        left  : Box<Node>,
        right : Box<Node>
    },

    // #header_function_name(arg1, arg2, etc)
    HeaderFuncFrame {
        x : i32,
        y : i32,
        w : i32,
        h : i32
    },
    HeaderFuncResolution {
        w : i32,
        h : i32
    },
    HeaderFuncExport {
        filename : String
    },

    // function_name(arg1, arg2, etc)
    FunctionSin {
        a : Box<Node>
    },
    FunctionCos {
        a : Box<Node>
    },
    FunctionTan {
        a : Box<Node>
    },
    FunctionSqrt {
        a : Box<Node>
    }



}
// Method for evaluating the value of an expression.
impl Node {
    pub fn evaluate(&self, x : f32) -> EvaluationResult {
        return match &self.base {

            // If variable name is `x`, return the value of x.
            NodeBase::Variable          {name}        => {
                if *name == "x".to_string() {
                    return EvaluationResult {
                        success   : true,
                        value     : x,
                        exception : exceptions::RendererException {
                            base    : exceptions::RendererExceptionBase::NoException,
                            message : "".to_string(),
                            range   : self.range.clone()
                        }
                    };
                }
                logger::error("Invalid Variable Name");
                return EvaluationResult {
                    success   : false,
                    value     : 0.0,
                    exception : exceptions::RendererException {
                        base    : exceptions::RendererExceptionBase::InvalidVariableException,
                        message : format!("Invalid variable `{}` was found.", name),
                        range   : self.range.clone()
                    }
                };
            },

            // Return number value.
            NodeBase::Number            {value}       => EvaluationResult {
                success   : true,
                value     : *value,
                exception : exceptions::RendererException {
                    base    : exceptions::RendererExceptionBase::NoException,
                    message : "".to_string(),
                    range   : self.range.clone()
                }
            },

            // Evaluate left and right values, then add right to left.
            NodeBase::AdditionOperation {left, right} => {
                let left_res  = left.evaluate(x);
                if ! left_res.success {
                    return left_res;
                }
                let right_res = right.evaluate(x);
                if ! right_res.success {
                    return right_res;
                }
                return EvaluationResult {
                    success   : true,
                    value     : left_res.value + right_res.value,
                    exception : exceptions::RendererException {
                        base    : exceptions::RendererExceptionBase::NoException,
                        message : "".to_string(),
                        range   : data::Range {
                            start    : left_res.exception.range.start,
                            end      : right_res.exception.range.end,
                            filename : left_res.exception.range.filename
                        }
                    }
                };
            },

            // Evaluate left and right values, then subtraft right from left.
            NodeBase::SubtractionOperation {left, right} => {
                let left_res  = left.evaluate(x);
                if ! left_res.success {
                    return left_res;
                }
                let right_res = right.evaluate(x);
                if ! right_res.success {
                    return right_res;
                }
                return EvaluationResult {
                    success   : true,
                    value     : left_res.value - right_res.value,
                    exception : exceptions::RendererException {
                        base    : exceptions::RendererExceptionBase::NoException,
                        message : "".to_string(),
                        range   : data::Range {
                            start    : left_res.exception.range.start,
                            end      : right_res.exception.range.end,
                            filename : left_res.exception.range.filename
                        }
                    }
                };
            },

            // Evaluate left and right values, then multiply left and right.
            NodeBase::MultiplicationOperation {left, right} => {
                let left_res  = left.evaluate(x);
                if ! left_res.success {
                    return left_res;
                }
                let right_res = right.evaluate(x);
                if ! right_res.success {
                    return right_res;
                }
                return EvaluationResult {
                    success   : true,
                    value     : left_res.value * right_res.value,
                    exception : exceptions::RendererException {
                        base    : exceptions::RendererExceptionBase::NoException,
                        message : "".to_string(),
                        range   : data::Range {
                            start    : left_res.exception.range.start,
                            end      : right_res.exception.range.end,
                            filename : left_res.exception.range.filename
                        }
                    }
                };
            },

            // If right is not 0, evaluate left and right values, then divide right from left.
            NodeBase::DivisionOperation {left, right} => {
                let left_res  = left.evaluate(x);
                if ! left_res.success {
                    return left_res;
                }
                let right_res = right.evaluate(x);
                if ! right_res.success {
                    return right_res;
                }
                if right_res.value == 0.0 {
                    return EvaluationResult {
                        success   : false,
                        value     : 0.0,
                        exception : exceptions::RendererException {
                            base    : exceptions::RendererExceptionBase::DivisionByZeroException,
                            message : "".to_string(),
                            range   : data::Range {
                                start    : left_res.exception.range.start,
                                end      : right_res.exception.range.end,
                                filename : left_res.exception.range.filename
                            }
                        }
                    }
                }
                return EvaluationResult {
                    success   : true,
                    value     : left_res.value / right_res.value,
                    exception : exceptions::RendererException {
                        base    : exceptions::RendererExceptionBase::NoException,
                        message : "".to_string(),
                        range   : data::Range {
                            start    : left_res.exception.range.start,
                            end      : right_res.exception.range.end,
                            filename : left_res.exception.range.filename
                        }
                    }
                };
            },

            // Evaluate argument and return sin value.
            NodeBase::FunctionSin {a} => {
                let res = a.evaluate(x);
                if ! res.success {
                    return res;
                }
                return EvaluationResult {
                    success   : true,
                    value     : res.value.sin(),
                    exception : exceptions::RendererException {
                        base    : exceptions::RendererExceptionBase::NoException,
                        message : "".to_string(),
                        range   : data::Range {
                            start    : res.exception.range.start,
                            end      : res.exception.range.end,
                            filename : res.exception.range.filename
                        }
                    }
                }
            },

            // Evaluate argument and return cos value.
            NodeBase::FunctionCos {a} => {
                let res = a.evaluate(x);
                if ! res.success {
                    return res;
                }
                return EvaluationResult {
                    success   : true,
                    value     : res.value.cos(),
                    exception : exceptions::RendererException {
                        base    : exceptions::RendererExceptionBase::NoException,
                        message : "".to_string(),
                        range   : data::Range {
                            start    : res.exception.range.start,
                            end      : res.exception.range.end,
                            filename : res.exception.range.filename
                        }
                    }
                }
            },

            // Evaluate argument and return tan value.
            NodeBase::FunctionTan {a} => {
                let res = a.evaluate(x);
                if ! res.success {
                    return res;
                }
                return EvaluationResult {
                    success   : true,
                    value     : res.value.tan(),
                    exception : exceptions::RendererException {
                        base    : exceptions::RendererExceptionBase::NoException,
                        message : "".to_string(),
                        range   : data::Range {
                            start    : res.exception.range.start,
                            end      : res.exception.range.end,
                            filename : res.exception.range.filename
                        }
                    }
                }
            },

            // Evaluate argument and return sqrt value.
            NodeBase::FunctionSqrt {a} => {
                let res = a.evaluate(x);
                if ! res.success {
                    return res;
                }
                return EvaluationResult {
                    success   : true,
                    value     : res.value.sqrt(),
                    exception : exceptions::RendererException {
                        base    : exceptions::RendererExceptionBase::NoException,
                        message : "".to_string(),
                        range   : data::Range {
                            start    : res.exception.range.start,
                            end      : res.exception.range.end,
                            filename : res.exception.range.filename
                        }
                    }
                }
            }

            // Unknown node found.
            _ => return EvaluationResult {
                success   : false,
                value     : 0.0,
                exception : exceptions::RendererException {
                    base    : exceptions::RendererExceptionBase::InternalException,
                    message : format!("Unknown node `{}` found.", self).to_string(),
                    range   : data::Range {
                        start    : self.range.start,
                        end      : self.range.end,
                        filename : self.range.filename.clone()
                    }
                }
            }

        };
    }
}
// Displays for different types of nodes for debugging.
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.base {
            NodeBase::EqualsExpression        {left, right} => write!(f, "({} = {})", left, right),
            NodeBase::Number                  {value}       => write!(f, "{}", value),
            NodeBase::Variable                {name}        => write!(f, "{}", name),
            NodeBase::AdditionOperation       {left, right} => write!(f, "({} + {})", left, right),
            NodeBase::SubtractionOperation    {left, right} => write!(f, "({} - {})", left, right),
            NodeBase::MultiplicationOperation {left, right} => write!(f, "({} * {})", left, right),
            NodeBase::DivisionOperation       {left, right} => write!(f, "({} / {})", left, right),
            NodeBase::HeaderFuncFrame         {x, y, w, h}  => write!(f, "#frame({}, {}, {}, {})", x, y, w, h),
            NodeBase::HeaderFuncResolution    {w, h}        => write!(f, "#resolution({}, {})", w, h),
            NodeBase::HeaderFuncExport        {filename}    => write!(f, "#export(`{}`)", data::escapify(filename.clone())),
            NodeBase::FunctionSin             {a}           => write!(f, "sin({})", a),
            NodeBase::FunctionCos             {a}           => write!(f, "cos({})", a),
            NodeBase::FunctionTan             {a}           => write!(f, "tan({})", a),
            NodeBase::FunctionSqrt             {a}          => write!(f, "sqrt({})", a)
        }
    }
}
// Ease of use addition implementation.
impl ops::Add for Node {
    type Output = Self;
    fn add(self, other: Node) -> Self {
        return Node {
            base : NodeBase::AdditionOperation {
                left  : Box::new(self.clone()),
                right : Box::new(other.clone())
            },
            range : data::Range {
                filename : self.range.filename,
                start    : self.range.start,
                end      : other.range.end
            }
        };
    }
}
// Ease of use subtraction implementation.
impl ops::Sub for Node {
    type Output = Self;
    fn sub(self, other: Node) -> Self {
        return Node {
            base : NodeBase::SubtractionOperation {
                left  : Box::new(self.clone()),
                right : Box::new(other.clone())
            },
            range : data::Range {
                filename : self.range.filename,
                start    : self.range.start,
                end      : other.range.end
            }
        };
    }
}
// Ease of use multiplication implementation.
impl ops::Mul for Node {
    type Output = Self;
    fn mul(self, other: Node) -> Self {
        return Node {
            base : NodeBase::MultiplicationOperation {
                left  : Box::new(self.clone()),
                right : Box::new(other.clone())
            },
            range : data::Range {
                filename : self.range.filename,
                start    : self.range.start,
                end      : other.range.end
            }
        };
    }
}
// Ease of use division implementation.
impl ops::Div for Node {
    type Output = Self;
    fn div(self, other: Node) -> Self {
        return Node {
            base : NodeBase::DivisionOperation {
                left  : Box::new(self.clone()),
                right : Box::new(other.clone())
            },
            range : data::Range {
                filename : self.range.filename,
                start    : self.range.start,
                end      : other.range.end
            }
        };
    }
}
