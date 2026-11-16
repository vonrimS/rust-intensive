# Custom Raw Vector (`custom-vector-raw`)

## Project Overview
`custom-vector-raw` is a low-level, zero-dependency Rust library implementing a contiguous, growable array (`RawVec<T>` / `CustomVec<T>`) from scratch using manual memory management. 

Built without relying on `std::vec::Vec`, the project demonstrates safe encapsulation of `unsafe` Rust primitives—managing raw pointers, manual heap allocations, layout calculations, deterministic `Drop` semantics, and Zero-Sized Type (ZST) edge cases behind a sound, idiomatic safe API.

---

## Core Philosophy: Safe Abstractions Over Unsafe Primitives

In systems programming, high-level collections rely on strict invariants to prevent Undefined Behavior (UB), memory leaks, and dangling pointers. `custom-vector-raw` separates memory management from element lifecycle management into two distinct layers:

1. **Layer 1: Raw Memory Buffer (`RawVec<T>`)**  
   Handles low-level heap allocation, geometric capacity growth (`amortized O(1)`), and deallocation via `std::alloc`, completely agnostic to initialized elements.
2. **Layer 2: Typed Collection (`CustomVec<T>`)**  
   Tracks initialized length (`len`), enforces bounds checking, performs raw pointer reads/writes (`ptr::read`, `ptr::write`), and guarantees deterministic destruction via the `Drop` trait.

- **Soundness & Miri Verification:** Every `unsafe` block is isolated, documented with explicit `// SAFETY:` invariants, and verified under Miri to detect out-of-bounds accesses, use-after-free, and stacked borrows violations.
- **Zero-Sized Types (ZST) Support:** Handles zero-byte types (e.g., `()`) without performing illegal zero-sized system allocations, utilizing `NonNull::dangling()` and virtual capacity (`usize::MAX`).

---

## Architecture & Memory Management

- **Raw Pointer Representation (`NonNull<T>` & `PhantomData<T>`):** Uses `NonNull<T>` to enable niche pointer optimizations (`Option<CustomVec<T>>` has the same size as `CustomVec<T>`) and `PhantomData<T>` to inform the Rust borrow checker and drop checker (`#[may_dangle]` / ownership semantics).
- **Manual Heap Allocation (`std::alloc`):** Directly computes memory layouts (`Layout::array::<T>(cap)`), handles out-of-memory (OOM) conditions via `handle_alloc_error`, and performs in-place reallocation (`alloc::realloc`) while guarding against `isize::MAX` overflow.
- **Deterministic Cleanup (`Drop` & `IntoIter`):** Implements `Drop` to explicitly invoke `ptr::drop_in_place` on all initialized elements (`0..len`) before freeing the backing heap buffer, preventing resource leaks even during unwinds or partial iterator consumption.

---

## Library Usage & Testing

### Running Tests & Miri Checks

```bash
# Run standard unit and integration test suite
cargo test

# Run tests with output logs
cargo test -- --nocapture

# Verify memory safety and Stacked Borrows compliance via Miri
cargo +nightly miri test

# Run benchmarks comparing CustomVec<T> against std::vec::Vec
cargo bench
```

## Code Example
```rust
use custom_vector_raw::CustomVec;

fn main() {
    let mut vec = CustomVec::new();
    vec.push(String::from("Unsafe"));
    vec.push(String::from("Rust"));
    vec.push(String::from("Rocks"));

    assert_eq!(vec.len(), 3);
    assert_eq!(vec.pop(), Some(String::from("Rocks")));

    // Safe slice dereferencing via Deref / DerefMut

    let slice: &[String] = &vec;
    assert_eq!(slice.len(), 2);

    for item in &vec {
        println!("Element: {}", item);
    }
} // Drop automatically runs destructors for remaining Strings and frees heap memory
```

## Output Examples
Miri Memory Safety Verification (cargo +nightly miri test)

```bash
test tests::test_push_pop_and_reallocation ... ok
test tests::test_drop_execution_count ... ok
test tests::test_zero_sized_types ... ok
test tests::test_into_iter_partial_drop ... ok
test tests::test_drain_and_insert_shift ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Tech Stack
- Language: Rust (Edition 2024)
- Core Modules: std::alloc, std::ptr, std::mem, std::marker::PhantomData
- Verification & Tooling: Cargo Test, Miri (Undefined Behavior & memory leak sanitizer)

## Roadmap & Future Horizon
1. Custom Allocator API (Allocator trait):
    - Parameterize CustomVec<T, A: Allocator> to support arena allocators, bump allocators, and custom memory pools.
2. Small Buffer Optimization (SBO):
    - Store small collections inline on the stack (similar to SmallVec) before spilling over to heap allocation when capacity exceeds inline threshold.