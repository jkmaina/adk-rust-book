# Prelude Example

Discover what's **automatically available** in every Rust program.

## What This Example Shows

- **Primitive types**: `i32`, `f64`, `bool`, `char`, `&str` (all available without imports)
- **Collections**: `Vec`, `String`
- **Core enums**: `Option`, `Result`
- **Common traits**: `Clone`, `Copy`, `Debug`, `Display`, `Eq`, `Ord`, `PartialEq`, `PartialOrd`
- **Useful macros**: `println!`, `assert!`, `assert_eq!`

## Run It

```bash
cargo run
```

## Key Concepts

- The prelude is automatically imported into every Rust crate
- You can use `Vec`, `String`, `Option`, `Result` without `use` statements
- Traits and macros are part of the prelude, making common operations available by default
- See https://doc.rust-lang.org/std/prelude/ for the full list
