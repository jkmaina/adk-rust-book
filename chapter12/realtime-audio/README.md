# Chapter 12 Realtime Audio

This crate is the book adaptation of `../adk-playground/playground/backend/examples/realtime_audio.rs`.

## What It Demonstrates

- connecting to OpenAI Realtime over the low-level session API
- streaming transcript and audio chunks as they arrive
- saving the final PCM16 stream as a WAV file for replay

## ADK-Rust 0.8.2 Connection

Local `0.8.2` expands the realtime surface beyond this basic example with interruption detection, session updates, and context mutation for long-lived calls. This crate stays lower-level on purpose so the reader can see the raw event model first: transcript deltas, audio deltas, completion, and error handling.

## Run

```bash
export OPENAI_API_KEY=your-api-key
cargo run -p chapter12-realtime-audio
```

## Expected Behavior

On a successful provider response, the program should print transcript text as it arrives, emit HTML-style markers for audio chunk consumers, and save a `.wav` file under `audio-output/`. If the provider rejects the request, the example should print the error cleanly and exit without writing audio.
