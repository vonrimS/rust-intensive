# P10: Safe Bank Account (`p10-safe-bank-account`)
---

## 📌 Project Overview
`p10-safe-bank-account` is a library and command-line application designed to practice field privacy, data encapsulation, state validation, and custom error handling using `Result` and `Option` types in Rust.

The application models a bank account system (`BankAccount`) that strictly guards internal state (such as account balance and transaction logs) using private fields. All mutations—deposits, withdrawals, transfers, and balance queries—go through explicit, validated methods that enforce invariants like non-negative amounts and sufficient funds.

---

## 🎯 Technical Requirements

1. **Domain Modeling & Error Handling:**
   - Define a custom `AccountError` enum representing operational failure states:
     - `InvalidAmount` (e.g., negative or zero deposit/withdrawal)
     - `InsufficientFunds { balance: f64, requested: f64 }`
     - `AccountLocked`
   - Implement derive traits (`Debug`, `Clone`, `PartialEq`, `Eq`) for `AccountError`.
   - Define a private struct `Transaction` recording:
     - `id: usize`
     - `kind: TransactionKind` (`Deposit`, `Withdrawal`)
     - `amount: f64`
   - Define a `BankAccount` struct with strictly private fields:
     - `account_number: String`
     - `balance: f64`
     - `is_locked: bool`
     - `transactions: Vec<Transaction>`

2. **Business Logic & API (`impl` Methods):**
   - **`BankAccount::new(account_number: String, initial_deposit: f64) -> Result<Self, AccountError>`**: Construct an account ensuring the initial deposit is positive.
   - **`deposit(&mut self, amount: f64) -> Result<f64, AccountError>`**: Validate amount, update balance, log transaction, and return updated balance.
   - **`withdraw(&mut self, amount: f64) -> Result<f64, AccountError>`**: Validate amount and sufficient balance, update state, log transaction, and return updated balance.
   - **`balance(&self) -> f64`**: Read-only accessor for current balance.
   - **`last_transaction(&self) -> Option<&Transaction>`**: Safe optional access to the most recent transaction log without exposing internal vector mutation.
   - **`lock(&mut self)`** / **`unlock(&mut self)`**: Toggle account lock state to reject operations when locked.

3. **Architecture (`lib.rs` / `main.rs`):**
   - Keep all domain logic, private struct definitions, and error enums inside `src/lib.rs`.
   - Build a CLI runner in `src/main.rs` that demonstrates account creation, successful financial transactions, boundary error handling (overdraft, invalid amounts), and inspection via `Option`.

4. **Testing Suite:**
   - Write unit tests (`#[cfg(test)]`) in `src/lib.rs`.
   - Test successful deposits and withdrawals.
   - Verify boundary check failures (`InsufficientFunds`, `InvalidAmount`) and account locking mechanics.

---

## 🛠 Tech Stack & Core Concepts

- **Language & Edition:** Rust (Edition 2021)
- **External Dependencies:** None (Pure Standard Library)
- **Standard Library Components:**
  - `std::result::Result` (Explicit error management for state mutations)
  - `std::option::Option` (Safe optional values without null references)
  - `std::fmt::{Display, Formatter}` (Clean formatting for errors and transaction logs)
- **Language Features:**
  - **Encapsulation & Visibility (`pub` / private):** Hiding internal struct fields to maintain state invariants.
  - **Bounds Checking & State Validation:** Preventing invalid domain operations at runtime.
  - **Pattern Matching:** Handling operational outcomes via `match` and `if let`.

---

## 🚀 Building & Running

From the workspace root directory:

```bash
# Build and run the project
cargo run -p p10-safe-bank-account

# Run unit tests
cargo test -p p10-safe-bank-account
```

## 💡 Expected Behavior Example
```bash
=== Safe Bank Account Management System ===

[+] Created account: ACC-1001 with initial deposit: $100.00
Current Balance: $100.00

--- Executing Transactions ---
[+] Deposited: $50.00 | New Balance: $150.00
[+] Withdrew:  $30.00 | New Balance: $120.00

--- Inspecting Last Transaction ---
Last Action: Withdrawal of $30.00

--- Boundary & Error Checks ---
[!] Attempting invalid deposit ($0.00):
    Error: InvalidAmount

[!] Attempting overdraft withdrawal ($200.00):
    Error: InsufficientFunds { balance: 120.0, requested: 200.0 }
```

