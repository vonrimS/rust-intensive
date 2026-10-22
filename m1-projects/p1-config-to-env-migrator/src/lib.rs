use serde_json::Value;
use std::fmt::{Display, Formatter};

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

pub fn flatten_json(value: &serde_json::Value, prefix: &str, acc: &mut Vec<(String, String)>) {
    match value {
        Value::Object(map) => {
            for (key, val) in map {
                let new_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{prefix}.{key}")
                };
                flatten_json(val, &new_prefix, acc);
            }
        }

        Value::String(s) => {
            acc.push((to_upper_snake_case(prefix), s.clone()));
        }
        Value::Number(n) => {
            acc.push((to_upper_snake_case(prefix), n.to_string()));
        }
        Value::Bool(b) => {
            acc.push((to_upper_snake_case(prefix), b.to_string()));
        }
        Value::Null => {
            acc.push((to_upper_snake_case(prefix), String::new()));
        }

        Value::Array(_) => acc.push((to_upper_snake_case(prefix), value.to_string())),
    }
}

pub fn format_env_value(val: &str) -> String {
    let needs_quots = val.contains([' ', '\t', '\n', '\r', '#', '=', '$', '"', '\\']);

    if !needs_quots {
        return val.to_string();
    }

    let mut res = String::with_capacity(val.len() + 8);
    res.push('"');

    for c in val.chars() {
        match c {
            '\\' => res.push_str("\\\\"),
            '"' => res.push_str("\\\""),
            _ => res.push(c),
        }
    }

    res.push('"');
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_upper_snake_cases() {
        let test1 = "some-test-1";
        let test2 = "someTest_1";
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

    #[test]
    fn test_flatten_json() {
        let raw_json = r#"
        {
            "server": {
                "host": "127.0.0.1",
                "port": 8080
            },
            "database": {
                "connectionUrl": "postgres://localhost/db",
                "maxConnections": 20
            }
        }
        "#;

        let parsed: serde_json::Value = serde_json::from_str(raw_json).unwrap();
        let mut acc = Vec::new();

        flatten_json(&parsed, "", &mut acc);

        let mut expected = vec![
            (
                "DATABASE_CONNECTION_URL".to_string(),
                "postgres://localhost/db".to_string(),
            ),
            ("DATABASE_MAX_CONNECTIONS".to_string(), "20".to_string()),
            ("SERVER_HOST".to_string(), "127.0.0.1".to_string()),
            ("SERVER_PORT".to_string(), "8080".to_string()),
        ];

        expected.sort_by(|a, b| a.0.cmp(&b.0));

        assert_eq!(acc, expected);
    }

    #[test]
    fn test_format_env_value() {
        assert_eq!(format_env_value("hello"), String::from("hello"));
        assert_eq!(format_env_value("hello world!"), String::from("\"hello world!\""));
        assert_eq!(format_env_value("hello#world!"), String::from("\"hello#world!\""));
        assert_eq!(format_env_value("say \"hello world!\""), String::from("\"say \\\"hello world!\\\"\""));
    }
}
