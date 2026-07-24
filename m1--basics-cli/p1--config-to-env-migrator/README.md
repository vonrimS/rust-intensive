# Project M1-P1: Config-to-Env Migrator

## Business Goal
In enterprise environments (like classic `w` setups), services often rely on heavy, nested, or legacy `.ini` configurations. Modern cloud-native environments and 12-Factor apps require configuration via Environment Variables (`.env`). This CLI tool securely and efficiently parses a legacy `.ini` file and converts it into a flat, well-formatted `.env` file.



## Technical Constraints & Requirements
* **Zero-Copy Parsing (Target):** Do not read the entire file into a single `String` bucket. Stream it line-by-line using `std::io::BufReader`. 
* **Strict String Handling:** Avoid allocating new `String` objects on the heap for intermediate tokens. Use string slices (`&str`) wherever possible.
* **Error Architecture:** No `.unwrap()` or `.expect()` in the parsing engine. Define a custom error type using an `enum` (e.g., `ConfigParserError`) to handle missing keys, malformed lines, or I/O issues.
* **No External Dependencies:** Use only Rust's standard library (`std`). No external crates like `regex` or `ini` for parsing. Write the string manipulation mechanisms from scratch using `.split()`, `.trim()`, and pattern matching.


## Technical Specification (Scope of Work)

### 1. CLI Interface
The binary must accept flags using native standard arguments processing (`std::env::args`):
* `--input` or `-i`: Path to the source `.ini` file.
* `--output` or `-o`: Path to the target `.env` file (defaults to `./.env` if not provided).

### 2. INI Format Sub-set to Support
Your parser must handle:
* Properties: `KEY=VALUE`
* Whitespace resilience: `  KEY   =    VALUE  ` -> should become `KEY=VALUE`
* Comments: Lines starting with `;` or `#` must be ignored.
* Section headers: Lines like `[DATABASE]` should be processed. To keep the `.env` flat, keys inside sections should be prefixed: `DATABASE_HOST=localhost`.

### 3. Verification & Tests
* Write unit tests that pass malformed strings into your parser to verify that your `Result::Err` types work correctly.
* Write an integration test that takes a dummy `.ini` file and asserts the exact content of the generated `.env` file.