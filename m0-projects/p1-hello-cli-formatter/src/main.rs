use p1_hello_cli_formatter::{check_access, parse_age, AccessStatus};
use std::io::{self, Write};

fn main() {
    print!("Enter your name: ");

    io::stdout().flush().unwrap();
    
    let mut name = String::new();
    
    if io::stdin().read_line(&mut name).is_err() {
        eprint!("[ERROR] Failed to read name from stdin.");
        return;
    }
    
    let name = name.trim();
    
    print!("Enter your age: ");
    io::stdout().flush().unwrap();
    
    let mut age_input = String::new();

    if io::stdin().read_line(&mut age_input).is_err() {
        eprintln!("[ERROR] Failed to read age from stdin.");
        return;
    }

    match parse_age(&age_input) {
        Ok(age) => {
            let status = check_access(age);
            match status {
                AccessStatus::Granted => {
                    println!("[SUCCESS] Hello, {}! Access granted.", name);
                }
                AccessStatus::Restricted => {
                    println!("[RESTRICTED] Hello, {}! Access is restricted because age is under 18", name);
                }
            }
        }
        Err(err) => {
            eprintln!("[ERRRO] {}", err);
        }
    }


}
