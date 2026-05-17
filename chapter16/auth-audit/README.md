# Chapter 16 Auth Audit

This crate adapts the audit-oriented parts of `../adk-playground/playground/backend/examples/auth_audit.rs` into the book workspace.

## What It Demonstrates

- role definitions and permission matrices
- recording allowed and denied tool access into an audit sink
- framing audit logs as a production control, not just a debug aid

## ADK-Rust 0.8.2 Connection

Local `0.8.2` keeps audit close to runtime authorization rather than treating it as an afterthought. This crate stays offline and deterministic, but it teaches the production habit that matters: denied actions belong in the audit trail too.

## Run

```bash
cargo run -p chapter16-auth-audit
```
