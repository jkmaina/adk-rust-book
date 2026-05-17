# Chapter 12 CLI Launcher

This crate is the book adaptation of `../adk-playground/playground/backend/examples/cli_launcher.rs`.

## What It Demonstrates

- configuring `Launcher` with custom session and artifact services
- choosing a streaming mode for console runs
- validating the launcher configuration by running the same agent through `Runner`

## ADK-Rust 0.8.2 Connection

Local `0.8.2` makes `Launcher` a stronger production bridge than earlier drafts suggested. It is not only a console-or-serve convenience wrapper now; it also exposes `build_app()` paths for teams that want ADK wiring while still owning the surrounding web application shape. This crate keeps the example simple by showing the launcher configuration and then validating the same runtime through `Runner`.

## Run

```bash
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter12-cli-launcher
```

## Expected Behavior

The program should print the launcher configuration it would use for console or serve mode, then stream one demo answer from the underlying agent.
