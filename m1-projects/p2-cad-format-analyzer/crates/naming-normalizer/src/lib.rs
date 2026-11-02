use std::error::Error;
use std::fmt::Display;
use std::fmt;

/// Represents the normalized "passport" of a CAD part extracted from its filename.
#[derive(Debug, Clone, PartialEq)]
pub struct NormalizedPart {
    pub company: String,
    pub project: String,
    pub metal_grade: String,
    pub thickness: f64,
}

impl NormalizedPart {
    /// Generates a unique canonical key for the part to link it in the database.
    pub fn normal_key(&self) -> String {
        format!(
            "{}_{}_{}_{:.1}",
            self.company.to_lowercase(),
            self.project.to_lowercase(),
            self.metal_grade.to_lowercase(),
            self.thickness
        )
    }
}

/// Errors that can occur during filename parsing and normalization.
#[derive(Debug, PartialEq)]
pub enum NormalizerError {
    /// The filename does not contain the minimum required segments.
    InvalidFormat { filename: String, reason: String },
    /// Failed to parse the metal thickness into a floating-point number.
    InvalidThickness { raw_value: String },
}

impl Display for NormalizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NormalizerError::InvalidFormat { filename, reason } => {
                write!(f, "Invalid filename format for '{}': {}", filename, reason)
            }
            NormalizerError::InvalidThickness { raw_value } => {
                write!(f, "Could not parse thickness from value: '{}'", raw_value)
            }
        } 
    }
}

impl Error for NormalizerError {}

/// Parses a "dirty" CAD document filename, strips separators and suffixes,
/// and returns a structured part passport.
pub fn normalize_filename(filename: &str) -> Result<NormalizedPart, NormalizerError> {
    // 1. Strip the file extension (.dxf, .step, etc.)
    let name_without_ext = filename
        .rsplit_once('.')
        .map(|(name, _)| name)
        .unwrap_or(filename);

    // 2. Universal split: split the string by underscores, hyphens, and whitespace
    let parts: Vec<String> = name_without_ext
        .split(|c: char| c == '_' || c == '-' || c.is_whitespace())
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();

    // Verify that at least 4 basic segments are present
    if parts.len() < 4 {
        return Err(NormalizerError::InvalidFormat {
            filename: filename.to_string(),
            reason: format!("Expected at least 4 parts (Company, Project, Metal, Thickness), found {}", parts.len()),
        });
    }

    let company = parts[0].clone();
    let project = parts[1].clone();
    let metal_grade = parts[2].to_uppercase();

    // 3. Clean up noise from the thickness value (e.g., "2.0mm" -> "2.0")
    let thickness_raw = parts[3]
        .replace("mm", "")
        .replace("м", "");

    let thickness: f64 = thickness_raw.parse().map_err(|_| {
        NormalizerError::InvalidThickness {
            raw_value: parts[3].clone(),
        }
    })?;

    Ok(NormalizedPart {
        company,
        project,
        metal_grade,
        thickness,
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_filename_parsing() {
        let filename = "Gazprom-Kronshteyn-st3-2.0mm.dxf";
        let part = normalize_filename(filename).unwrap();

        assert_eq!(part.company, "gazprom");
        assert_eq!(part.project, "kronshteyn");
        assert_eq!(part.metal_grade, "ST3");
        assert_eq!(part.thickness, 2.0);
        assert_eq!(part.normal_key(), "gazprom_kronshteyn_st3_2.0");
    }

    #[test]
    fn test_different_separators() {
        let filename = "Rosneft Vint st3 3mm.DWG";
        let part = normalize_filename(filename).unwrap();

        assert_eq!(part.company, "rosneft");
        assert_eq!(part.project, "vint");
        assert_eq!(part.metal_grade, "ST3");
        assert_eq!(part.thickness, 3.0);
        assert_eq!(part.normal_key(), "rosneft_vint_st3_3.0");
    }

    #[test]
    fn test_invalid_format() {
        let filename = "BadName.dxf";
        let result = normalize_filename(filename);
        assert!(matches!(result, Err(NormalizerError::InvalidFormat { .. })));
    }
}