# Chapter 8 MongoDB Sessions

This crate adapts `../adk-playground/playground/backend/examples/mongodb_sessions.rs` for the book and uses the local `MongoSessionService` type exposed by `adk-rust` 0.5.

## What It Demonstrates

- durable session lifecycle operations in MongoDB
- nested JSON session state that feels natural in a document store
- running a live shopping-oriented agent turn against persisted state
- retrieving and deleting sessions after the interaction

## ADK-Rust 0.8.2 Connection

- the example uses the local `MongoSessionService` directly from the current workspace
- the session state demonstrates why `HashMap<String, Value>` is useful: nested profile data fits naturally
- local `0.8.2` no longer requires the old `retryWrites=false` workaround for standalone MongoDB deployments
- the runner still uses typed `UserId` and `SessionId` values at execution time

## Runtime Requirements

- `GOOGLE_API_KEY`
- `MONGODB_URL`
- a reachable MongoDB instance

## Run

```bash
export GOOGLE_API_KEY=your-api-key
export MONGODB_URL=mongodb://localhost:27017
cargo run -p chapter8-mongodb-sessions
```

## Expected Behavior

The program should connect, migrate, create two sessions with nested profile data, list Charlie's sessions, run a gift-shopping turn against `mongo-session-1`, show that persisted events were recorded, and then clean both sessions up.
