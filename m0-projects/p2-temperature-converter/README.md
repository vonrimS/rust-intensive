# P2: Temperature Converter (`p2-temperature-converter`)

Part of the **Rust Intensive (5-Month Systems Engineering Track)** — Month 0 Foundations.

---

## 📌 Project Overview
`p2-temperature-converter` is a CLI tool designed to practice core numeric operations, function signatures with explicit types, precision handling on floating-point numbers (`f64`), and controlled string-to-number/enum parsing in Rust.

The utility converts temperatures between Celsius and Fahrenheit (both ways) based on user selection, applying floating-point precision formatting to the final output.

---

## 🎯 Technical Requirements

1. **Conversion Logic:**
   - Implement `celsius_to_fahrenheit(c: f64) -> f64` using formula: $F = (C \times 9/5) + 32$.
   - Implement `fahrenheit_to_celsius(f: f64) -> f64` using formula: $C = (F - 32) \times 5/9$.
2. **Interactive CLI Menu:**
   - Prompt user to select target conversion scale (`1` for Celsius to Fahrenheit, `2` for Fahrenheit to Celsius).
   - Read the input value as `f64`.
3. **Parsing & Precision:**
   - Parse input strings safely into floating-point numbers (`f64`) without triggering `panic!`.
   - Format output values to exactly **2 decimal places** (e.g., `21.50°C`).
4. **Error Handling:**
   - Catch invalid menu selections and bad numeric inputs, printing human-readable errors to `stderr`.

---

## 🛠 Tech Stack & Core Concepts

- **Language & Edition:** Rust (Edition 2021)
- **Standard Library Components:**
  - Primitive types (`f64`) and floating-point arithmetic.
  - Display formatting macros (`println!("{:.2}", value)`).
- **Language Features:**
  - Explicit function signatures with primitive types.
  - Strongly typed `enum` for conversion modes.
  - Immutable (`let`) vs Mutable (`let mut`) bindings.
  - Unit testing (`#[cfg(test)]`) for mathematical formulas.

---

## 🚀 Building & Running

From the workspace root directory:

```bash
# Build and run the project
cargo run -p p2-temperature-converter

# Run unit tests for conversion logic
cargo test -p p2-temperature-converter
```

---

💡 Expected Behavior Example

```bash
Select mode:
1. Celsius -> Fahrenheit
2. Fahrenheit -> Celsius
Choice: 1
Enter temperature in Celsius: 25.5
25.50°C is equal to 77.90°F
```

```bash
Select mode:
1. Celsius -> Fahrenheit
2. Fahrenheit -> Celsius
Choice: 2
Enter temperature in Fahrenheit: 100
100.00°F is equal to 37.78°C
```