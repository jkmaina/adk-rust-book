# Chapter 9 Callbacks Guardrails

This crate is the book adaptation of `../adk-playground/playground/backend/examples/callbacks_guardrails.rs`.

## What It Demonstrates

- enforcing simple policy checks in `before_callback`
- short-circuiting an agent run with a synthetic assistant response
- separating accepted input from blocked input without changing the rest of the runner flow

## ADK-Rust 0.8.2 Connection

In local `0.8.2`, guardrails are part of a more explicit runtime control story. This crate shows the core policy boundary clearly: `Ok(None)` allows the run to continue, `Ok(Some(Content))` returns an intentional early answer, and `Err(...)` fails the run. That makes the example useful beyond the specific blocked-word rule.

## Run

```bash
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter9-callbacks-guardrails
```

## Expected Behavior

The first turn should pass the guardrail and call the model normally. The second turn contains `blocked_word` and should be intercepted before the model is called.
