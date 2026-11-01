use std::{
    fmt::{self, Display},
    path::Path,
};

/// Errors that can occur during CAD file analysis and parsing.
#[derive(Debug)]
pub enum CadAnalyzerError {
    /// Failded to read the file or perform I/O operations.
    IoError(String),
    /// File header is invalid, corrupted, or unsupported.
    InvalidHeader(String),
    /// Suntax or structural error encountered at a specific byte offset.
    ParseError { offset: usize, message: String },
    /// The specified CAD file format version is not supported.
    UnsupportedFormat(String),
}

impl Display for CadAnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CadAnalyzerError::IoError(msg) => write!(f, "I/O error: {msg}"),
            CadAnalyzerError::InvalidHeader(msg) => write!(f, "Invalid file header: {msg}"),
            CadAnalyzerError::ParseError { offset, message } => {
                write!(f, "Parse error at offset {offset}: {message}")
            }
            CadAnalyzerError::UnsupportedFormat(msg) => write!(f, "Unsupported format: {msg}"),
        }
    }
}

impl std::error::Error for CadAnalyzerError {}

/// Represents a point in 3D space with floating-point coordinates.
#[derive(Debug)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Represents a 3D axis-aligned bounding box enclosing CAD geometry.
#[derive(Debug)]
pub struct BoundingBox {
    /// Minimun coordinates boundary (lower-left-back corner).
    pub min: Point3D,
    /// Maximum coordinates boundary (upper-right-front corner).
    pub max: Point3D,
}

/// Summary statistics counting the types of graphical entities in a CAD drawing.
#[derive(Debug)]
pub struct EntitySummary {
    /// Number of straight line segments.
    pub lines: usize,
    /// Number of arc segments.
    pub arcs: usize,
    /// Number of full circles.
    pub circles: usize,
    /// Number of text blocks and labels.
    pub texts: usize,
    /// Number of other or unclassified entities.
    pub other: usize,
}

/// Complete structural analysis report generated for a parsed CAD file.
#[derive(Debug)]
pub struct CadReport {
    /// Name of the inspected CAD file.
    pub file_name: String,
    /// Format specification version (e.g., AutoCAD DXF version).
    pub format_version: String,
    /// Collection of unique layer names defined in the drawing.
    pub layers: Vec<String>,
    /// Aggregated count of graphical entities.
    pub entities: EntitySummary,
    /// Overall spatial bounding box of the drawing, if geometry is present.
    pub bounding_box: Option<BoundingBox>,
}

/// Helper to accumulate spatial bounds during entity traversal.
#[derive(Debug)]
struct BoundingBoxAccumulator {
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
    has_geometry: bool,
}

impl BoundingBoxAccumulator {
    fn new() -> Self {
        Self {
            min_x: f64::MAX,
            min_y: f64::MAX,
            max_x: f64::MIN,
            max_y: f64::MIN,
            has_geometry: false,
        }
    }

    /// Updates bounding box bounds based on DXF group code and value.
    fn update(&mut self, code: &str, value: &str) {
        if let Ok(val) = value.parse::<f64>() {
            match code {
                "10" | "11" => {
                    self.min_x = self.min_x.min(val);
                    self.max_x = self.max_x.max(val);
                    self.has_geometry = true;
                }
                "20" | "21" => {
                    self.min_y = self.min_y.min(val);
                    self.max_y = self.max_y.max(val);
                    self.has_geometry = true;
                }
                _ => {}
            }
        }
    }

    /// Finalizes the accumulation into an optional BoundingBox.
    fn finalize(self) -> Option<BoundingBox> {
        if self.has_geometry {
            Some(BoundingBox {
                min: Point3D {
                    x: self.min_x,
                    y: self.min_y,
                    z: 0.0,
                },
                max: Point3D {
                    x: self.max_x,
                    y: self.max_y,
                    z: 0.0,
                },
            })
        } else {
            None
        }
    }
}

