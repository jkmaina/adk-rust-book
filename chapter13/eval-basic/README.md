# Chapter 13 Eval Basic

This crate adapts `../adk-playground/docs_examples/evaluation/eval_test/src/basic.rs` into the book workspace.

## What It Demonstrates

- parsing an `adk-eval` test file from JSON
- validating the expected conversation structure
- creating and round-tripping a test file programmatically

## ADK-Rust 0.8.2 Connection

In local `0.8.2`, evaluation sits in the broader `standard` tier rather than in the default `minimal` starter set. This crate stays intentionally deterministic: it validates schema shape and builder usage without requiring a model call, which makes it useful in ordinary local and CI workflows.

## Run

```bash
cargo run -p chapter13-eval-basic
```

## Expected Behavior

The binary performs assertions against the parsed evaluation schema and exits after printing that all basic evaluation checks passed.
