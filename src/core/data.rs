pub const ALPHABETIC : &'static str      = "abcdefghijklmnopqrstuvwxyz";
pub const NUMERIC    : &'static str      = "0123456789";
pub const HEADFUNCS  : [&'static str; 2] = [
    "frame",
    "resolution"
];



#[derive(Clone, Debug)]
pub struct Range {
    pub filename : String,
    pub start    : usize,
    pub end      : usize
}



#[derive(Clone, Debug)]
pub struct Vector2 {
    pub x : i32,
    pub y : i32
}



#[derive(Clone, Debug)]
pub struct Colour {
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub a : f32
}
pub fn colour(r : f32, g : f32, b : f32, a : f32) -> Colour {
    return Colour {
        r : r,
        g : g,
        b : b,
        a : a
    };
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
