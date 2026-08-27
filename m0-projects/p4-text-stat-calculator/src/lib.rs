pub struct TextStats<'a> {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
    pub bytes: usize,
    pub longest_word: Option<&'a str>,
}

pub fn analyze<'a>(text: &'a str) -> TextStats<'a> {
    TextStats {
        lines: count_lines(text),
        words: count_words(text),
        chars: count_chars(text),
        bytes: count_bytes(text),
        longest_word: find_longest_word(text),
    }
}

fn count_lines(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }
    text.lines().count()
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn count_chars(text: &str) -> usize {
    text.chars().count()
}

fn count_bytes(text: &str) -> usize {
    text.len()
}

fn find_longest_word<'a>(text: &'a str) -> Option<&'a str> {
    text.split_whitespace().max_by_key(|w| w.chars().count())
}
