use std::fmt::{self, Display};

/// Errors that can occur during CAD file analysis and parsing.
#[derive(Debug)]
pub enum CadAnalyzerError {
    /// Failded to read the file or perform I/O operations.
    IoError(String),
    /// File header is invalid, corrupted, or unsupported.
    InvalidHeader(String),
    /// Suntax or structural error encountered at a specific byte offset.
    ParseError { offset: usize, message: String },
    /// The specified CAD file format version is not supported.
    UnsupportedFormat(String),
}


impl Display for CadAnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CadAnalyzerError::IoError(msg) => write!(f, "I/O error: {msg}"),
            CadAnalyzerError::InvalidHeader(msg) => write!(f, "Invalid file header: {msg}"),
            CadAnalyzerError::ParseError { offset, message } => {
                write!(f, "Parse error at offset {offset}: {message}")
            }
            CadAnalyzerError::UnsupportedFormat(msg) => write!(f, "Unsupported format: {msg}"),
        }
    }
}

impl std::error::Error for CadAnalyzerError {}
