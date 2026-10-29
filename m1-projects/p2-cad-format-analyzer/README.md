# CAD Format Analyzer (`cad-format-analyzer`)

## Project Overview
`cad-format-analyzer` is a lightweight Rust CLI tool and library designed to inspect, analyze, and extract structural metadata from CAD files (such as DXF, DWG headers, or STEP files). 

The application reads binary or text-based CAD inputs, validates header signatures, parses structural elements (layers, entities, blocks, metadata tables), and produces detailed diagnostic reports in JSON or human-readable summary formats.

---

## Technical Requirements

1. **CLI & File I/O Interface:**
   - Parse command-line arguments using a lightweight argument parser:
     - Input CAD file path (`-i` / `--input`)
     - Output report path (`-o` / `--output`, optional)
     - Output format selector (`-f` / `--format`, defaults to `text`, supports `json`)
     - Verbose diagnostic mode (`-v` / `--verbose`)
   - Handle filesystem operations gracefully, providing clean error messaging for missing files, unreadable paths, or invalid permissions.

2. **Parsing & Metadata Extraction Engine:**
   - Detect and validate file signatures and headers for target formats.
   - Inspect structural components:
     - Header section flags, drawing units, and encoding details.
     - Table definitions (layers, line types, coordinate systems).
     - Entity counts and spatial bounding box metadata.
   - Extract embedded text properties, block counts, and revision metadata.

3. **Domain Error Handling:**
   - Define a custom `CadAnalyzerError` enum:
     - `IoError(String)`
     - `InvalidHeader(String)`
     - `ParseError { offset: usize, message: String }`
     - `UnsupportedFormat(String)`
   - Implement `Display` and `std::error::Error` for `CadAnalyzerError`.

4. **Architecture (`lib.rs` / `main.rs`):**
   - Place binary parsers, signature detectors, metadata extractors, and report formatters in `src/lib.rs`.
   - Keep argument parsing, CLI feedback, and file I/O orchestration in `src/main.rs`.

5. **Testing Suite:**
   - Unit tests for header validation and binary tag decoding.
   - Unit tests for metadata formatting and spatial bounds calculation.
   - Integration tests covering end-to-end file analysis pipelines with sample CAD files.

---

## Tech Stack & Core Concepts

- **Language & Edition:** Rust (Edition 2021)
- **Dependencies:**
  - `serde` / `serde_json` for structured report serialization
- **Standard Library Usage:**
  - `std::fs` & `std::io` for buffered byte reading and report writing
  - `std::path::Path` for file extension and path verification
  - `std::env` for CLI argument evaluation
  - `std::fmt::{Display, Formatter}` for custom error and status formatting
- **Core Rust Concepts:**
  - **Memory Efficiency:** Minimal heap allocations when parsing binary buffers using slice references (`&[u8]`).
  - **Pattern Matching:** Byte signature detection and enum-driven state evaluation.
  - **Error Propagation:** Strong domain error handling using standard Rust traits.

---

## Building & Running

From the workspace root directory:

```bash
# Build and analyze a CAD file with text summary output
cargo run -p cad-format-analyzer -- -i sample.dxf

# Export analysis report in JSON format to a file
cargo run -p cad-format-analyzer -- -i sample.dxf -o report.json -f json

# Run with verbose diagnostic logs
cargo run -p cad-format-analyzer -- -i sample.dxf --verbose

# Run unit and integration tests
cargo test -p cad-format-analyzer
```

## Expected Behavior Example

Command:

```bash
cargo run -p cad-format-analyzer -- -i sample.dxf
```
Execution Output:

```bash
=== CAD Format Analyzer ===

[+] Analyzing input file: sample.dxf
[+] Detected format: AutoCAD DXF (ASCII text)
[+] Version: AC1015 (AutoCAD 2000)

Summary Metrics:
  - Header Variables: 42
  - Defined Layers:   12
  - Total Entities:   1,248
  - Block Records:    8

Bounding Box:
  - Min: (X: 0.00, Y: 0.00, Z: 0.00)
  - Max: (X: 420.00, Y: 297.00, Z: 0.00)

Analysis completed successfully in 3.4ms.
```