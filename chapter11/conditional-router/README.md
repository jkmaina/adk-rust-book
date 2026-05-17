# Chapter 11 Conditional Router

This crate is the book adaptation of `../adk-playground/playground/backend/examples/conditional_router.rs`.

## What It Demonstrates

- using `LlmConditionalAgent` for intent classification
- routing one request to a specialized sub-agent without tool wrapping
- keeping a fallback route for ambiguous queries

## ADK-Rust 0.8.0 Connection

Local `0.8.0` tightened `LlmConditionalAgent` behavior so overlapping route labels resolve deterministically. That makes this example more than a toy classifier: it shows a routing primitive you can reason about when specialist labels and fallback behavior both matter.

## Run

```bash
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter11-conditional-router
```

## Expected Behavior

The router should classify the prompt as `creative` and forward the request to the creative writer specialist.
