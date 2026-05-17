# Chapter 15 Telemetry Spans

This crate adapts `../adk-playground/docs_examples/observability/telemetry_test/src/spans.rs` into the book workspace.

## What It Demonstrates

- pre-configured spans for agent runs, model calls, tools, and callbacks
- adding user and session context to spans
- recording dynamic attributes after work completes

## ADK-Rust 0.8.0 Connection

Local `0.8.0` gives these span helpers a clearer role in the runtime story. They are not just tracing utilities; they mirror the actual agent, model, tool, and callback boundaries that operators need to inspect when a run becomes slow, expensive, or strange.

## Run

```bash
cargo run -p chapter15-telemetry-spans
```
