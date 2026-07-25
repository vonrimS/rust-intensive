use std::fmt;
use std::io;

#[derive(Debug)]
enum ConfigParserError {
    Io(io::Error),
    InvalidSyntax { 
        line_number: usize, 
        message: String},
    InvalidArguments(String),
}

impl fmt::Display for ConfigParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigParserError::Io(err) => {
                write!(f, "I/O error occurred: {}", err)
            }
            ConfigParserError::InvalidSyntax { line_number, message } => {
                write!(f, "Syntax error on line {}: {}", line_number, message)
            }
            ConfigParserError::InvalidArguments(msg) => {
                write!(f, "CLI Argument error: {}", msg)
            }
        }
    }
}

impl std::error::Error for ConfigParserError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigParserError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for ConfigParserError {
    fn from(err: io::Error) -> Self {
        ConfigParserError::Io(err)
    }
}

