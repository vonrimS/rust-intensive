# CAD Format Analyzer (`cad-format-analyzer`)

## Project Overview
`cad-format-analyzer` is a high-performance, lightweight Rust CLI tool and library designed for headless inspection, structural auditing, and geometric bounding box extraction from CAD files (such as 2D DXF and 3D STEP Part 21 files).

Built with a focus on **CAD Interoperability (Zoo Ecosystem)**, the analyzer parses binary or text-based CAD inputs, validates header signatures, extracts structural metadata (layers, entity counts, units), and calculates spatial boundaries — all without launching heavy graphical CAD software.

---

## Why CAD Format Analyzer? (Use Cases & Value)

In engineering, R&D, and manufacturing environments, teams often deal with thousands of legacy or third-party CAD files (`.dxf`, `.step`, `.dwg`). Manually inspecting these files in AutoCAD or SolidWorks wastes critical engineering time and risks crashing software on corrupted inputs.

`cad-format-analyzer` acts as a **Swiss Army Knife for CAD Interoperability**, providing:

1. **Headless Bounding Box Calculation:** Instantly extract 3D spatial boundaries ($X_{min}, Y_{min}, Z_{min} \rightarrow X_{max}, Y_{max}, Z_{max}$) to check if a part fits machine tables (e.g., CNC, laser cutters) or stock materials without rendering.
2. **Resilient Legacy Data Auditing:** Built for "wild" archives. Features **Graceful Degradation** — recovers valid geometry and metadata from corrupted or incomplete files instead of panicking.
3. **CI/CD Quality Control:** Automate pre-flight checks on supplier files: validate format versions, flag missing layer definitions, or reject non-compliant geometries before production.
4. **2D & 3D Multi-Format Support:** Unifies parsing of 2D draft formats (ASCII/Binary DXF) and 3D solid models (STEP Part 21) under a single fast CLI interface.

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
   - Detect and validate file signatures and headers for target formats (DXF versions, STEP ISO 10303 headers).
   - Inspect structural components:
     - Header section flags, drawing units, and encoding details.
     - Table definitions (layers, line types, missing layer references).
     - Entity counts (`LINE`, `CIRCLE`, `ARC`, `TEXT`, STEP surface primitives) and spatial bounding box metadata.
   - Extract embedded text properties, block counts, and revision metadata.

3. **Domain Error Handling & Fault Tolerance:**
   - Define a custom `CadAnalyzerError` enum:
     - `IoError(String)`
     - `InvalidHeader(String)`
     - `ParseError { offset: usize, message: String }`
     - `UnsupportedFormat(String)`
   - Implement **Graceful Degradation**: recover partial metrics from malformed files and output non-fatal diagnostic warnings in verbose mode.

4. **Architecture (`lib.rs` / `main.rs`):**
   - Place format detectors, stream parsers, entity accumulators, and report formatters in `src/lib.rs`.
   - Keep argument parsing, CLI feedback, and file I/O orchestration in `src/main.rs`.

5. **Testing Suite:**
   - Unit tests for header validation and binary/text tag decoding.
   - Unit tests for metadata formatting and spatial bounds calculation (`BoundingBoxAccumulator`).
   - Integration tests covering real-world valid and corrupted CAD snippets.

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
  - **Memory Efficiency:** Minimal heap allocations when parsing text streams or binary buffers.
  - **Pattern Matching:** Byte signature detection and enum-driven state evaluation.
  - **Error Propagation:** Strong domain error handling using standard Rust traits (`Display`, `std::error::Error`).

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