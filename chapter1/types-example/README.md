# Rust Type System Example

Explore Rust's **static type system** and how it infers types.

## What This Example Shows

- **Integer types**: `i32` (default), `u32`, `i64`, `u8`, etc.
- **Floating-point**: `f64` (default), `f32`
- **Booleans & characters**: `bool`, `char` (Unicode)
- **Tuples**: Fixed-size, mixed types (`(i32, f64, u8)`)
- **Arrays**: Fixed-size, same type (`[1, 2, 3, 4, 5]`)
- **Type inference**: Rust figures out types automatically
- **Type casting**: Using `as` for explicit conversions

## Run It

```bash
cargo run
```

## Key Concepts

- Rust is **statically typed**—types are checked at compile time
- Type inference means you often don't need to annotate types
- Tuples let you group different types; arrays group the same type
- Default types: `i32` for integers, `f64` for floats
