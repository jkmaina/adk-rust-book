# Chapter 8 Neo4j Sessions

This crate adapts `../adk-playground/playground/backend/examples/neo4j_sessions.rs` for the book.

## What It Demonstrates

- durable session lifecycle operations in Neo4j
- preserving the same session abstraction over a graph-oriented backend
- running a live multi-turn conversation through a persisted graph-backed session
- retrieving the session and its linked events before cleanup

## ADK-Rust 0.8.2 Connection

- the example shows that the session abstraction remains stable even when the persistence model changes dramatically
- the runner still executes with typed `UserId` and `SessionId` values
- the graph backend stores the same core ideas as the other backends: user, session, state, and event continuity
- this is a good reminder that storage shape should not force the rest of the agent runtime to relearn what a session means

## Runtime Requirements

- `GOOGLE_API_KEY`
- `NEO4J_URL`
- `NEO4J_USER`
- `NEO4J_PASS`
- a reachable Neo4j instance

## Run

```bash
export GOOGLE_API_KEY=your-api-key
export NEO4J_URL=bolt://localhost:7687
export NEO4J_USER=neo4j
export NEO4J_PASS=your-password
cargo run -p chapter8-neo4j-sessions
```

## Expected Behavior

The program should connect, migrate, create two sessions, list Eve's sessions, run a two-turn conversation in `neo4j-session-1`, report the number of linked event nodes stored for that session, and then clean both sessions up.
