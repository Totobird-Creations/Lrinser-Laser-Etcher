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
pub const FUNCTIONS  : [&'static str; 5] = [
    "sin",
    "cos",
    "tan",
    "root",
    "pow"
];

pub const ROOT_MAX_RECURSION : i32 = 25;



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

    pub fn pow(self, exp : MultipleValues) -> MultipleValues {
        let mut res = vec![];
        for x in self.values {
            for y in exp.values.clone() {
                res.push(x.powf(y));
            }
        }
        return MultipleValues {
            values : res
        }
    }

    fn num_root(self, exp : f32, main : f32) -> f32 {
        let p      = 1e-9_f32;
        let mut x0 = main / exp;
        for _i in 0..ROOT_MAX_RECURSION {
            let x1 = ((exp - 1.0) * x0 + main / f32::powf(x0, exp - 1.0)) / exp;
            if (x1 - x0).abs() < (x0 * p).abs() {
                x0 = x1;
                break;
            }
            x0 = x1;
        }
        return x0;
    }

    pub fn root(self, exp : MultipleValues, user_typed : bool) -> MultipleValues {
        let mut res = vec![];
        for x in self.values.clone() {
            for y in exp.values.clone() {
                if y % 2.0 != 0.0 || x >= 0.0 {
                    let v = self.clone().num_root(y, x);
                    res.push(v);
                    if ! user_typed {
                        res.push(-v);
                    }
                }
            }
        }
        return MultipleValues {
            values : res
        }
    }

    pub fn abs(self) -> MultipleValues {
        let mut res = vec![];
        for x in self.values {
            res.push(x.abs());
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
impl ops::Neg for MultipleValues {
    type Output = Self;
    fn neg(self) -> Self {
        let mut res = vec![];
        for x in self.values {
            res.push(-x);
        }
        return MultipleValues {
            values : res
        };
    }
}



// Renderer pixel left-right values.
#[derive(Clone, Debug)]
pub struct LeftRight {
    pub left  : MultipleValues,
    pub right : MultipleValues
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
