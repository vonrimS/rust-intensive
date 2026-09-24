use std::{
    fmt::{self}, fs::File, io::{self, BufRead, BufReader, Error},
};

#[derive(Debug)]
pub enum FileCounterError {
    IoError(Error),
    EmptyPath,
}

impl From<io::Error> for FileCounterError {
    fn from(value: io::Error) -> Self {
        FileCounterError::IoError(value)
    }
}

impl PartialEq for FileCounterError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FileCounterError::EmptyPath, FileCounterError::EmptyPath) => true,
            (FileCounterError::IoError(err1), FileCounterError::IoError(err2)) => {
                err1.kind() == err2.kind()
            }
            _ => false,
        }
    }
}

impl fmt::Display for FileCounterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileCounterError::EmptyPath => write!(f, "File path cannot be empty"),
            FileCounterError::IoError(err) => write!(f, "IO Error: {}", err),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct TextStats {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
    pub chars: usize,
}

impl TextStats {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn parse_line(line: &str, stats: &mut TextStats) {
    stats.lines += 1;
    stats.bytes += line.len();
    stats.words += line.split_whitespace().count();
    stats.chars += line.chars().count();    
}

pub fn count_stats(path: &str) -> Result<TextStats, FileCounterError> {
    if path.trim().is_empty() {
        return Err(FileCounterError::EmptyPath);
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut stats = TextStats::new();

    for line_result in reader.lines() {
        let line = line_result?;
        parse_line(&line, &mut stats);
    }

    Ok(stats)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line(){
        let mut stats = TextStats::new();
        let s = "A B C";
        parse_line(s, &mut stats);

        assert_eq!(stats.lines, 1);
        assert_eq!(stats.words, 3);
        assert_eq!(stats.bytes, 5);
    }

    #[test]
    fn test_empty_path() {
        let path = "   ";
        assert_eq!(count_stats(path), Err(FileCounterError::EmptyPath));
    }

    #[test]
    fn test_unexisting_file() {
        let path = "./tests/123.txt";
        let result = count_stats(path);
        assert!(matches!(result, Err(FileCounterError::IoError(_))));        
    }
}