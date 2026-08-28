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



// #[cfg(test)]