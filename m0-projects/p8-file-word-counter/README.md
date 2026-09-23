# P8: File Word Counter (`p8-file-word-counter`)
---

## 📌 Project Overview
`p8-file-word-counter` is a command-line tool and library designed to practice file system I/O, streaming text data efficiently using `BufReader`, memory-conscious line-by-line processing, and custom error propagation when working with files.

The application reads a target text file from the filesystem, processes its contents in a streaming fashion (avoiding reading the entire file into memory at once), and computes key text metrics: total lines, words, characters, and bytes count.

---

## 🎯 Technical Requirements

1. **Domain Modeling & Error Handling:**
   - Define a custom `FileCounterError` enum covering all failure states: `IoError(String)`, `FileNotFound(String)`, and `InvalidPath`.
   - Implement derive traits (`Debug`, `PartialEq`, `Clone`) for `FileCounterError`.
   - Define a `TextStats` struct containing fields: `lines: usize`, `words: usize`, `chars: usize`, `bytes: usize`.

2. **Business Logic & API (`impl` Methods):**
   - **`TextStats::new() -> Self`**: Create a `TextStats` instance initialized with zeros.
   - **`count_from_reader<R: BufRead>(reader: R) -> Result<TextStats, FileCounterError>`**: Stream text line-by-line from any buffer reader, updating lines, words (split by whitespace), characters (`chars().count()`), and byte length.
   - **`count_file<P: AsRef<Path>>(path: P) -> Result<TextStats, FileCounterError>`**: Open a file safely using `File::open`, wrap it in `BufReader`, and delegate processing to `count_from_reader`.

3. **Architecture (`lib.rs` / `main.rs`):**
   - Keep all counting logic, stream processing, data structures, and `FileCounterError` variants inside `src/lib.rs`.
   - Implement a CLI interface in `src/main.rs` that accepts a file path as an argument, triggers counting, and prints a formatted summary to standard output.

4. **Testing Suite:**
   - Include unit tests (`#[cfg(test)]`) in `src/lib.rs` using in-memory byte slices (`&[u8]` as `BufRead`) to verify accurate line, word, character, and byte calculations, including empty file edge cases.

---

## 🛠 Tech Stack & Core Concepts

- **Language & Edition:** Rust (Edition 2021)
- **External Dependencies:** None (Pure Standard Library)
- **Standard Library Components:**
  - `std::fs::File` (File handle opening and access)
  - `std::io::{BufReader, BufRead}` (Buffered, memory-efficient streaming reads)
  - `std::path::Path` & `std::path::PathBuf` (Safe path handling and cross-platform abstractions)
  - `std::result::Result` (Explicit error management for I/O operations)
- **Language Features:**
  - Streaming I/O with generic traits (`BufRead`).
  - Memory-efficient line processing using `lines()` iterator.
  - Generics with trait bounds (`<P: AsRef<Path>>`).
  - Error wrapping and mapping from standard `std::io::Error`.

---

## 🚀 Building & Running

From the workspace root directory:

```bash
# Build and run the project with a file argument
cargo run -p p8-file-word-counter -- sample.txt

# Run unit tests
cargo test -p p8-file-word-counter
```

## 💡 Expected Behavior Example
```bash
=== File Word Counter ===
Reading file: "sample.txt"

[+] File processed successfully!

Statistics Summary:
- Total Lines:      12
- Total Words:      148
- Total Characters: 982
- Total Bytes:      995 bytes

Error handling check:
[!] Attempting to open missing file 'non_existent.txt':
    Error: FileNotFound("non_existent.txt")
```