# Content Pipeline

**What:** Build a multi-agent pipeline where agents work sequentially, each using the previous agent's output.

**Why:** Real applications need complex workflows where multiple agents collaborate. This shows how to chain agents using SequentialAgent and session state.

## What This Example Shows

- **SequentialAgent**: Running multiple agents in order
- **Agent chaining**: Each agent uses previous agent's output
- **State templates**: Using `{research_findings}` and `{draft_article}` placeholders
- **Multi-step workflow**: Research → Write → Edit pipeline
- **output_key**: Each agent saves its result for the next agent

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

When prompted, enter your message with the topic:
```
You: Write about Rust programming language
```

The SequentialAgent will automatically:
1. **Researcher** gathers facts → saves to `state["research_findings"]`
2. **Writer** reads findings, writes article → saves to `state["draft_article"]`
3. **Editor** reads draft, polishes it → saves to `state["final_article"]`

You'll see the final polished article as output.

## Why This Matters

**SequentialAgent** enables:
- **Complex workflows**: Break big tasks into smaller steps
- **Specialization**: Each agent does one thing well
- **Reusability**: Mix and match agents for different pipelines
- **State passing**: Automatic data flow between agents

**The pipeline pattern**:
```
Input: "Write about Rust"
         ↓
Agent 1 (Researcher):
  Output → state["research_findings"]
         ↓
Agent 2 (Writer):
  Input: state["research_findings"]
  Output → state["draft_article"]
         ↓
Agent 3 (Editor):
  Input: state["draft_article"]
  Output → state["final_article"]
         ↓
Final Result: Polished article
```

## Beginner's Explanation

**SequentialAgent** = Run agents one after another:
```rust
SequentialAgent::new("pipeline", vec![
    Arc::new(researcher),  // Step 1
    Arc::new(writer),      // Step 2
    Arc::new(editor),      // Step 3
])
```

**How data flows**:
```
Researcher instruction:
  "Research {topic} and provide key facts."
  → Saves to state["research_findings"]

Writer instruction:
  "Write about {topic}. Use: {research_findings}"
  → Reads from state["research_findings"]
  → Saves to state["draft_article"]

Editor instruction:
  "Edit this article: {draft_article}"
  → Reads from state["draft_article"]
  → Saves to state["final_article"]
```

**Template placeholders** get replaced:
- `{topic}` → User's input
- `{research_findings}` → Output from researcher
- `{draft_article}` → Output from writer

**Real-world use cases**:
1. **Content creation**: Research → Write → Edit → Publish
2. **Data processing**: Extract → Transform → Validate → Store
3. **Customer support**: Classify → Research → Draft → Review
4. **Code generation**: Plan → Code → Test → Document

**Try modifying**:
```rust
// Add a 4th agent
let publisher = LlmAgentBuilder::new("publisher")
    .instruction("Format {final_article} as HTML")
    .model(model.clone())
    .output_key("html_article")
    .build()?;

let pipeline = SequentialAgent::new("pipeline", vec![
    Arc::new(researcher),
    Arc::new(writer),
    Arc::new(editor),
    Arc::new(publisher),  // New step!
]);
```

Each agent in the sequence can read from any previous agent's output using state templates!

**Note**: The `{topic}` placeholder needs to be in your input message for the agents to extract it.
