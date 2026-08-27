use p4_text_stat_calculator::{TextStats, analyze};
use std::io::{self, Read, Write};

fn main() {
    println!("Welcome to Text Stat Calculator!");
    println!("Type some message below (press Ctrl+D or Ctrl+Z to submit):");
    println!("> ");

    io::stdout().flush().unwrap();

    let input = read_input();

    let stats = analyze(&input);    
    print_stats(&stats);
}

fn read_input() -> String {
    let mut buffer = String::new();

    if io::stdin().read_to_string(&mut buffer).is_err() {
        eprintln!("[ERROR] Failed to read input from stdin.");
    }

    buffer
}

fn print_stats(stats: &TextStats) {
    println!("\n================ TEXT STATISTICS ================");
    println!("Lines:        {}", stats.lines);
    println!("Words:        {}", stats.words);
    println!("Characters:   {}", stats.chars);
    println!("Bytes:        {}", stats.bytes);
    
    match stats.longest_word {
        Some(word) => println!("Longest Word: {} ({} chars)", word, word.chars().count()),
        None => println!("Longest Word: N/A"),
    }
    println!("=================================================");
}
