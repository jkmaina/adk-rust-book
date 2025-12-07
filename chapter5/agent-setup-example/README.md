# Agent Setup Example

**What:** Learn how to configure multiple agents with output_key and state templates without running them.

**Why:** Understand agent configuration before building complex pipelines. See how agents pass data through session state.

## What This Example Shows

- **Agent configuration**: Setting up multiple agents with different roles
- **output_key**: How each agent saves its output to session state
- **State templates**: Using `{research_findings}` and `{draft_article}` placeholders
- **Agent composition**: Preparing agents for sequential execution
- **No execution**: Just setup and configuration demonstration

## Prerequisites

Set your Google API key in `.env` file:
```text
GOOGLE_API_KEY=your_api_key_here
```

Or in PowerShell:
```powershell
$env:GOOGLE_API_KEY = 'your_api_key_here'
```

## Run It

```bash
cargo run
```

Output:
```
Content pipeline agents created!
1. researcher saves to 'research_findings'
2. writer uses findings, saves to 'draft_article'
3. editor uses draft, saves to 'final_article'
```

## Why This Matters

This example shows the **configuration pattern** for multi-agent systems:
- Each agent has a specific role
- Each agent saves its output with `output_key`
- Subsequent agents read previous outputs using state templates

**The data flow**:
```
researcher
  ↓ saves to state["research_findings"]
writer (reads {research_findings})
  ↓ saves to state["draft_article"]
editor (reads {draft_article})
  ↓ saves to state["final_article"]
```

## Beginner's Explanation

**This example doesn't run the agents** - it just shows how to set them up.

**Agent 1 - Researcher**:
```rust
.instruction("Research {topic} and provide key facts.")
.output_key("research_findings")
```
- Takes `{topic}` from user input
- Saves result to `state["research_findings"]`

**Agent 2 - Writer**:
```rust
.instruction("Write about {topic}. Use: {research_findings}")
.output_key("draft_article")
```
- Reads `{topic}` from user input
- Reads `{research_findings}` from state (Agent 1's output)
- Saves result to `state["draft_article"]`

**Agent 3 - Editor**:
```rust
.instruction("Edit this article: {draft_article}")
.output_key("final_article")
```
- Reads `{draft_article}` from state (Agent 2's output)
- Saves result to `state["final_article"]`

**To actually run these agents**, you would:
1. Use `SequentialAgent` to run them in order (see content-pipeline example)
2. Or manually orchestrate them with a custom runner

**Key concepts**:
- `output_key("name")` → Saves agent output to `state["name"]`
- `{variable}` in instruction → Reads from `state["variable"]`
- State persists across agent executions in the same session

This pattern is the foundation for building complex multi-agent workflows!
