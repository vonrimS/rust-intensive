use std::{
    env, fs,
    path::PathBuf,
    process::{self},
};

use naming_normalizer::normalize_filename;

use p2_cad_format_analyzer::{ClassificationResult, classify_cad_file};

#[derive(Debug, PartialEq)]
enum OutputFormat {
    Text,
    Json,
}

/// CLI Configuration structure parsed from command-line arguments.
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

struct FileReport {
    file_name: String,
    naming_status: Result<naming_normalizer::NormalizedPart, String>,
    geometry_status: Option<ClassificationResult>,
}


fn process_file(path: &PathBuf, max_bed_x: f64, max_bed_y: f64) -> FileReport {
    let file_name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "unknown".to_string());

    let naming_result = match normalize_filename(&file_name) {
        Ok(part) => Ok(part),
        Err(err) => Err(err.to_string()),
    };

    let geometry_status = if naming_result.is_ok() {
        Some(classify_cad_file(path, max_bed_x, max_bed_y))
    } else {
        None
    };

    FileReport {
        file_name,
        naming_status: naming_result,
        geometry_status,
    }
}


fn main() {
    let cli = parse_args();

    let input_path = match cli.input_path {
        Some(path) => path,
        None => {
            eprintln!("Error: Missing required input path. Use -i <path> or pass it as an argument.");
            print_usage(&env::args().next().unwrap_or_default());
            process::exit(1);
        }
    };

    let max_bed_x = 10000.0;
    let max_bed_y = 7500.0;

    let mut files_to_process = Vec::new();

    if input_path.is_dir() {
        if cli.verbose {
            println!("[+] Scanning directory: {:?}", input_path);
        }
        match fs::read_dir(&input_path) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        files_to_process.push(path);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading directory {:?}: {}", input_path, e);
                process::exit(1);
            }
        }
    } else if input_path.is_file() {
        files_to_process.push(input_path);
    } else {
        eprintln!("Error: Input path does not exist: {:?}", input_path);
        process::exit(1);
    }

    if files_to_process.is_empty() {
        println!("[-] No files found to process.");
        process::exit(0);
    }

    let mut reports = Vec::new();
    for file_path in &files_to_process {
        if cli.verbose {
            println!("[+] Processing: {:?}", file_path);
        }
        let report = process_file(file_path, max_bed_x, max_bed_y);
        reports.push(report);
    }

    let mut output_content = String::from("=== CAD Batch Gatekeeper Report ===\n\n");
    let mut has_errors = false;

    for rep in reports {
        output_content.push_str(&format!("File: {}\n", rep.file_name));
        
        match rep.naming_status {
            Ok(part) => {
                output_content.push_str(&format!("  [✔] Naming: PASSED (Key: {})\n", part.normal_key()));
                
                if let Some(geom) = rep.geometry_status {
                    match geom {
                        ClassificationResult::Passed { sheet_name, cut_length, width, height } => {
                            output_content.push_str(&format!("  [✔] Geometry: PASSED (Sheet: {}, {:.1}x{:.1}mm, Cut: {:.2}mm)\n", sheet_name, width, height, cut_length));
                        }
                        ClassificationResult::OutOfBound { reason, width, height } => {
                            output_content.push_str(&format!("  [✖] Geometry: OUT OF BOUND ({:.1}x{:.1}mm): {}\n", width, height, reason));
                            has_errors = true;
                        }
                        ClassificationResult::Corrupted { reason, offset } => {
                            output_content.push_str(&format!("  [✖] Geometry: CORRUPTED (Offset: {}): {}\n", offset, reason));
                            has_errors = true;
                        }
                    }
                }
            }
            Err(err) => {
                output_content.push_str(&format!("  [✖] Naming Gatekeeper Rejected: {}\n", err));
                has_errors = true;
            }
        }
        output_content.push_str("----------------------------------------\n");
    }

    if let Some(out_path) = cli.output_path {
        if let Err(e) = fs::write(&out_path, &output_content) {
            eprintln!("Error writing report to '{out_path:?}': {e}");
            process::exit(1);
        }
        if cli.verbose {
            println!("[+] Batch report saved to {:?}", out_path);
        }
    } else {
        print!("{output_content}");
    }

    if has_errors {
        process::exit(1);
    } else {
        process::exit(0);
    }
}
