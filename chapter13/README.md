# Chapter 13 Examples

These crates are the active Chapter 13 ADK-Rust examples for testing, evaluation, and validation workflows.

In local `0.8.2`, this chapter also maps more cleanly onto the framework surface than earlier drafts did. The root crate now defaults to `minimal`, while `adk-eval` lives in the broader `standard` tier, so the examples deliberately separate the always-available offline checks from the narrower live smoke path.

## Included Crates

- `chapter13-quickstart-validation`
- `chapter13-eval-basic`
- `chapter13-eval-trajectory`

## Running from the Repo Root

```bash
cargo run -p chapter13-quickstart-validation
cargo run -p chapter13-eval-basic
cargo run -p chapter13-eval-trajectory
```

`quickstart-validation` is smoke-friendly:

- without environment variables, it explains the offline validation path and exits successfully
- with `BOOK_RUN_LIVE_SMOKE=1` and `GOOGLE_API_KEY`, it runs a one-prompt end-to-end validation

The two `eval-*` crates are deterministic offline validations and do not require API keys.
