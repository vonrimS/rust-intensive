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
    entries: HashMap<String, String>
}

impl Config {
    pub fn new() -> Self {
        Config { 
            entries: HashMap::new(), 
        }
    }
    
   pub fn parse(text: &str) -> Result<Self, ConfigError> {
        todo!()
   }
   
   pub fn get(&self, key: &str) -> Option<&str> {
        todo!()
   }
   
   pub fn get_as<T: FromStr>(&self, key: &str) -> Result<T, ConfigError> {
        todo!()
   }
}

