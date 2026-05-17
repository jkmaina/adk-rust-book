# Chapter 9 Plugin System

This crate is the book adaptation of `../adk-playground/playground/backend/examples/plugin_system.rs`.

## What It Demonstrates

- composing multiple plugins into one `PluginManager`
- mixing run-lifecycle hooks with request and event hooks
- intercepting model traffic with `before_model` and `after_model`
- attaching the plugin manager through `RunnerConfig`

## ADK-Rust 0.8.0 Connection

This example shows the runner-level side of the `0.8.0` extension model. Agent callbacks remain useful for local control, but reusable runtime concerns such as lifecycle logging, metrics, and model interception belong more naturally in plugins. That architectural distinction is the main lesson of the crate.

## Run

```bash
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter9-plugin-system
```

## Expected Behavior

The program logs plugin lifecycle messages before, during, and after the model run, then streams the assistant response.
