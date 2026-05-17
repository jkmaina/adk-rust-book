# Chapter 4 Examples

These crates are the active Chapter 4 ADK-Rust examples for controlled outputs.

## Baseline

- Rust `1.92+`
- Cargo `1.92+`
- Rust edition `2024`
- Local `adk-rust` `0.8.0` workspace at `../adk-rust`

## How This Chapter Fits ADK-Rust 0.8.0

Chapter 4 takes the explicit first-agent runtime from Chapter 3 and adds the next important control surface: `output_schema(...)`.

That makes this chapter about three levers working together:

- instructions shape behavior
- session state shapes context
- schemas shape output

The old archived minimal-agent material is not the active path anymore. The current `0.8.0` baseline is the validated structured-output example plus the Chapter 3 quickstart flow it builds on.

## Included Crates

- `chapter4-structured-output`

## Running from the Repo Root

```bash
cargo run -p chapter4-structured-output
```

This chapter replaces the old archived `my-minimal-agent` branch with a single backend-derived structured output example that reflects the current `adk-rust` `0.5` API surface.
