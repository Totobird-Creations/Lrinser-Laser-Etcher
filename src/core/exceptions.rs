use std::fmt;
use colored::*;

use super::data;



#[derive(Clone, Debug)]
pub struct LexerException {
    pub base    : LexerExceptionBase,
    pub message : String,
    pub range   : data::Range
}
impl fmt::Display for LexerException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_as_string : String;
        base_as_string = match &self.base {
            LexerExceptionBase::IllegalCharacterException => "IllegalCharacterException",
            LexerExceptionBase::EscapeException           => "EscapeException",
            LexerExceptionBase::EndException              => "EndException",
            LexerExceptionBase::NoException               => "NoException"
        }.to_string();

        let mut exc = "".to_string();
        exc += format!("{}: {}", base_as_string.red().bold(), self.message.red()).as_str();

        write!(f, "{}", exc)
    }
}
#[derive(Clone, Debug)]
pub enum LexerExceptionBase {
    NoException,

    IllegalCharacterException,
    EscapeException,
    EndException
}



#[derive(Clone, Debug)]
pub struct ParserException {
    pub base    : ParserExceptionBase,
    pub message : String,
    pub range   : data::Range
}
impl fmt::Display for ParserException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_as_string : String;
        base_as_string = match &self.base {
            ParserExceptionBase::IllegalTokenException => "IllegalTokenException",
            ParserExceptionBase::MissingTokenException => "MissingTokenException",
            ParserExceptionBase::NoException           => "NoException"
        }.to_string();

        let mut exc = "".to_string();
        exc += format!("{}: {}", base_as_string.red().bold(), self.message.red()).as_str();

        write!(f, "{}", exc)
    }
}
#[derive(Clone, Debug)]
pub enum ParserExceptionBase {
    NoException,

    IllegalTokenException,
    MissingTokenException
}



#[derive(Clone, Debug)]
pub struct InterpreterException {
    pub base    : InterpreterExceptionBase,
    pub message : String,
    pub range   : data::Range
}
impl fmt::Display for InterpreterException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_as_string : String;
        base_as_string = match &self.base {
            InterpreterExceptionBase::InvalidValueException          => "InvalidValueException",
            InterpreterExceptionBase::HeaderAlreadyAccessedException => "HeaderAlreadyAccessedException"
        }.to_string();

        let mut exc = "".to_string();
        exc += format!("{}: {}", base_as_string.red().bold(), self.message.red()).as_str();

        write!(f, "{}", exc)
    }
}
#[derive(Clone, Debug)]
pub enum InterpreterExceptionBase {
    InvalidValueException,
    HeaderAlreadyAccessedException
}



#[derive(Clone, Debug)]
pub struct RendererException {
    pub base    : RendererExceptionBase,
    pub message : String,
    pub range   : data::Range
}
impl fmt::Display for RendererException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_as_string : String;
        base_as_string = match &self.base {
            RendererExceptionBase::NoException => "NoException"
        }.to_string();

        let mut exc = "".to_string();
        exc += format!("{}: {}", base_as_string.red().bold(), self.message.red()).as_str();

        write!(f, "{}", exc)
    }
}
#[derive(Clone, Debug)]
pub enum RendererExceptionBase {
    NoException
}



#[derive(Clone, Debug)]
pub struct PrinterException {
    pub base    : PrinterExceptionBase,
    pub message : String,
    pub range   : data::Range
}
impl fmt::Display for PrinterException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_as_string : String;
        base_as_string = match &self.base {
            PrinterExceptionBase::NoException => "NoException"
        }.to_string();

        let mut exc = "".to_string();
        exc += format!("{}: {}", base_as_string.red().bold(), self.message.red()).as_str();

        write!(f, "{}", exc)
    }
}
#[derive(Clone, Debug)]
pub enum PrinterExceptionBase {
    NoException
}
