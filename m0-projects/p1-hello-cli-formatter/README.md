# P1: Hello CLI Formatter (`p1-hello-cli-formatter`)

## 📌 Project Overview
`p1-hello-cli-formatter` is a simple, interactive Command Line Interface (CLI) application built to master fundamental I/O operations, string manipulations, and error handling mechanics in Rust without triggering runtime panics.

The utility prompts the user for their name and age via standard input (`stdin`), parses and validates the inputs, and outputs a formatted access status message.

---

## 🎯 Technical Requirements

1. **Standard I/O:**
   - Prompt the user and safely read lines from `std::io::stdin`.
   - Strip trailing newline characters (`\n` / `\r\n`) using string slicing techniques.
2. **Parsing & Validation:**
   - Convert the age string into a valid unsigned integer (`u8`).
   - Gracefully handle invalid inputs (e.g., non-numeric strings, overflow) without calling `panic!`.
3. **Control Flow:**
   - Age $\ge 18$: Output access granted status to `stdout`.
   - Age $< 18$: Output restricted access status to `stdout`.
   - Parsing Error: Print a human-readable error message to `stderr` (`eprintln!`).

---

## 🛠 Tech Stack & Core Concepts

- **Language & Edition:** Rust (Edition 2021)
- **Standard Library Components:**
  - `std::io::{self, Write}` (Input/Output handling & buffer flushing)
  - `str::trim()` (Whitespace / newline stripping)
  - `str::parse::<u8>()` (Primitive type parsing)
- **Language Features:**
  - `String` allocation & mutable references (`&mut String`)
  - Pattern matching (`match` / `if let`) on `Result<T, E>`
  - Console output routing (`println!` vs `eprintln!`)

---

## 🚀 Building & Running

From the workspace root directory:

```bash
# Build and run the project
cargo run -p p1-hello-cli-formatter

# Run tests (if applicable)
cargo test -p p1-hello-cli-formatter
```

---

💡 Expected Behavior Example
```bash
Enter your name: Alice
Enter your age: 24
[SUCCESS] Hello, Alice! Access granted.

Enter your name: Bob
Enter your age: abc
[ERROR] Invalid age format. Please enter a valid number (0-255).
```