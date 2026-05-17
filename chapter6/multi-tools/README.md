# Chapter 6 Multi Tools

This crate is the book adaptation of `../adk-playground/playground/backend/examples/multi_tools.rs`.

## What It Demonstrates

- registering multiple tool types on one agent
- mixing weather lookup, arithmetic, and unit conversion
- routing different parts of a single request to the right tool

## ADK-Rust 0.8.0 Connection

- the example shows one agent selecting among several typed custom tools through the shared `Tool` surface
- `0.8.0` can also mix custom tools with provider-native built-in tools, but this example keeps every capability custom so tool-selection logic is easy to study
- the explicit `Runner` path and typed runtime identity remain visible instead of being hidden behind a convenience helper
- the tool handlers return data-first results, which is the right default when later systems may need to inspect or reuse the outputs

## Run

```bash
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter6-multi-tools
```

## Expected Behavior

The program asks one combined question that should lead the assistant to use weather, conversion, and arithmetic tools in one response flow. With the current static tool data, the answer should include Tokyo weather at `22°C`, the conversion `22°C = 71.6°F`, and the calculation `15% of 250 = 37.5`.
