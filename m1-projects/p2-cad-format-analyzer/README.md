# CAD Format Analyzer & Estimator (`cad-format-analyzer`)

## Project Overview
`cad-format-analyzer` is a high-performance, lightweight Rust CLI tool and library designed for headless inspection, geometric auditing, and production pre-flight checks of ASCII DXF CAD files (such as sheet metal flat patterns from SolidWorks).

Built with a focus on speed and reliability, the analyzer parses text-based CAD inputs, validates header signatures, extracts structural metadata, calculates spatial boundaries, and computes the **total cutting perimeter** — all without launching heavy graphical CAD software.

---

## Why CAD Format Analyzer? (Use Cases & Value)

In manufacturing and CNC cutting environments, teams often deal with unverified "wild" CAD files (`.dxf`) sent by customers. Manually inspecting these files wastes engineering time and risks software crashes.

`cad-format-analyzer` acts as a **Pre-flight & Cost Estimation Gatekeeper**, providing:

1. **Headless Bounding Box Calculation:** Instantly extract 3D spatial boundaries ($X_{min}, Y_{min} \rightarrow X_{max}, Y_{max}$) to check if a part fits machine beds (laser, plasma, waterjet) without rendering.
2. **Total Cut Length (Perimeter) Extraction:** Automatically sum lengths of straight lines, arcs, and full circles to compute exact cutting metrics.
3. **Cost & Production Estimation:** Provide rough preliminary cost estimations based on cutting distance and equipment tariffs.
4. **CI/CD Quality Control:** Automate pre-flight checks on incoming orders: validate format versions, flag oversized parts, and inspect entity distributions before production.

---

## Technical Requirements & Core Features

1. **CLI & File I/O Interface:**
   - Parse command-line arguments:
     - Input CAD file path (`-i` / `--input`)
     - Output report path (`-o` / `--output`, optional)
     - Output format selector (`-f` / `--format`, defaults to `text`, supports `json`)
     - Verbose diagnostic mode (`-v` / `--verbose`)

2. **Parsing & Geometry Engine:**
   - Detect and validate file signatures and headers for ASCII DXF versions.
   - Inspect structural components:
     - Header variables count and unique layer names.
     - Entity counts (`LINE`, `ARC`, `CIRCLE`) and spatial bounding box metadata.
     - **On-the-fly cut length accumulation** (precise math for lines, circles, and arcs).

3. **Domain Error Handling:**
   - Define a custom `CadAnalyzerError` enum (`IoError`, `InvalidHeader`, `ParseError`, `UnsupportedFormat`).

4. **Testing Suite:**
   - Unit tests for header validation, entity parsing, and spatial bounds calculation (`GeometryAccumulator`).

---

## Tech Stack & Core Concepts

- **Language & Edition:** Rust (Edition 2021)
- **Standard Library Usage:**
  - `std::fs` & `std::io` for buffered text reading and report writing
  - `std::path::Path` for path verification
  - `std::time::Instant` for high-precision analysis benchmarking
- **Core Rust Concepts:**
  - **Memory Efficiency:** Single-pass streaming parsers with minimal heap allocations.
  - **Pattern Matching:** State-machine flag evaluation for DXF sections (`HEADER`, `BLOCKS`, `ENTITIES`).

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