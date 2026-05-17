# Chapter 10 Artifact Agent

This crate is the book adaptation of `../adk-playground/playground/backend/examples/artifact_agent.rs`.

## What It Demonstrates

- saving generated content as versioned artifacts
- loading and listing artifacts through typed tools
- attaching an artifact service to the runner so the agent has durable file-like state

## ADK-Rust 0.8.2 Connection

In local `0.8.2`, artifacts are a mature subsystem with scoped access, versioning, and event integration. This crate keeps the save/load/list operations explicit because that makes the architectural boundary legible: generated outputs with future value belong in artifact storage, not only in conversation history.

## Run

```bash
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter10-artifact-agent
```

## Expected Behavior

The agent should generate a short poem, save it as `poem.txt`, and confirm what it stored.
