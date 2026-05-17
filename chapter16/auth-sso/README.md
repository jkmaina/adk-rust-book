# Chapter 16 Auth SSO

This crate adapts the identity-provider and claims-mapping parts of `../adk-playground/playground/backend/examples/auth_sso.rs` into the book workspace.

## What It Demonstrates

- built-in SSO providers and JWT validator setup
- mapping external groups to internal ADK roles
- optional OIDC discovery when smoke mode is enabled

## ADK-Rust 0.8.0 Connection

Local `0.8.0` sharpens the SSO story with clearer claims mapping, stronger tenant controls, and the auth-bridge path for deployed servers. This crate stays focused on provider setup and role mapping so the organizational identity boundary is easy to inspect.

## Run

```bash
cargo run -p chapter16-auth-sso
```
