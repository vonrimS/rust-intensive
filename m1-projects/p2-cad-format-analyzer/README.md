# CAD Format Analyzer (`cad-format-analyzer`)

## Project Overview
`cad-format-analyzer` is a high-performance, lightweight Rust CLI tool and library designed for headless inspection, structural auditing, and geometric bounding box extraction from ASCII DXF CAD files.

Built with a focus on speed and reliability, the analyzer parses text-based CAD inputs, validates header signatures, extracts structural metadata (layers, header variables, block records, entity counts), and calculates spatial boundaries — all without launching heavy graphical CAD software.

---

## Why CAD Format Analyzer? (Use Cases & Value)

In engineering, R&D, and manufacturing environments, teams often deal with legacy or third-party CAD files (`.dxf`). Manually inspecting these files in AutoCAD wastes critical engineering time and risks crashing software on corrupted inputs.

`cad-format-analyzer` acts as a **Swiss Army Knife for CAD Interoperability**, providing:

1. **Headless Bounding Box Calculation:** Instantly extract 3D spatial boundaries ($X_{min}, Y_{min}, Z_{min} \rightarrow X_{max}, Y_{max}, Z_{max}$) to check if a part fits machine tables (e.g., CNC, laser cutters) or stock materials without rendering.
2. **Resilient Data Auditing:** Built for "wild" archives, extracting valid geometry and metadata efficiently.
3. **CI/CD Quality Control:** Automate pre-flight checks on supplier files: validate format versions, flag missing layer definitions, or inspect entity distributions before production.

---

## Technical Requirements

1. **CLI & File I/O Interface:**
   - Parse command-line arguments:
     - Input CAD file path (`-i` / `--input`)
     - Output report path (`-o` / `--output`, optional)
     - Output format selector (`-f` / `--format`, defaults to `text`, supports `json`)
     - Verbose diagnostic mode (`-v` / `--verbose`)
   - Handle filesystem operations gracefully, providing clean error messaging for missing files, unreadable paths, or invalid permissions.

2. **Parsing & Metadata Extraction Engine:**
   - Detect and validate file signatures and headers for ASCII DXF versions.
   - Inspect structural components:
     - Header section variables count.
     - Table definitions (unique layer names).
     - Entity counts (`LINE`, etc.) and spatial bounding box metadata.
     - Block record counts and analysis execution duration benchmarks.

3. **Domain Error Handling:**
   - Define a custom `CadAnalyzerError` enum:
     - `IoError(String)`
     - `InvalidHeader(String)`
     - `ParseError { offset: usize, message: String }`
     - `UnsupportedFormat(String)`

4. **Architecture (`lib.rs` / `main.rs`):**
   - Place format detectors, stream parsers, entity accumulators, and report logic in `src/lib.rs`.
   - Keep argument parsing, CLI feedback, and file I/O orchestration in `src/main.rs`.

5. **Testing Suite:**
   - Unit tests for header validation and tag decoding.
   - Unit tests for metadata formatting and spatial bounds calculation (`BoundingBoxAccumulator`).

---

## Tech Stack & Core Concepts

- **Language & Edition:** Rust (Edition 2021)
- **Dependencies:**
  - `serde` / `serde_json` (if applicable for structured exports)
- **Standard Library Usage:**
  - `std::fs` & `std::io` for buffered text reading and report writing
  - `std::path::Path` for path verification
  - `std::time::Instant` for high-precision analysis benchmarking
- **Core Rust Concepts:**
  - **Memory Efficiency:** Single-pass streaming parsers with minimal heap allocations.
  - **Pattern Matching:** State-machine flag evaluation for DXF sections (`HEADER`, `BLOCKS`, `ENTITIES`).
  - **Error Propagation:** Strong domain error handling using standard Rust traits.

---

## Building & Running

From the workspace root directory:

```bash
# Build and analyze a CAD file with text summary output 
cargo run -p cad-format-analyzer -- -i test_data/sample.DXF

# Export analysis report in JSON format to a file
cargo run -p cad-format-analyzer -- -i test_data/sample.DXF -o report.json -f json

# Run unit tests
cargo test -p cad-format-analyzer
```

## Expected Behavior Example

Command:
```bash
cargo run -p cad-format-analyzer -- -i test_data/sample.DXF
```
Execution Output:
```bash
=== CAD Format Analyzer Report ===

[+] Analyzing input file: sample.DXF
[+] Detected format: AutoCAD DXF (ASCII text)
[+] Version: DXF (ASCII) - AC1015 (AutoCAD 2000)

Summary Metrics:
  - Header Variables: 60
  - Defined Layers:   1
  - Total Entities:   21
  - Block Records:    1

Bounding Box:
  - Min: (X: 33.43, Y: 760.41, Z: 0.00)
  - Max: (X: 494.57, Y: 869.50, Z: 0.00)

Analysis completed successfully in 1.9278ms.
```