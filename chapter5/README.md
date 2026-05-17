# Chapter 5 Examples

These crates are the active Chapter 5 ADK-Rust examples for session-aware and multi-turn agent behavior.

Chapter 5 is where the book stops treating `LlmAgentBuilder` as a bag of methods and starts treating it as a behavioral control surface. The examples here show how continuity, history, and tool use interact in local `adk-rust` `0.8.2`.

## Baseline

- Rust `1.92+`
- Cargo `1.92+`
- edition `2024`
- local `adk-rust` `0.8.2` workspace
- `GOOGLE_API_KEY` for live Gemini runs

## Included Crates

- `chapter5-session-state`
- `chapter5-multi-turn`

## How This Chapter Fits ADK-Rust 0.8.2

- sessions in `0.8.2` carry both keyed state and an event history
- continuity depends on stable typed `UserId` and `SessionId`, not just repeated prompts
- the multi-turn tool path is more dependable in `0.8.2` because the local Gemini integration fixed a `thought_signature` issue that could break the second model call after tool execution
- these examples keep the explicit `Runner` path so the runtime remains visible while you learn the model

## Running from the Repo Root

```bash
cargo run -p chapter5-session-state
cargo run -p chapter5-multi-turn
```

Both crates compile offline, but live execution requires `GOOGLE_API_KEY` to be set.
