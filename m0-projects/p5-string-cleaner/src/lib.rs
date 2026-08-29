pub fn clean_string(input: &mut String) {
    remove_control_chars(input);
    collapse_spaces(input);
    trim_in_place(input);
}

pub fn trim_in_place(input: &mut String) {
    let trimmed = input.trim();
    let start_bytes = trimmed.as_ptr() as usize - input.as_ptr() as usize;
    let trimmed_len = trimmed.len();

    input.drain(..start_bytes);
    input.truncate(trimmed_len);
}

fn remove_control_chars(input: &mut String) {
    input.retain(|ch| ch != '\t' && ch != '\r' && ch != '\n');
}

fn collapse_spaces(input: &mut String) {
    let mut seen_space = false;

    input.retain(|ch| {
        if ch == ' '{
            if seen_space {
                false
            } else {
                seen_space = true;
                true
            }
        } else {
            seen_space = false;
            true
        }
    });
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_in_place(){
        let mut input = String::from("  abc xyz  ");
        trim_in_place(&mut input);
        assert_eq!(input, String::from("abc xyz"));
    }

    #[test]
    fn test_collapse_spaces(){
        let mut input = String::from("abc  xyz");
        collapse_spaces(&mut input);
        assert_eq!(input, String::from("abc xyz"));
    }

    #[test]
    fn test_remove_chontrol_chars(){
        let mut input = String::from("abc \n\r\txyz");
        remove_control_chars(&mut input);
        assert_eq!(input, String::from("abc xyz"));
    }

    #[test]
    fn test_clean_string_full_pipeline(){
        let mut input = String::from("  abc\n\r  \t  xyz  ");
        clean_string(&mut input);
        assert_eq!(input, String::from("abc xyz"));
    }

}