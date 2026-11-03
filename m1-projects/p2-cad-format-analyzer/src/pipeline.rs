use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};

use naming_normalizer::{NormalizedPart, normalize_filename};
use p2_cad_format_analyzer::{ClassificationResult, classify_cad_file};

use crate::db::{Database, MachineRecord};

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


/// Runs a single CAD file through the validation pipeline, matching geometry
/// against all available active machines stored in the database.
pub fn process_file_with_machines(path: &PathBuf, machines: &[MachineRecord]) -> FileReport {
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

    // 3. Geometry Gatekeeper — routing across active machine beds
    let geometry_status = if naming_result.is_ok() {
        let mut matched_geom = None;

        // Try routing to the first machine bed that fits the part
        for machine in machines {
            let geom = classify_cad_file(path, machine.max_x, machine.max_y);
            if matches!(geom, ClassificationResult::Passed { .. }) {
                // Override sheet name with the actual machine identifier
                if let ClassificationResult::Passed {
                    cut_length,
                    width,
                    height,
                    ..
                } = geom
                {
                    matched_geom = Some(ClassificationResult::Passed {
                        sheet_name: machine.name.clone(),
                        cut_length,
                        width,
                        height,
                    });
                }
                break;
            } else if matched_geom.is_none() {
                // Save first non-passing result as fallback (e.g., Corrupted or OutOfBound)
                matched_geom = Some(geom);
            }
        }

        let geom = matched_geom.unwrap_or_else(|| ClassificationResult::OutOfBound {
            reason: "Part dimensions exceed all available active machine beds".to_string(),
            width: 0.0,
            height: 0.0,
        });

        if !matches!(geom, ClassificationResult::Passed { .. }) {
            is_passed = false;
        }

        Some(geom)
    } else {
        None
    };

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
fn format_file_report(
    report: &FileReport,
    estimated_cost: Option<f64>,
    has_errors: &mut bool,
) -> String {
    let mut out = String::new();
    out.push_str(&format!("File: {}\n", report.file_name));
    out.push_str(&format!("  [#] SHA-256: {}\n", report.file_hash));

    match &report.naming_status {
        Ok(part) => {
            out.push_str(&format!(
                "  [✔] Naming: PASSED (Key: {}, Material: {}, Thickness: {}mm)\n",
                part.normal_key(),
                part.metal_grade,
                part.thickness
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
                            "  [✔] Geometry: PASSED (Machine: {}, {:.1}x{:.1}mm, Cut: {:.2}mm)\n",
                            sheet_name, width, height, cut_length
                        ));
                        if let Some(cost) = estimated_cost {
                            out.push_str(&format!("  [$] Estimated Cost: {:.2} EUR\n", cost));
                        } else {
                            out.push_str("  [$] Estimated Cost: N/A (Tariff not found)\n");
                        }
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
pub fn run_batch(
    input_path: &PathBuf,
    output_path: Option<&PathBuf>,
    verbose: bool,
) -> Result<bool, String> {
    let db_path = PathBuf::from("gatekeeper.db");
    let db = Database::init(&db_path)
        .map_err(|e| format!("Failed to initialize SQLite database: {e}"))?;

    if verbose {
        println!("[+] SQLite database initialized at {:?}", db_path);
    }

    // Load active machines from SQLite database
    let machines = db
        .get_active_machines()
        .map_err(|e| format!("Failed to fetch active machines from DB: {e}"))?;

    if machines.is_empty() {
        return Err("No active machines found in database.".to_string());
    }

    if verbose {
        println!("[+] Loaded {} active machine(s) for routing", machines.len());
    }

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
        let report = process_file_with_machines(&file_path, &machines);

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

        let mut estimated_cost = None;
        let mut width_opt = None;
        let mut height_opt = None;
        let mut cut_len_opt = None;
        let mut error_msg = None;

        match &report.naming_status {
            Ok(part) => {
                if let Some(geom) = &report.geometry_status {
                    match geom {
                        ClassificationResult::Passed {
                            width,
                            height,
                            cut_length,
                            ..
                        } => {
                            width_opt = Some(*width);
                            height_opt = Some(*height);
                            cut_len_opt = Some(*cut_length);

                            // Calculate commercial Quick Quote estimate
                            if let Ok(Some(price_per_meter)) =
                                db.get_cutting_tariff(&part.metal_grade, part.thickness)
                            {
                                estimated_cost = Some((*cut_length / 1000.0) * price_per_meter);
                            }
                        }
                        ClassificationResult::OutOfBound {
                            width,
                            height,
                            reason,
                        } => {
                            width_opt = Some(*width);
                            height_opt = Some(*height);
                            error_msg = Some(reason.clone());
                        }
                        ClassificationResult::Corrupted { reason, .. } => {
                            error_msg = Some(reason.clone());
                        }
                    }
                }
            }
            Err(err) => {
                error_msg = Some(err.clone());
            }
        }

        // Log inspection event into SQLite database
        if let Err(e) = db.log_inspection(
            &report.file_name,
            &report.file_hash,
            verdict,
            width_opt,
            height_opt,
            cut_len_opt,
            estimated_cost,
            error_msg.as_deref(),
        ) {
            eprintln!(
                "Warning: Failed to log inspection into SQLite for {}: {}",
                report.file_name, e
            );
        }

        output_content.push_str(&format_file_report(
            &report,
            estimated_cost,
            &mut has_errors,
        ));
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


#[cfg(test)]
mod tests {
    use crate::db::MachineRecord;

use super::*;
    use std::io::Write;

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

    #[test]
    fn test_process_file_invalid_naming() {
        let temp_dir = std::env::temp_dir();
        let invalid_path = temp_dir.join("bad_file_name.dxf");

        let mut file = File::create(&invalid_path).unwrap();
        writeln!(file, "dummy content").unwrap();

        let machines = vec![MachineRecord {
            id: 1,
            name: "Test-Laser".to_string(),
            max_x: 3000.0,
            max_y: 1500.0,
        }];

        let report = process_file_with_machines(&invalid_path, &machines);

        assert!(!report.is_passed);
        assert!(report.naming_status.is_err());
        assert_eq!(report.file_hash, "N/A");

        let _ = fs::remove_file(invalid_path);
    }

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
        let formatted = format_file_report(&report, None, &mut has_errors);

        assert!(has_errors);
        assert!(formatted.contains("[✖] Naming Gatekeeper Rejected"));
    }
}