use core::fmt;


#[derive(Debug, PartialEq)]
pub enum ConversionError {
    InvalidMode,
    InvalidNumber,
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversionError::InvalidMode => {
                write!(f, "Invalid mode selection. Please enter 1 or 2.")
            }
            ConversionError::InvalidNumber => {
                write!(f, "Invalid temperature value. Please enter a valid number.")
            }
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum ConversionMode {
    CelsiusToFahrenheit,
    FahrenheitToCelsius,
}

impl ConversionMode {
    pub fn from_input(input: &str) -> Result<Self, ConversionError> {
        match input.trim() {
            "1" => Ok(ConversionMode::CelsiusToFahrenheit),
            "2" => Ok(ConversionMode::FahrenheitToCelsius),
            _ => Err(ConversionError::InvalidMode)
        }
    }
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

pub fn farenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

pub fn parse_temperature(input: &str) -> Result<f64, ConversionError>{
    input
        .trim()
        .parse::<f64>()
        .map_err(|_| ConversionError::InvalidNumber)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
    } 

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(farenheit_to_celsius(32.0), 0.0);
        assert_eq!(farenheit_to_celsius(212.0), 100.0);
        assert_eq!(farenheit_to_celsius(-40.0), -40.0);
    }

    #[test]
    fn test_mode_parsing() {
        assert_eq!(ConversionMode::from_input("1\n"), Ok(ConversionMode::CelsiusToFahrenheit));
        assert_eq!(ConversionMode::from_input("2\n"), Ok(ConversionMode::FahrenheitToCelsius));
        assert_eq!(ConversionMode::from_input("10\n"), Err(ConversionError::InvalidMode));
    }
}
