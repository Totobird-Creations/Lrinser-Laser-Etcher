use super::data;
use super::defaults;
use super::nodes;
use super::exceptions;



// Data class for the renderer.
#[derive(Clone, Debug)]
pub struct InterpreterData {
    pub set_frame      : bool,
    pub set_resolution : bool,
    pub set_export     : bool,
    pub print_now      : bool,

    pub position       : data::Vector2,
    pub size           : data::Vector2,

    pub resolution     : data::Vector2,

    pub export         : String,


    pub equations      : Vec<nodes::Node>
}



// Success/Failure identification class.
#[derive(Clone, Debug)]
pub struct InterpreterResult {
    pub success    : bool,
    pub data       : InterpreterData,
    pub exceptions : Vec<exceptions::InterpreterException>
}



// Function for interpreting a vector of nodes.
pub fn interpret(nodes : Vec<nodes::Node>) -> InterpreterResult {
    let mut data = InterpreterData {
        set_frame      : false,
        set_resolution : false,
        set_export     : false,
        print_now      : false,

        position       : defaults::POSITION,
        size           : defaults::SIZE,

        resolution     : defaults::RESOLUTION,

        export         : defaults::EXPORT.to_string(),


        equations      : vec![]
    };

    let mut exceptions = vec![];
    for node in nodes {
        let mut result = match node.base {
            nodes::NodeBase::HeaderFuncFrame      {x, y, w, h}  => interpret_headerfunc_frame(data.clone(), node.range, x, y, w, h),
            nodes::NodeBase::HeaderFuncResolution {w, h}        => interpret_headerfunc_resolution(data.clone(), node.range, w, h),
            nodes::NodeBase::HeaderFuncExport     {filename}    => interpret_headerfunc_export(data.clone(), node.range, filename),
            nodes::NodeBase::HeaderFuncPrintNow                 => interpret_headerfunc_print_now(data.clone(), node.range),
            nodes::NodeBase::EqualsExpression     {left, right} => interpret_equation_equals(data.clone(), node.range, *left, *right),
            _                                                   => {
                println!("{}", node);
                interpret_unknown(data.clone())
            },
        };
        if result.success {
            data = result.data;
        } else {
            exceptions.append(&mut result.exceptions);
        }
    }
    return InterpreterResult {
        success    : exceptions.len() <= 0,
        data       : data,
        exceptions : exceptions
    };
}



// Frame header function interpreter
pub fn interpret_headerfunc_frame(mut data : InterpreterData, range : data::Range, x : i32, y : i32, w : i32, h : i32) -> InterpreterResult {
    if w <= 0 || h <= 0 {
        return InterpreterResult {
            success    : false,
            data       : data,
            exceptions : vec![exceptions::InterpreterException {
                base    : exceptions::InterpreterExceptionBase::InvalidValueException,
                message : "Frame width and height must be at least 1.".to_string(),
                range   : range
            }]
        };
    }
    if data.set_frame {
        return InterpreterResult {
            success    : false,
            data       : data,
            exceptions : vec![exceptions::InterpreterException {
                base    : exceptions::InterpreterExceptionBase::HeaderAlreadyAccessedException,
                message : "Header `frame` has already been accessed.".to_string(),
                range   : range
            }]
        };
    }

    data.set_frame = true;
    data.position = data::Vector2 {
        x : x,
        y : y
    };
    data.size = data::Vector2 {
        x : w,
        y : h
    };

    return InterpreterResult {
        success    : true,
        data       : data,
        exceptions : vec![]
    };
}



// Resolution header function interpreter
pub fn interpret_headerfunc_resolution(mut data : InterpreterData, range : data::Range, w : i32, h : i32) -> InterpreterResult {
    if w < 0 || h < 0 {
        return InterpreterResult {
            success    : false,
            data       : data,
            exceptions : vec![exceptions::InterpreterException {
                base    : exceptions::InterpreterExceptionBase::InvalidValueException,
                message : "Resolution width and height must be at least 0.".to_string(),
                range   : range
            }]
        };
    }
    if data.set_resolution {
        return InterpreterResult {
            success    : false,
            data       : data,
            exceptions : vec![exceptions::InterpreterException {
                base    : exceptions::InterpreterExceptionBase::HeaderAlreadyAccessedException,
                message : "Header `resolution` has already been accessed.".to_string(),
                range   : range
            }]
        };
    }

    data.set_resolution = true;
    data.resolution = data::Vector2 {
        x : w,
        y : h
    };

    return InterpreterResult {
        success    : true,
        data       : data,
        exceptions : vec![]
    };
}



// Export header function interpreter
pub fn interpret_headerfunc_export(mut data : InterpreterData, range : data::Range, filename : String) -> InterpreterResult {
    if data.set_export {
        return InterpreterResult {
            success    : false,
            data       : data,
            exceptions : vec![exceptions::InterpreterException {
                base    : exceptions::InterpreterExceptionBase::HeaderAlreadyAccessedException,
                message : "Header `export` has already been accessed.".to_string(),
                range   : range
            }]
        };
    }

    data.set_export = true;
    data.export = filename;

    return InterpreterResult {
        success    : true,
        data       : data,
        exceptions : vec![]
    };
}



// Print now header function interpreter
pub fn interpret_headerfunc_print_now(mut data : InterpreterData, range : data::Range) -> InterpreterResult {
    if data.print_now {
        return InterpreterResult {
            success    : false,
            data       : data,
            exceptions : vec![exceptions::InterpreterException {
                base    : exceptions::InterpreterExceptionBase::HeaderAlreadyAccessedException,
                message : "Header `print_now` has already been accessed.".to_string(),
                range   : range
            }]
        };
    }

    data.print_now = true;

    return InterpreterResult {
        success    : true,
        data       : data,
        exceptions : vec![]
    };
}



// Equality equation interpreter
pub fn interpret_equation_equals(mut data : InterpreterData, range : data::Range, left : nodes::Node, right : nodes::Node) -> InterpreterResult {
    data.equations.push(
        nodes::Node {
            base  : nodes::NodeBase::EqualsExpression {
                left  : Box::new(left),
                right : Box::new(right)
            },
            range : range
        }
    );
    return InterpreterResult {
        success    : true,
        data       : data,
        exceptions : vec![]
    };
}



// Fallback for unknown nodes
pub fn interpret_unknown(data : InterpreterData) -> InterpreterResult {
    println!("Unknown node found");
    return InterpreterResult {
        success    : true,
        data       : data,
        exceptions : vec![]
    }
}
