# Chapter 6 Agent Tool

This crate is the book adaptation of `../adk-playground/playground/backend/examples/agent_tool.rs`.

## What It Demonstrates

- wrapping specialist agents as tools with `AgentTool`
- routing user intent through a coordinator agent
- mixing an LLM-only specialist with a tool-backed specialist

## ADK-Rust 0.8.0 Connection

- `AgentTool` is the clean bridge from custom tools to multi-agent delegation in the local framework
- local `0.8.0` fixed a sub-agent response-extraction issue that could previously make streamed delegation loops unreliable, which makes this example materially more dependable
- the coordinator still uses the same explicit `Runner` flow and typed `UserId` / `SessionId` boundary as earlier chapters
- the math specialist is tool-backed while the trivia specialist is LLM-only, which makes the architectural contrast easier to understand

## Run

```bash
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter6-agent-tool
```

## Expected Behavior

The program asks one combined math-and-trivia question. The coordinator should delegate the arithmetic part to the math specialist and the factual part to the trivia specialist, then summarize both results. In the current run, that means returning `37.5` for the calculation and explaining that the percentage symbol evolved from the Italian phrase "per 100" rather than coming from one named inventor.
