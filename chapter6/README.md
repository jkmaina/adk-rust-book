# Chapter 6 Examples

These crates are the active Chapter 6 ADK-Rust examples for tools and agent capabilities.

Chapter 6 is where the book moves from agent behavior to agent capability. The examples here teach the custom typed-tool path first, then show how one agent can coordinate multiple capabilities and finally how a specialist agent can be exposed as a tool.

## Baseline

- Rust `1.92+`
- Cargo `1.92+`
- edition `2024`
- local `adk-rust` `0.8.2` workspace
- `GOOGLE_API_KEY` for live Gemini runs

## Included Crates

- `chapter6-function-tool`
- `chapter6-multi-tools`
- `chapter6-agent-tool`

## How This Chapter Fits ADK-Rust 0.8.2

- `#[tool]` is the preferred zero-boilerplate path for ordinary custom tools in this repo
- `0.8.2` also broadened the tool story with provider-native built-in tool wrappers and richer MCP support, but this chapter keeps the learning surface legible by starting with typed custom tools
- the `AgentTool` example is more trustworthy in `0.8.2` because the runtime fixed a sub-agent response-extraction issue that could previously lead to repeated delegation loops
- the examples keep the explicit `Runner` path and typed `UserId` / `SessionId` values visible so the runtime boundary stays understandable

## Running from the Repo Root

```bash
cargo run -p chapter6-function-tool
cargo run -p chapter6-multi-tools
cargo run -p chapter6-agent-tool
```

All three crates compile offline, but live execution requires `GOOGLE_API_KEY`.
