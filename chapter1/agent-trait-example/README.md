# Agent Trait Example

Learn how to define **async traits** for flexible agent interfaces.

## What This Example Shows

- **Async traits**: Using `#[async_trait]` to write traits with `async fn` methods
- **Trait implementation**: Implementing an `Agent` trait for `CustomerServiceAgent`
- **Multiple methods**: Both async (`handle`) and sync (`name`)
- **Calling async methods**: Using `.await` on trait methods

## Run It

```bash
cargo run
```

## Key Concepts

- The `async-trait` crate enables `async fn` inside trait definitions
- Traits define contracts—types implement them
- `#[async_trait]` macro transforms async trait methods into Futures
- Useful for building extensible agent systems with pluggable behavior
