# Customer Support Agent

Beginner-friendly customer support agent that combines the main ADK-Rust lessons from the book into one runnable project.

## What This Example Teaches

- Chapter 3 concepts: explicit model, session, runner, content, and streamed responses
- Chapter 5 concepts: session-backed personalization and multi-turn continuity
- Chapter 6 concepts: typed tools for exact operations instead of free-form guessing
- Chapter 7 concepts: specialist delegation through agent-as-tool patterns
- Chapter 16 habits: clear boundaries around refunds, escalation, and support actions

## How to Read the Code

If you are using this project as a study example, read it in this order:

1. `src/main.rs`: the typed tool inputs and tool functions
2. `src/main.rs`: `create_session`, which seeds per-customer state
3. `src/main.rs`: the specialist agents, which each own one job
4. `src/main.rs`: the coordinator agent, which delegates instead of doing every task itself
5. `src/main.rs`: `build_runner` and `print_turn`, which show the runtime boundary and streamed execution

That order matches the book's progression from basic runner setup to session state, tools, delegation, and production boundaries.

## What the Agent Does

The example simulates a small support team:

- an order specialist checks order details with a typed tool
- a billing specialist decides whether a refund can be approved immediately
- the same billing specialist creates a support ticket when manager approval is required
- a coordinator agent delegates to the right specialist and gives the customer one clear answer

The runtime also seeds session state with customer metadata so the coordinator can personalize replies without hardcoding a different prompt for every user.

## Why This Project Is Useful

This is the kind of example that helps a new reader move from isolated chapter exercises to a realistic application shape. It is still small enough to read in one sitting, but it shows how the pieces from the book fit together:

- typed runtime identity
- session-backed instructions
- tool-backed business actions
- specialist delegation
- multi-turn execution in one session

## Run It

```bash
cargo run -p customer-support-agent
```

You will need:

- `GOOGLE_API_KEY` in your environment or `.env`

The program runs two support turns in one session:

1. a damaged order with an immediate refund
2. a higher-value order that requires escalation and ticket creation

## What to Notice

- The tools return structured data; the agents narrate the result.
- The coordinator does not do billing or order lookup directly; it delegates.
- Session state is used for personalization, while session history is used for continuity.
- Refund rules are enforced through code-level tools rather than prompt-only instructions.
- The example is intentionally explicit. It does not hide the model, session, or runner behind helper wrappers, because those are the core runtime concepts the book is teaching.
