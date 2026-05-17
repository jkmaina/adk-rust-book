# Chapter 13 Quickstart Validation

This crate adapts the validated README quickstart snippet from `../adk-playground/docs_examples/readme_validation/src/gemini_basic.rs` into a smoke-friendly book example.

## What It Demonstrates

- separating offline compile validation from live smoke validation
- validating the `Launcher` quickstart wiring against ADK-Rust 0.8
- running one deterministic prompt through the runner stack when live smoke is enabled

## ADK-Rust 0.8.2 Connection

Local `0.8.2` makes this example slightly more instructive than it first appears. The validator now has to track the current runner boundary correctly, including the fuller `RunnerConfig` shape used by manual runner construction, while the live provider path stays optional. That is exactly the kind of runtime drift a smoke-friendly validation crate should catch early.

## Run

```bash
cargo run -p chapter13-quickstart-validation
```

To exercise the live path:

```bash
export BOOK_RUN_LIVE_SMOKE=1
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter13-quickstart-validation
```

## Expected Behavior

Without environment variables, the binary prints the offline validation guidance and exits successfully. With live smoke enabled, it creates the agent, validates the launcher wiring, runs one prompt, and prints the returned text.
