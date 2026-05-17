# Chapter 7 Loop Workflow

This crate is the book adaptation of `../adk-playground/playground/backend/examples/loop_workflow.rs`.

## What It Demonstrates

- iterative refinement with `LoopAgent`
- bounded retries using `with_max_iterations`
- explicit early exit with `ExitLoopTool`

## ADK-Rust 0.8.0 Connection

- this example uses the current `LoopAgent::new(...).with_max_iterations(...)` API shape directly
- `ExitLoopTool` gives the semantic stop signal, while `with_max_iterations(...)` provides the operational safety limit
- the loop pattern is useful precisely because it is bounded; without those limits it would be a bad production design
- the explicit `Runner` path and typed `UserId` / `SessionId` values remain visible just as in the earlier chapters

## Run

```bash
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter7-loop-workflow
```

## Expected Behavior

The program asks for a haiku about programming in Rust. The refiner should either improve the draft a few times or decide it is already good enough and exit the loop early. In either case, the example is meant to demonstrate bounded refinement rather than endless iteration.
