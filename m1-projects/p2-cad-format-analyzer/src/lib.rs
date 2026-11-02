use std::{
    collections::HashMap,
    fmt::{self, Display},
    path::Path,
};

/// Detailed result of the CAD file classification by the production gatekeeper.
#[derive(Debug, PartialEq)]
pub enum ClassificationResult {
    /// File is fully valid and fits within the machine/sheet limits.
    Passed {
        sheet_name: String,
        cut_length: f64,
        width: f64,
        height: f64,
    },
    /// File is syntactically/geometrically valid, but exceeds machine bed limits.
    OutOfBound {
        reason: String,
        width: f64,
        height: f64,
    },
    /// File is corrupted, unreadable, or missing critical sections.
    Corrupted { reason: String, offset: usize },
}

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

/// Accumulates geometric entities (lines, arcs, circles) on the fly,
/// calculating their precise cutting length and updating overall spatial boundaries.
#[derive(Debug, Default)]
pub struct GeometryAccumulator {
    /// Accumulated total cut length (perimeter) across all parsed entities, in drawing units (mm).
    pub total_cut_length: f64,
    /// Total number of parsed straight line segments.
    pub line_count: usize,
    /// Total number of parsed full circles.
    pub circle_count: usize,
    /// Total number of parsed circular arcs.
    pub arc_count: usize,
    /// Minimum X coordinate of the bounding box.
    pub min_x: f64,
    /// Maximum X coordinate of the bounding box.
    pub max_x: f64,
    /// Minimum Y coordinate of the bounding box.
    pub min_y: f64,
    /// Maximum Y coordinate of the bounding box.
    pub max_y: f64,
    /// Internal flag indicating whether valid geometry entities were found.
    pub has_geometry: bool,
}

impl GeometryAccumulator {
    /// Creates a new instance of `GeometryAccumulator` with initialized extreme boundary states.
    pub fn new() -> Self {
        Self {
            total_cut_length: 0.0,
            line_count: 0,
            circle_count: 0,
            arc_count: 0,
            min_x: f64::MAX,
            max_x: f64::MIN,
            min_y: f64::MAX,
            max_y: f64::MIN,
            has_geometry: false,
        }
    }

    /// Processes a straight line segment, updating total cut length and bounding box limits.
    pub fn add_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let length = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
        self.total_cut_length += length;
        self.line_count += 1;

