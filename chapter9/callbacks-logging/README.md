# Chapter 9 Callbacks Logging

This crate is the book adaptation of `../adk-playground/playground/backend/examples/callbacks_logging.rs`.

## What It Demonstrates

- registering `before_callback` and `after_callback` hooks on an `LlmAgent`
- logging agent, session, and user context around a live run
- keeping callback logic side-effect oriented while the agent still produces its normal answer

## ADK-Rust 0.8.0 Connection

This example teaches the smallest stable callback pattern in the local `0.8.0` repo: observe the run boundary without changing the agent's job. The broader framework now also supports richer callback surfaces, including model and tool interception, but this crate stays focused on the first pattern readers need to understand.

## Run

```bash
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter9-callbacks-logging
```

## Expected Behavior

The program prints callback log lines before and after the agent run, then streams the model response to the terminal.
