use std::env::args;

use std::fmt::{Display, Formatter};
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigratorError {
    IoError(String),
    ParseError { line: usize, message: String },
    InvalidKeyFormat(String),
    UnsupportedExtension(String),
}

impl Display for MigratorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MigratorError::IoError(err) => {
                write!(f, "IO Error: {}", err)
            }
            MigratorError::ParseError { line, message } => {
                write!(f, "Parse Error at line {}, {}", line, message)
            }
            MigratorError::InvalidKeyFormat(err) => {
                write!(f, "Invalid Key Format Error: {}", err)
            }
            MigratorError::UnsupportedExtension(err) => {
                write!(f, "Unsupported Extension Error: {}", err)
            }
        }
    }
}

impl std::error::Error for MigratorError {}

impl From<std::io::Error> for MigratorError {
    fn from(err: std::io::Error) -> Self {
        MigratorError::IoError(err.to_string())
    }
}
