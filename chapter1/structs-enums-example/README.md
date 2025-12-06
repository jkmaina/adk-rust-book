# Structs & Enums Example

Learn how to define **custom types** using structs and enums.

## What This Example Shows

- **Structs**: Named data types with fields (`User` with username, email, etc.)
- **Methods**: Impl blocks to define behavior (`User::new()`)
- **Enums**: Types with multiple possible variants (`Message::Quit`, `Move { x, y }`, etc.)
- **Pattern matching**: Using `match` to handle enum variants

## Run It

```bash
cargo run
```

## Key Concepts

- Structs bundle related data together
- Enums allow a value to be one of several options
- `impl` blocks add methods to types
- `match` exhaustively handles all enum variants
