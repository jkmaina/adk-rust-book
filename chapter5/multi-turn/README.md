# Chapter 5 Multi-Turn

This crate is the book adaptation of the validated backend example at `../adk-playground/playground/backend/examples/multi_turn.rs`.

## What It Demonstrates

- multi-turn conversation in a single session
- typed function tools with the `#[tool]` macro
- combining session memory with tool usage
- following references from earlier turns when later requests rely on prior context

## ADK-Rust 0.8.0 Connection

- the example uses stable typed `UserId` and `SessionId` values so continuity is explicit at the runner boundary
- local `adk-rust` `0.8.0` fixed Gemini 3.x `thought_signature` handling for multi-turn function calling, which makes examples like this one materially more dependable
- typed `#[tool]` functions keep the tool boundary readable and schema-driven instead of passing opaque JSON blobs around
- the example intentionally keeps the explicit `Runner` path visible so readers can see how tool use and session continuity are wired together

## Setup

Set your Gemini API key:

```bash
export GOOGLE_API_KEY=your-api-key
```

## Run

From the book repo root:

```bash
cargo run -p chapter5-multi-turn
```

## Expected Behavior

The program simulates three shopping-assistant turns. The assistant should browse products, add items to a cart, and then answer a follow-up about the cart total using the context built up across earlier turns.
