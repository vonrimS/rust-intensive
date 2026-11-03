use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};

use naming_normalizer::{NormalizedPart, normalize_filename};
use p2_cad_format_analyzer::{ClassificationResult, classify_cad_file};

#[derive(Debug)]
pub struct FileReport {
    pub file_name: String,
    pub file_hash: String,
    pub naming_status: Result<NormalizedPart, String>,
    pub geometry_status: Option<ClassificationResult>,
    pub is_passed: bool,
}

fn compute_file_hash<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    Ok(hex::encode(hasher.finalize()))
}

pub fn process_file(path: &PathBuf, max_bed_x: f64, max_bed_y: f64) -> FileReport {
    let file_name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "unknown".to_string());

    let naming_result = normalize_filename(&file_name).map_err(|e| e.to_string());

    let mut is_passed = true;

    let file_hash = if naming_result.is_ok() {
        compute_file_hash(path).unwrap_or_else(|_| {
            is_passed = false;
            "HASH_ERROR".to_string()
        })
    } else {
        is_passed = false;
        "N/A".to_string()
    };

    let geometry_status = if naming_result.is_ok() {
        let geom = classify_cad_file(path, max_bed_x, max_bed_y);
        if let ClassificationResult::Passed { .. } = geom {
        } else {
            is_passed = false;
        }
        Some(geom)
    } else {
        None
    };

    if naming_result.is_err() {
        is_passed = false;
    }

    FileReport {
        file_name,
        file_hash,
        naming_status: naming_result,
        geometry_status,
        is_passed,
    }
}

fn move_file_to_target(file_path: &Path, target_dir: &Path, verbose: bool) {
    if let Some(file_name) = file_path.file_name() {
        let destination = target_dir.join(file_name);
        if let Err(e) = fs::rename(file_path, &destination) {
            eprintln!(
                "Warning: Failed to move file {:?} to {:?}: {}",
                file_path, destination, e
            );
        } else if verbose {
            println!("[+] Moved to {:?}", destination);
        }
    }
}


fn format_file_report(report: &FileReport, has_errors: &mut bool) -> String {
    let mut out = String::new();
    out.push_str(&format!("File: {}\n", report.file_name));
    out.push_str(&format!("  [#] SHA-256: {}\n", report.file_hash));

    match &report.naming_status {
        Ok(part) => {
            out.push_str(&format!(
                "  [✔] Naming: PASSED (Key: {})\n",
                part.normal_key()
            ));

            if let Some(geom) = &report.geometry_status {
                match geom {
                    ClassificationResult::Passed {
                        sheet_name,
                        cut_length,
                        width,
                        height,
                    } => {
                        out.push_str(&format!(
                            "  [✔] Geometry: PASSED (Sheet: {}, {:.1}x{:.1}mm, Cut: {:.2}mm)\n",
                            sheet_name, width, height, cut_length
                        ));
                    }
                    ClassificationResult::OutOfBound {
                        reason,
                        width,
                        height,
                    } => {
                        out.push_str(&format!(
                            "  [✖] Geometry: OUT OF BOUND ({:.1}x{:.1}mm): {}\n",
                            width, height, reason
                        ));
                        *has_errors = true;
                    }
                    ClassificationResult::Corrupted { reason, offset } => {
                        out.push_str(&format!(
                            "  [✖] Geometry: CORRUPTED (Offset: {}): {}\n",
                            offset, reason
                        ));
                        *has_errors = true;
                    }
                }
            }
        }
        Err(err) => {
            out.push_str(&format!("  [✖] Naming Gatekeeper Rejected: {}\n", err));
            *has_errors = true;
        }
    }
    out.push_str("----------------------------------------\n");
    out
}


pub fn run_batch(
    input_path: &PathBuf,
    output_path: Option<&PathBuf>,
    verbose: bool,
) -> Result<bool, String> {
    let max_bed_x = 10000.0;
    let max_bed_y = 7500.0;

    let base_dir = if input_path.is_dir() {
        input_path.clone()
    } else {
        input_path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."))
    };

    let passed_dir = base_dir.join("passed");
    let rejected_dir = base_dir.join("rejected");

    fs::create_dir_all(&passed_dir)
        .map_err(|e| format!("Failed to create 'passed' folder: {e}"))?;
    fs::create_dir_all(&rejected_dir)
        .map_err(|e| format!("Failed to create 'rejected' folder: {e}"))?;

    let mut files_to_process = Vec::new();
    
    if input_path.is_dir() {
        if verbose {
            println!("[+] Scanning directory: {:?}", input_path);
        }
        let entries = fs::read_dir(input_path)
            .map_err(|e| format!("Error reading directory {:?}: {}", input_path, e))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                files_to_process.push(path);
            }
        }
    } else if input_path.is_file() {
        files_to_process.push(input_path.clone());
    } else {
        return Err(format!("Input path does not exist: {:?}", input_path));
    }

    if files_to_process.is_empty() {
        println!("[-] No files found to process.");
        return Ok(false);
    }

    let mut has_errors = false;
    let mut output_content = String::from("=== CAD Batch Gatekeeper Report ===\n\n");

    for file_path in files_to_process {
        if verbose {
            println!("[+] Processing: {:?}", file_path);
        }
        let report = process_file(&file_path, max_bed_x, max_bed_y);

        let target_dir = if report.is_passed {
            &passed_dir
        } else {
            &rejected_dir
        };
      
        move_file_to_target(&file_path, target_dir, verbose);

        output_content.push_str(&format_file_report(&report, &mut has_errors));

    }

        if let Some(out_path) = output_path {
        fs::write(out_path, &output_content)
            .map_err(|e| format!("Error writing report to '{out_path:?}': {e}"))?;
        if verbose {
            println!("[+] Batch report saved to {:?}", out_path);
        }
    } else {
        print!("{output_content}");
    }

    Ok(has_errors)
}
