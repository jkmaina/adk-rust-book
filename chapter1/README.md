# Chapter 1 Examples

These crates are the active Chapter 1 Rust fundamentals examples for the book.

## Baseline

- Rust `1.92+`
- Cargo `1.92+`
- Rust edition `2024`

The local `adk-rust` workspace that this book tracks is currently `0.8.2`. That workspace itself declares Rust `1.85+`, but the book standardizes on Rust `1.92+` so every chapter in this repo uses one consistent toolchain.

## How This Chapter Fits ADK-Rust 0.8.2

Chapter 1 is intentionally close to the language. ADK-Rust `0.8.2` defaults to the lean `minimal` tier and gives new users a faster entry path through `adk_rust::run()` and `provider_from_env()`, but those conveniences rest on ideas that are easier to learn in isolation first:

- async execution for model calls, runners, and streaming
- explicit `Result` handling for fallible operations
- shared ownership with `Arc`
- contract-based design with traits
- strong typing for IDs and structured values

The goal of these crates is to make later ADK examples feel readable rather than magical.

## Running from the Repo Root

From the book root, you can run any Chapter 1 crate with:

```bash
cargo run -p variables-example
cargo run -p async-example
```

You can also `cd` into an individual crate directory and run `cargo run` there.

## Included Crates

- `agent-trait-example`
- `arc-example`
- `async-example`
- `borrowing-example`
- `control-flow-example`
- `functions-example`
- `prelude-example`
- `result-example`
- `structs-enums-example`
- `types-example`
- `variables-example`

## Why These Examples Still Matter

- `async-example` prepares you for the async API surface used by `adk_rust::run()` and `Runner`
- `arc-example` prepares you for shared handles such as the `Arc<dyn Llm>` returned by `provider_from_env()`
- `result-example` prepares you for `Result<T, AdkError>` and the structured error envelope used throughout current ADK-Rust
- `structs-enums-example` previews the typed identifiers used by `Runner::run()`, such as `UserId` and `SessionId`
- `prelude-example` makes later `use adk_rust::prelude::*` imports feel natural
- `agent-trait-example` is the conceptual bridge into ADK's trait-driven agent architecture

## Stability Note

ADK-Rust `0.8.2` also changes the feature-tier story materially: `minimal` is now the default, while richer production surfaces are deliberate opt-ins. Chapter 1 stays on language foundations first so those later choices make sense when they appear.
