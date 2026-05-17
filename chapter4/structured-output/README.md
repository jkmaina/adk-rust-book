# Chapter 4 Structured Output

This crate is the book adaptation of the validated backend example at `../adk-playground/playground/backend/examples/structured_output.rs`.

## What It Demonstrates

- adding an `output_schema` to an `LlmAgent`
- requesting schema-shaped JSON from the model
- running a structured review flow through `Runner`
- passing typed `UserId` and `SessionId` values through the runtime
- streaming the JSON response to the terminal

## Setup

Set your Gemini API key:

```bash
export GOOGLE_API_KEY=your-api-key
```

## Run

From the book repo root:

```bash
cargo run -p chapter4-structured-output
```

Or from this crate directory:

```bash
cargo run
```

## Expected Behavior

The program prints JSON for a movie review request. The exact values vary by model, but the response should include `title`, `year`, `rating`, `genre`, `summary`, and `recommended`.

## ADK-Rust 0.8.2 Connection

This example keeps the same explicit runtime shape taught in Chapter 3 and adds one new control surface: `output_schema(...)`.

That matters in `0.8.2` for two reasons:

- the runtime boundary is still explicit, including `RunnerConfig` and typed `UserId` / `SessionId`
- structured output is powerful, but it still depends on provider support and only guarantees shape, not truth

So this crate is best read as "the first agent, but with output discipline" rather than as a separate pattern.
