# Agents Example (Chapter 2)

This small example shows how to compose different "agent" implementations in a single collection using trait objects and shared ownership.

It's written for beginners and explains each step so you can follow along even if you're new to Rust.

**What it demonstrates**

- **Trait objects**: the `Agent` trait uses `async fn` (via `async-trait`) and different types implement the trait.
- **Shared ownership**: agents are stored as `Arc<dyn Agent>` so the same agent can be shared and used across contexts.
- **Composition**: `SequentialAgent` contains a `Vec<Arc<dyn Agent>>` and runs its sub-agents in order.

**Files**

- `Cargo.toml` — project manifest and dependencies (`tokio`, `async-trait`).
- `src/main.rs` — the full example with `Agent` trait and three implementations:
  - `LlmAgent` — a mock LLM-like agent that "processes" input.
  - `CustomAgent` — a simple agent that echoes or manipulates input.
  - `SequentialAgent` — runs a sequence of other agents in order.

## Getting started

1. Open a PowerShell terminal.
2. Change into the example folder:

```powershell
Set-Location -Path C:\Projects\adk-learning\chapter2\agents-example
```

3. Build and run the example:

```powershell
cargo run
```

## What you should see

The program prints messages showing each agent being invoked. For the `SequentialAgent` you will see its sub-agents run one after another.

## Why this is useful

Storing `Arc<dyn Agent>` in a `Vec` lets you build heterogeneous pipelines and compositions of behavior while keeping ownership simple and thread-safe. This pattern is useful when building modular systems (e.g., chatbots, pipelines, or micro-agents).

## Next steps (suggestions)

- Try changing one of the agents to perform a different action (e.g., uppercase input).
- Modify `SequentialAgent` to run sub-agents concurrently with `tokio::join!` and observe the difference.
- Add unit tests for individual agent implementations.

If you want, I can update the top-level README to link this example into the book's table of contents.

## Exact behavior and example output

When you run the program it constructs three agents and runs them in order:

- `LlmAgent` named `llm-main` — prints a processing line, waits a short time, then prints done.
- `CustomAgent` named `custom-echo` — prints received, waits, then prints finished.
- `SequentialAgent` named `pipeline-1` — prints a starting line, then runs its two sub-agents in sequence:
  - `llm-sub` (an `LlmAgent`)
  - `custom-sub` (a `CustomAgent`)

A representative run shows this sequence (timestamps removed):

```
=== Running agent: llm-main ===
[LLM:llm-main] processing: Hello agent world
[LLM:llm-main] done
=== Running agent: custom-echo ===
[Custom:custom-echo] received: Hello agent world
[Custom:custom-echo] finished
=== Running agent: pipeline-1 ===
[Seq:pipeline-1] starting sequence
[Seq:pipeline-1] -> running sub-agent llm-sub
[LLM:llm-sub] processing: Hello agent world
[LLM:llm-sub] done
[Seq:pipeline-1] -> running sub-agent custom-sub
[Custom:custom-sub] received: Hello agent world
[Custom:custom-sub] finished
[Seq:pipeline-1] sequence complete
All agents finished.
```

This README now reflects the exact agents and names used in `src/main.rs`.
A minimal example showing how to hold different agent implementations in a single vector using `Arc<dyn Agent>`.

- Defines an async `Agent` trait (requires `async-trait` and `tokio`).
- Implements `LlmAgent`, `CustomAgent`, and `SequentialAgent` (the latter runs sub-agents in order).
- Demonstrates `Arc::new(...)` and storing `Arc<dyn Agent>` in a `Vec`.

Run:

```powershell
Set-Location -Path C:\Projects\adk-learning\chapter2\agents-example
cargo run
```

This prints each agent running and shows how a `SequentialAgent` runs its sub-agents in sequence.