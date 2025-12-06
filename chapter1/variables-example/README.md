# Variables Example

Master Rust's **variable binding and mutability** system.

## What This Example Shows

- **Immutable variables**: Default behavior with `let`
- **Mutable variables**: Using `let mut` to allow changes
- **Variable shadowing**: Rebinding names with `let`
- **Scope**: Variables are scoped to their block
- **Type inference**: Rust infers types from initialization

## Run It

```bash
cargo run
```

## Key Concepts

- Variables are **immutable by default** (safety-first design)
- Use `let mut` explicitly to allow reassignment
- Shadowing allows rebinding a name with a new type or value
- Scope matters—variables are dropped when out of scope
- Pattern matching in `let` (destructuring)
