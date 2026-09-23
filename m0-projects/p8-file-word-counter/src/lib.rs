use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Error},
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

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct TextStats {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
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
