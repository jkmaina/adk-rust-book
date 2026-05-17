# Chapter 14 A2A Client

This crate adapts `../adk-playground/docs_examples/deployment/a2a_test/src/client.rs` into a smoke-friendly client example for the book.

## What It Demonstrates

- connecting to a deployed A2A agent
- fetching the remote agent card
- sending a message over the A2A protocol

## ADK-Rust 0.8.0 Connection

Local `0.8.0` treats A2A as part of the normal deployment surface rather than as an exotic extra. This crate stays smoke-friendly by default, but it still teaches the real remote-client shape: discover the agent card first, then send protocol-level messages to a published agent service.

## Run

```bash
cargo run -p chapter14-a2a-client
```

To attempt a live client call, start the matching Chapter 14 A2A server and set:

```bash
export BOOK_RUN_LIVE_SMOKE=1
export A2A_BASE_URL=http://localhost:8090
cargo run -p chapter14-a2a-client
```
