# Text Summarizer

**What:** Build an agent that summarizes text and automatically saves the result to session state.

**Why:** Learn how to persist agent outputs in session state for later use by other agents or application logic.

## What This Example Shows

- **output_key**: Automatically save agent response to session state
- **Session state persistence**: Access saved summaries after agent runs
- **State inspection**: View what's stored in session state
- **Chaining agents**: Foundation for multi-agent workflows where one agent's output becomes another's input

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

Example session:
```
🤖 Agent ready! Paste text to summarize (or 'exit' to quit).

You: Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. It accomplishes these goals without garbage collection.
Summary: Rust is a fast, memory-safe systems language without garbage collection.

📝 Saved to state['summary']: String("Rust is a fast, memory-safe systems language without garbage collection.")

You: exit
```

## Why This Matters

**output_key** enables:
- **Agent chaining**: One agent's output feeds into another
- **State persistence**: Keep results across conversation turns
- **Workflow building**: Create complex multi-step processes
- **Data accumulation**: Collect information over time

**Without output_key**:
```rust
// Response is displayed but not saved
agent.run(content).await?;
// Summary is lost!
```

**With output_key**:
```rust
.output_key("summary")
// After agent runs:
// state["summary"] = "The agent's summary text..."
```

## Beginner's Explanation

**output_key** = Automatic save to session state:
```
Input: "Long text about Rust..."
         ↓
Agent summarizes
         ↓
Output: "Rust is a fast, memory-safe language"
         ↓
Automatically saved to state["summary"]
```

**Session state** = Memory that persists:
```rust
{
  "summary": "Rust is a fast, memory-safe language"
}
```

**How it works**:
1. You set `.output_key("summary")` when building the agent
2. Agent processes your text and generates a summary
3. The summary is automatically saved to `state["summary"]`
4. You can access it later: `session.state().get("summary")`

**Real-world use case** - Multi-agent pipeline:
```
Agent 1 (Summarizer):
  Input: Long article
  Output → state["summary"]

Agent 2 (Translator):
  Input: state["summary"]
  Output → state["translation"]

Agent 3 (Formatter):
  Input: state["translation"]
  Output: Final formatted result
```

Each agent reads from and writes to session state, creating a pipeline!

**Try modifying**:
```rust
.output_key("my_summary")  // Save to different key
.output_key("result")      // Or use generic key name
```

The key name is up to you—just remember it when accessing the state later.
