use rand;
use core::fmt;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum GuessErrors{
    NotANumber,
    OutOfTheRange,
}

impl fmt::Display for GuessErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuessErrors::NotANumber => write!(f, "Please enter a valid number!"),
            GuessErrors::OutOfTheRange => write!(f, "Number must be between 1 and 100!"),
        }
    }
}

pub fn generate_secret_number() -> u32 {
    rand::random_range(1..=100)
}

pub fn parse_guess(input: &str) -> Result<u32, GuessErrors> {
    let num = input
        .trim()
        .parse::<u32>()
        .map_err(|_| GuessErrors::NotANumber)?;

    if !(1..=100).contains(&num) {
        return Err(GuessErrors::OutOfTheRange);
    }

    Ok(num)
}


pub fn compare_guess(guess: u32, secret: u32) -> Ordering{
    guess.cmp(&secret)
}


#[cfg(test)]

mod tests {
    use super::*;


    #[test]
    fn test_parse_guess_valid_number() {
        assert_eq!(parse_guess(" 52 \n"), Ok(52));
    }

    #[test]
    fn test_parse_guess_invalid_number() {
        assert_eq!(parse_guess("52a"), Err(GuessErrors::NotANumber));
    }

    #[test]
    fn test_parse_guess_out_of_range() {
        assert_eq!(parse_guess("101"), Err(GuessErrors::OutOfTheRange));
    }

    #[test]
    fn test_compare_guess() {
        assert_eq!(compare_guess(1, 5), Ordering::Less);
        assert_eq!(compare_guess(5, 1), Ordering::Greater);
        assert_eq!(compare_guess(5, 5), Ordering::Equal);
    }
}

