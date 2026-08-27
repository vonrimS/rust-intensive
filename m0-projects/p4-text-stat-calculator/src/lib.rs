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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_multiline_text(){
        let input = "Rust\r\nis\r\nfast\r\n";
        let stats = analyze(input);

        assert_eq!(stats.lines, 3);
        assert_eq!(stats.words, 3);
        assert_eq!(stats.chars, 16);
        assert_eq!(stats.bytes, 16);
        assert_eq!(stats.longest_word, Some("fast"));
    }

    #[test]
    fn test_empty_text() {
        let input = "";
        let stats = analyze(input);

        assert_eq!(stats.lines, 0);
        assert_eq!(stats.words, 0);
        assert_eq!(stats.chars, 0);
        assert_eq!(stats.bytes, 0);
        assert_eq!(stats.longest_word, None);

    }

    #[test]
    fn test_unicode_and_cyrillic() {
        let input = "Привет Rust 🦀";
        let stats = analyze(input);

        assert_eq!(stats.lines, 1);
        assert_eq!(stats.words, 3);
        assert_eq!(stats.chars, 13);
        assert_eq!(stats.bytes, 22);
        assert_eq!(stats.longest_word, Some("Привет"));
    }

    #[test]
    fn test_only_whitespaces() {
        let input = "   \r\n\t  ";
        let stats = analyze(input);
        
        assert_eq!(stats.lines, 2);
        assert_eq!(stats.words, 0);
        
        assert_eq!(stats.longest_word, None);
    }
}
