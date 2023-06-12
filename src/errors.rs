use std::{error::Error, fmt::Display};
use crate::scanner::Position;

#[derive(Debug)]
pub enum ScriptError {
    FileIo(std::io::Error),
    ScannerError(ScannerError),
    ParserError(ParserError),
    AstPrinterError,
}

impl Display for ScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileIo(err) => err.fmt(f),
            Self::ScannerError(err) => err.fmt(f),
            Self::ParserError(err) => err.fmt(f),
            Self::AstPrinterError => write!(f, "Error printing AST"),
        }
    }
}

impl Error for ScriptError {}

impl From<std::io::Error> for ScriptError {
    fn from(value: std::io::Error) -> Self {
        Self::FileIo(value)
    }
}

impl From<ScannerError> for ScriptError {
    fn from(value: ScannerError) -> Self {
        Self::ScannerError(value)
    }
}

impl From<ParserError> for ScriptError {
    fn from(value: ParserError) -> Self {
        Self::ParserError(value)
    }
}







#[derive(Debug)]
pub enum ScannerError {
    UnexpectedToken(Position),
    NumberLiteralParsingError(Position),
    UnterminatedMultilineComment(Position),
}

impl Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken(position) => write!(f, "Unexpected token at {}", position),
            Self::NumberLiteralParsingError(position) => write!(f, "Error parsing number at {}", position),
            Self::UnterminatedMultilineComment(position) => write!(f, "Unterminated multiline comment at {}", position),
        }
    }
}





impl Error for ScannerError {}

#[derive(Debug)]
pub enum ParserError {
    Unexpected,
    Consume
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Placeholder")
    }
}

impl Error for ParserError {}