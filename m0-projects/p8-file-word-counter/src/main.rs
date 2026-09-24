use std::env;

use p8_file_word_counter::{TextStats, count_stats};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <path>");
        return;
    }

    let path= &args[1];

    println!("=== File Word Counter ===");
    println!("Reading file: \"{}\"", path);

    match count_stats(path) {
        Ok(stats) => print_stats(stats),
        Err(err) => eprintln!("[!] Attempting to open missing file '{}':\n    Error: {}", path, err),
    }
}

fn print_stats(stats: TextStats) {
    println!("
[+] File processed successfully!\n
Statistics Summary:
- Total Lines:       {}
- Total Words:       {}
- Total Characters:  {}
- Total Bytes:       {} bytes",
    stats.lines, stats.words, stats.chars, stats.bytes);
}


