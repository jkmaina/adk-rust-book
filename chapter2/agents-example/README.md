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