        self.update_bounds(x1, y1);
        self.update_bounds(x2, y2);
        self.has_geometry = true;
    }

    /// Processes a full circle, calculating its circumference and expanding bounding box limits.
    pub fn add_circle(&mut self, cx: f64, cy: f64, radius: f64) {
        let length = 2.0 * std::f64::consts::PI * radius;
        self.total_cut_length += length;
        self.circle_count += 1;

        self.update_bounds(cx - radius, cy - radius);
        self.update_bounds(cx + radius, cy + radius);
        self.has_geometry = true;
    }

    /// Processes a circular arc, calculating its curved length based on start and end angles,
    /// and expanding bounding box limits.
    pub fn add_arc(&mut self, cx: f64, cy: f64, radius: f64, start_deg: f64, end_deg: f64) {
        let mut angle_diff = end_deg - start_deg;
        if angle_diff < 0.0 {
            angle_diff += 360.0;
        }
        let rad = angle_diff.to_radians();
        let length = radius * rad;

        self.total_cut_length += length;
        self.arc_count += 1;

        self.update_bounds(cx - radius, cy - radius);
        self.update_bounds(cx + radius, cy + radius);
        self.has_geometry = true;
    }

    /// Internal helper to expand the bounding box extremes.
    fn update_bounds(&mut self, x: f64, y: f64) {
        if x < self.min_x {
            self.min_x = x;
        }
        if x > self.max_x {
            self.max_x = x;
        }
        if y < self.min_y {
            self.min_y = y;
        }
        if y > self.max_y {
            self.max_y = y;
        }
    }

    /// Finalizes accumulation into an optional `BoundingBox`.
    pub fn finalize_bounding_box(self) -> Option<BoundingBox> {
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
    /// Number of header variables parsed from the drawing header section.
    pub header_count: usize,
    /// Collection of unique layer names defined in the drawing.
    pub layers: Vec<String>,
    /// Aggregated count of graphical entities.
    pub entities: EntitySummary,
    /// Total calculated cutting perimeter/length across all supported entities (mm).
    pub total_cut_length: f64,
    /// Number of block definitions (block records) present in the drawing.
    pub block_count: usize,
    /// Overall spatial bounding box of the drawing, if geometry is present.
    pub bounding_box: Option<BoundingBox>,
    /// Total duration elapsed during the CAD file parsing and analysis.
    pub analysis_duration: std::time::Duration,
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
/// header variables count, block records count, unique layer names,
/// entity summaries, and overall spatial bounding boxes.
fn parse_ascii_dxf(content: &str) -> Result<CadReport, CadAnalyzerError> {
    let format_version = detect_format(content)?;

    let mut lines = content.lines();

    // State flags for tracking active sections within the DXF file structure
    let mut in_header = false;
    let mut in_blocks = false;
    let mut in_entities = false;

    // Acumulators for metrics and counters
    let mut header_count = 0;
    let mut block_count = 0;

    let mut layers_set = std::collections::HashSet::new();
    let mut geom_acc = GeometryAccumulator::new();

    let mut current_entity = String::new();

    // Temporary storage for entity parameters during iteration
    let mut entity_data: HashMap<String, String> = HashMap::new();

    // Iterate through lines in pairs (group code and value)
    while let Some(code_line) = lines.next() {
        let value_line = lines.next().unwrap_or("").trim();
        let code = code_line.trim();

        // Reset section flags upon encountering a new section boundary
        if code == "0" && value_line == "SECTION" {
            in_header = false;
            in_blocks = false;
            in_entities = false;
            continue;
        }

        // Identify the specific section name via group code '2'
        if code == "2" {
            in_header = value_line == "HEADER";
            in_blocks = value_line == "BLOCKS";
            in_entities = value_line == "ENTITIES";
            continue;
        }

        // Terminate active section tracking on 'ENDSEC'
        if code == "0" && value_line == "ENDSEC" {
            in_header = false;
            in_blocks = false;
            in_entities = false;
            continue;
        }

        // Count header variables (identified by group code '9' within the HEADER section)
        if in_header && code == "9" {
            header_count += 1;
        }

        // Count block definitions (identified by 'BLOCK' entity type within the BLOCK section)
        if in_blocks && code == "0" && value_line == "BLOCK" {
            block_count += 1;
        }

        // Parse entities and geometry when inside the ENTITIES section
        if in_entities {
            // Track the start of a new graphical entity type
            if code == "0" {
                // Before switching to a new entity, commit the previous one if applicable
                process_entity(&current_entity, &entity_data, &mut geom_acc);

                current_entity = value_line.to_string();
                entity_data.clear();
                continue;
            }

            // Track unique layer names assigned via gropu code '8'
            if code == "8" && !value_line.is_empty() {
                layers_set.insert(value_line.to_string());
            }

            entity_data.insert(code.to_string(), value_line.to_string());
        }
    }

    // Process the final entity in the stream
    process_entity(&current_entity, &entity_data, &mut geom_acc);

    let mut layers: Vec<String> = layers_set.into_iter().collect();
    layers.sort();

    Ok(CadReport {
        file_name: String::new(),
        format_version,
        header_count,
        layers,
        entities: EntitySummary {
            lines: geom_acc.line_count,
            arcs: geom_acc.arc_count,
            circles: geom_acc.circle_count,
            texts: 0,
            other: 0,
        },
        total_cut_length: geom_acc.total_cut_length,
        block_count,
        bounding_box: geom_acc.finalize_bounding_box(),
        analysis_duration: std::time::Duration::default(),
    })
}

/// Helper function to extract numerical coordinates/parameters based on entity type and pass to accumulator.
fn process_entity(
    entity_type: &str,
    data: &std::collections::HashMap<String, String>,
    acc: &mut GeometryAccumulator,
) {
    match entity_type {
        "LINE" => {
            let x1 = data.get("10").and_then(|v| v.parse().ok()).unwrap_or(0.0);
            let y1 = data.get("20").and_then(|v| v.parse().ok()).unwrap_or(0.0);
            let x2 = data.get("11").and_then(|v| v.parse().ok()).unwrap_or(0.0);
            let y2 = data.get("21").and_then(|v| v.parse().ok()).unwrap_or(0.0);
            acc.add_line(x1, y1, x2, y2);
        }
        "CIRCLE" => {
            let cx = data.get("10").and_then(|v| v.parse().ok()).unwrap_or(0.0);
            let cy = data.get("20").and_then(|v| v.parse().ok()).unwrap_or(0.0);
            let r = data.get("40").and_then(|v| v.parse().ok()).unwrap_or(0.0);
            acc.add_circle(cx, cy, r);
        }
        "ARC" => {
            let cx = data.get("10").and_then(|v| v.parse().ok()).unwrap_or(0.0);
            let cy = data.get("20").and_then(|v| v.parse().ok()).unwrap_or(0.0);
            let r = data.get("40").and_then(|v| v.parse().ok()).unwrap_or(0.0);
            let start = data.get("50").and_then(|v| v.parse().ok()).unwrap_or(0.0);
            let end = data.get("51").and_then(|v| v.parse().ok()).unwrap_or(0.0);
            acc.add_arc(cx, cy, r, start, end);
        }
        _ => {}
    }
}

/// Public entry point to analyze any supported CAD file by its path.
pub fn parse_cad_file<P: AsRef<Path>>(path: P) -> Result<CadReport, CadAnalyzerError> {
    let start_time = std::time::Instant::now();
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
    report.analysis_duration = start_time.elapsed();

    Ok(report)
}

/// Compares a parsed CAD report against machine bed dimensions in memory.
///
/// Returns `Corrupted` if no bounding box/geometry is present,
/// `OutOfBound` if dimensions exceed `max_x` / `max_y`,
/// or `Passed` with cut metrics if it fits.
pub fn classify_report(report: &CadReport, max_x: f64, max_y: f64) -> ClassificationResult {
    let bbox = match &report.bounding_box {
        Some(bb) => bb,
        None => {
            return ClassificationResult::Corrupted {
                reason: "No geometry entities found in DXF file".to_string(),
                offset: 0,
            };
        }
    };

    let width = bbox.max.x - bbox.min.x;
    let height = bbox.max.y - bbox.min.y;

    if width > max_x || height > max_y {
        ClassificationResult::OutOfBound {
            reason: format!(
                "Part dimensions ({width:.1}x{height:.1} mm) exceed machine limits ({max_x}x{max_y} mm)"
            ),
            width,
            height,
        }
    } else {
        ClassificationResult::Passed {
            sheet_name: format!("Mock Bed ({}x{} mm)", max_x, max_y),
            cut_length: report.total_cut_length,
            width,
            height,
        }
    }
}

/// Helper that reads and parses a CAD file from disk, then classifies it.
///
/// Converts any I/O or syntax parse error into `ClassificationResult::Corrupted`.
pub fn classify_cad_file<P: AsRef<Path>>(path: P, max_x: f64, max_y: f64) -> ClassificationResult {
    match parse_cad_file(path) {
        Ok(report) => classify_report(&report, max_x, max_y),
        Err(e) => ClassificationResult::Corrupted {
            reason: e.to_string(),
            offset: match e {
                CadAnalyzerError::ParseError { offset, .. } => offset,
                _ => 0,
            },
        },
    }
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

    #[test]
    fn test_classify_report_passed() {
        let report = CadReport {
            file_name: "part.dxf".to_string(),
            format_version: "AC1015".to_string(),
            header_count: 1,
            layers: vec![],
            entities: EntitySummary {
                lines: 4,
                arcs: 0,
                circles: 0,
                texts: 0,
                other: 0,
            },
            total_cut_length: 400.0,
            block_count: 0,
            bounding_box: Some(BoundingBox {
                min: Point3D {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                max: Point3D {
                    x: 200.0,
                    y: 100.0,
                    z: 0.0,
                },
            }),
            analysis_duration: std::time::Duration::default(),
        };

        let result = classify_report(&report, 3000.0, 1500.0);
        match result {
            ClassificationResult::Passed {
                width,
                height,
                cut_length,
                ..
            } => {
                assert_eq!(width, 200.0);
                assert_eq!(height, 100.0);
                assert_eq!(cut_length, 400.0);
            }
            other => panic!("Expected Passed, got {:?}", other),
        }
    }

    #[test]
    fn test_classify_report_out_of_bound() {
        let report = CadReport {
            file_name: "huge_part.dxf".to_string(),
            format_version: "AC1015".to_string(),
            header_count: 1,
            layers: vec![],
            entities: EntitySummary {
                lines: 4,
                arcs: 0,
                circles: 0,
                texts: 0,
                other: 0,
            },
            total_cut_length: 12000.0,
            block_count: 0,
            bounding_box: Some(BoundingBox {
                min: Point3D {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                max: Point3D {
                    x: 4000.0,
                    y: 2000.0,
                    z: 0.0,
                },
            }),
            analysis_duration: std::time::Duration::default(),
        };

        let result = classify_report(&report, 3000.0, 1500.0);
        match result {
            ClassificationResult::OutOfBound { width, height, .. } => {
                assert_eq!(width, 4000.0);
                assert_eq!(height, 2000.0);
            }
            other => panic!("Expected OutOfBound, got {:?}", other),
        }
    }

    #[test]
    fn test_classify_report_corrupted_no_geometry() {
        let report = CadReport {
            file_name: "empty.dxf".to_string(),
            format_version: "AC1015".to_string(),
            header_count: 1,
            layers: vec![],
            entities: EntitySummary {
                lines: 0,
                arcs: 0,
                circles: 0,
                texts: 0,
                other: 0,
            },
            total_cut_length: 0.0,
            block_count: 0,
            bounding_box: None,
            analysis_duration: std::time::Duration::default(),
        };

        let result = classify_report(&report, 3000.0, 1500.0);
        match result {
            ClassificationResult::Corrupted { reason, .. } => {
                assert!(reason.contains("No geometry entities found"));
            }
            other => panic!("Expected Corrupted, got {:?}", other),
        }
    }
}
