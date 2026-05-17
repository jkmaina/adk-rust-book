# Chapter 3 Examples

These crates are the active Chapter 3 ADK-Rust quickstart examples.

## Baseline

- Rust `1.92+`
- Cargo `1.92+`
- Rust edition `2024`
- Local `adk-rust` `0.5` workspace at `../adk-rust`

The upstream `adk-rust` workspace currently declares Rust `1.85+`, but this book standardizes on Rust `1.92+` so every chapter in this repo shares one current baseline.

## How This Chapter Fits ADK-Rust 0.8.2

Chapter 3 is the first chapter where setup and runtime meet.

In `0.8.2`, ADK-Rust offers multiple entry paths:

- `cargo-adk` for scaffolding
- `adk_rust::run()` for the shortest smoke test
- `Launcher` for a simple application surface
- explicit `Runner` construction for full visibility

The book starts with the explicit `Runner` path because it teaches the runtime honestly. That is why these Chapter 3 crates create `GeminiModel`, sessions, `RunnerConfig`, and typed `UserId`/`SessionId` values directly instead of hiding them behind a shorter helper.

## Included Crates

- `chapter3-quickstart`
- `chapter3-template`

## Running from the Repo Root

```bash
cargo run -p chapter3-quickstart
cargo run -p chapter3-template
```

Both crates expect `GOOGLE_API_KEY` to be set. They compile offline, but live execution requires a valid Gemini API key.

That requirement is deliberate. The local `0.8.2` framework can auto-detect providers with `provider_from_env()`, but these examples use `GeminiModel` directly so the reader can see the full runtime shape before moving to convenience wrappers.
