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