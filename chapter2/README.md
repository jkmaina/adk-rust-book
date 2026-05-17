# Chapter 2 Examples

These crates support the Chapter 2 architecture and composition discussion.

## Baseline

- Rust `1.92+`
- Cargo `1.92+`
- Rust edition `2024`

The local `adk-rust` workspace this chapter aligns with is `0.8.2`. That workspace itself declares Rust `1.85+`, but the book stays on Rust `1.92+` so every chapter in this repo shares one consistent baseline.

## How This Chapter Fits ADK-Rust 0.8.2

Chapter 2 is where the book stops treating ADK-Rust as a bag of APIs and starts treating it as a system.

In `0.8.2`, that means three things in particular:

- the framework now has a published stability contract, so the architecture can be read as a stable core, a beta platform, and an experimental edge
- convenience entry points such as `adk_rust::run()` and `Launcher` are now part of the onboarding story, but they still sit on top of the same service, agent, runner, and application layers
- the runner boundary is more explicit, including typed `UserId` and `SessionId` values instead of anonymous strings

## Included Crates

- `agents-example`

## Running from the Repo Root

```bash
cargo run -p agents-example
```

Chapter 2 stays partly repo-native on purpose. It uses plain Rust composition patterns to explain trait objects, shared ownership, and sequencing before the book moves into ADK-Rust quickstart flows in Chapter 3.

Read `agents-example` as a conceptual bridge, not as a literal reimplementation of the full runtime. Chapter 3 is where the validated ADK quickstart makes the runner, session service, and event flow concrete.
