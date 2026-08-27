# P5: String Cleaner (`p5-string-cleaner`)
---

## 📌 Project Overview
`p5-string-cleaner` is a text-sanitization utility designed to practice in-place string mutation, borrowing mechanics, and working with `&mut String`. 

The application accepts user input and cleans it up by stripping leading and trailing whitespace, removing illegal or unwanted control characters, and replacing multiple consecutive spaces with a single space — all without creating unnecessary heap allocations.

---

## 🎯 Technical Requirements

1. **In-Place Mutation & Borrowing:**
   - Accept string buffers via mutable references (`&mut String`) to mutate data directly in-place.
   - Demonstrate clean borrowing boundaries between input reading and string transformation logic.
2. **Text Sanitization Pipeline:**
   - Trim leading and trailing whitespace.
   - Remove forbidden/control characters (e.g., non-printable Unicode or custom forbidden symbols).
   - Collapse multiple consecutive whitespace characters into a single space.
3. **Architecture (`lib.rs` / `main.rs`):**
   - Place core transformation functions and sanitization logic inside `src/lib.rs`.
   - Maintain a lightweight CLI interface in `src/main.rs` that reads from `stdin` and handles user output.
4. **Testing Suite:**
   - Include unit tests (`#[cfg(test)]`) in `src/lib.rs` to cover edge cases: empty strings, all-spaces input, consecutive tabs/spaces, and special Unicode characters.

---

## 🛠 Tech Stack & Core Concepts

- **Language & Edition:** Rust (Edition 2021)
- **External Dependencies:** None (Pure Standard Library)
- **Standard Library Components:**
  - `std::string::String` (Owned, mutable UTF-8 buffer)
  - `std::io::{self, Read, Write}`
- **Language Features:**
  - In-place mutation via mutable borrowing (`&mut String`).
  - Zero-allocation iterator pipelines (`.retain()`, `.trim()`).
  - Strict ownership boundaries between CLI layer and business logic.

---

## 🚀 Building & Running

From the workspace root directory:

```bash
# Build and run the project
cargo run -p p5-string-cleaner

# Run unit tests
cargo test -p p5-string-cleaner
```

💡 Expected Behavior Example

```Bash
Welcome to String Cleaner!
Enter text to sanitize:

>   Hello,    world!   \t  This   is   Rust!   

================ CLEANED RESULT ================
"Hello, world! This is Rust!"
================================================
```