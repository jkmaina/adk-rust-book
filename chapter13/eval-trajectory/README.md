# Chapter 13 Eval Trajectory

This crate adapts `../adk-playground/docs_examples/evaluation/eval_test/src/trajectory.rs` into the book workspace.

## What It Demonstrates

- configuring tool trajectory and response similarity criteria
- using builder helpers like `exact_tools()` and `semantic_match()`
- scoring actual versus expected tool calls with `ToolTrajectoryScorer`

## ADK-Rust 0.8.2 Connection

Local `0.8.2` broadens the evaluation surface beyond exact response matching, including semantic and rubric-based criteria in `adk-eval`. This crate keeps the example offline and deterministic while still teaching the more realistic question: whether the agent behaved correctly, not merely whether it phrased the answer one exact way.

## Run

```bash
cargo run -p chapter13-eval-trajectory
```

## Expected Behavior

The binary performs assertion-based checks over evaluation criteria and scoring behavior, then exits after printing that all trajectory evaluation checks passed.
