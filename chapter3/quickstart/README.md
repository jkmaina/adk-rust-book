# Chapter 3 Quickstart

This crate is the book adaptation of the validated backend example at `../adk-playground/playground/backend/examples/quickstart.rs`.

## What It Demonstrates

- building a minimal `LlmAgent` with ADK-Rust `0.8`
- creating an in-memory session
- configuring `RunnerConfig` explicitly
- running a prompt through `Runner`
- passing typed `UserId` and `SessionId` values
- streaming text events back to the terminal

## Setup

Set your Gemini API key:

```bash
export GOOGLE_API_KEY=your-api-key
```

## Run

From the book repo root:

```bash
cargo run -p chapter3-quickstart
```

Or from this crate directory:

```bash
cargo run
```

## Expected Behavior

The program prints a short 2-3 sentence explanation of Rust. Output varies by model, but you should see streamed natural-language text printed to the terminal.

## ADK-Rust 0.8.0 Connection

The local framework now offers shorter entry points such as `adk_rust::run()` and `Launcher`. This example does not use them because it is the first place the book teaches the full runtime shape on purpose.

What you are seeing here is the explicit path:

- create `GeminiModel` directly
- build an `LlmAgent` directly
- create an in-memory session service
- construct `RunnerConfig`
- call `Runner::run()` with typed `UserId` and `SessionId`
- read the streamed events yourself

That is why this example is more important than a one-line smoke test. It shows the pieces that the convenience helpers package for you.
