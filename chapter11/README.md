# Chapter 11 Examples

These crates are the active Chapter 11 ADK-Rust examples for multi-agent coordination and routing.

They cover two different coordination shapes on purpose. In local `0.8.0`, ADK-Rust offers both hardened workflow-style routing and a richer graph-agent surface. The examples in this folder keep those patterns visible instead of collapsing them into one generic "multi-agent" story.

## Included Crates

- `chapter6-agent-tool` (reused)
- `chapter11-conditional-router`
- `chapter7-customer-service` (reused)
- `chapter11-supervisor-routing`

## Running from the Repo Root

```bash
cargo run -p chapter6-agent-tool
cargo run -p chapter11-conditional-router
cargo run -p chapter7-customer-service
cargo run -p chapter11-supervisor-routing
```

The `agent-tool`, `conditional-router`, and `customer-service` examples need `GOOGLE_API_KEY`. The graph-based supervisor example also needs `GOOGLE_API_KEY` but does not use the runner/session stack.

## How This Chapter Fits ADK-Rust 0.8.0

- `chapter11-conditional-router` shows the cleanest `LlmConditionalAgent` path for one-shot specialist selection.
- `chapter11-supervisor-routing` shows the lower-level `StateGraph` plus `AgentNode` path for repeated routing with shared state.
- The reused `agent-tool` and `customer-service` examples remain important because not every multi-agent problem needs an LLM router or a supervisor graph.

One practical `0.8.0` note matters here: route handling is more deterministic now, and the graph ecosystem has grown enough that the book needs to explain when to stay low-level versus when to wrap the graph as a higher-level agent.
