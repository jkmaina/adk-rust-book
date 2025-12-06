# Agent Trait Example

**What:** Learn how to create flexible "agent" types that can handle messages.

**Why:** Real programs need pluggable, swappable components. Traits let you define a contract that many types can implement.

## What This Example Shows

- **Traits**: Define what methods a type must have (like a contract)
- **Async methods**: Methods that pause and resume (using `async`)
- **Implementing traits**: Making a `CustomerServiceAgent` that follows the trait contract
- **Calling trait methods**: Using the agent to handle requests

## Run It

```bash
cargo run
```

Output:
```
Agent Alice responded: Processing: I need help with my order
```

## Why This Matters

**Traits** let you write flexible code:
- Define "what an Agent must do" in one place
- Let multiple types implement it (different agents)
- Swap agents without rewriting code

## Beginner's Explanation

**Trait** = A contract or interface:
```
An Agent must:
1. Have a name() method
2. Have an async handle() method for processing requests
```

**Implementation** = Making a type follow the contract:
```
CustomerServiceAgent:
✓ Has name() → returns "Alice"
✓ Has async handle() → processes customer requests
```

**Using it**: You can call methods on any Agent, knowing they have these methods!

This is how you build extensible systems where different agents can plug in easily.
