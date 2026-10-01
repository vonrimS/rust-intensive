# P9: Shape Calculator (`p9-shape-calculator`)
---

## 📌 Project Overview
`p9-shape-calculator` is a library and command-line application designed to practice fundamental object-oriented design patterns in Rust. The project focuses on contract-driven development using traits, zero-cost abstractions through **static dispatch (Monomorphization)**, and generic programming basics.

The application defines a unified geometric interface (Shape) implemented by various concrete structures (`Circle`, `Rectangle`, `Triangle`). It provides utility functions to compute individual metrics (area, perimeter) and calculate aggregate statistics across collections of homogeneous geometric figures using static dispatch.

## 🎯 Technical Requirements
1. Domain Modeling & Traits:
    * Define a core `Shape` trait requiring two method signatures:
        * `fn area(&self) -> f64`
        * `fn perimeter(&self) -> f64`
    * Implement three concrete geometric structures:
        * `Circle { radius: f64 }`
        * `Rectangle { width: f64, height: f64 }`
        * `Triangle { a: f64, b: f64, c: f64 }` (calculating area using Heron's formula)
    * Derive standard traits (`Debug`, `Clone`, `PartialEq`) where appropriate.

2. Generic Utilities & Static Dispatch:
    * `print_shape_info<T: Display + Shape>(shape: &T)`: Accept any type implementing both `Shape` and `Display` via trait bounds, printing formatted details about the shape to standard output.
    * `total_area<T: Shape>(shapes: &[T]) -> f64`: Compute the total combined area of a slice of homogeneous shapes using compile-time static dispatch (`impl Trait` / generic parameters).

3. Architecture (`lib.rs` / `main.rs`):
    * Place all traits, shape definitions, mathematical formulas, and generic function utilities in `src/lib.rs`.
    * Implement `Display` for each shape to allow human-readable output formatting.
    * Construct a CLI runner in `src/main.rs` that instantiates sample shapes, calls static dispatch methods, and prints formatted summary statistics.

4. Testing Suite:
    * Write comprehensive unit tests (`#[cfg(test)]`) in `src/lib.rs`.
    * Test area and perimeter formulas against verified mathematical expectations for `Circle`, `Rectangle`, and `Triangle`.
    * Verify zero-value edge cases and aggregate computations performed by `total_area`.

## 🛠 Tech Stack & Core Concepts
* Language & Edition: Rust (Edition 2021)
* External Dependencies: None (Pure Standard Library)
* Standard Library Components:
    * `std::fmt::{Display, Formatter, Result}` (Custom string formatting for geometric types)
    * `std::f64::consts::PI` (Precision mathematical constants)

* Language Features:
    * Traits & Contracts: Polymorphic behavior abstractions without runtime overhead.
    * Static Dispatch: Generic monomorphization generating zero-cost specialized machine code at compile time.
    * Trait Bounds: Restricting generic types using syntax like `<T: Display + Shape>`.
    * Generics & Slices: Operating over slices of unknown concrete types that conform to trait contracts.

## 🚀 Building & Running
From the workspace root directory:

```Bash
# Build and run the project
cargo run -p p9-shape-calculator

# Run unit tests
cargo test -p p9-shape-calculator
```

## 💡 Expected Behavior Example
```Bash
=== Geometric Shape Calculator ===

--- Circle (r = 5.00) ---
Area:      78.54
Perimeter: 31.42

--- Rectangle (w = 4.00, h = 6.00) ---
Area:      24.00
Perimeter: 20.00

--- Triangle (a = 3.00, b = 4.00, c = 5.00) ---
Area:      6.00
Perimeter: 12.00

--- Aggregate Calculations (Static Dispatch) ---
Total area of 2 Rectangles: 48.00
```