# Chapter 6 Function Tool

This crate is the book adaptation of `../adk-playground/playground/backend/examples/function_tool.rs`.

## What It Demonstrates

- typed tool arguments with `#[tool]`
- a single agent using multiple small function tools
- delegating weather and time questions to tools instead of free-form guessing

## ADK-Rust 0.8.0 Connection

- this is the recommended `#[tool]` path for custom capabilities in the current repo
- the example keeps the explicit `Runner` boundary and typed `UserId` / `SessionId` values visible
- each tool returns structured data, leaving the model to narrate the user-facing answer
- `FunctionTool::new(...)` still exists in `0.8.0`, but this example uses the clearer macro-based path on purpose

## Run

```bash
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter6-function-tool
```

## Expected Behavior

The program asks about weather and time in Tokyo. With the current static tool data, the assistant should report that Tokyo is sunny at `22°C` and that the local time is `14:30` on `March 17, 2026`.
