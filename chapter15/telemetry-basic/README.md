# Chapter 15 Telemetry Basic

This crate adapts `../adk-playground/docs_examples/observability/telemetry_test/src/basic.rs` into the book workspace.

## What It Demonstrates

- initializing ADK telemetry
- emitting structured logs at multiple levels
- instrumenting async functions and recording structured fields

## ADK-Rust 0.8.2 Connection

In local `0.8.2`, telemetry is part of the `standard` tier rather than the default `minimal` surface. This crate keeps the example simple by focusing on structured logs and `#[instrument]`, which are still the first operational habits teams need before they move on to spans, exporters, and usage tracking.

## Run

```bash
cargo run -p chapter15-telemetry-basic
```
