# Chapter 12 Thinking Gemini

This crate is the book adaptation of `../adk-playground/playground/backend/examples/thinking_gemini.rs`.

## What It Demonstrates

- reasoning traces when a supported model surfaces `Part::Thinking`
- tool calls that carry `thought_signature`
- a multi-turn session where Gemini can reuse prior reasoning context

## ADK-Rust 0.8.0 Connection

In local `0.8.0`, reasoning visibility is a provider capability rather than a universal guarantee. This crate uses Gemini because it makes `Part::Thinking` and thought-signature handling easy to inspect, while the broader repo also adds provider-specific reasoning controls elsewhere in the model layer.

## Run

```bash
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter12-thinking-gemini
```

## Expected Behavior

The program should print tool calls, function responses, and the final answers for two turns in the same session. When the selected model emits visible `Part::Thinking` content, the example also prints those reasoning blocks; otherwise, thinking may only be visible through usage metadata or `thought_signature`.
