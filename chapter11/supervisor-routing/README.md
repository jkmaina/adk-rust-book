# Chapter 11 Supervisor Routing

This crate is the book adaptation of `../adk-playground/playground/backend/examples/supervisor_routing.rs`.

## What It Demonstrates

- graph-based supervisory routing instead of direct runner orchestration
- wrapping agents as `AgentNode`s with state mappers
- iterating between a supervisor and specialists until the graph decides the task is done

## ADK-Rust 0.8.2 Connection

The local `0.8.2` graph story is broader than this single crate: `GraphAgent` now gives you a higher-level wrapper for graph workflows. This example intentionally stays with `StateGraph` and `AgentNode` so the routing loop, state channels, and stop conditions remain visible to the reader.

One implementation detail matters in practice: `AgentNode` output mappers receive the full event slice for a node run. This crate aggregates text across those events before updating `next_agent` or `work_done`, which keeps routing and shared state stable under streaming model output.

## Run

```bash
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter11-supervisor-routing
```

## Expected Behavior

The supervisor should route work to a researcher or writer, carry forward the latest `work_done` state, and stop after either a `done` decision or the iteration limit.
