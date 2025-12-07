# Agent Sandbox

**What:** An interactive AI assistant you can chat with in your terminal.

**Why:** Experience how agents work by having a real conversation with one.

## What This Example Shows

- **Interactive CLI**: Chat with your agent in real-time
- **Agent launcher**: Setting up an interactive session
- **Conversation loop**: Sending messages and receiving responses
- **Real AI integration**: Using Gemini to power responses

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

Then type your messages and press Enter. Type `exit` or `quit` to stop.

## Why This Matters

This is your playground for:
- Testing agent behavior
- Experimenting with prompts
- Understanding how agents respond
- Building intuition for agent development

## Beginner's Explanation

**Interactive agent** = A chatbot you control:
```
You: What is Rust?
Agent: Rust is a systems programming language...
You: Tell me more
Agent: It focuses on safety and performance...
```

**How it works**:
1. You type a message
2. Agent sends it to Gemini
3. Gemini generates a response
4. Agent displays the response
5. Repeat!

This is your sandbox to experiment and learn how agents behave.
