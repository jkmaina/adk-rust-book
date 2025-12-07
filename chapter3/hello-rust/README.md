# Hello Rust

**What:** Your first Rust program that prints "Hello, world!".

**Why:** Every programming journey starts here. This verifies your Rust installation works.

## What This Example Shows

- **Basic Rust project structure**: `Cargo.toml` and `src/main.rs`
- **The main function**: Entry point of every Rust program
- **println! macro**: Printing text to the console
- **Cargo commands**: Building and running Rust projects

## Run It

```bash
cargo run
```

Output:
```
Hello, world!
```

## Why This Matters

This simple program confirms:
- Rust is installed correctly
- Cargo (Rust's build tool) works
- You can compile and run Rust code

## Beginner's Explanation

**Cargo** is Rust's build tool and package manager:
- `cargo run` = compile + run your program
- `cargo build` = just compile (creates executable)
- `cargo check` = check for errors without building

**Project structure**:
```
hello-rust/
├── Cargo.toml    (project configuration)
└── src/
    └── main.rs   (your code)
```

**The code**:
```rust
fn main() {              // Entry point
    println!("Hello, world!");  // Print to console
}
```
