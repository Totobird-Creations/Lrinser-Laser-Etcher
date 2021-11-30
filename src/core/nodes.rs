use std::fmt;
use std::ops;

use super::data;



#[derive(Clone, Debug)]
pub struct Node {
    pub base  : NodeBase,
    pub range : data::Range
}
#[derive(Clone, Debug)]
pub enum NodeBase {



    EqualsExpression {
        left  : Box<Node>,
        right : Box<Node>
    },

    Number {
        value : f32
    },
    Variable {
        name  : String
    },

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
    }



}
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
            NodeBase::HeaderFuncExport        {filename}    => write!(f, "#export(`{}`)", data::escapify(filename.clone()))
        }
    }
}
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
