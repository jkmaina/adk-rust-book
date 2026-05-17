# Chapter 14 A2A Server

This crate adapts `../adk-playground/docs_examples/deployment/a2a_test/src/server.rs` into a non-blocking deployment example for the book.

## What It Demonstrates

- building an A2A-enabled HTTP app around an ADK agent
- preparing an agent card and `/a2a` endpoint for deployment
- separating app construction from actually binding the server port

## ADK-Rust 0.8.0 Connection

In local `0.8.0`, the deployment story is more explicit than older drafts assumed. A2A is part of the stable specialist server surface, and ADK-Rust distinguishes clearly between constructing an app with A2A routes and actually taking ownership of the listener and serve loop. This crate keeps those two stages separate on purpose.

## Run

```bash
cargo run -p chapter14-a2a-server
```

To build the live server wiring:

```bash
export BOOK_RUN_LIVE_SMOKE=1
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter14-a2a-server
```

To actually bind and serve:

```bash
export BOOK_RUN_LIVE_SMOKE=1
export BOOK_RUN_A2A_SERVER=1
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter14-a2a-server
```
