# My Minimal Agent

**What:** Learn how to build agents with different capabilities through four example binaries.

**Why:** See how agents evolve from basic to advanced—minimal setup, interactive chat, search tools, and custom functions.

## What This Example Shows

- **Four agent types**: Minimal, interactive, search-enabled, and tool-enabled
- **Multiple binaries**: Different executables in one project
- **Tool integration**: How agents use external tools (search, calculator)
- **Progressive complexity**: Start simple, add features step by step

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
# 1. Minimal agent (creates agent and exits)
cargo run --bin minimal

# 2. Interactive agent (chat in terminal)
cargo run --bin interactive

# 3. Search-enabled agent (can search the web)
cargo run --bin search

# 4. Calculator agent (can do math)
cargo run --bin custom_tools
```

## Why This Matters

Each binary teaches a different concept:
- **minimal**: Basic agent creation
- **interactive**: User interaction loop
- **search**: Integrating external tools
- **custom_tools**: Building your own tools

## Beginner's Explanation

**Multiple binaries** = Multiple programs in one project:
```
my-minimal-agent/
├── src/
    ├── bin/
        ├── minimal.rs       (Binary 1)
        ├── interactive.rs   (Binary 2)
        ├── search.rs        (Binary 3)
        └── custom_tools.rs  (Binary 4)
```

**Tools** = Functions agents can call:
```
You: "What is 25 * 4?"
Agent: Calls calculator tool with {a: 25, b: 4, operation: "multiply"}
Tool: Returns 100
Agent: "The answer is 100"
```

**Calculator tool accepts**:
- Simple expressions: `{"expression": "1+1"}`
- Structured input: `{"a": 3, "b": 4, "operation": "multiply"}`
- Operations: `add`, `subtract`, `multiply`, `divide`

This shows how agents become more powerful by adding tools!
