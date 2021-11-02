use super::exceptions;



#[derive(Clone, Debug)]
pub struct PrinterResult {
    pub success   : bool,
    pub exception : exceptions::PrinterException
}



pub fn print(filename : String) -> PrinterResult {
    panic!("printed");
}
