use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};

use naming_normalizer::{NormalizedPart, normalize_filename};
use p2_cad_format_analyzer::{ClassificationResult, classify_cad_file};

use crate::db::Database;

/// Comprehensive inspection verdict for a single CAD file.
///
/// Combines everything from naming and hash checks to geometry validation,
/// along with a direct `is_passed` flag for fast folder sorting.
#[derive(Debug)]
pub struct FileReport {
    /// File name (e.g., `CICAR_backet_st3_2.0mm.dxf`).
    pub file_name: String,
    /// SHA-256 hash of the file. Set to `"N/A"` if the file fails the initial naming check.
    pub file_hash: String,
    /// Result from the Naming Gatekeeper.
    pub naming_status: Result<NormalizedPart, String>,
    /// Result from the Geometry analysis (populated if the naming check passes).
    pub geometry_status: Option<ClassificationResult>,
    /// Master flag: did the file pass every single check in the pipeline?
    pub is_passed: bool,
}

/// Computes the SHA-256 hash of a file using a streaming approach.
///
/// Reads the file in small 8 KB buffers to avoid memory spikes,
/// even when processing large CAD blueprints of hundreds of megabytes.
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

/// Runs a single CAD file through the validation pipeline.
///
/// First, the name is checked. If the name format is invalid, we skip calculating
/// the hash and parsing the geometry to save CPU and disk resources — the file is rejected immediately.
pub fn process_file(path: &PathBuf, max_bed_x: f64, max_bed_y: f64) -> FileReport {
    // Extract file name safely, falling back to "unknown" if missing
    let file_name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "unknown".to_string());

    // 1. Naming Gatekeeper — primary filter
    let naming_result = normalize_filename(&file_name).map_err(|e| e.to_string());
    let mut is_passed = naming_result.is_ok();

    // 2. Compute hash only for surviving files
    let file_hash = if naming_result.is_ok() {
        compute_file_hash(path).unwrap_or_else(|_| {
            is_passed = false;
            "HASH_ERROR".to_string()
        })
    } else {
        is_passed = false;
        "N/A".to_string()
    };

    // 3. Geometry Gatekeeper — checks dimensions and contour integrity
    let geometry_status = if naming_result.is_ok() {
        let geom = classify_cad_file(path, max_bed_x, max_bed_y);
        if !matches!(geom, ClassificationResult::Passed { .. }) {
            is_passed = false; // OutOfBound or Corrupted automatically flags the file as rejected
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

/// Safely moves a file to its target directory (`passed` or `rejected`).
fn move_file_to_target(file_path: &Path, target_dir: &Path, verbose: bool) {
    if let Some(file_name) = file_path.file_name() {
        let destination = target_dir.join(file_name);
        if let Err(e) = fs::rename(file_path, &destination) {
            // If the move fails (e.g. cross-device link limits), log a warning
            eprintln!(
                "Warning: Failed to move file {:?} to {:?}: {}",
                file_path, destination, e
            );
        } else if verbose {
            println!("[+] Moved to {:?}", destination);
        }
    }
}

/// Formats a human-readable text report block for a single file
/// and flips the `has_errors` flag if any issues were detected.
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

/// Entry point for batch file or directory processing.
///
/// Scans the path, creates `passed/` and `rejected/` subfolders,
/// sorts incoming files accordingly, and generates an aggregated report.
///
/// Returns `Ok(true)` if errors were found (for exit code 1),
/// or `Ok(false)` if absolutely all files passed successfully.
pub fn run_batch(
    input_path: &PathBuf,
    output_path: Option<&PathBuf>,
    verbose: bool,
) -> Result<bool, String> {
    // Default laser bed dimensions (in mm)
    let max_bed_x = 10000.0;
    let max_bed_y = 7500.0;

    let db_path = PathBuf::from("gatekeeper.db");
    let db = Database::init(&db_path)
        .map_err(|e| format!("Failed to initialize SQLite database: {e}"))?;

    if verbose {
        println!("[+] SQLite database initialized at {:?}", db_path);
    }

    // Determine base directory to place auxiliary folders relative to
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

    // Ensure target directories exist on disk
    fs::create_dir_all(&passed_dir)
        .map_err(|e| format!("Failed to create 'passed' folder: {e}"))?;
    fs::create_dir_all(&rejected_dir)
        .map_err(|e| format!("Failed to create 'rejected' folder: {e}"))?;

    // Collect files to process
    let mut files_to_process = Vec::new();

    if input_path.is_dir() {
        if verbose {
            println!("[+] Scanning directory: {:?}", input_path);
        }
        let entries = fs::read_dir(input_path)
            .map_err(|e| format!("Error reading directory {:?}: {}", input_path, e))?;

        for entry in entries.flatten() {
            let path = entry.path();
            // Filter out directories so we only process files
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

    // Main pipeline loop
    for file_path in files_to_process {
        if verbose {
            println!("[+] Processing: {:?}", file_path);
        }
        let report = process_file(&file_path, max_bed_x, max_bed_y);

        // Sort file into the appropriate directory based on the verdict
        let target_dir = if report.is_passed {
            &passed_dir
        } else {
            &rejected_dir
        };

        move_file_to_target(&file_path, target_dir, verbose);

        let verdict = if report.is_passed {
            "PASSED"
        } else {
            "REJECTED"
        };

        let (width_opt, height_opt, cut_len_opt, error_msg) = match &report.naming_status {
            Ok(_) => {
                if let Some(geom) = &report.geometry_status {
                    match geom {
                        ClassificationResult::Passed {
                            width,
                            height,
                            cut_length,
                            ..
                        } => (Some(*width), Some(*height), Some(*cut_length), None),
                        ClassificationResult::OutOfBound {
                            width,
                            height,
                            reason,
                        } => (Some(*width), Some(*height), None, Some(reason.clone())),
                        ClassificationResult::Corrupted { reason, .. } => {
                            (None, None, None, Some(reason.clone()))
                        }
                    }
                } else {
                    (None, None, None, None)
                }
            }
            Err(err) => (None, None, None, Some(err.clone())),
        };

        if let Err(e) = db.log_inspection(
            &report.file_name,
            &report.file_hash,
            verdict,
            width_opt,
            height_opt,
            cut_len_opt,
            None,
            error_msg.as_deref(),
        ) {
            eprintln!(
                "Warning: Failed to log inspection into SQLite for {}: {}",
                report.file_name, e
            );
        }

        // Accumulate text report
        output_content.push_str(&format_file_report(&report, &mut has_errors));
    }

    // Save report to file or print to stdout
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    /// Verifies that the hash is computed accurately and has the correct hex-string length (64 chars)
    #[test]
    fn test_compute_file_hash() {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_hash_sample.txt");

        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello CAD Gatekeeper!").unwrap();

        let hash_result = compute_file_hash(&file_path);
        assert!(hash_result.is_ok());
        let hash = hash_result.unwrap();

        assert_eq!(hash.len(), 64);

        let _ = fs::remove_file(file_path);
    }

    /// Verifies that an invalid file name immediately rejects the file and bypasses hashing
    #[test]
    fn test_process_file_invalid_naming() {
        let temp_dir = std::env::temp_dir();
        let invalid_path = temp_dir.join("bad_file_name.dxf");

        let mut file = File::create(&invalid_path).unwrap();
        writeln!(file, "dummy content").unwrap();

        let report = process_file(&invalid_path, 10000.0, 7500.0);

        assert!(!report.is_passed);
        assert!(report.naming_status.is_err());
        assert_eq!(report.file_hash, "N/A");

        let _ = fs::remove_file(invalid_path);
    }

    /// Verifies report formatting and error flag behavior
    #[test]
    fn test_format_file_report_error_flag() {
        let report = FileReport {
            file_name: "test_part.dxf".to_string(),
            file_hash: "N/A".to_string(),
            naming_status: Err("Invalid name format".to_string()),
            geometry_status: None,
            is_passed: false,
        };

        let mut has_errors = false;
        let formatted = format_file_report(&report, &mut has_errors);

        assert!(has_errors);
        assert!(formatted.contains("[✖] Naming Gatekeeper Rejected"));
    }
}
