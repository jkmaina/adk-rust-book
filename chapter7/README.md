# Chapter 7 Examples

These crates are the active Chapter 7 ADK-Rust examples for workflows and orchestration.

Chapter 7 is where the book shifts from tool-level capability to process-level design. These examples show how `adk-rust` `0.8.0` handles ordered pipelines, parallel specialist analysis, bounded loops, and a realistic escalation workflow.

## Baseline

- Rust `1.92+`
- Cargo `1.92+`
- edition `2024`
- local `adk-rust` `0.8.0` workspace
- `GOOGLE_API_KEY` for live Gemini runs

## Included Crates

- `chapter7-sequential-workflow`
- `chapter7-parallel-workflow`
- `chapter7-loop-workflow`
- `chapter7-customer-service`

## How This Chapter Fits ADK-Rust 0.8.0

- this chapter teaches the three core deterministic workflow families: `SequentialAgent`, `ParallelAgent`, and `LoopAgent`
- the wider `0.8.0` orchestration surface also includes conditional routers and graph workflows, but these examples intentionally start with the simplest shapes that still expose the runtime clearly
- the customer-service example benefits from the hardened `AgentTool` path in `0.8.0`, which makes specialist delegation more dependable
- the examples keep the explicit `Runner` path and typed `UserId` / `SessionId` boundary visible instead of hiding orchestration behind convenience helpers

## Running from the Repo Root

```bash
cargo run -p chapter7-sequential-workflow
cargo run -p chapter7-parallel-workflow
cargo run -p chapter7-loop-workflow
cargo run -p chapter7-customer-service
```

All four crates compile offline, but live execution requires `GOOGLE_API_KEY`.
