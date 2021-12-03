// Character lists for lexing.
pub const ALPHABETIC : &'static str      = "abcdefghijklmnopqrstuvwxyz";
pub const NUMERIC    : &'static str      = "0123456789";

// Header functions for defining export settings.
pub const HEADFUNCS  : [&'static str; 3] = [
    "frame",
    "resolution",
    "export"
];
// Functions that can be used in expressions.
pub const FUNCTIONS  : [&'static str; 4] = [
    "sin",
    "cos",
    "tan",
    "sqrt"
];



// Range struct used for identifying where characters came from.
#[derive(Clone, Debug)]
pub struct Range {
    pub filename : String,
    pub start    : usize,
    pub end      : usize
}



// Positioning struct used for header function arguments.
#[derive(Clone, Debug)]
pub struct Vector2 {
    pub x : i32,
    pub y : i32
}



// Better colour class to make rendering code prettier.
#[derive(Clone, Debug)]
pub struct Colour {
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub a : f32
}



pub fn escapify(string: String) -> String {
    let mut ret = String::from("");

    for ch in string.chars() {
        if ch == '\n' {
            ret += "\\n";
        } else if ch == '`' {
            ret += "\\`";
        } else if ch == '\'' {
            ret += "\\'";
        } else if ch == '\"' {
            ret += "\\\"";
        } else if ch == '\t' {
            ret += "\t";
        } else if ch == '\\' {
            ret += "\\\\";
        } else {
            ret += ch.to_string().as_str();
        }
    }
    return ret;
}
