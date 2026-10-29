use serde_json::Value;
use std::fmt::{Display, Formatter};
use std::path::Path;

/// Represents errors that can occur during the configuration migration process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigratorError {
    /// Input/Output error (e.g., file not found or permission denied).
    IoError(String),
    /// JSON parsing error containing the line number and error message.
    ParseError { line: usize, message: String },
    /// Invalid configuration key format.
    InvalidKeyFormat(String),
    /// Unsupported input file extension.
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

/// Automatic conversion from standard `std::io::Error` to `MigratorError`.
/// Enables seamless use of the `?` operator when performing file I/O operations.
impl From<std::io::Error> for MigratorError {
    fn from(err: std::io::Error) -> Self {
        MigratorError::IoError(err.to_string())
    }
}

/// Converts an arbitrary string (kebab-case, camelCase, dot.notation)
/// into `UPPER_SNAKE_CASE` format.
///
/// # Optimization
/// Allocates memory up front using `String::with_capacity` matching the exact length
/// of the input string to avoid redundant heap reallocations during processing.
///
/// # Examples
/// ```rust
/// use config_to_env_migrator::to_upper_snake_case;
///
/// assert_eq!(to_upper_snake_case("server.maxConnections"), "SERVER_MAX_CONECTIONS");
/// assert_eq!(to_upper_snake_case("app-name"), "APP_NAME");
/// ```
pub fn to_upper_snake_case(input: &str) -> String {
    // Pre-allocate buffer matching input length to eliminate reallocation overhead
    let mut result = String::with_capacity(input.len());
    let mut prev: Option<char> = None;

    for c in input.chars() {
        match c {
            // Convert delimiters into underscores while preventing consecutive duplicates
            '-' | '.' | ' ' | '_' => {
                if !result.is_empty() && !result.ends_with('_') {
                    result.push('_');
                }
            }
            // Handle camelCase transitions: insert an underscore before uppercase letters
            // if preceded by a lowercase letter or digit
            c if c.is_uppercase() => {
                if let Some(p) = prev {
                    if (p.is_lowercase() || p.is_numeric()) && !result.ends_with('_') {
                        result.push('_');
                    }
                }
                result.push(c);
            }
            // Convert all remaining characters to uppercase ASCII
            _ => {
                result.push(c.to_ascii_uppercase());
            }
        }

        prev = Some(c);
    }

    // Strip leading and trailing underscores if any were introduced
    result.trim_matches('_').to_string()
}

/// Recursively flattens a nested JSON tree structure into a vector of key-value string pairs.
///
/// # Arguments
/// * `value` - eference to the current `serde_json::Value` node.
/// * `prefix` - Current key path formatted in dot notation (e.g., `"server.db"`).
/// * `acc` - Mutable accumulator vector collecting resulting `(Key, Value)` tuples.
pub fn flatten_json(value: &serde_json::Value, prefix: &str, acc: &mut Vec<(String, String)>) {
    match value {
        // For object nodes, recursively traverse nested levels building the dot-prefixed path
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

        // Terminal scalar nodes: transform prefix to UPPER_SNAKE_CASE and store value as string
        Value::String(s) => {
            acc.push((to_upper_snake_case(prefix), s.clone()));
        }
        Value::Number(n) => {
            acc.push((to_upper_snake_case(prefix), n.to_string()));
        }
        Value::Bool(b) => {
            acc.push((to_upper_snake_case(prefix), b.to_string()));
        }
        // Serialize array nodes into a compact JSON string representation
        Value::Null => {
            acc.push((to_upper_snake_case(prefix), String::new()));
        }

        Value::Array(_) => acc.push((to_upper_snake_case(prefix), value.to_string())),
    }
}

/// Escapes an environment variable value in compliance with standard `.env` specs.
///
/// Wraps values containing special characters (` `, `#`, `=`, `"`, etc.) in double quotes,
/// escaping inner quotes `"` and backslashes `\`.
///
/// # Performance
/// Performs **exactly 1 allocation** using a single-pass character iterator without intermediate strings.
pub fn format_env_value(val: &str) -> String {
    // Fast path: check if surrounding quotes are required
    let needs_quots = val.contains([' ', '\t', '\n', '\r', '#', '=', '$', '"', '\\']);

    if !needs_quots {
        return val.to_string();
    }

    // Reserve capacity with padding for quotes and escape backslashes
    let mut res = String::with_capacity(val.len() + 8);
    res.push('"');

    // Process characters in a single iteration pass
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

/// Formats a slice of key-value pairs into final `.env` file content.
///
/// # Optimization
/// Allocates a single output string buffer assuming ~40 bytes per line to avoid
/// memory reallocations and unnecessary `format!` macro invocations in loop.
pub fn format_env_file(pairs: &[(String, String)]) -> String {
    let mut res = String::with_capacity(pairs.len() * 40);

    for (key, val) in pairs {
        let formatted_val = format_env_value(val);
        res.push_str(key);
        res.push('=');
        res.push_str(&formatted_val);
        res.push('\n');
    }

    res
}

/// Primary public facade entry point for the library.
///
/// Executes the complete migration pipeline:
/// 1. Reads raw file content from disk.
/// 2. Parses JSON document structure.
/// 3. Flattens keys and normalizes them into `UPPER_SNAKE_CASE`.
/// 4. Sorts key-value pairs lexicographically for deterministic output</char>
/// 5. Formats and returns the generated `.env` string.
///
/// # Errors
/// Returns [`MigratorError`] if file I/O fails or JSON content is malformed.
pub fn migrate_config(input_path: &Path) -> Result<String, MigratorError> {
    let content = std::fs::read_to_string(input_path)?;

    let parsed: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| MigratorError::ParseError {
            line: e.line(),
            message: e.to_string(),
        })?;

    let mut acc = Vec::new();
    flatten_json(&parsed, "", &mut acc);

    acc.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(format_env_file(&acc))
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
        assert_eq!(
            format_env_value("hello world!"),
            String::from("\"hello world!\"")
        );
        assert_eq!(
            format_env_value("hello#world!"),
            String::from("\"hello#world!\"")
        );
        assert_eq!(
            format_env_value("say \"hello world!\""),
            String::from("\"say \\\"hello world!\\\"\"")
        );
    }

    #[test]
    fn test_format_env_file() {
        let pairs = vec![
            (
                "DATABASE_URL".to_string(),
                "postgres://localhost/db".to_string(),
            ),
            ("SERVER_PORT".to_string(), "8080".to_string()),
            ("SECRET_KEY".to_string(), "my #secret key".to_string()),
        ];

        let expected = "\
DATABASE_URL=postgres://localhost/db
SERVER_PORT=8080
SECRET_KEY=\"my #secret key\"
";

        assert_eq!(format_env_file(&pairs), expected);
    }

    #[test]
    fn test_migrate_config() {
        use std::io::Write;

        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        write!(
            temp_file,
            r#"{{
                "app": {{
                    "name": "My App",
                    "port": 3000
                }}
            }}"#
        )
        .unwrap();

        let result = migrate_config(temp_file.path()).unwrap();

        let expected = "\
APP_NAME=\"My App\"
APP_PORT=3000
";

        assert_eq!(result, expected);
    }
}
