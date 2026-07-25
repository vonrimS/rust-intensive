use std::{env::args};
use crate::error::ConfigParserError;

pub struct Config {
    pub input_path: String,
    pub output_path: String,
}

pub fn parse_args() -> Result<Config, ConfigParserError> {
    // skip the app's name
    let mut args = args().skip(1);

    let mut input_path = None;
    let mut output_path = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-i" | "--input" => {
                match args.next() {
                    Some(val) => input_path = Some(val),
                    None => {
                        return Err(ConfigParserError::InvalidArguments(
                            "Flag --input requires a file path".to_string()
                        ));
                    }
                }
            }
            "-o" | "--output" => {
                match args.next() {
                    Some(val) => output_path = Some(val),
                    None => {
                        return Err(ConfigParserError::InvalidArguments(
                            "Flag --output requires a file path".to_string()
                        ));
                    }
                }
            }
            unknown => {
                return Err(ConfigParserError::InvalidArguments(
                    format!("Unknown argument: {}", unknown)
                ));
            }
        }
    }

    let input_path = match input_path {
        Some(path) => path,
        None => {
            return Err(ConfigParserError::InvalidArguments(
                "Missing required argument: --input".to_string(),
            ));
        }
    };

    let output_path = output_path.unwrap_or_else(|| "./env".to_string());

    Ok(Config {
        input_path,
        output_path
    })
}