use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Clone)]
pub enum ConfigError {
    InvalidLine(String),
    EmptyKey(String),
    ParseValueError {
        key: String,
        expected_type: &'static str,
    },
    NotFound(String),
}

pub struct Config {
    entries: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            entries: HashMap::new(),
        }
    }

    // Parse a configuration string line-by-line, stripping whitespace and skipping empty lines or comments (starting with `#`)
    pub fn parse(text: &str) -> Result<Self, ConfigError> {
        let mut config = Config::new();
            
        for line in text.lines() {
            // Strip leading and trailing whitespace and tabs from the line
            let trimmed = line.trim();

            // Ignore empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with("#") {
                continue;
            }

            // Split the line at the FIRST '=' character
            let (raw_key, raw_value) = trimmed
                .split_once('=')
                .ok_or_else(|| ConfigError::InvalidLine(trimmed.to_string()))?;

            // Strip surrounding whitespace from both key and value
            let key = raw_key.trim();
            let value = raw_value.trim();

            // Ensure the key is not empty
            if key.is_empty() {
                return Err(ConfigError::EmptyKey(trimmed.to_string()));
            }

            // Store in the HashMap (converting &str into owned String)
            config.entries.insert(key.to_string(), value.to_string());

        }

        Ok(config)
    }

    // Retrieve a reference to a raw string value by key
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(|s| s.as_str())
    }

    // Safely fetch and convert a stored string value into a target type `T`
    pub fn get_as<T: FromStr>(&self, key: &str) -> Result<T, ConfigError> {
        let raw_val= self.get(key).ok_or_else(|| ConfigError::NotFound(key.to_string()))?;

        raw_val.parse::<T>().map_err(|_| ConfigError::ParseValueError { 
            key: key.to_string(), 
            expected_type: std::any::type_name::<T>(),
        })
    }
}
