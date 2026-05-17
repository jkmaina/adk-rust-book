# ADK-Rust Book Companion Examples

This repository is the official runnable companion workspace for the ADK-Rust book.

It contains the example crates only. The manuscript, mdBook source, chapter markdown, planning files, and legacy `.docx` source are intentionally excluded from this repository.

## Baseline

- ADK-Rust crates: `0.8.2`
- Rust: `1.92+`
- Edition: `2024`

The root workspace lets you compile the chapter crates together while still keeping each example in its own directory.

## What Is Included

- `chapter1/` through `chapter16/`: runnable example crates grouped by chapter
- `scripts/check-examples.sh`: offline workspace validation
- `scripts/check-drift.py`: edition and dependency drift checks
- `scripts/smoke-examples.sh`: opt-in live smoke runs for networked examples

## Repository Layout

- [chapter1](chapter1): Rust foundations examples
- [chapter2](chapter2): architecture and composition examples
- [chapter3](chapter3): first runnable ADK agents
- [chapter4](chapter4): output control and first production levers
- [chapter5](chapter5): session-backed agent behavior
- [chapter6](chapter6): function tools and agent-as-tool patterns
- [chapter7](chapter7): sequential, parallel, loop, and escalation workflows
- [chapter8](chapter8): session backends and persistence
- [chapter9](chapter9): callbacks, plugins, and guardrails
- [chapter10](chapter10): memory, artifacts, and RAG
- [chapter11](chapter11): routing and supervisor patterns
- [chapter12](chapter12): streaming, realtime, reasoning, and launcher examples
- [chapter13](chapter13): validation and evaluation examples
- [chapter14](chapter14): deployment and A2A packaging examples
- [chapter15](chapter15): telemetry and observability examples
- [chapter16](chapter16): auth, guardrails, audit, and retrieval governance

## Concepts Covered

- Rust foundations for agent systems: ownership, borrowing, traits, async, `Arc`, and `Result`
- ADK-Rust architecture: service, agent, runner, and application layers
- First-agent construction: model setup, session setup, runner wiring, and streamed responses
- Instruction design and session-backed template personalization
- Structured output and schema-shaped responses
- Session history, keyed state, and persistent session backends
- Function tools, multi-tool selection, and agent-as-tool delegation
- Sequential, parallel, loop, and escalation workflows
- Callbacks, plugins, and runtime guardrails
- Memory, artifacts, embedding pipelines, and retrieval-augmented generation
- Conditional routing, supervisor patterns, and graph-based coordination
- Streaming, realtime interaction, reasoning traces, and launcher-based app wiring
- Validation, smoke testing, and evaluation workflows
- Deployment packaging, A2A servers, and A2A clients
- Telemetry, spans, usage tracking, and observability patterns
- Typed identity, RBAC, SSO, audit trails, and retrieval governance

## Quick Start

```bash
cargo check --workspace
cargo run -p chapter3-quickstart
```

Examples that call live providers require credentials such as `GOOGLE_API_KEY` or `OPENAI_API_KEY`. The per-example READMEs call out those requirements.

## Validation

Offline validation:

```bash
./scripts/check-examples.sh
```

Opt-in live smoke validation:

```bash
BOOK_RUN_LIVE_SMOKE=1 ./scripts/smoke-examples.sh
```
