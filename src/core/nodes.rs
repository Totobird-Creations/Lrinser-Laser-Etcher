use std::fmt;
use std::ops;

use super::logger;
use super::data;
use super::exceptions;



// Success/Failure identification class.
#[derive(Clone, Debug)]
pub struct EvaluationResult {
    pub success   : bool,
    pub value     : Node,
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

    // Debug for simplifier
    Void,
    MultipleNumber {
        value : data::MultipleValues
    },



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
    HeaderFuncPrintNow,

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
    FunctionRoot {
        exp        : Box<Node>,
        base       : Box<Node>,
        user_typed : bool
    },
    FunctionPow {
        base : Box<Node>,
        exp  : Box<Node>
    }



}
// Method for evaluating the value of an expression.
impl Node {
    pub fn simplify(&self, x : f32) -> EvaluationResult {
        return match &self.base {

            // Handle left = right.
            NodeBase::EqualsExpression  {left, right} => {
                let left_res  = left.simplify(x);
                if ! left_res.success {
                    return left_res;
                }
                let right_res = right.simplify(x);
                if ! right_res.success {
                    return right_res;
                }
                match left_res.value.base.clone() {
                    NodeBase::Variable {name} => {
                        if name == "y" {
                            match right_res.value.base.clone() {
                                NodeBase::MultipleNumber {value : _value} => {
                                    return right_res;
                                },
                                _                         => ()
                            }
                        }
                    },
                    _                         => ()
                }
                match right_res.value.base.clone() {
                    NodeBase::Variable {name} => {
                        if name == "y" {
                            match left_res.value.base.clone() {
                                NodeBase::MultipleNumber {value : _value} => {
                                    return left_res;
                                },
                                _                         => ()
                            }
                        }
                    },
                    _                         => ()
                }
                return EvaluationResult {
                    success   : true,
                    value     : Node {
                        base  : NodeBase::EqualsExpression {
                            left  : Box::new(left_res.value),
                            right : Box::new(right_res.value)
                        },
                        range : data::Range {
                            start    : left_res.exception.range.start,
                            end      : right_res.exception.range.end,
                            filename : left_res.exception.range.filename.clone()
                        }
                    },
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
            }

            // If variable name is `x`, return the value of x.
            NodeBase::Variable          {name}        => {
                if *name == "x".to_string() {
                    return EvaluationResult {
                        success   : true,
                        value     : Node {
                            base  : NodeBase::MultipleNumber {
                                value : data::MultipleValues::new_single(x)
                            },
                            range : self.range.clone()
                        },
                        exception : exceptions::RendererException {
                            base    : exceptions::RendererExceptionBase::NoException,
                            message : "".to_string(),
                            range   : self.range.clone()
                        }
                    };
                } else if *name == "y".to_string() {
                    return EvaluationResult {
                        success   : true,
                        value     : self.clone(),
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
                    value     : Node {
                        base  : NodeBase::Void,
                        range : self.range.clone()
                    },
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
                value     : Node {
                    base  : NodeBase::MultipleNumber {
                        value : data::MultipleValues::new_single(*value)
                    },
                    range : self.range.clone()
                },
                exception : exceptions::RendererException {
                    base    : exceptions::RendererExceptionBase::NoException,
                    message : "".to_string(),
                    range   : self.range.clone()
                }
            },

            // Evaluate left and right values, then add right to left.
            NodeBase::AdditionOperation {left, right} => {
                let left_res  = left.simplify(x);
                if ! left_res.success {
                    return left_res;
                }
                let right_res = right.simplify(x);
                if ! right_res.success {
                    return right_res;
                }
                let mut ret = EvaluationResult {
                    success   : true,
                    value     : left_res.value.clone() + right_res.value.clone(),
                    exception : exceptions::RendererException {
                        base    : exceptions::RendererExceptionBase::NoException,
                        message : "".to_string(),
                        range   : data::Range {
                            start    : left_res.exception.range.start,
                            end      : right_res.exception.range.end,
                            filename : left_res.exception.range.filename.clone()
                        }
                    }
                };
                match left_res.value.base {
                    NodeBase::MultipleNumber {value : left_value} => {
                        match right_res.value.base {
                            NodeBase::MultipleNumber {value : right_value} => {
                                let range = data::Range {
                                    start    : left_res.exception.range.start,
                                    end      : right_res.exception.range.end,
                                    filename : left_res.exception.range.filename.clone()
                                };
                                ret = EvaluationResult {
                                    success   : true,
                                    value     : Node {
                                        base  : NodeBase::MultipleNumber {
                                            value : left_value + right_value
                                        },
                                        range : range.clone()
                                    },
                                    exception : exceptions::RendererException {
                                        base    : exceptions::RendererExceptionBase::NoException,
                                        message : "".to_string(),
                                        range   : range
                                    }
                                };
                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }
                return ret;
            },

            // Evaluate left and right values, then subtraft right from left.
            NodeBase::SubtractionOperation {left, right} => {
                let left_res  = left.simplify(x);
                if ! left_res.success {
                    return left_res;
                }
                let right_res = right.simplify(x);
                if ! right_res.success {
                    return right_res;
                }
                let mut ret = EvaluationResult {
                    success   : true,
                    value     : left_res.value.clone() - right_res.value.clone(),
                    exception : exceptions::RendererException {
                        base    : exceptions::RendererExceptionBase::NoException,
                        message : "".to_string(),
                        range   : data::Range {
                            start    : left_res.exception.range.start,
                            end      : right_res.exception.range.end,
                            filename : left_res.exception.range.filename.clone()
                        }
                    }
                };
                match left_res.value.base {
                    NodeBase::MultipleNumber {value : left_value} => {
                        match right_res.value.base {
                            NodeBase::MultipleNumber {value : right_value} => {
                                let range = data::Range {
                                    start    : left_res.exception.range.start,
                                    end      : right_res.exception.range.end,
                                    filename : left_res.exception.range.filename
                                };
                                ret = EvaluationResult {
                                    success   : true,
                                    value     : Node {
                                        base  : NodeBase::MultipleNumber {
                                            value : left_value - right_value
                                        },
                                        range : range.clone()
                                    },
                                    exception : exceptions::RendererException {
                                        base    : exceptions::RendererExceptionBase::NoException,
                                        message : "".to_string(),
                                        range   : range
                                    }
                                };
                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }
                return ret;
            },

            // Evaluate left and right values, then multiply left and right.
            NodeBase::MultiplicationOperation {left, right} => {
                let left_res  = left.simplify(x);
                if ! left_res.success {
                    return left_res;
                }
                let right_res = right.simplify(x);
                if ! right_res.success {
                    return right_res;
                }
                let mut ret = EvaluationResult {
                    success   : true,
                    value     : left_res.value.clone() * right_res.value.clone(),
                    exception : exceptions::RendererException {
                        base    : exceptions::RendererExceptionBase::NoException,
                        message : "".to_string(),
                        range   : data::Range {
                            start    : left_res.exception.range.start,
                            end      : right_res.exception.range.end,
                            filename : left_res.exception.range.filename.clone()
                        }
                    }
                };
                match left_res.value.base {
                    NodeBase::MultipleNumber {value : left_value} => {
                        match right_res.value.base {
                            NodeBase::MultipleNumber {value : right_value} => {
                                let range = data::Range {
                                    start    : left_res.exception.range.start,
                                    end      : right_res.exception.range.end,
                                    filename : left_res.exception.range.filename
                                };
                                ret = EvaluationResult {
                                    success   : true,
                                    value     : Node {
                                        base  : NodeBase::MultipleNumber {
                                            value : left_value * right_value
                                        },
                                        range : range.clone()
                                    },
                                    exception : exceptions::RendererException {
                                        base    : exceptions::RendererExceptionBase::NoException,
                                        message : "".to_string(),
                                        range   : range
                                    }
                                };
                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }
                return ret;
            },

            // If right is not 0, evaluate left and right values, then divide right from left.
            NodeBase::DivisionOperation {left, right} => {
                
                let left_res  = left.simplify(x);
                if ! left_res.success {
                    return left_res;
                }
                let right_res = right.simplify(x);
                if ! right_res.success {
                    return right_res;
                }
                let mut ret = EvaluationResult {
                    success   : true,
                    value     : left_res.value.clone() / right_res.value.clone(),
                    exception : exceptions::RendererException {
                        base    : exceptions::RendererExceptionBase::NoException,
                        message : "".to_string(),
                        range   : data::Range {
                            start    : left_res.exception.range.start,
                            end      : right_res.exception.range.end,
                            filename : left_res.exception.range.filename.clone()
                        }
                    }
                };
                match left_res.value.base {
                    NodeBase::MultipleNumber {value : left_value} => {
                        match right_res.value.base {
                            NodeBase::MultipleNumber {value : right_value} => {
                                let range = data::Range {
                                    start    : left_res.exception.range.start,
                                    end      : right_res.exception.range.end,
                                    filename : left_res.exception.range.filename
                                };
                                ret = EvaluationResult {
                                    success   : true,
                                    value     : Node {
                                        base  : NodeBase::MultipleNumber {
                                            value : left_value / right_value
                                        },
                                        range : range.clone()
                                    },
                                    exception : exceptions::RendererException {
                                        base    : exceptions::RendererExceptionBase::NoException,
                                        message : "".to_string(),
                                        range   : range
                                    }
                                };
                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }
                return ret;
            },

            // Evaluate argument and return sin value.
            NodeBase::FunctionSin {a} => {
                let res = a.simplify(x);
                if ! res.success {
                    return res;
                }
                match res.value.base {
                    NodeBase::MultipleNumber {value : _} => {
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
                        };
                    },
                    _ => {panic!("INTERNAL ERROR | TODO FIX")}
                }
                
            },

            // Evaluate argument and return cos value.
            NodeBase::FunctionCos {a} => {
                let res = a.simplify(x);
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
                let res = a.simplify(x);
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

            // Evaluate arguments and return nth root value.
            NodeBase::FunctionRoot {exp, base, user_typed} => {
                let exp_res = exp.simplify(x);
                if ! exp_res.success {
                    return exp_res;
                }
                let base_res = base.simplify(x);
                if ! base_res.success {
                    return base_res;
                }
                return EvaluationResult {
                    success   : true,
                    value     : base_res.value.root(exp_res.value, *user_typed),
                    exception : exceptions::RendererException {
                        base    : exceptions::RendererExceptionBase::NoException,
                        message : "".to_string(),
                        range   : data::Range {
                            start    : exp_res.exception.range.start,
                            end      : base_res.exception.range.end,
                            filename : exp_res.exception.range.filename
                        }
                    }
                }
            }

            // Evaluate arguments and return powed value.
            NodeBase::FunctionPow {base, exp} => {
                let base_res = base.simplify(x);
                if ! base_res.success {
                    return base_res;
                }
                let exp_res = exp.simplify(x);
                if ! exp_res.success {
                    return exp_res;
                }
                return EvaluationResult {
                    success   : true,
                    value     : base_res.value.pow(exp_res.value),
                    exception : exceptions::RendererException {
                        base    : exceptions::RendererExceptionBase::NoException,
                        message : "".to_string(),
                        range   : data::Range {
                            start    : base_res.exception.range.start,
                            end      : exp_res.exception.range.end,
                            filename : base_res.exception.range.filename
                        }
                    }
                }
            }

            // Unknown node found.
            _ => {
                let range = data::Range {
                    start    : self.range.start,
                    end      : self.range.end,
                    filename : self.range.filename.clone()
                };
                return EvaluationResult {
                    success   : false,
                    value     : Node {
                        base  : NodeBase::Void,
                        range : range.clone()
                    },
                    exception : exceptions::RendererException {
                        base    : exceptions::RendererExceptionBase::InternalException,
                        message : format!("Unknown node `{}` found.", self).to_string(),
                        range   : range
                    }
                }
            }

        };
    }



    fn sin(&self) -> Node {
        match self.base.clone() {
            NodeBase::MultipleNumber {value} => {
                return Node {
                    base  : NodeBase::MultipleNumber {
                        value : value.sin()
                    },
                    range : self.range.clone()
                }
            }
            _ => ()
        }
        return self.clone();
    }

    fn cos(&self) -> Node {
        match self.base.clone() {
            NodeBase::MultipleNumber {value} => {
                return Node {
                    base  : NodeBase::MultipleNumber {
                        value : value.cos()
                    },
                    range : self.range.clone()
                }
            }
            _ => ()
        }
        return self.clone();
    }

    fn tan(&self) -> Node {
        match self.base.clone() {
            NodeBase::MultipleNumber {value} => {
                return Node {
                    base  : NodeBase::MultipleNumber {
                        value : value.tan()
                    },
                    range : self.range.clone()
                }
            }
            _ => ()
        }
        return self.clone();
    }

    fn root(&self, exp : Node, user_typed : bool) -> Node {
        panic!("Root");
    }

    fn pow(&self, exp : Node) -> Node {
        match self.base.clone() {
            NodeBase::MultipleNumber {ref value} => {
                return Node {
                    base : NodeBase::MultipleNumber {
                        value : value.clone().pow(value.clone())
                    },
                    range : self.range.clone()
                }
            }
            _ => ()
        }
        return self.clone();
    }
}
// Displays for different types of nodes for debugging.
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.base {
            NodeBase::Void                                            => write!(f, "void"),
            NodeBase::MultipleNumber          {value}                 => write!(f, "{:?}", value.values),

            NodeBase::EqualsExpression        {left, right}           => write!(f, "({} = {})", left, right),
            NodeBase::Number                  {value}                 => write!(f, "{}", value),
            NodeBase::Variable                {name}                  => write!(f, "{}", name),
            NodeBase::AdditionOperation       {left, right}           => write!(f, "({} + {})", left, right),
            NodeBase::SubtractionOperation    {left, right}           => write!(f, "({} - {})", left, right),
            NodeBase::MultiplicationOperation {left, right}           => write!(f, "({} * {})", left, right),
            NodeBase::DivisionOperation       {left, right}           => write!(f, "({} / {})", left, right),
            NodeBase::HeaderFuncFrame         {x, y, w, h}            => write!(f, "#frame({}, {}, {}, {})", x, y, w, h),
            NodeBase::HeaderFuncResolution    {w, h}                  => write!(f, "#resolution({}, {})", w, h),
            NodeBase::HeaderFuncExport        {filename}              => write!(f, "#export(`{}`)", data::escapify(filename.clone())),
            NodeBase::HeaderFuncPrintNow                              => write!(f, "#print_now()"),
            NodeBase::FunctionSin             {a}                     => write!(f, "sin({})", a),
            NodeBase::FunctionCos             {a}                     => write!(f, "cos({})", a),
            NodeBase::FunctionTan             {a}                     => write!(f, "tan({})", a),
            NodeBase::FunctionPow             {base, exp}             => write!(f, "pow({}, {})", base, exp),
            NodeBase::FunctionRoot            {exp, base, user_typed} => {
                if *user_typed {
                    write!(f, "({}root({}))", exp, base)
                } else {
                    write!(f, "(± ( {}root({})))", exp, base)
                }
            }
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
