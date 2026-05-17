# Deployment-Ready Agent Service

This example shows how to package one ADK-Rust agent for more than one delivery surface without rewriting the runtime. The same release-assistant agent is exposed through an explicit `Runner` path for smoke checks, a `Launcher` path for CLI and HTTP serving, and a composed Axum router for production-style deployment.

## What It Demonstrates

- one agent reused across runner, launcher, and server surfaces
- session-backed release context for a production support assistant
- launcher-based `chat` and `serve` entrypoints
- `build_app()` for custom router composition
- `build_app_with_a2a(...)` for A2A-capable packaging
- a custom operations route merged into the ADK-generated router

## Book Concepts It Reinforces

- Chapter 3: explicit runner construction and streamed execution
- Chapter 5: session-backed context for user and environment state
- Chapter 12: launcher wiring as an application entry surface
- Chapter 14: deployment packaging, composed routers, and A2A-ready exposure

## Architecture

![Deployment-Ready Agent Service Architecture](/Users/jameskaranja/Developer/projects/adk-rust-book-companion/deployment-ready-agent-service/assets/deployment-ready-agent-service-architecture.svg)

### System Overview: How it Works

1. The crate builds one `release_assistant` agent with a Gemini model and a short operational instruction.
2. An in-memory session stores deployment-specific state such as the team, target environment, and response style.
3. The explicit `Runner` path is used for a one-prompt smoke check so the underlying runtime remains visible and testable.
4. The same agent is then wrapped in `Launcher`, which can expose interactive `chat` or HTTP `serve` modes.
5. For production-style composition, `Launcher::build_app()` or `Launcher::build_app_with_a2a(...)` returns an Axum router that can be extended with custom routes such as `/ops/ready`.

### Design Choices

- The example keeps the agent logic separate from the deployment surface. This is the main architectural lesson: change the delivery mechanism without rebuilding the agent.
- The `Runner` smoke path stays in the crate because deployment examples are easier to trust when the core runtime can still be exercised directly.
- The custom `/ops/ready` route shows why `build_app()` matters. Real services often need health, readiness, metrics, or admin routes alongside the agent API.
- A2A is made explicit as a separate surface because it changes what remote systems can discover and call, even though the underlying agent stays the same.

### Request Flow

1. A release manager asks for rollout guidance.
2. The runner or launcher executes the same agent against the same session-backed release context.
3. In composed-app mode, ADK exposes its own routes and the application merges in `/ops/ready`.
4. In A2A mode, the service also exposes an agent card and A2A endpoint for remote discovery and calls.

## Run

Offline compile validation:

```bash
cargo check -p deployment-ready-agent-service
```

Live smoke with the explicit runner path:

```bash
export GOOGLE_API_KEY=your-api-key
BOOK_RUN_LIVE_SMOKE=1 cargo run -p deployment-ready-agent-service
```

Launcher surfaces:

```bash
export GOOGLE_API_KEY=your-api-key
cargo run -p deployment-ready-agent-service -- chat
cargo run -p deployment-ready-agent-service -- serve --port 8094
```

Composed router surfaces:

```bash
export GOOGLE_API_KEY=your-api-key
BOOK_RUN_LIVE_SMOKE=1 cargo run -p deployment-ready-agent-service -- app
BOOK_RUN_LIVE_SMOKE=1 cargo run -p deployment-ready-agent-service -- a2a
BOOK_RUN_LIVE_SMOKE=1 BOOK_RUN_AGENT_SERVICE=1 cargo run -p deployment-ready-agent-service -- a2a
```

## Why This Example Matters

Many teams get a first agent working, then discover that packaging it cleanly is the harder problem. This example shows the bridge from agent logic to deployable service shape: one runtime, several delivery surfaces, and explicit control over how the surrounding application is assembled.
