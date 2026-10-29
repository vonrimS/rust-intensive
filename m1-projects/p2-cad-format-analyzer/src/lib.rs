use std::fmt::{self, Display};

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
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Represents a 3D axis-aligned bounding box enclosing CAD geometry.
pub struct BoundingBox {
    /// Minimun coordinates boundary (lower-left-back corner).
    pub min: Point3D,
    /// Maximum coordinates boundary (upper-right-front corner).
    pub max: Point3D,
}

/// Summary statistics counting the types of graphical entities in a CAD drawing.
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
    pub bounding_box: Option<BoundingBox>
}
