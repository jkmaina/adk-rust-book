# Chapter 14 Deployment Launcher

This crate adapts `../adk-playground/docs_examples/deployment/launcher_test/src/basic.rs` into a deployment-oriented example for the book.

## What It Demonstrates

- packaging a launcher-based agent as a CLI binary
- using the same binary for interactive chat or HTTP serving
- separating offline validation from opt-in live smoke execution

## ADK-Rust 0.8.0 Connection

Local `0.8.0` makes the launcher example more than a convenience wrapper. `Launcher` now also exposes `build_app()` and `build_app_with_a2a(...)` for teams that want ADK server wiring while still owning the surrounding Axum application. This crate keeps the example simple by teaching the binary packaging path first, then validating the same runtime through `Runner`.

## Run

```bash
cargo run -p chapter14-deployment-launcher
```

To use the launcher modes directly:

```bash
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter14-deployment-launcher -- chat
cargo run -p chapter14-deployment-launcher -- serve --port 8080
```
