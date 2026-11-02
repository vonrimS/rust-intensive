# CAD Format Analyzer & Estimator (`cad-format-analyzer`)

## Project Overview
`cad-format-analyzer` is a high-performance, lightweight Rust CLI tool and library designed as an automated **Production Gatekeeper** for CNC manufacturing pipelines (laser, plasma, waterjet cutting). 

Instead of relying on human guesswork or heavy CAD GUI software, the analyzer performs fast, headless pre-flight checks on untrusted ("wild") customer ASCII DXF files, returning deterministic pass/fail verdicts and standard exit codes.

---

## Core Philosophy: The Gatekeeper Pattern

In automated sheet metal manufacturing, unverified CAD files often cause machine crashes, scrap material, or broken production queues. `cad-format-analyzer` acts as a strict inspector at the entry point of the pipeline:

- **No auto-repair:** Bad files are rejected immediately rather than silently fixed.
- **Unified Classification (`ClassificationResult`):** Every processed file is categorized into one of three standard outcomes:
  1. **`Passed` (Exit Code `0`):** File is geometrically valid and fits within machine bed/sheet limits. Total cutting perimeter (`total_cut_length`) is computed for cost estimation.
  2. **`OutOfBound` (Exit Code `2`):** File is valid DXF, but part dimensions ($Width \times Height$) exceed physical bed boundaries.
  3. **`Corrupted` (Exit Code `1`):** File is unreadable, has invalid DXF syntax, or contains no geometry entities.

---

## Architecture & Performance

- **Single-Pass Parsing:** Reads and parses the file from disk once into a `CadReport` structure.
- **In-Memory Classification (`classify_report`):** Business logic operates strictly on in-memory reports without redundant I/O operations, ensuring sub-millisecond classification per file.
- **CI/CD & Script Integration:** Native support for standard exit codes enables seamless integration into automated bash scripts, ERP systems, and CI/CD pipelines.

---

## CLI Usage

### Basic Commands

```bash
# Inspect a CAD file with text report output
cargo run -p p2-cad-format-analyzer -- -i test_data/sample.DXF

# Output result as JSON for automated downstream services
cargo run -p p2-cad-format-analyzer -- -i test_data/sample.DXF -f json

# Save report directly to a file
cargo run -p p2-cad-format-analyzer -- -i test_data/sample.DXF -o verdict.json -f json

# Run test suite
cargo test -p p2-cad-format-analyzer
```

## Output Examples

### Text Output (Out of Bound Verdict)
```bash
=== CAD Format Analyzer Report ===

[+] Inspected file: sample.DXF
[+] Gatekeeper Verdict: [OUT OF BOUND] Exceeds machine bed limits (3000x1500mm)
  - Part Dimensions: 6008.0 x 6083.5 mm
  - Reason: Part dimensions (6008.0x6083.5 mm) exceed machine limits (3000x1500 mm)
```
(Process exits with code 2)

```bash
=== CAD Format Analyzer Report ===

[+] Inspected file: sample.DXF
[+] Gatekeeper Verdict: [PASSED] Ready for production
  - Assigned Sheet: Mock Bed (10000x7500 mm)
  - Part Dimensions: 6008.0 x 6083.5 mm
  - Total Cut Length: 1791.29 mm
```

### JSON Output (Passed Verdict)
```json
{
  "file_name": "bracket.dxf",
  "status": "PASSED",
  "sheet_name": "Mock Bed (3000x1500 mm)",
  "width": 1200.50,
  "height": 800.00,
  "cut_length_mm": 4350.25
}
```
(Process exits with code 0)

## Tech Stack & Core Concepts
- Language: Rust (Edition 2021)
- Zero Heavy Dependencies: Written with maximum focus on low overhead and fast compilation.
- Domain Error Handling: Custom CadAnalyzerError enum for structured I/O and DXF parsing diagnostics.
- Precision Math: Bounding box calculations and exact arc/circle perimeter integration.

---

## Roadmap & Future Horizon

The project is actively evolving from a single-file checker into a complete production automation system. Planned improvements include:

1. **Batch Processing & Directory Inspection (`-d` / `--dir`):**
   - Support scanning entire folders of incoming customer files with aggregate summary reports and pipeline manifests.

2. **SQLite Database Integration:**
   - Move from hardcoded mock dimensions (`3000x1500 mm`) to dynamic machine bed and sheet inventory lookup stored in a local SQLite store.

3. **Smart Orientation & Minimal Bounding Box (OBB):**
   - Implement convex hull and Rotating Calipers algorithms to compute Orientated Bounding Boxes (OBB). This will automatically evaluate whether rotated parts fit machine limits without requiring manual CAD re-alignment.

4. **ERP & Webhook Integration:**
   - Add daemon mode / webhook triggers to allow direct execution by factory ERPs (1C, SAP, Odoo) for real-time order pricing and automated nesting queues.