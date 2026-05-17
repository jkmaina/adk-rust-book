# Chapter 8 Session State

This crate adapts the same validated `session_state` pattern used in Chapter 5, but frames it here as the in-memory baseline before moving to persistent session backends.

## What It Demonstrates

- creating a session with initial JSON-valued state
- reusing the same typed `UserId` and `SessionId` across multiple turns
- distinguishing explicit session state from conversation history
- retrieving the session afterward to inspect continuity and stored events

## ADK-Rust 0.8.2 Connection

- the example keeps the explicit `Runner` path and typed runtime identity visible
- it demonstrates the modern `HashMap<String, Value>` session-state model rather than an old string-only mental model
- the visible recall comes from both state and history: the session stores preferences explicitly and also preserves prior turns as events
- it is the clean in-memory baseline before the persistent backends later in the chapter

## Run

```bash
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter8-session-state
```

## Expected Behavior

The program runs two turns in one in-memory session. In the second turn, the assistant should recall that the user was shopping for a cooking-related birthday gift, surface the stored `discovery` stage, and keep the reply short in line with the stored answer-style preference. The program then prints the session ID and event count.
