# Chapter 5 Session State

This crate is the book adaptation of the validated backend example at `../adk-playground/playground/backend/examples/session_state.rs`.

## What It Demonstrates

- creating a session with initial state
- sending multiple turns through the same `Runner`
- relying on session history for continuity across turns
- inspecting stored session metadata after the interaction

## ADK-Rust 0.8.2 Connection

- continuity depends on reusing the same typed `UserId` and `SessionId`
- a session can carry both explicit keyed state and an event log of what happened
- this example keeps `InMemorySessionService` for clarity, but local `0.8.2` also adds `EncryptionKey` and `EncryptedSession<S>` for protected storage layers
- the visible recall in the second turn comes mainly from shared session history, while the seeded state shows that structured session context can live alongside that history

## Setup

Set your Gemini API key:

```bash
export GOOGLE_API_KEY=your-api-key
```

## Run

From the book repo root:

```bash
cargo run -p chapter5-session-state
```

## Expected Behavior

The program runs two turns in one session, then prints the session ID and the number of stored events. The second turn should reflect the name and programming language introduced in the first turn, and the final metadata output shows that the session retained inspectable runtime history.
