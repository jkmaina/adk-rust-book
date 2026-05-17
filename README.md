# ADK-Rust Book Companion Examples

This repository is the official runnable companion workspace for the ADK-Rust book.

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

- **Rust foundations for agent systems**: readers learn the Rust ideas that show up repeatedly in ADK-Rust code, including ownership, borrowing, traits, async execution, `Arc`, and `Result`. The goal is not generic language theory; it is learning how Rust’s type system, concurrency model, and error handling make production agent runtimes safer and easier to reason about.
- **ADK-Rust architecture**: readers learn how ADK-Rust is organized into service, agent, runner, and application layers. This gives them a mental model for where models, sessions, tools, plugins, memory, and deployment surfaces belong, so later examples feel like one coherent runtime instead of a pile of unrelated APIs.
- **First-agent construction**: readers learn the smallest honest runnable flow, including model setup, session creation, runner wiring, content construction, and streamed response handling. This matters because it teaches what actually happens during execution instead of hiding the runtime behind a convenience wrapper too early.
- **Instruction design and session-backed template personalization**: readers learn how instructions shape behavior and how session state can inject user-specific context such as name, language, expertise, or role. This is where agents stop feeling generic and start behaving like systems that respond differently for different users.
- **Structured output and schema-shaped responses**: readers learn how to move from conversational prose to responses that another system can consume predictably. The examples show how output schemas improve downstream integration, while also making clear that structural validity is not the same thing as factual correctness.
- **Session history, keyed state, and persistent session backends**: readers learn the difference between conversation history and explicit state, and how that model extends from in-memory development to PostgreSQL, MongoDB, and Neo4j-backed persistence. This is the foundation for multi-turn behavior, continuity, and long-running agent systems.
- **Function tools, multi-tool selection, and agent-as-tool delegation**: readers learn how agents move beyond language-only behavior by calling typed tools, choosing between multiple capabilities, and delegating to specialist sub-agents. This section shows how to design narrow capability boundaries that are easier to trust, test, and extend.
- **Sequential, parallel, loop, and escalation workflows**: readers learn when to choose ordered pipelines, concurrent specialist analysis, bounded refinement loops, or escalation flows. The emphasis is on workflow shape as an engineering decision: when process matters, orchestration often matters more than a single clever prompt.
- **Callbacks, plugins, and runtime guardrails**: readers learn the difference between local agent interception and runner-wide lifecycle interception, and how those surfaces are used for logging, early rejection, safety controls, and reusable runtime policy. This helps them place cross-cutting logic in the right layer instead of burying everything inside the agent.
- **Memory, artifacts, embedding pipelines, and retrieval-augmented generation**: readers learn how ADK-Rust separates short-term conversation state from longer-term recall, persistent artifacts, and external knowledge retrieval. The examples show when to store generated outputs, when to retrieve memory deliberately, and how embedding-based retrieval fits into a larger application architecture.
- **Conditional routing, supervisor patterns, and graph-based coordination**: readers learn how to move beyond static workflows into rule-based routing, model-driven routing, and explicit graph coordination. This matters for systems with multiple specialists, branching control flow, or stateful orchestration that has to remain inspectable.
- **Streaming, realtime interaction, reasoning traces, and launcher-based app wiring**: readers learn the difference between standard streamed responses, specialist realtime transports, and reasoning-aware execution surfaces. They also see how launcher-based wiring helps connect ADK logic to CLI, server, or app-level entry points without changing the underlying runtime model.
- **Validation, smoke testing, and evaluation workflows**: readers learn how to validate example crates offline, run selective live smokes when credentials are available, and think about evaluation as a separate discipline from compilation or unit-style checks. This section is about keeping agents correct over time, not just getting them to compile once.
- **Deployment packaging, A2A servers, and A2A clients**: readers learn how to package agent systems for real execution surfaces, including launcher-based entrypoints, HTTP serving, and Agent-to-Agent protocol scenarios. The key lesson is that deployment is not an afterthought; it is part of how the runtime gets exposed safely and predictably.
- **Telemetry, spans, usage tracking, and observability patterns**: readers learn how to instrument an agent system so operators can understand latency, usage, failure modes, and runtime behavior. This includes structured logs, spans, token usage recording, and the operational patterns needed when an agent becomes part of a production service.
- **Typed identity, RBAC, SSO, audit trails, and retrieval governance**: readers learn how security becomes concrete in an agent system through typed identifiers, runtime authorization, audit logging, claims mapping, and scoped retrieval boundaries. The focus is on building systems where unsafe behavior is harder to perform and easier to explain when it is blocked.

## Real-World AI Agent Systems You Can Build

- [customer-support-agent](customer-support-agent): a customer support agent that looks up order status, escalates refunds, and enforces role boundaries
- [internal-knowledge-assistant](internal-knowledge-assistant): an internal knowledge assistant with scoped RAG collections for engineering docs, HR policies, and operations playbooks
- [multilingual-user-assistant](multilingual-user-assistant): a multilingual assistant that adapts language, tone, and explanation depth from session-backed user state
- [structured-intake-extractor](structured-intake-extractor): a structured data extractor for contacts, tickets, or intake forms that returns schema-shaped output for downstream systems
- [research-writing-workflow](research-writing-workflow): a research-to-writing workflow that separates research, drafting, and editorial refinement into explicit stages
- [parallel-review-system](parallel-review-system): a parallel review system that gathers technical, product, and user-experience perspectives before producing one response
- [iterative-content-refiner](iterative-content-refiner): an iterative content refiner that improves drafts until a quality condition is met or a safe iteration limit is reached
- [guarded-agent-surface](guarded-agent-surface): a guarded agent surface that redacts PII, blocks unsafe requests, and records audit events for allowed and denied actions
- [role-aware-operations-assistant](role-aware-operations-assistant): a role-aware operations assistant that can search, summarize, and execute tool actions only when runtime permissions allow them
- [deployment-ready-agent-service](deployment-ready-agent-service): a deployment-ready agent service exposed through CLI, server, or A2A interfaces using the same underlying runtime model
- [telemetry-aware-production-agent](telemetry-aware-production-agent): a telemetry-aware production agent that emits logs, spans, and usage data for latency, cost, and failure analysis
- [realtime-voice-assistant](realtime-voice-assistant): a realtime voice or streaming assistant that handles incremental responses instead of waiting for one final blocking reply
- [realtime-voice-assistant-gemini](realtime-voice-assistant-gemini): a Gemini Live variant that shows the same event-driven realtime model with Google-native voice and transcript streaming

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
