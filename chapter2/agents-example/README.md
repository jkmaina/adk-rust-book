# Agents Example

**What:** Learn how to compose different agent implementations using trait objects and shared ownership.

**Why:** Real agent systems need multiple agents working together. This shows how to build modular, composable agent pipelines.

## What This Example Shows

- **Trait objects**: Using `Arc<dyn Agent>` to store different agent types together
- **Shared ownership**: `Arc` lets multiple parts of your code share the same agent safely
- **Composition**: `SequentialAgent` runs multiple sub-agents in order
- **Three agent types**: `LlmAgent`, `CustomAgent`, and `SequentialAgent`

## Run It

```bash
cargo run
```

Output:
```
=== Running agent: llm-main ===
[LLM:llm-main] processing: Hello agent world
[LLM:llm-main] done
=== Running agent: custom-echo ===
[Custom:custom-echo] received: Hello agent world
[Custom:custom-echo] finished
=== Running agent: pipeline-1 ===
[Seq:pipeline-1] starting sequence
[Seq:pipeline-1] -> running sub-agent llm-sub
[LLM:llm-sub] processing: Hello agent world
[LLM:llm-sub] done
[Seq:pipeline-1] -> running sub-agent custom-sub
[Custom:custom-sub] received: Hello agent world
[Custom:custom-sub] finished
[Seq:pipeline-1] sequence complete
All agents finished.
```

## Why This Matters

Storing `Arc<dyn Agent>` in a `Vec` lets you:
- Build heterogeneous pipelines (different agent types together)
- Keep ownership simple and thread-safe
- Create modular systems (chatbots, pipelines, micro-agents)

## Beginner's Explanation

**Trait objects** let you store different types together:
```
Vec<Arc<dyn Agent>> can hold:
- LlmAgent (talks to AI models)
- CustomAgent (custom logic)
- SequentialAgent (runs other agents in order)
```

**Arc** = "Atomic Reference Counter":
- Multiple parts of your code can share the same agent
- Thread-safe and memory-safe
- No copying needed

**Composition** = Building complex behavior from simple parts:
```
SequentialAgent {
  agents: [
    LlmAgent,      // Step 1: Process with AI
    CustomAgent    // Step 2: Custom logic
  ]
}
```

This is how you build flexible agent systems where agents can be mixed, matched, and composed!