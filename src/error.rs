use std::{error::Error, fmt::Display};
use crate::scanner::ScannerError;

#[derive(Debug)]
pub enum ScriptError {
    FileIo(std::io::Error),
    ScannerError(ScannerError),
}

impl Display for ScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileIo(err) => err.fmt(f),
            Self::ScannerError(err) => err.fmt(f),
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