# Prelude Example

**What:** Discover what "just works" in every Rust program without imports.

**Why:** Rust automatically gives you the most common types and functions. This saves you from writing `use` statements for everyday things.

## What This Example Shows

- **Number types**: `i32`, `f64`, `bool` (available instantly)
- **Text**: `String`, `char`, `&str`
- **Collections**: `Vec` (lists), arrays
- **Special enums**: `Option` (something or nothing), `Result` (success or failure)
- **Macros you've used**: `println!`, `assert!`
- **Methods**: `to_string()`, `push()`, `len()`

## Run It

```bash
cargo run
```

## Why This Matters

**"Prelude"** is Rust's way of saying: "Here are the essentials."

You've been using prelude types all along without knowing it:
- When you write `let x = 42`, that's an `i32` from prelude
- When you write `println!()`, that's a macro from prelude
- When you write `let items = vec![1, 2, 3]`, that's `Vec` from prelude

## Beginner's Explanation

Without the prelude, you'd have to write:
```rust
use std::prelude::v1::*;  // Every single file!
use std::vec::Vec;
use std::string::String;
// ... dozens more
```

With the prelude, you just write:
```rust
let numbers = vec![1, 2, 3];  // Vec is ready to go!
```

Rust does this automatically so you can focus on your code, not imports.
