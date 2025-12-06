# Borrowing Example

Understand Rust's **borrowing** system—how to share data without transferring ownership.

## What This Example Shows

- **Creating a String** with `String::from()`
- **Passing references**: Using `&s1` instead of moving `s1`
- **Function parameters with references**: `fn calculate_length(s: &String) -> usize`
- **Original ownership preserved**: `s1` is still valid after the function call

## Run It

```bash
cargo run
```

## Key Concepts

- `&s1` creates a **reference** (borrow) without taking ownership
- References are immutable by default
- The function only borrows—it can't modify the data (unless `&mut`)
- No ownership transfer = original variable stays valid
