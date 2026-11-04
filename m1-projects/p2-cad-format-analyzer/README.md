# CAD Format Analyzer & Estimator (`cad-format-analyzer`)

## Project Overview
`cad-format-analyzer` is a high-performance, lightweight Rust CLI tool and library designed as an automated **Production Gatekeeper** for CNC sheet metal pipelines (laser, plasma, waterjet cutting).

Instead of relying on human guesswork or heavy GUI software, the analyzer performs fast, headless pre-flight checks on untrusted ("wild") customer DXF files, normalizes part names, evaluates geometry against active machine beds, calculates cutting tariffs via SQLite, sorts files into `passed/rejected` directories, and exits with standard CI/CD exit codes.

---

## Core Philosophy: The Two-Stage Gatekeeper Pattern

In automated sheet metal manufacturing, unverified CAD files cause machine crashes, scrap material, or broken production queues. `cad-format-analyzer` acts as a strict inspector at the entry point:

1. **Stage 1: Naming Gatekeeper (`naming-normalizer`)**  
   Strictly validates file naming conventions (`Company_Project_Material_Thickness.dxf`). Invalid naming causes immediate rejection without wasting CPU cycles on geometry parsing.
2. **Stage 2: Geometry & Routing Gatekeeper**  
   Validates DXF entities, calculates bounding boxes ($Width \times Height$) and cutting perimeters, and dynamically routes parts across active machine beds loaded from SQLite.

- **Automated Directory Sorting:** Valid files are automatically moved to `./passed/`, invalid ones to `./rejected/`.
- **Exit Code Conventions:** Returns exit code `0` if all batch files pass, and exit code `1` if any file fails (flagging pipeline failure for CI/CD/automation scripts).

---

## Architecture & Database Integration

- **SQLite Registry (`gatekeeper.db`):** Stores active machine dimensions (`max_x`, `max_y`), cutting tariffs per material/thickness, and logs every inspection event with SHA-256 hashes and error reasons.
- **Commercial Quick Quote:** Automatically calculates estimated cutting costs in EUR based on total cutting length ($m$) and database tariffs.
- **Streaming SHA-256 Hashing:** Computes file hashes via 8 KB buffered reads to handle large CAD blueprints with zero memory spikes.

---

## CLI Usage

### Basic Commands

```bash
# Inspect a single DXF file or an entire directory (text output by default)
cargo run -- -i test_data/

# Output structured JSON report for downstream ERP/Web integration
cargo run -- -i test_data/ -f json

# Save report directly to a file
cargo run -- -i test_data/ -o batch_report.json -f json

# Run in verbose mode for diagnostic logs
cargo run -- -i test_data/ -v
```

## Output Examples
Text Output (-f text)
```bash
=== CAD Batch Gatekeeper Report ===

File: test_test_st3_3mm.DXF
  [#] SHA-256: 9b089d83f77d76cfc63e97f66225dd38f734518fd63b0984f2ba5e9f54813710
  [✔] Naming: PASSED (Key: test_test_st3_3.0, Material: ST3, Thickness: 3mm)
  [✔] Geometry: PASSED (Machine: Laser-Fiber-01, 6008.0x6083.5mm, Cut: 1791.29mm)
  [$] Estimated Cost: 98.52 EUR
----------------------------------------
File: sample.DXF
  [#] SHA-256: N/A
  [✖] Naming Gatekeeper Rejected: Invalid filename format for 'sample.DXF': Expected at least 4 parts
----------------------------------------
```
JSON Output (-f json)
```json
{
  "total_files": 2,
  "passed_count": 1,
  "rejected_count": 1,
  "files": [
    {
      "file_name": "test_test_st3_3mm.DXF",
      "file_hash": "9b089d83f77d76cfc63e97f66225dd38f734518fd63b0984f2ba5e9f54813710",
      "verdict": "PASSED",
      "normal_key": "test_test_st3_3.0",
      "material": "ST3",
      "thickness_mm": 3.0,
      "machine_name": "Laser-Fiber-01",
      "width_mm": 6008.0,
      "height_mm": 6083.5,
      "cut_length_mm": 1791.29,
      "estimated_cost_eur": 98.52
    },
    {
      "file_name": "sample.DXF",
      "file_hash": "N/A",
      "verdict": "REJECTED",
      "error_reason": "Invalid filename format for 'sample.DXF': Expected at least 4 parts"
    }
  ]
}
```

## Tech Stack
- Language: Rust (Edition 2024)
- Database: SQLite via rusqlite (bundled feature)
- Serialization: serde, serde_json
- Crypto: sha2, hex

## Roadmap & Future Horizon
1. Smart Orientation & Minimal Bounding Box (OBB):
  - Implement Convex Hull and Rotating Calipers algorithms to compute Oriented Bounding Boxes (OBB). Evaluates whether rotated parts fit machine limits without requiring manual CAD re-alignment.
2. ERP & Webhook Daemon:
  - Background daemon mode with webhook triggers for factory ERP systems (1C, SAP, Odoo) for real-time order pricing and nesting queue automation.