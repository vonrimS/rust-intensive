use crate::error::ConfigParserError;


pub enum ParsedLine<'a> {
    EmptyOrComment,
    Section(&'a str),
    Entry { key: &'a str, value: &'a str },
}

pub fn parse_line<'a>(line: &'a str, line_number: usize) -> Result<ParsedLine<'a>, ConfigParserError> {
    let trimmed= line.trim();


    if trimmed.is_empty() || trimmed.starts_with(";") || trimmed.starts_with("#") {
        return Ok(ParsedLine::EmptyOrComment);
    }

    if trimmed.starts_with('[') {
        if trimmed.ends_with(']') {
            let section_name = trimmed[1..trimmed.len() - 1].trim();

            if section_name.is_empty() {
                return Err(ConfigParserError::InvalidSyntax { 
                    line_number, 
                    message: "Empty section name inside []".to_string(), 
                });
            }

            return Ok(ParsedLine::Section(section_name));
        } else {
            return Err(ConfigParserError::InvalidSyntax { 
                line_number, 
                message:  "Missing closing bracket ']' in section header".to_string()
            });
        }
    }

    if let Some((raw_key, raw_value)) = trimmed.split_once('=') {
        let key = raw_key.trim();
        let value = raw_value.trim();

        if key.is_empty() {
            return Err(ConfigParserError::InvalidSyntax { 
                line_number, 
                message: "Key cannot be empty".to_string() 
            });
        }

        Ok(ParsedLine::Entry { key, value })
    } else {
        Err(ConfigParserError::InvalidSyntax { 
            line_number, 
            message: "Expected 'KEY=VALUE' or section header '[SECTION]'".to_string() 
        })
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_comments_and_empty_lines() {
        assert!(matches!(
            parse_line("", 1),
            Ok(ParsedLine::EmptyOrComment)
        ));
        assert!(matches!(
            parse_line("   ", 1),
            Ok(ParsedLine::EmptyOrComment)
        ));
        assert!(matches!(
            parse_line("; this is a comment", 1),
            Ok(ParsedLine::EmptyOrComment)
        ));
        assert!(matches!(
            parse_line(" # another comment", 1),
            Ok(ParsedLine::EmptyOrComment)
        ));
    }

    #[test]
    fn test_parse_valid_section() {
        let res = parse_line("  [DATABASE]  ", 1).unwrap();
        if let ParsedLine::Section(name) = res {
            assert_eq!(name, "DATABASE");
        } else {
            panic!("Expected ParsedLine::Section");
        }
    }

    #[test]
    fn test_parse_invalid_section() {
        let res = parse_line("[DATABASE", 1);
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_valid_entry() {
        let res = parse_line("  HOST  =   localhost ", 1).unwrap();
        if let ParsedLine::Entry { key, value } = res {
            assert_eq!(key, "HOST");
            assert_eq!(value, "localhost");
        } else {
            panic!("Expected ParsedLine::Entry");
        }
    }

    #[test]
    fn test_parse_entry_with_extra_equals() {
        // Проверяем, что split_once корректно разделяет по первому '='
        let res = parse_line("URL = https://site.com?a=1&b=2", 1).unwrap();
        if let ParsedLine::Entry { key, value } = res {
            assert_eq!(key, "URL");
            assert_eq!(value, "https://site.com?a=1&b=2");
        } else {
            panic!("Expected ParsedLine::Entry");
        }
    }
}