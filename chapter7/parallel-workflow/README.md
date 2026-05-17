# Chapter 7 Parallel Workflow

This crate is the book adaptation of `../adk-playground/playground/backend/examples/parallel.rs`.

## What It Demonstrates

- running multiple specialists concurrently
- combining perspectives in one orchestration layer
- using `ParallelAgent` when order does not need to be fixed

## ADK-Rust 0.8.0 Connection

- the example shows the current `ParallelAgent` shape directly instead of hiding it behind a higher-level abstraction
- `0.8.0` hardened workflow behavior around callbacks and error handling, which makes parallel orchestration a safer building block
- every analyst receives the same prompt, so the value comes from perspective diversity rather than staged handoff
- the explicit `Runner` path and typed runtime identity remain visible for the reader

## Run

```bash
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter7-parallel-workflow
```

## Expected Behavior

The program asks whether a startup should adopt WebAssembly for its web app. The result should feel like a combined analysis from technical, business, and user-experience viewpoints rather than one single blended persona pretending to be all three at once.
