use super::exceptions;
use super::data;



#[derive(Clone, Debug)]
pub struct PrinterResult {
    pub success   : bool,
    pub exception : exceptions::PrinterException
}



pub fn print(_filename : String) -> PrinterResult {
    return PrinterResult {
        success   : false,
        exception : exceptions::PrinterException {
            base    : exceptions::PrinterExceptionBase::NoException,
            message : "Printing Reached".to_string(),
            range   : data::Range {
                filename : "<PANIC>".to_string(),
                start    : 0,
                end      : 0
            }
        }
    };
}
