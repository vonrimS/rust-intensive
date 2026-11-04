use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};

use naming_normalizer::{NormalizedPart, normalize_filename};
use p2_cad_format_analyzer::{ClassificationResult, classify_cad_file};

use crate::db::{self, Database, MachineRecord};

/// Comprehensive inspection verdict for a single CAD file.
#[derive(Debug)]
pub struct FileReport {
    pub file_name: String,
    pub file_hash: String,
    pub naming_status: Result<NormalizedPart, String>,
    pub geometry_status: Option<ClassificationResult>,
    pub is_passed: bool,
}

impl FileReport {
    /// Extracts normalized scalar fields for database logging and JSON reporting.
    pub fn extract_metrics(&self, db: &Database) -> ExtractedMetrics {
        let mut metrics = ExtractedMetrics::default();

        match &self.naming_status {
            Ok(part) => {
                metrics.normal_key = Some(part.normal_key());
                metrics.material = Some(part.metal_grade.clone());
                metrics.thickness_mm = Some(part.thickness);

                if let Some(geom) = &self.geometry_status {
                    match geom {
                        ClassificationResult::Passed {
                            sheet_name,
                            width,
                            height,
                            cut_length,
                        } => {
                            metrics.width_mm = Some(round_to_2_decimals(*width));
                            metrics.height_mm = Some(round_to_2_decimals(*height));
                            metrics.cut_length_mm = Some(round_to_2_decimals(*cut_length));
                            metrics.machine_name = Some(sheet_name.clone());

                            if let Ok(Some(price_per_meter)) =
                                db.get_cutting_tariff(&part.metal_grade, part.thickness)
                            {
                                let cost = (*cut_length / 1000.0) * price_per_meter;
                                metrics.estimated_cost_eur = Some(round_to_2_decimals(cost));
                            }
                        }
                        ClassificationResult::OutOfBound {
                            width,
                            height,
                            reason,
                        } => {
                            metrics.width_mm = Some(round_to_2_decimals(*width));
                            metrics.height_mm = Some(round_to_2_decimals(*height));
                            metrics.error_reason = Some(reason.clone());
                        }
                        ClassificationResult::Corrupted { reason, .. } => {
                            metrics.error_reason = Some(reason.clone());
                        }
                    }
                }
            }
            Err(err) => {
                metrics.error_reason = Some(err.clone());
            }
        }

        metrics
    }
}

