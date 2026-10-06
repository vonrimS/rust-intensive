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

pub fn to_upper_snake_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut prev: Option<char> = None;

    for c in input.chars() {
        match c {
            '-' | '.' | ' ' | '_' => {
                if !result.is_empty() && !result.ends_with('_') {
                    result.push('_');
                }
            }

            c if c.is_uppercase() => {
                if let Some(p) = prev {
                    if (p.is_lowercase() || p.is_numeric()) && !result.ends_with('_') {
                        result.push('_');
                    }
                }
                result.push(c);
            }

            _ => {
                result.push(c.to_ascii_uppercase());
            }
        }

        prev = Some(c);
    }

    result.trim_matches('_').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_upper_snake_cases() {
        let test1 = "some-test-1";
        let test2 = "someTest1";
        let test3 = "some__Test__1";
        let test4 = "_some__Test__1";
        let test5 = "_SOME_TEST_1_";

        let res = "SOME_TEST_1".to_string();

        assert_eq!(to_upper_snake_case(test1), res);
        assert_eq!(to_upper_snake_case(test2), res);
        assert_eq!(to_upper_snake_case(test3), res);
        assert_eq!(to_upper_snake_case(test4), res);
        assert_eq!(to_upper_snake_case(test5), res);
    }
}
