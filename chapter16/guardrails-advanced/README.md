# Chapter 16 Guardrails Advanced

This crate adapts `../adk-playground/playground/backend/examples/guardrails_advanced.rs` into the book workspace.

## What It Demonstrates

- PII redaction and content filtering
- composing multiple guardrails into a single execution path
- an optional live guarded-agent run when smoke mode is enabled

## ADK-Rust 0.8.0 Connection

Local `0.8.0` makes guardrails a clearer runtime boundary than earlier drafts implied. Input guardrails can transform or block content before model execution, and output guardrails can validate generated responses before they return to the caller. This crate focuses on the input side first because that is where many production systems need immediate boundary control.

## Run

```bash
cargo run -p chapter16-guardrails-advanced
```
