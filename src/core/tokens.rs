use std::fmt;

use super::data;



pub const TK_NULL     : &'static str = "null";

pub const TK_VARIABLE : &'static str = "variable";
pub const TK_INTEGER  : &'static str = "integer";

pub const TK_ADD      : &'static str = "+";
pub const TK_SUBTRACT : &'static str = "-";
pub const TK_MULTIPLY : &'static str = "*";
pub const TK_DIVIDE   : &'static str = "/";

pub const TK_EQUALS   : &'static str = "=";

pub const TK_LPAREN   : &'static str = "lparen";
pub const TK_RPAREN   : &'static str = "rparen";

pub const TK_HEADER   : &'static str = "#";
pub const TK_HEADFUNC : &'static str = "headfunc";
pub const TK_COMMA    : &'static str = ",";

pub const TK_EOL      : &'static str = "eol";
pub const TK_EOF      : &'static str = "eof";



#[derive(Clone, Debug)]
pub struct Token {
    pub name  : String,
    pub value : String,
    pub range : data::Range
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value.len() >= 1 {
            write!(f, "{}", format!("<{}: {}>", self.name, self.value))
        } else {
            write!(f, "{}", format!("<{}>", self.name))
        }
    }
}
