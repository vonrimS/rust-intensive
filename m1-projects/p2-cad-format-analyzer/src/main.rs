use std::{
    env, fs, path::PathBuf, process::{self},
};

use p2_cad_format_analyzer::parse_cad_file;

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

    match parse_cad_file(&input_path) {
        Ok(report) => {
            let output_content = match cli.format {
                OutputFormat::Text => {

                    let metrics_str = format!(
                        "  - Header Variables: {}\n  - Defined Layers:   {}\n  - Total Entities:   {}\n  - Block Records:    {}",
                        report.header_count,
                        report.layers.len(),
                        report.entities.lines,
                        report.block_count
                    );

                    let bbox_str = match report.bounding_box {
                        Some(bb) => format! (
                            "  - Min: (X: {:.2}, Y: {:.2}, Z: {:.2})\n  - Max: (X: {:.2}, Y: {:.2}, Z: {:.2})",
                            bb.min.x, bb.min.y, bb.min.z,
                            bb.max.x, bb.max.y, bb.max.z
                        ),
                        None => "  - None (No geometry)".to_string(),
                    };

                    format!(
                        "=== CAD Format Analyzer Report ===\n\n\
                         [+] Analyzing input file: {}\n\
                         [+] Detected format: AutoCAD DXF (ASCII text)\n\
                         [+] Version: {}\n\n\
                         Summary Metrics:\n\
                         {}\n\n\
                         Bounding Box:\n\
                         {}\n\nAnalysis completed successfully in {:?}.\n",
                        report.file_name,
                        report.format_version,
                        metrics_str,
                        bbox_str,
                        report.analysis_duration
                    )
                }
                
                OutputFormat::Json => {
                    let bbox_json = match &report.bounding_box {
                        Some(bb) => format!(
                            "{{\"min\": {{\"x\": {}, \"y\": {}, \"z\": {}}}, \"max\": {{\"x\": {}, \"y\": {}, \"z\": {}}}}}",
                            bb.min.x, bb.min.y, bb.min.z, bb.max.x, bb.max.y, bb.max.z
                        ),
                        None => "null".to_string(),
                    };

                    format!(
                        "{{\n\
                        \t\"file_name\": \"{}\",\n\
                        \t\"format_version\": \"{}\",\n\
                        \t\"header_count\": {},\n\
                        \t\"defined_layers_count\": {},\n\
                        \t\"lines_count\": {},\n\
                        \t\"block_count\": {},\n\
                        \t\"bounding_box\": {},\n\
                        \t\"analysis_duration_ms\": {:.4}\n\
                        }}",
                        report.file_name,
                        report.format_version,
                        report.header_count,
                        report.layers.len(),
                        report.entities.lines,
                        report.block_count,
                        bbox_json,
                        report.analysis_duration.as_secs_f64() * 1000.0
                    )
                }
            };

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
        }
        Err(err) => {
            eprintln!("Analysis failed: {err}");
            process::exit(1);
        }
    }
}
