# Functions Example

**What:** Learn how to write reusable blocks of code (functions).

**Why:** Functions let you write once, use many times. Keep your code clean and organized.

## What This Example Shows

- **Defining functions**: Using `fn` keyword
- **Parameters**: Accepting input with types like `fn greet(name: String) { ... }`
- **Return types**: Saying what type you'll give back `-> i32`
- **Returning values**: Without `return`, use the last expression
- **Calling functions**: Using them in your code

## Run It

```bash
cargo run
```

## Why This Matters

Functions let you:
- **Reuse code** (don't repeat yourself)
- **Organize logic** (break big problems into small ones)
- **Test easily** (test one function at a time)

## Beginner's Explanation

**Function structure:**
```rust
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
```

Break it down:
- `fn`: "I'm defining a function"
- `greet`: Function's name
- `(name: String)`: Parameters (input). `name` is a String
- `-> String`: Return type. This function gives back a String
- `{ ... }`: Function body (what it does)
- No semicolon on last line = return that value

**Calling it:**
```rust
greet("Alice");  // Call greet with "Alice" input
```

**Return types are REQUIRED** in Rust:
```rust
fn add(a: i32, b: i32) -> i32 {  // Must say "I return an i32"
    a + b  // No semicolon = return this value
}
```
