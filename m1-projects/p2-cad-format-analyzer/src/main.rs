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

fn main() {
    let cli = parse_args();

    let input_path = match cli.input_path {
        Some(path) => path,
        None => {
            eprintln!(
                "Error: Missing required input file. Use -i <file> or pass it as an argument."
            );
            print_usage(&env::args().next().unwrap_or_default());
            process::exit(1);
        }
    };

    if cli.verbose {
        println!("[+] Analyzing CAD file: {:?}", input_path);
    }

    
    let file_name = input_path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "unknown.dxf".to_string());

    let normalized_info = match normalize_filename(&file_name) {
        Ok(part) => {
            if cli.verbose {
                println!("[+] Filename structure verified successfully.");
                println!("    Canonical Key: {}", part.normal_key());
            }
            Some(part)
        }
        Err(err) => {
            if cli.verbose {
                println!("[!] Naming gatekeeper warning: {}", err);
            }
            None
        }
    };

    let max_bed_x = 10000.0;
    let max_bed_y = 7500.0;

    let classification = classify_cad_file(&input_path, max_bed_x, max_bed_y);


    let output_content = match cli.format {
        OutputFormat::Text => {
            let naming_str = match &normalized_info {
                Some(part) => format!(
                    " [PASSED] Canonical Key: {}\n  - Company: {}\n  - Project: {}\n  - Material: {}\n  - Thickness: {:.1} mm",
                    part.normal_key(), part.company, part.project, part.metal_grade, part.thickness
                ),
                None => " [FAILED] Invalid format (Expected: Company_Project_Material_Thickness)".to_string(),
            };

            let status_str = match &classification {
                ClassificationResult::Passed {
                    sheet_name,
                    cut_length,
                    width,
                    height,
                } => {
                    format!(
                        " [PASSED] Ready for production\n  - Assigned Sheet: {}\n  - Part Dimensions: {:.1} x {:.1} mm\n  - Total Cut Length: {:.2} mm",
                        sheet_name, width, height, cut_length
                    )
                }
                ClassificationResult::OutOfBound {
                    reason,
                    width,
                    height,
                } => {
                    format!(
                        " [OUT OF BOUND] Exceeds machine bed limits ({max_bed_x}x{max_bed_y}mm)\n  - Part Dimensions: {:.1} x {:.1} mm\n  - Reason: {}",
                        width, height, reason
                    )
                }
                ClassificationResult::Corrupted { reason, offset } => {
                    format!(
                        " [CORRUPTED] File rejected by gatekeeper\n  - Offset: {}\n  - Reason: {}",
                        offset, reason
                    )
                }
            };

            format!(
                "=== CAD Format Analyzer Report ===\n\n\
                 [+] Inspected file: {}\n\
                 [+] Naming Gatekeeper: {}\n\
                 [+] Gatekeeper Verdict:{}\n\n",
                file_name, naming_str, status_str
            )
        }

        OutputFormat::Json => {
            let (status_name, details_json) = match &classification {
                ClassificationResult::Passed {
                    sheet_name,
                    cut_length,
                    width,
                    height,
                } => (
                    "PASSED",
                    format!(
                        "\"sheet_name\": \"{}\", \"width\": {:.2}, \"height\": {:.2}, \"cut_length_mm\": {:.2}",
                        sheet_name, width, height, cut_length
                    ),
                ),
                ClassificationResult::OutOfBound {
                    reason,
                    width,
                    height,
                } => (
                    "OUT_OF_BOUND",
                    format!(
                        "\"width\": {:.2}, \"height\": {:.2}, \"reason\": \"{}\"",
                        width, height, reason
                    ),
                ),
                ClassificationResult::Corrupted { reason, offset } => (
                    "CORRUPTED",
                    format!("\"offset\": {}, \"reason\": \"{}\"", offset, reason),
                ),
            };

            format!(
                "{{\n\
                \t\"file_name\": \"{}\",\n\
                \t\"status\": \"{}\",\n\
                \t{}\n\
                }}",
                file_name, status_name, details_json
            )
        }
    };

    // Запись в файл или печать в stdout
    if let Some(out_path) = cli.output_path {
        if let Err(e) = fs::write(&out_path, &output_content) {
            eprintln!("Error writing to file '{out_path:?}': {e}");
            process::exit(1);
        }
        if cli.verbose {
            println!("[+] Report successfully saved to {:?}", out_path);
        }
    } else {
        print!("{output_content}");
    }

    // Возвращаем код выхода в зависимости от статуса (полезно для CI/скриптов цеха)
    match classification {
        ClassificationResult::Passed { .. } => process::exit(0),
        ClassificationResult::OutOfBound { .. } => process::exit(2),
        ClassificationResult::Corrupted { .. } => process::exit(1),
    }
}
