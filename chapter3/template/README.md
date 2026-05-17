# Chapter 3 Template

This crate is the book adaptation of the validated backend example at `../adk-playground/playground/backend/examples/template.rs`.

## What It Demonstrates

- session-backed instruction templates
- injecting user context into an `LlmAgent` instruction
- running a personalized prompt through `Runner`
- passing typed `UserId` and `SessionId` values
- streaming the response to the terminal

## Setup

Set your Gemini API key:

```bash
export GOOGLE_API_KEY=your-api-key
```

## Run

From the book repo root:

```bash
cargo run -p chapter3-template
```

Or from this crate directory:

```bash
cargo run
```

## Expected Behavior

The program responds in French and should adapt the explanation for a beginner user named Alice because those values are pre-seeded in session state.

## ADK-Rust 0.8.2 Connection

This crate shows why the explicit `Runner` path still matters in `0.8.2`. The framework now has shorter entry points, but template injection, session-backed state, and typed runtime identity are easier to understand when the setup stays visible.

By keeping the runtime explicit, the example makes three things clear:

- instruction templates are resolved against session state, not magic globals
- `Runner::run()` is now part of a typed execution boundary through `UserId` and `SessionId`
- the agent, session service, and runner are separate concerns even in a tiny program
