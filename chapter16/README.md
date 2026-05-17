# Chapter 16 Examples

These crates are the active Chapter 16 ADK-Rust examples for security and production best practices.

In local `0.8.0`, this chapter maps directly onto the current runtime surface. Guardrails and auth are part of the `standard` tier rather than the default `minimal` starter set, while the examples show how identity, policy, audit, and retrieval boundaries compose into a real production posture.

## Included Crates

- `chapter16-guardrails-advanced`
- `chapter16-auth-identity`
- `chapter16-auth-audit`
- `chapter16-auth-rbac`
- `chapter16-auth-sso`
- `chapter16-rag-multi-collection`
- `chapter12-thinking-gemini` (reused)

## Running from the Repo Root

```bash
cargo run -p chapter16-guardrails-advanced
cargo run -p chapter16-auth-identity
cargo run -p chapter16-auth-audit
cargo run -p chapter16-auth-rbac
cargo run -p chapter16-auth-sso
cargo run -p chapter16-rag-multi-collection
```

Most Chapter 16 crates have meaningful offline behavior by default and gate live model or network paths behind `BOOK_RUN_LIVE_SMOKE=1`.
