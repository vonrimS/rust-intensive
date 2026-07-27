# P3: Number Guesser (`p3-number-guesser`)
---

## 📌 Project Overview
`p3-number-guesser` is a classic interactive guessing game designed to practice continuous execution loops (`loop`), external crate integration (`rand`), pattern matching on numeric comparisons using `std::cmp::Ordering`, and keeping track of state (attempt counts) across iterations.

The application generates a secret random number within a designated range (1 to 100) and prompts the user for guesses until the correct number is identified.

---

## 🎯 Technical Requirements

1. **Random Number Generation:**
   - Integrate the external `rand` crate into `Cargo.toml`.
   - Generate a secret unsigned integer in the range $1..=100$.
2. **Game Loop & State:**
   - Run an interactive loop (`loop`) that prompts the user for input on each iteration.
   - Track total user attempts using a mutable counter (`let mut attempts`).
3. **Comparison & Evaluation:**
   - Compare the user's parsed guess against the secret number using `u32::cmp(&secret)`.
   - Match on `std::cmp::Ordering` variants:
     - `Ordering::Less`: Print "Too small!" and continue loop.
     - `Ordering::Greater`: Print "Too big!" and continue loop.
     - `Ordering::Equal`: Print win message with total attempt count and break the loop (`break`).
4. **Input Handling & Resiliency:**
   - Gracefully handle invalid (non-numeric) inputs without crashing or terminating the game (skip invalid turns with `continue`).

---

## 🛠 Tech Stack & Core Concepts

- **Language & Edition:** Rust (Edition 2021)
- **External Dependencies:**
  - `rand` (Random number generator)
- **Standard Library Components:**
  - `std::cmp::Ordering` (Enum representing relative order of two values: `Less`, `Equal`, `Greater`)
  - `std::io::{self, Write}`
- **Language Features:**
  - Infinite loops (`loop`) with explicit control flow (`break`, `continue`).
  - Expression-based pattern matching (`match`).
  - Mutable state management across iterations.

---

## 🚀 Building & Running

From the workspace root directory:

```bash
# Build and run the project
cargo run -p p3-number-guesser
```

💡 Expected Behavior Example

```bash
Welcome to Guess the Number!
I'm thinking of a number between 1 and 100.

Please input your guess: 50
Too small!

Please input your guess: 75
Too big!

Please input your guess: abc
[WARNING] Please enter a valid number!

Please input your guess: 62
You win! You guessed the number in 3 attempts!
```