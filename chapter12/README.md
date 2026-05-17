# Chapter 12 Examples

These crates are the active Chapter 12 ADK-Rust examples for streaming, realtime transport, and interactive launcher flows.

They intentionally show three different layers of responsiveness. In local `0.8.0`, ADK-Rust now has a wider surface for reasoning controls, realtime session management, and launcher-driven app wiring than the earlier manuscript assumed. These examples stay concrete by showing one readable path through each layer.

## Included Crates

- `chapter12-realtime-audio`
- `chapter12-thinking-gemini`
- `chapter12-cli-launcher`

## Running from the Repo Root

```bash
cargo run -p chapter12-realtime-audio
cargo run -p chapter12-thinking-gemini
cargo run -p chapter12-cli-launcher
```

`realtime-audio` requires `OPENAI_API_KEY`. The Gemini-based examples require `GOOGLE_API_KEY`.

## How This Chapter Fits ADK-Rust 0.8.0

- `chapter12-thinking-gemini` shows reasoning-rich event parts and tool metadata on the standard runner path.
- `chapter12-realtime-audio` shows that low-level realtime sessions are a different transport model, not just faster text streaming.
- `chapter12-cli-launcher` shows how launcher configuration bridges the same runtime into console and serve-oriented surfaces.

One practical `0.8.0` note matters here: realtime and graph remain specialist crates in the wider feature surface, while launcher and runner remain central bridges between local examples and deployed applications.
