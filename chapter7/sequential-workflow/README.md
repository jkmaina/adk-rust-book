# Chapter 7 Sequential Workflow

This crate is the book adaptation of `../adk-playground/playground/backend/examples/sequential.rs`.

## What It Demonstrates

- chaining specialist agents in a fixed order
- handing the output of one stage into the next stage
- using `SequentialAgent` for deterministic pipelines

## ADK-Rust 0.8.0 Connection

- this is the clearest starting point for deterministic orchestration in the local framework
- the example keeps the explicit `Runner` path and typed `UserId` / `SessionId` values visible
- the current `0.8.0` workflow surface is broader than this single pattern, but sequential handoff is still the right first mental model
- the handoff contract between researcher, writer, and editor is the real lesson, not merely the fact that several agents run in sequence

## Run

```bash
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter7-sequential-workflow
```

## Expected Behavior

The program runs a three-stage pipeline over the prompt `The impact of Rust on systems programming`. The output should read like a progressively refined short piece rather than three disconnected answers, because the researcher, writer, and editor are meant to behave like one ordered process.