#[derive(Debug, Default)]
pub struct ExtractedMetrics {
    pub normal_key: Option<String>,
    pub material: Option<String>,
    pub thickness_mm: Option<f64>,
    pub machine_name: Option<String>,
    pub width_mm: Option<f64>,
    pub height_mm: Option<f64>,
    pub cut_length_mm: Option<f64>,
    pub estimated_cost_eur: Option<f64>,
    pub error_reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct JsonFileItem {
    pub file_name: String,
    pub file_hash: String,
    pub verdict: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thickness_mm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_mm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_mm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cut_length_mm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_cost_eur: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<String>,
}

impl JsonFileItem {
    pub fn from_report(report: &FileReport, metrics: &ExtractedMetrics) -> Self {
        Self {
            file_name: report.file_name.clone(),
            file_hash: report.file_hash.clone(),
            verdict: if report.is_passed {
                "PASSED"
            } else {
                "REJECTED"
            }
            .to_string(),
            normal_key: metrics.normal_key.clone(),
            material: metrics.material.clone(),
            thickness_mm: metrics.thickness_mm,
            machine_name: metrics.machine_name.clone(),
            width_mm: metrics.width_mm,
            height_mm: metrics.height_mm,
            cut_length_mm: metrics.cut_length_mm,
            estimated_cost_eur: metrics.estimated_cost_eur,
            error_reason: metrics.error_reason.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct JsonBatchReport {
    pub total_files: usize,
    pub passed_count: usize,
    pub rejected_count: usize,
    pub execution_time_ms: u128,
    pub files: Vec<JsonFileItem>,
}

/// Computes the SHA-256 hash of a file using a streaming approach.
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
pub fn process_file_with_machines(path: &PathBuf, machines: &[MachineRecord]) -> FileReport {
    let file_name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "unknown".to_string());

    let naming_result = normalize_filename(&file_name).map_err(|e| e.to_string());
    let mut is_passed = naming_result.is_ok();

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
        let mut matched_geom = None;

        for machine in machines {
            let geom = classify_cad_file(path, machine.max_x, machine.max_y);
            if matches!(geom, ClassificationResult::Passed { .. }) {
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
            eprintln!(
                "Warning: Failed to move file {:?} to {:?}: {}",
                file_path, destination, e
            );
        } else if verbose {
            println!("[+] Moved to {:?}", destination);
        }
    }
}

/// Formats a human-readable text report block for a single file.
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

#[inline]
fn round_to_2_decimals(val: f64) -> f64 {
    (val * 100.0).round() / 100.0
}

/// Entry point for batch file or directory processing.
pub fn run_batch(
    input_path: &PathBuf,
    output_path: Option<&PathBuf>,
    is_json_format: bool,
    verbose: bool,
) -> Result<bool, String> {
    let start_time = std::time::Instant::now();

    let db_path = PathBuf::from("gatekeeper.db");
    let db = Database::init(&db_path)
        .map_err(|e| format!("Failed to initialize SQLite database: {e}"))?;

    if verbose {
        println!("[+] SQLite database initialized at {:?}", db_path);
    }

    let machines = db
        .get_active_machines()
        .map_err(|e| format!("Failed to fetch active machines from DB: {e}"))?;

    if machines.is_empty() {
        return Err("No active machines found in database.".to_string());
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
    let mut json_items = Vec::with_capacity(files_to_process.len());
    let mut output_text_content = String::from("=== CAD Batch Gatekeeper Report ===\n\n");

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

        // Извлекаем все вычисленные метрики за один проход
        let metrics = report.extract_metrics(&db);

        // Логирование в SQLite
        let verdict = if report.is_passed {
            "PASSED"
        } else {
            "REJECTED"
        };
        if let Err(e) = db.log_inspection(db::InspectionParams {
            file_name: &report.file_name,
            file_hash: &report.file_hash,
            verdict,
            width: metrics.width_mm,
            height: metrics.height_mm,
            cut_length: metrics.cut_length_mm,
            estimated_cost: metrics.estimated_cost_eur,
            error_reason: metrics.error_reason.as_deref(),
        }) {
            eprintln!(
                "Warning: Failed to log inspection into SQLite for {}: {}",
                report.file_name, e
            );
        }

        // Текстовый отчет
        output_text_content.push_str(&format_file_report(
            &report,
            metrics.estimated_cost_eur,
            &mut has_errors,
        ));

        // JSON отчет
        json_items.push(JsonFileItem::from_report(&report, &metrics));
    }

    let elapsed = start_time.elapsed();
    let elapsed_ms = elapsed.as_millis();

    let final_output = if is_json_format {
        let batch_report = JsonBatchReport {
            total_files: json_items.len(),
            passed_count: json_items.iter().filter(|i| i.verdict == "PASSED").count(),
            rejected_count: json_items
                .iter()
                .filter(|i| i.verdict == "REJECTED")
                .count(),
            execution_time_ms: elapsed_ms,
            files: json_items,
        };
        serde_json::to_string_pretty(&batch_report)
            .map_err(|e| format!("Failed to serialize JSON report: {e}"))?
    } else {
        let mut text_report = output_text_content;
        text_report.push_str(&format!("\n[+] Total execution time: {:.2?}ms\n", elapsed_ms));
        text_report
    };

    if let Some(out_path) = output_path {
        fs::write(out_path, &final_output)
            .map_err(|e| format!("Error writing report to '{out_path:?}': {e}"))?;
        if verbose {
            println!("[+] Batch report saved to {:?}", out_path);
        }
    } else {
        print!("{final_output}");
    }

    Ok(has_errors)
}
