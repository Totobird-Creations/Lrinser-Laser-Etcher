use std::env;
use std::process;

use super::exceptions;



// Success/Failure identification class.
#[derive(Clone, Debug)]
pub struct PrinterResult {
    pub success   : bool,
    pub exception : exceptions::PrinterException
}



// Function for printing an image.
pub fn print(filename : String) -> PrinterResult {
    return match env::consts::OS {
        "windows" => print_windows(filename),
        _         => PrinterResult {
            success   : false,
            exception : exceptions::PrinterException {
                base    : exceptions::PrinterExceptionBase::UnsupportedOperatingSystemException,
                message : format!("Operating system `{}` is not supported for printing.", env::consts::OS)
            }
        }
    };
}



fn print_windows(filename : String) -> PrinterResult {
    match process::Command::new("cmd")
        //.args(&["/C", format!("mspaint /pt \"{}\\{}\"", env::current_dir().unwrap().display(), filename).as_str()])
        .args(&["/C", format!("mspaint /pt {}", filename).as_str()])
        .output()
    {
        Ok(value) => println!("OK {:?}", value),
        Err(value) => println!("ERR {}", value)
    }

    return PrinterResult {
        success   : true,
        exception : exceptions::PrinterException {
            base    : exceptions::PrinterExceptionBase::NoException,
            message : "".to_string()
        }
    };
}
