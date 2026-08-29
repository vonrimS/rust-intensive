# P6: Todo Memory List (`p6-todo-memory-list`)
---

## 📌 Project Overview
`p6-todo-memory-list` is an in-memory task management utility designed to practice `struct` design, method implementations (`impl`), `enum` state representation, and ownership dynamics when working with `Vec<T>`.

The application provides a clean CLI interface allowing users to create, view, mark as completed, and delete tasks — maintaining an isolated task collection in memory with unique auto-incrementing IDs and safe error handling.

---

## 🎯 Technical Requirements

1. **Domain Modeling & Data Structures:**
   - Define a `Status` enum (`Pending`, `Done`) with appropriate derive traits (`Debug`, `Equals`, `Clone`).
   - Construct a `TodoItem` struct holding unique `id: usize`, `title: String`, and `status: Status`.
   - Encapsulate task storage within a `TodoList` struct managing a internal `Vec<TodoItem>`.
2. **Business Logic & API (`impl` Methods):**
   - **`add_item(&mut self, title: &str) -> &TodoItem`**: Insert new tasks with auto-incrementing IDs.
   - **`mark_done(&mut self, id: usize) -> Result<(), String>`**: Toggle task status safely, returning clear error messages for invalid IDs.
   - **`remove_item(&mut self, id: usize) -> Option<TodoItem>`**: Remove and return ownership of an item from the vector.
   - **`list_items(&self) -> &[TodoItem]`**: Borrow the inner task list as a read-only slice.
3. **Architecture (`lib.rs` / `main.rs`):**
   - Keep all data structures, methods, and error-handling logic inside `src/lib.rs`.
   - Implement an interactive CLI command loop in `src/main.rs` for processing user actions (`add`, `list`, `done`, `remove`, `exit`).
4. **Testing Suite:**
   - Include unit tests (`#[cfg(test)]`) in `src/lib.rs` covering task insertion, status mutation, deletion, out-of-bounds ID lookup, and ID generation state.

---

## 🛠 Tech Stack & Core Concepts

- **Language & Edition:** Rust (Edition 2021)
- **External Dependencies:** None (Pure Standard Library)
- **Standard Library Components:**
  - `std::vec::Vec` (Contiguous growable array)
  - `std::option::Option` & `std::result::Result` (Safe state & error handling)
  - `std::io::{self, Write}` (Interactive CLI I/O)
- **Language Features:**
  - Custom data structures using `struct` and `enum`.
  - Encapsulated behavior via `impl` blocks and constructor patterns (`new()`).
  - Ownership transfers and element removal in collections (`Vec::retain`, `Vec::remove`, or `Vec::swap_remove`).
  - Borrowing slices (`&[T]`) for efficient zero-copy read operations.

---

## 🚀 Building & Running

From the workspace root directory:

```bash
# Build and run the project
cargo run -p p6-todo-memory-list

# Run unit tests
cargo test -p p6-todo-memory-list
```

## 💡 Expected Behavior Example

```Plaintext
=== Todo Memory List ===
Commands: add <title> | list | done <id> | remove <id> | exit

> add Buy milk
[+] Added task #1: "Buy milk"

> add Read Rust Book
[+] Added task #2: "Read Rust Book"

> list
================ TODO LIST ================
[1] [Pending] Buy milk
[2] [Pending] Read Rust Book
===========================================

> done 1
[✓] Task #1 marked as Done!

> list
================ TODO LIST ================
[1] [Done]    Buy milk
[2] [Pending] Read Rust Book
===========================================
```