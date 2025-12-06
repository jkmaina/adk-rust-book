# Result & Error Handling Example

Master Rust's **Result** type for safe error handling.

## What This Example Shows

- **Result enum**: `Ok(value)` or `Err(error)`
- **match-based error handling**: Using `match` to handle success and failure
- **The `?` operator**: Propagates errors up the call stack
- **File operations**: Opening files and reading to string

## Run It

```bash
cargo run
```

## Key Concepts

- `Result<T, E>` is Rust's way to represent success or failure
- `match` gives you explicit control over error handling
- The `?` operator shorthand: returns the error if Result is Err, unwraps if Ok
- Errors propagate through the call stack without crashing
