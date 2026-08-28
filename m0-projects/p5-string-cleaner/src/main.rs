use std::io::{self, Write};

use p5_string_cleaner::clean_string;



fn main() {
    println!("Welcome to String Cleaner!");
    println!("Enter text to sanitize: ");

    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = read_input();

    clean_string(&mut input);

    println!("\n=== CLEANED RESULT ===");
    println!("\"{input}\"");
    println!("======================")


}

fn read_input() -> String {
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line from stdin");

    buffer
}
