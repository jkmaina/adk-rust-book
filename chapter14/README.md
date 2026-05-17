# Chapter 14 Examples

These crates are the active Chapter 14 ADK-Rust examples for deployment and packaging workflows.

In local `0.8.0`, this chapter maps more cleanly onto the framework surface than earlier drafts did. The root crate defaults to `minimal`, so deployment paths are explicit feature choices rather than part of the starter footprint, and `Launcher` now supports both simple `run()` entrypoints and production-oriented app-building paths.

## Included Crates

- `chapter14-deployment-launcher`
- `chapter14-a2a-server`
- `chapter14-a2a-client`

## Running from the Repo Root

```bash
cargo run -p chapter14-deployment-launcher
cargo run -p chapter14-a2a-server
cargo run -p chapter14-a2a-client
```

The new Chapter 14 crates are packaging-friendly by default:

- they explain the deployment path and exit successfully with no credentials
- they activate live behavior only when the required environment variables are set

To exercise the launcher or A2A paths against a live model, set `BOOK_RUN_LIVE_SMOKE=1` and `GOOGLE_API_KEY`.
