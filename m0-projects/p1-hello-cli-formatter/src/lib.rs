use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    InvalidNumber,
    AgeOutOfRange,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::InvalidNumber => write!(f, "Invalid age format. Please enter digits only"),
            ValidationError::AgeOutOfRange => write!(f, "Age is out of realistic range (0-255)."),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AccessStatus {
    Granted,
    Restricted,
}

pub fn parse_age(input: &str) -> Result<u8, ValidationError> {

}

pub fn check_access(age: u8) -> AccessStatus {

}
