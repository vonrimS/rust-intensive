use p3_number_guesser::{compare_guess, generate_secret_number, parse_guess};
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("Welcome to Guess the Number!");
    println!("I'm thinking of a number between 1 and 100.");

    let secret_number = generate_secret_number();
    let mut attempts = 0;

    loop {
        print!("\nPlease input your guess: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("[ERROR] Failed to read line");
            continue;
        }

        let guess = match parse_guess(&input) {
            Ok(num) => num,
            Err(err) => {
                println!("[WARNING] {}", err);
                continue;
            }
        };

        attempts += 1;

        match compare_guess(guess, secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("\n You win! You guessed the number in {} attempts!", attempts);
                break;
            }
        }
    }
}
