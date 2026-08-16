# P4: Text Stat Calculator (`p4-text-stat-calculator`)

Part of the **Rust Intensive (5-Month Systems Engineering Track)** — Month 0 Foundations.

---

## 📌 Project Overview
`p4-text-stat-calculator` is a high-performance text analysis CLI tool designed to practice memory efficiency in Rust. It explores the core distinctions between owned strings (`String`) and string slices (`&str`), leveraging zero-allocation techniques, string iteration (Unicode scalar values vs. bytes), and iterator combinators.

The utility reads a multi-line or single-line text input from standard input (`stdin`) and produces detailed statistics without making unnecessary heap allocations during processing.

---

## 🎯 Technical Requirements

1. **Zero-Allocation Text Analysis:**
   - Accept input text as an immutable string slice reference (`&str`).
   - Calculate text metrics **without** cloning strings or allocating temporary vectors/collections where possible.
2. **Computed Metrics:**
   - **Total Characters:** Total count of Unicode scalar values (`char`).
   - **Total Bytes:** Exact memory size in bytes (`UTF-8`).
   - **Total Words:** Count of non-whitespace tokens (using `split_whitespace()`).
   - **Total Lines:** Count of lines (using `lines()`).
   - **Longest Word:** Identify and return a reference (`&str`) to the longest word in the text without allocating a new `String`.
3. **Interactive & Batch Processing:**
   - Prompts the user to enter text via `stdin` until an empty line or EOF is encountered.
   - Outputs a clean, formatted statistical summary table to `stdout`.

---

## 🛠 Tech Stack & Core Concepts

- **Language & Edition:** Rust (Edition 2021)
- **Standard Library Components:**
  - `std::primitive::str` (Borrowing & Slicing)
  - `std::iter::Iterator` (`map`, `max_by_key`, `filter`, `count`)
  - `std::io::{self, Read, Write}`
- **Language Features:**
  - `String` vs `&str` semantics (Ownership vs Borrowing).
  - Lifetimes in function signatures (returning references derived from input slices: `fn longest_word<'a>(text: &'a str) -> &'a str`).
  - UTF-8 awareness (`char_indices()`, `chars().count()` vs `len()`).

---

## 🚀 Building & Running

From the workspace root directory:

```bash
# Build and run the project
cargo run -p p4-text-stat-calculator

# Run unit and benchmark tests
cargo test -p p4-text-stat-calculator
```

## Expected Behavior Example

Enter or paste your text (press Enter on empty line to analyze):
> Rust is a systems programming language that runs blazingly fast,
> prevents segfaults, and guarantees thread safety.

================ TEXT STATISTICS ================
Lines:          2
Words:          13
Characters:     118
Bytes:          118
Longest Word:   programming (11 chars)
=================================================

---