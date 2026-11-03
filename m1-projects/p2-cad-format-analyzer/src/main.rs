mod pipeline;

use std::{
    env,
    path::PathBuf,
    process::{self},
};

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum OutputFormat {
    Text,
    Json,
}

/// CLI Configuration structure parsed from command-line arguments.
#[allow(dead_code)]
struct CLiArgs {
    input_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
    format: OutputFormat,
    verbose: bool,
}

fn print_usage(exe_name: &str) {
    eprintln!("Usage: {exe_name} <input_path> [options]");
    eprintln!("\nOptions:");
    eprintln!(" -i, --input <FILE>      Path to input CAD file (DXF/STEP)");
    eprintln!(" -o, --output <FILE>     Path to output report file (optional)");
    eprintln!(" -f, --format <FORMAT>   Output format: text or json (default: text)");
    eprintln!(" -v, --verbose           Enable verbose diagnostic logging");
    eprintln!(" -h, --help              Print help information");
}

fn parse_args() -> CLiArgs {
    let args: Vec<String> = env::args().collect();
    let mut input_path = None;
    let mut output_path = None;
    let mut format = OutputFormat::Text;
    let mut verbose = false;

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
            "-f" | "--format" => {
                if i + 1 < args.len() {
                    match args[i + 1].to_lowercase().as_str() {
                        "text" => format = OutputFormat::Text,
                        "json" => format = OutputFormat::Json,
                        other => {
                            eprintln!(
                                "Error: Unknown output format '{other}'. Supported: text, json"
                            );
                            process::exit(1);
                        }
                    }
                    i += 1;
                } else {
                    eprintln!("Error: Missing value for option: '{}'", args[i]);
                    process::exit(1);
                }
            }
            "-v" | "--verbose" => {
                verbose = true;
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
        format,
        verbose,
    }
}

fn main() {
    let cli = parse_args();

    let input_path = match cli.input_path {
        Some(path) => path,
        None => {
            eprintln!(
                "Error: Missing required input path. Use -i <path> or pass it as an argument."
            );
            print_usage(&env::args().next().unwrap_or_default());
            process::exit(1);
        }
    };

    match pipeline::run_batch(&input_path, cli.output_path.as_ref(), cli.verbose) {
        Ok(has_errors) => {
            if has_errors {
                process::exit(1);
            } else {
                process::exit(0);
            }
        }
        Err(err_msg) => {
            eprintln!("Error: {}", err_msg);
            process::exit(1);
        }
    }
}
