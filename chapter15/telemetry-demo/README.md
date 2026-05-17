# Chapter 15 Telemetry Demo

This crate adapts the operational ideas from `../adk-playground/examples/telemetry_demo/src/main.rs` into a book-friendly example.

## What It Demonstrates

- structured telemetry initialization for a real application
- context propagation, custom spans, and manual usage recording
- an optional live model path that runs one traced prompt

## ADK-Rust 0.8.2 Connection

Local `0.8.2` broadens the telemetry story beyond the manual examples in this crate. `llm_generate_span` and `record_llm_usage` still make the usage model easy to inspect, while the provider layer now also auto-records token usage on real model calls using OpenTelemetry GenAI-style fields. This example keeps the live path opt-in so the operational model is still teachable offline.

## Run

```bash
cargo run -p chapter15-telemetry-demo
```

To enable the live model path:

```bash
export BOOK_RUN_LIVE_SMOKE=1
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter15-telemetry-demo
```
