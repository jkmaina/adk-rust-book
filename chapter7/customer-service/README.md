# Chapter 7 Customer Service

This crate is the book adaptation of `../adk-playground/playground/backend/examples/customer_service.rs`.

## What It Demonstrates

- multi-step orchestration through a coordinator agent
- agent-as-tool delegation for billing and manager specialists
- escalation flow where the first specialist cannot fully resolve the request

## ADK-Rust 0.8.0 Connection

- this example combines workflow thinking with `AgentTool`-based delegation instead of using a pure workflow container alone
- local `0.8.0` hardened sub-agent-as-tool execution, which makes escalation flows like this materially more dependable
- the billing specialist is allowed to initiate the refund but not fully approve amounts over `$50`, which creates a real handoff contract
- the coordinator still uses the same explicit `Runner` flow and typed `UserId` / `SessionId` boundary as the rest of the book

## Run

```bash
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter7-customer-service
```

## Expected Behavior

The program submits a duplicate-charge refund request for `$79`. The billing specialist should detect that the refund exceeds the approval limit, the manager specialist should approve the escalation, and the final answer should summarize the full resolution rather than stopping at the first handoff.
