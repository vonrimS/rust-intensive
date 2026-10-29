use std::{env, fs, path::PathBuf, process};

use p1_config_to_env_migrator::migrate_config;

/// CLI Configuration structure parsed from command-line arguments.
struct CLiArgs {
    input_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
    dry_run: bool,
}

fn parse_args() -> CLiArgs {
    let args: Vec<String> = env::args().collect();
    let mut input_path = None;
    let mut output_path = None;
    let mut dry_run = false;

    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_usage(&args[0]);
                process::exit(0);
            }
            "-i" | "--input" => {
                if i + 1 < args.len() {
                    input_path = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                } else {
                    eprintln!("Error: Missing value for option '{}'", args[i]);
                    process::exit(1);
                }
            }
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    output_path = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                } else {
                    eprintln!("Error: Missing value for option '{}'", args[i]);
                    process::exit(1);
                }
            }
            "--dry-run" => {
                dry_run = true;
            }

            // Support positional argument fallback if no -i is specified
            arg if !arg.starts_with('-') && input_path.is_none() => {
                input_path = Some(PathBuf::from(arg));
            }

            unknown => {
                eprintln!("Error: Unknown argument '{unknown}'");
                print_usage(&args[0]);
                process::exit(1);
            }
        }

        i += 1;
    }

    CLiArgs {
        input_path,
        output_path,
        dry_run,
    }
}

fn print_usage(exe_name: &str) {
    eprintln!("Usage: {exe_name} <input_json_path> [output_env_path]");
    eprintln!("\nOptions:");
    eprintln!(" -i, --input <FILE>      Path to input JSON configuration file");
    eprintln!(" -o, --output <FILE>     Path to output .env file");
    eprintln!("     --dry-run           Print output to stdout without writing to file");
    eprintln!(" -h, --help              Print help information");
}

fn main() {
    let cli = parse_args();

    let input_path = match cli.input_path {
        Some(path) => path,
        None => {
            eprintln!("Error: Missing required argument '-i <input_json>'");
            print_usage(&env::args().next().unwrap_or_default());
            process::exit(1);
        }
    };

    match migrate_config(&input_path) {
        Ok(env_content) => {
            // If dry-run or no output path specified, print to stdout
            if cli.dry_run || cli.output_path.is_none() {
                print!("{env_content}");
            } else if let Some(out_path) = cli.output_path {
                if let Err(e) = fs::write(&out_path, &env_content) {
                    eprintln!("Error writing to file '{out_path:?}': {e}");
                    process::exit(1);
                }
                println!("Successfully migrated '{input_path:?}' -> '{out_path:?}'");
            }
        }
        Err(err) => {
            eprintln!("Migration failed: {err}");
            process::exit(1);
        }
    }
}
