# P7: Simple Config Parser (`p7-simple-config-parser`)
---

## 📌 Project Overview
`p7-simple-config-parser` is a configuration parsing library designed to practice `HashMap` storage, custom `enum` error types, string manipulation (`trim`, `split_once`), and safe type casting using the `std::str::FromStr` trait.

The application reads key-value configuration strings, ignores comments and whitespace, stores settings in a key-value hash map, and provides a safe API to retrieve typed values (e.g., `usize`, `bool`) without panicking on invalid input or missing keys.

---

## 🎯 Technical Requirements

1. **Domain Modeling & Error Handling:**
   - Define a custom `ConfigError` enum covering all failure states: `InvalidLine(String)`, `EmptyKey(String)`, `ParseValueError { key: String, expected_type: &'static str }`, and `NotFound(String)`.
   - Implement derive traits (`Debug`, `PartialEq`, `Clone`) for `ConfigError`.
   - Construct a `Config` struct encapsulating internal storage via `HashMap<String, String>`.

2. **Business Logic & API (`impl` Methods):**
   - **`new() -> Self`**: Create an empty `Config` instance.
   - **`parse(text: &str) -> Result<Self, ConfigError>`**: Parse a configuration string line-by-line, stripping whitespace and skipping empty lines or comments (starting with `#`).
   - **`get(&self, key: &str) -> Option<&str>`**: Retrieve a reference to a raw string value by key.
   - **`get_as<T: FromStr>(&self, key: &str) -> Result<T, ConfigError>`**: Safely fetch and convert a stored string value into a target type `T`.

3. **Architecture (`lib.rs` / `main.rs`):**
   - Keep all parsing logic, data structures, and `ConfigError` variants inside `src/lib.rs`.
   - Implement a CLI interface and robust input sanitizer (`parse_input`) in `src/main.rs` to parse configuration inputs interactively or from sample text.

4. **Testing Suite:**
   - Include unit tests (`#[cfg(test)]`) in `src/lib.rs` covering valid line parsing, comment skipping, empty key detection, type conversion success/failure, and missing key lookups.

---

## 🛠 Tech Stack & Core Concepts

- **Language & Edition:** Rust (Edition 2021)
- **External Dependencies:** None (Pure Standard Library)
- **Standard Library Components:**
  - `std::collections::HashMap` (Key-value map storage)
  - `std::str::FromStr` (Generic string parsing trait for typed values)
  - `std::result::Result` & `std::option::Option` (Explicit error and optional value handling)
- **Language Features:**
  - Custom error design via `enum` variants with structured fields.
  - Zero-copy string slicing (`&str`) during input sanitization and line splitting.
  - Generic methods with trait bounds (`<T: FromStr>`).
  - Pattern matching with `if let` and `match` constructs.

---

## 🚀 Building & Running

From the workspace root directory:

```bash
# Build and run the project
cargo run -p p7-simple-config-parser

# Run unit tests
cargo test -p p7-simple-config-parser
```

## 💡 Expected Behavior Example
```bash
=== Simple Config Parser ===
Parsing configuration string:

# Server configuration
host = 127.0.0.1
port = 8080
debug = true
max_connections = 100

[+] Config parsed successfully!

Retrieved values:
- Host (str): 127.0.0.1
- Port (usize): 8080
- Debug flag (bool): true
- Max Connections (usize): 100

Error handling check:
[!] Looking for missing key 'timeout': NotFound("timeout")
[!] Invalid type cast for 'host' as usize: ParseValueError { key: "host", expected_type: "usize" }

```