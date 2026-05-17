# Chapter 16 Auth RBAC

This crate adapts `../adk-playground/playground/backend/examples/auth_rbac.rs` into the book workspace.

## What It Demonstrates

- declarative role definitions with allow and deny permissions
- runtime tool checks enforced before execution
- an optional live agent run that encounters those permission boundaries

## ADK-Rust 0.8.2 Connection

In local `0.8.2`, auth is part of the `standard` tier rather than the default `minimal` surface, and the official docs now distinguish RBAC from request-level scopes more clearly. This crate focuses on the RBAC side first: the model may attempt a tool call, but the runtime still decides whether the action is allowed.

## Run

```bash
cargo run -p chapter16-auth-rbac
```
