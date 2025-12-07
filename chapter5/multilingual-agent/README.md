# Multilingual Agent

**What:** Build an agent that adapts to user language, name, and expertise level using session state.

**Why:** Real agents need to personalize responses based on user context. This shows how to use dynamic instructions with session state.

## What This Example Shows

- **Session state**: Storing user preferences (name, language, expertise)
- **Dynamic instructions**: Using placeholders like `{user:name}` that get replaced at runtime
- **SessionService**: Managing state across conversations
- **Custom runner loop**: Building your own interaction loop with pre-seeded state
- **Personalization**: Agent adapts responses based on user context

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

Example conversation:
```
🤖 Agent ready! Type your questions (or 'exit' to quit).

(Context: User=Alice, Language=French, Expertise=Intermediate)

You: What is Rust?
Assistant: Bonjour Alice! Rust est un langage de programmation...

You: Tell me about variables
Assistant: D'accord Alice, les variables en Rust...
```

## Why This Matters

**Session state** lets you:
- Remember user preferences across conversations
- Personalize agent behavior
- Adapt language and complexity dynamically
- Build context-aware applications

**Dynamic instructions** with placeholders:
- `{user:name}` → "Alice"
- `{user:language}` → "French"
- `{user:expertise}` → "intermediate"

## Beginner's Explanation

**Session state** = Memory for your agent:
```
Session {
  user:name = "Alice"
  user:language = "French"
  user:expertise = "intermediate"
}
```

**Dynamic instructions** = Templates that get filled in:
```
Template: "You are assisting {user:name} in {user:language}"
         ↓
Result:   "You are assisting Alice in French"
```

**How it works**:
1. Create session with initial state (name, language, expertise)
2. Agent instruction has placeholders: `{user:name}`, `{user:language}`
3. At runtime, placeholders are replaced with actual values
4. Agent responds in the user's language and expertise level

**Try modifying**:
```rust
state.insert("user:language".to_string(), "Spanish".into());
state.insert("user:expertise".to_string(), "beginner".into());
```

Now the agent will respond in Spanish with beginner-friendly explanations!
