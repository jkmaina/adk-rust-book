# Chapter 15 Examples

These crates are the active Chapter 15 ADK-Rust examples for observability and operations.

In local `0.8.0`, this chapter maps directly onto the current runtime surface. Telemetry is part of the `standard` tier rather than the default `minimal` starter set, and the examples show both manual instrumentation patterns and the wider OpenTelemetry-aligned usage story.

## Included Crates

- `chapter15-telemetry-basic`
- `chapter15-telemetry-spans`
- `chapter15-telemetry-demo`

## Running from the Repo Root

```bash
cargo run -p chapter15-telemetry-basic
cargo run -p chapter15-telemetry-spans
cargo run -p chapter15-telemetry-demo
```

The first two crates are deterministic and offline. `telemetry-demo` also runs offline by default and only activates its live model path when `BOOK_RUN_LIVE_SMOKE=1` and `GOOGLE_API_KEY` are set.
