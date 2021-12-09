use std::ops;



// Character lists for lexing.
pub const ALPHABETIC : &'static str      = "abcdefghijklmnopqrstuvwxyz";
pub const NUMERIC    : &'static str      = "0123456789";

// Header functions for defining export settings.
pub const HEADFUNCS  : [&'static str; 4] = [
    "frame",
    "resolution",
    "export",
    "print_now"
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



// Renderer multiple values.
#[derive(Clone, Debug)]
pub struct MultipleValues {
    pub values : Vec<f32>
}
impl MultipleValues {
    pub fn new_empty() -> MultipleValues {
        return MultipleValues {
            values : vec![]
        };
    }
    pub fn new_single(value : f32) -> MultipleValues {
        return MultipleValues {
            values : vec![value]
        };
    }

    pub fn sin(self) -> MultipleValues {
        let mut res = vec![];
        for x in self.values {
            res.push(x.sin());
        }
        return MultipleValues {
            values : res
        }
    }

    pub fn cos(self) -> MultipleValues {
        let mut res = vec![];
        for x in self.values {
            res.push(x.cos());
        }
        return MultipleValues {
            values : res
        }
    }

    pub fn tan(self) -> MultipleValues {
        let mut res = vec![];
        for x in self.values {
            res.push(x.tan());
        }
        return MultipleValues {
            values : res
        }
    }

    pub fn sqrt(self) -> MultipleValues {
        let mut res = vec![];
        for x in self.values {
            res.push(x.sqrt());
            res.push(-x.sqrt());
        }
        return MultipleValues {
            values : res
        }
    }
}
impl ops::Add for MultipleValues {
    type Output = Self;
    fn add(self, other: MultipleValues) -> Self {
        let mut res = vec![];
        for x in self.values {
            for y in other.values.clone() {
                res.push(x + y);
            }
        }
        return MultipleValues {
            values : res
        };
    }
}
impl ops::Sub for MultipleValues {
    type Output = Self;
    fn sub(self, other: MultipleValues) -> Self {
        let mut res = vec![];
        for x in self.values {
            for y in other.values.clone() {
                res.push(x - y);
            }
        }
        return MultipleValues {
            values : res
        };
    }
}
impl ops::Mul for MultipleValues {
    type Output = Self;
    fn mul(self, other: MultipleValues) -> Self {
        let mut res = vec![];
        for x in self.values {
            for y in other.values.clone() {
                res.push(x * y);
            }
        }
        return MultipleValues {
            values : res
        };
    }
}
impl ops::Div for MultipleValues {
    type Output = Self;
    fn div(self, other: MultipleValues) -> Self {
        let mut res = vec![];
        for x in self.values {
            for y in other.values.clone() {
                res.push(x / y);
            }
        }
        return MultipleValues {
            values : res
        };
    }
}


// Renderer pixel min-max value.
#[derive(Clone, Debug)]
pub struct MinMax {
    pub min : MultipleValues,
    pub max : MultipleValues
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
