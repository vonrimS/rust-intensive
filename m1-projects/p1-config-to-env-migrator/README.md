# M1-P1: Config to Env Migrator (`m1-p1-config-to-env-migrator`)
---

## 📌 Project Overview
`m1-p1-config-to-env-migrator` is a high-performance command-line tool (CLI) designed to automate the conversion and migration of configuration files into standard `.env` environment variables.

The application parses input configuration files (supporting `.json`, `.yaml`, `.ini`, and nested property structures), flattens complex hierarchical keys into UPPER_SNAKE_CASE environment variable names, validates string values, and generates clean, production-ready `.env` output files.

---

## 🎯 Technical Requirements

1. **CLI & File I/O Interface:**
   - Parse command-line arguments using `std::env::args` or a lightweight argument parser:
     - Input configuration path (`-i` / `--input`)
     - Output `.env` path (`-o` / `--output`, defaults to `.env`)
     - Key prefix option (`-p` / `--prefix`)
     - Dry-run mode (`--dry-run`) to print transformed key-value pairs to `stdout` without writing to disk.
   - Gracefully handle file system errors (file not found, permission denied, invalid encoding).

2. **Parsing & Key Transformation Engine:**
   - Handle structured input formats (e.g., JSON, YAML/TOML, or INI/Properties).
   - Implement recursive key flattening for nested structures:
     - Example: `database.connection.host` -> `DATABASE_CONNECTION_HOST`
   - Transform identifiers to strict `UPPER_SNAKE_CASE` rules:
     - Convert camelCase, kebab-case, and dot-notation into standard environment variable naming conventions.
   - Escape and quote string values containing spaces, newlines, or special characters (`#`, `=`, `$`).

3. **Domain Error Handling:**
   - Define a custom `MigratorError` enum:
     - `IoError(String)`
     - `ParseError { line: usize, message: String }`
     - `InvalidKeyFormat(String)`
     - `UnsupportedExtension(String)`
   - Implement `Display` and `std::error::Error` for `MigratorError`.

4. **Architecture (`lib.rs` / `main.rs`):**
   - Keep string manipulation, key flattening algorithms, parser traits, and file formatting in `src/lib.rs`.
   - Implement argument parsing, file reading/writing, and CLI status output in `src/main.rs`.

5. **Testing Suite:**
   - Unit tests for key transformation logic (camelCase to UPPER_SNAKE_CASE).
   - Unit tests for string escaping rules.
   - Integration tests covering full file conversion pipelines.

---

## 🛠 Tech Stack & Core Concepts

- **Language & Edition:** Rust (Edition 2021)
- **External Dependencies:**
  - `serde` / `serde_json` (or custom lightweight parsers) for deserialization
- **Standard Library Components:**
  - `std::fs` & `std::io` (File read/write streams)
  - `std::path::Path` (File extension validation)
  - `std::env` (CLI argument parsing)
  - `std::fmt::{Display, Formatter}` (Error and format output)
- **Language Features:**
  - **String Manipulation & Allocation:** Efficient usage of `String`, `&str`, `String::with_capacity`, and iterators.
  - **Pattern Matching & Recursion:** Traversing nested structures and string patterns.
  - **Custom Error Trait Implementation:** Robust CLI error propagation.

---

## 🚀 Building & Running

From the workspace root directory:

```bash
# Build and run with arguments
cargo run -p m1-p1-config-to-env-migrator -- -i config.json -o .env

# Run dry-run mode (output to stdout)
cargo run -p m1-p1-config-to-env-migrator -- -i config.json --dry-run

# Run unit and integration tests
cargo test -p m1-p1-config-to-env-migrator
```

## 💡 Expected Behavior Example

Input (config.json):
```JSON
{
  "server": {
    "host": "127.0.0.1",
    "port": 8080
  },
  "database": {
    "connectionUrl": "postgres://user:pass@localhost:5432/db",
    "maxConnections": 20
  }
}
```

Execution Output:
```Bash
=== Config to Env Migrator ===

[+] Reading input: config.json (JSON format detected)
[+] Flattening and transforming keys...
[+] Generated 4 environment variables:

    SERVER_HOST=127.0.0.1
    SERVER_PORT=8080
    DATABASE_CONNECTION_URL="postgres://user:pass@localhost:5432/db"
    DATABASE_MAX_CONNECTIONS=20

[+] Successfully saved output to .env
```