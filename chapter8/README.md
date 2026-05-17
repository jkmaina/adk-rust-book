# Chapter 8 Examples

These crates are the active Chapter 8 ADK-Rust examples for sessions and state backends.

Chapter 8 is where the book treats continuity as a runtime resource instead of a vague chatbot feature. The examples here move from the in-memory baseline to durable PostgreSQL, MongoDB, and Neo4j-backed session stores.

## Baseline

- Rust `1.92+`
- Cargo `1.92+`
- edition `2024`
- local `adk-rust` `0.8.0` workspace
- `GOOGLE_API_KEY` for live Gemini runs in the in-memory example
- database-specific environment variables and infrastructure for the persistent backends

## Included Crates

- `chapter8-session-state`
- `chapter8-postgres-sessions`
- `chapter8-mongodb-sessions`
- `chapter8-neo4j-sessions`

## How This Chapter Fits ADK-Rust 0.8.0

- the runner now expects typed `UserId` and `SessionId` values, which makes session addressing more explicit at the runtime boundary
- the local session crate also added stronger typed-identity session helpers and optional encrypted wrappers such as `EncryptedSession<S>` and `EncryptionKey`
- the MongoDB standalone workaround `retryWrites=false` is no longer required in the local `0.8.0` session backend
- these examples intentionally teach the request-level session APIs directly so readers can see session lifecycle operations clearly

## Running from the Repo Root

```bash
cargo run -p chapter8-session-state
cargo run -p chapter8-postgres-sessions
cargo run -p chapter8-mongodb-sessions
cargo run -p chapter8-neo4j-sessions
```

The in-memory crate needs `GOOGLE_API_KEY` for live execution. The backend-specific crates also need their corresponding database environment variables and running infrastructure.
