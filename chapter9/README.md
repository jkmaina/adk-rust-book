# Chapter 9 Examples

These crates are the active Chapter 9 ADK-Rust examples for callbacks, plugins, and guardrails.

They are intentionally small, but they map onto a broader `0.8.2` runtime story. Local ADK-Rust now has a richer callback surface than early drafts of the book assumed, including model and tool interception paths, explicit tool outcomes, and a more deliberate runtime guardrail story. This chapter starts with the simplest validated examples first so readers can learn the boundaries before they learn every hook.

## Included Crates

- `chapter9-callbacks-logging`
- `chapter9-callbacks-guardrails`
- `chapter9-plugin-system`

## Running from the Repo Root

```bash
cargo run -p chapter9-callbacks-logging
cargo run -p chapter9-callbacks-guardrails
cargo run -p chapter9-plugin-system
```

All three crates compile offline, but live execution requires `GOOGLE_API_KEY`.

## How This Chapter Fits ADK-Rust 0.8.2

- `chapter9-callbacks-logging` shows the smallest useful callback boundary: observe a run without changing the result.
- `chapter9-callbacks-guardrails` shows that callback return values are policy outcomes, not just control-flow tricks.
- `chapter9-plugin-system` shows how runner-level concerns belong in `PluginManager` and `RunnerConfig`, not inside every agent builder.

Together, those crates give the reader the first mental model they need before moving into richer callback surfaces elsewhere in the framework.
