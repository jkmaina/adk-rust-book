# Rust Type System Example

**What:** Learn about Rust's different data types and how it figures out what type you mean.

**Why:** Every piece of data has a type. Understanding types helps you write correct code.

## What This Example Shows

- **Integers**: Whole numbers (`i32`, `u32`, `i64`, `u8`)
- **Floats**: Decimals (`f64`, `f32`)
- **Booleans**: `true` or `false`
- **Characters**: Single Unicode characters like `'A'` or `'🦀'`
- **Tuples**: Mix different types together like `(42, 3.14, true)`
- **Arrays**: Lists of the same type like `[1, 2, 3, 4, 5]`
- **Type inference**: Rust figures out types automatically

## Run It

```bash
cargo run
```

## Why This Matters

**"Static typing"** means Rust checks all types at compile time (before running).

Benefits:
- **Catches bugs early**: Wrong type = compiler error
- **No surprises**: You know what type everything is
- **Performance**: Types are known ahead of time

## Beginner's Explanation

**Types are like containers:**

- **i32**: Container for whole numbers (-2 billion to +2 billion)
- **f64**: Container for decimals (3.14, 2.71828, etc.)
- **bool**: Container for true/false
- **String**: Container for text
- **char**: Container for a single character

**Tuples**: Mix containers together
```
(42, 3.14, true)
 ↓   ↓      ↓
i32  f64   bool
```

**Arrays**: Same container repeated
```
[1, 2, 3, 4, 5]
 ↓  ↓  ↓  ↓  ↓
i32 i32 i32 i32 i32
```

**Type inference**: Rust is smart
```
let x = 42;          // Rust: "That's an i32"
let pi = 3.14;       // Rust: "That's an f64"
let greeting = "hi"; // Rust: "That's a &str"
```
