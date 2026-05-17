# Chapter 10 Memory Agent

This crate is the book adaptation of `../adk-playground/playground/backend/examples/memory_agent.rs`.

## What It Demonstrates

- populating long-term memory outside the active session
- searching memory with a typed tool before answering
- carrying user preferences and project history across sessions

## ADK-Rust 0.8.0 Connection

Local `0.8.0` supports richer memory-aware context access than this single crate shows, but the explicit `recall_memory` tool remains the clearest way to teach selective recall. The important concept is that memory is retrieved deliberately across sessions, not automatically confused with current-session history.

## Run

```bash
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter10-memory-agent
```

## Expected Behavior

The program preloads two prior sessions into memory, then asks a new question. The assistant should search memory first and use that context in its answer.
