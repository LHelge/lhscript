use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ScriptError {
    FileIo(std::io::Error),
}

impl Display for ScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileIo(err) => err.fmt(f),
        }
    }
}

impl Error for ScriptError {}

impl From<std::io::Error> for ScriptError {
    fn from(value: std::io::Error) -> Self {
        Self::FileIo(value)
    }
}
