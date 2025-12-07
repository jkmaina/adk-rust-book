# My First Agent

**What:** Build your first AI agent using adk-rust and Google's Gemini model.

**Why:** This verifies your adk-rust setup and shows how to create a basic agent that can talk to an LLM.

## What This Example Shows

- **Environment setup**: Loading API keys with `dotenv`
- **GeminiModel**: Creating a connection to Google's AI model
- **Agent creation**: Building your first agent
- **Optional API test**: Making a real LLM call with `--test-api`

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
# Verify setup (no API call)
cargo run

# Test with real API call
cargo run -- --test-api
```

## Why This Matters

This example confirms:
- Your adk-rust dependencies are installed correctly
- Your API key is configured properly
- You can create and use agents
- The connection to Gemini works

## Beginner's Explanation

**Agent** = A program that can think and respond using AI:
```
You: "Say hello"
Agent: Uses Gemini model to generate response
Agent: "Hello!"
```

**GeminiModel** = Google's AI model:
- Takes your text input
- Generates intelligent responses
- Accessed via API (needs API key)

**The `--test-api` flag**:
- Without it: Just verifies setup
- With it: Makes a real API call ("Say 'Hello' in one word")
- Uses your API quota

This is your first step toward building intelligent agents!
