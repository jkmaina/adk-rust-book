# Chapter 16 Auth Identity

This crate adapts `../adk-playground/playground/backend/examples/auth_identity.rs` into the book workspace.

## What It Demonstrates

- validated typed identities at the boundary
- multi-tenant session isolation
- an optional live agent explanation scoped to a validated identity

## ADK-Rust 0.8.2 Connection

In local `0.8.2`, typed identity remains one of the most important low-level safety boundaries in the framework. This crate keeps that boundary visible by validating identities directly and then using those validated values when it reaches the live runner path.

## Run

```bash
cargo run -p chapter16-auth-identity
```
