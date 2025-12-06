# Functions Example

Learn how to **define and call functions** in Rust.

## What This Example Shows

- **Function definition**: Using `fn` keyword
- **Parameters**: Passing values with type annotations
- **Return types**: Explicit return type annotations
- **Return values**: Implicit returns (no semicolon) vs explicit `return`
- **Calling functions**: Different ways to invoke functions

## Run It

```bash
cargo run
```

## Key Concepts

- Functions are declared with `fn function_name(param: Type) -> ReturnType { ... }`
- Return types are **required** in function signatures
- The last expression in a function is the return value (no semicolon needed)
- Functions can be called before they're defined (Rust scans ahead)
