use p7_simple_config_parser::{Config, ConfigError};
use std::io::{self, Write};

fn main() {
    println!("=== Simple Config Parser ===\n");

    // Demonstraition on predefined configuration text
    let sample_config = r#"
        # Server settings
        host = 127.0.0.1
        port = 8080
        debug = true

        # Performance
        max_connections = 100
        "#;

    println!("Parsing sample configuration...");
    match Config::parse(sample_config) {
        Ok(config) => {
            println!("[+] Parsed successfully!\n");

            let host: Option<&str> = config.get("host");
            let port: Result<usize, ConfigError> = config.get_as("port");
            let debug: Result<bool, ConfigError> = config.get_as("debug");
            let max_conn: Result<usize, ConfigError> = config.get_as("max_connections");

            println!("=== Stored Values ===");
            println!("Host (str): {:?}", host);
            println!("Port (usize): {:?}", port);
            println!("Debug (bool): {:?}", debug);
            println!("Max connections (usize): {:?}", max_conn);
            println!();

            // Demonstrate error handling
            println!("--- Error Handling Checks ---");
            let missing: Result<String, ConfigError> = config.get_as("timeout");
            println!("Missing key 'timeout': {:?}", missing);

            let invalid_type: Result<usize, ConfigError> = config.get_as("host");
            println!("Invalid type cast 'host' -> usize: {:?}", invalid_type);
        }
        Err(err) => println!("[!] Failed to parse sample config: {:?}", err),
    }

    // Interactive CLI input mode
    println!("\n=== Interactive Parsing Mode ===");
    println!("Enter a KEY=VALUE line (or type 'exit' to quit):");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("[!] Failed to read input.");
            break;
        }

        let trimmed = input.trim();
        if trimmed.eq_ignore_ascii_case("exit") {
            println!("Exiting...");
            break;
        }

        if trimmed.is_empty() {
            continue;
        }

        match Config::parse(trimmed) {
            Ok(config) => {
                if let Some(val) = config.get(trimmed.split_once('=').unwrap_or(("", "")).0.trim())
                {
                    println!("[+] Successfully stored! Raw value: '{}'", val);
                } else {
                    println!("[+] Config parsed into empty map.");
                }
            }
            Err(err) => println!("[!] Parse Error: {:?}", err),
        }
    }
}