/// Detects the CAD file format and version from its header content.
/// Scans through the first sections of the DXF file looking for the `SECTION` marker
/// and the `$ACADVER` system variable to extract the exact AutoCAD version code.
fn detect_format(content: &str) -> Result<String, CadAnalyzerError> {
    let mut has_section = false;
    let mut version_code = String::new();

    let mut lines = content.lines();

    // Iterate through lines in pairs (group code and value)
    while let Some(code_line) = lines.next() {
        let value_line = lines.next().unwrap_or("").trim();
        let code = code_line.trim();

        // 1. Check for valid DXF section start
        if code == "0" && value_line == "SECTION" {
            has_section = true;
        }

        // 2. Look for the AutoCAD version variable
        if code == "9" && value_line == "$ACADVER" {
            let next_code = lines.next();
            let next_value = lines.next();

            // The version string is expected right after code '9' -> '$ACADVER'
            if let (Some(nc), Some(nv)) = (next_code, next_value)
                && nc.trim() == "1"
            {
                version_code = nv.trim().to_string();
            }
        }
    }

    // 3. Return the evaluated format report or fallback/error
    if has_section && !version_code.is_empty() {
        let readable_version = map_acad_version(&version_code);
        Ok(format!(
            "DXF (ASCII) - {} ({})",
            version_code, readable_version
        ))
    } else if has_section {
        Ok("DXF (ASCII) - Unknown Version".to_string())
    } else {
        Err(CadAnalyzerError::UnsupportedFormat(
            "Unable to detect a valid ASCII DXF header in the file".to_string(),
        ))
    }
}

/// Maps internal AutoCAD version codes to human-readable release names.
fn map_acad_version(code: &str) -> &str {
    match code {
        "AC1006" => "AutoCAD R10",
        "AC1009" => "AutoCAD R11/R12",
        "AC1012" => "AutoCAD R13",
        "AC1014" => "AutoCAD R14",
        "AC1015" => "AutoCAD 2000",
        "AC1018" => "AutoCAD 2004",
        "AC1021" => "AutoCAD 2007",
        "AC1024" => "AutoCAD 2010",
        "AC1027" => "AutoCAD 2013",
        "AC1032" => "AutoCAD 2018+",
        _ => "AutoCAD (Unknown version)",
    }
}

/// Parses an ASCII-encoded DXF file content, extracting version details,
/// unique layer names, entity summaries, and overall spatial bounding boxes.
fn parse_ascii_dxf(content: &str) -> Result<CadReport, CadAnalyzerError> {
    let format_version = detect_format(content)?;

    let mut lines = content.lines();
    let mut in_entities = false;

    let mut line_count = 0;
    let mut layers_set = std::collections::HashSet::new();
    let mut bbox_acc = BoundingBoxAccumulator::new();

    let mut current_entity = String::new();

    // Iterate through lines in pairs (group code and value)
    while let Some(code_line) = lines.next() {
        let value_line = lines.next().unwrap_or("").trim();
        let code = code_line.trim();

        // 1. Detect start of ENTITIES section
        if code == "2" && value_line == "ENTITIES" {
            in_entities = true;
            continue;
        }

        if in_entities {
            // End of section
            if code == "0" && value_line == "ENDSEC" {
                in_entities = false;
                continue;
            }

            // New entity boundary
            if code == "0" {
                current_entity = value_line.to_string();
                if current_entity == "LINE" {
                    line_count += 1;
                }
                continue;
            }

            // Track layer name if group code is '8'
            if code == "8" && !value_line.is_empty() {
                layers_set.insert(value_line.to_string());
            }

            // Delegate geometry aggregation for LINE entities
            if current_entity == "LINE" {
                bbox_acc.update(code, value_line);
            }
        }
    }

    // Convert layers set to sorted vector for deterministic reporting
    let mut layers: Vec<String> = layers_set.into_iter().collect();
    layers.sort();

    Ok(CadReport {
        file_name: "sample.dxf".to_string(),
        format_version,
        layers,
        entities: EntitySummary {
            lines: line_count,
            arcs: 0,
            circles: 0,
            texts: 0,
            other: 0,
        },
        bounding_box: bbox_acc.finalize(),
    })
}

