# Chapter 8 PostgreSQL Sessions

This crate adapts `../adk-playground/playground/backend/examples/postgres_sessions.rs` for the book.

## What It Demonstrates

- durable session creation in PostgreSQL
- listing sessions by user
- running a live multi-turn agent exchange against persisted session state
- retrieving and deleting sessions as part of the full lifecycle

## ADK-Rust 0.8.2 Connection

- the example uses the current `PostgresSessionService` exposed through the local workspace
- the live runner path uses typed `UserId` and `SessionId` values even though the CRUD request structs remain string-addressed
- local `0.8.2` session migrations are more mature than the older drafts assumed, including fixes around PostgreSQL version tracking types
- the point is not just storage; it is durable continuity for a real agent flow

## Runtime Requirements

- `GOOGLE_API_KEY`
- `POSTGRES_URL`
- a reachable PostgreSQL instance

## Run

```bash
export GOOGLE_API_KEY=your-api-key
export POSTGRES_URL=postgres://user:pass@localhost:5432/adk_sessions
cargo run -p chapter8-postgres-sessions
```

## Expected Behavior

The program should connect, migrate, create two sessions, list Alice's sessions, run a two-turn conversation in `pg-session-1`, show that the persisted session retained its events, and then clean both sessions up.