/// Public entry point to analyze any supported CAD file by its path.
pub fn parse_cad_file<P: AsRef<Path>>(path: P) -> Result<CadReport, CadAnalyzerError> {
    let path_ref = path.as_ref();

    // 1. Read file content safely
    let content = std::fs::read_to_string(path_ref).map_err(|e| {
        CadAnalyzerError::IoError(format!("Failed to read file '{}': {e}", path_ref.display()))
    })?;

    // 2. Extract file name for the report
    let file_name = path_ref
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "unknown.dxf".to_string());

    // 3. Dispatch based on format (currently supporting ASCII DXF)
    // In the future, we can check binary signatures or file extensions here (.dxf vs .step)
    let mut report = parse_ascii_dxf(&content)?;
    report.file_name = file_name;

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_format_with_real_snippet() {
        let sample_header = r#"0
        SECTION
        2
        HEADER
        9
        $ACADVER
        1
        AC1015
        0
        ENDSEC
        EOF        
        "#;

        let result = detect_format(sample_header);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "DXF (ASCII) - AC1015 (AutoCAD 2000)");
    }

    #[test]
    fn test_detect_format_unsupported_format() {
        let sample_header = r#"
        0
        SECTION
        2
        UNKNOWN_SECTION      
        "#;

        let result = detect_format(sample_header);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(CadAnalyzerError::UnsupportedFormat(_))
        ));
    }

    #[test]
    fn test_detect_format_empty_file() {
        let empty_sample = "";
        let result = detect_format(empty_sample);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_ascii_dxf() {
        let sample_dxf = r#"0
        SECTION
        2
        HEADER
        9
        $ACADVER
        1
        AC1015
        0
        ENDSEC
        0
        SECTION
        2
        ENTITIES
        0
        LINE
        8
        Contour_Layer
        10
        0.0
        20
        0.0
        11
        100.0
        21
        50.0
        0
        ENDSEC
        0
        EOF"#;

        let result = parse_ascii_dxf(sample_dxf);
        assert!(result.is_ok());

        let report = result.unwrap();
        assert_eq!(report.format_version, "DXF (ASCII) - AC1015 (AutoCAD 2000)");
        assert_eq!(report.entities.lines, 1);
        assert_eq!(report.layers, vec!["Contour_Layer".to_string()]);

        assert!(report.bounding_box.is_some());
        let bbox = report.bounding_box.unwrap();
        assert_eq!(bbox.min.x, 0.0);
        assert_eq!(bbox.min.y, 0.0);
        assert_eq!(bbox.max.x, 100.0);
        assert_eq!(bbox.max.y, 50.0);
    }

    #[test]
    fn test_parse_cad_file_real_file() {
        use std::env;
        use std::fs;

        let temp_dir = env::temp_dir();
        let file_path = temp_dir.join("integration_test_sample.dxf");

        let sample_content = r#"0
        SECTION
        2
        HEADER
        9
        $ACADVER
        1
        AC1015
        0
        ENDSEC
        0
        SECTION
        2
        ENTITIES
        0
        LINE
        8
        Layer_1
        10
        10.0
        20
        20.0
        11
        30.0
        21
        40.0
        0
        ENDSEC
        0
        EOF"#;

        fs::write(&file_path, sample_content).expect("Failed to create temporary test file");

        let result = parse_cad_file(&file_path);

        let _ = fs::remove_file(&file_path);

        assert!(result.is_ok());
        let report = result.unwrap();

        assert_eq!(report.file_name, "integration_test_sample.dxf");
        assert!(report.format_version.contains("AC1015"));
        assert_eq!(report.entities.lines, 1);
        assert_eq!(report.layers, vec!["Layer_1".to_string()]);
    }

    #[test]
    fn test_parse_cad_file_not_found() {
        let non_existent_path = "non_existent_cad_file_9999.dxf";
        let result = parse_cad_file(non_existent_path);

        assert!(result.is_err());
        match result.unwrap_err() {
            CadAnalyzerError::IoError(_) => {}
            other => panic!("Expected IoError, got: {:?}", other),
        }
    }
}
