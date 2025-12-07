# Structured Output

**What:** Force your agent to respond with structured JSON data instead of free-form text.

**Why:** Many applications need predictable, parseable responses. Structured output ensures the agent returns data in a specific format.

## What This Example Shows

- **Output schema**: Defining the exact JSON structure you want
- **JSON Schema**: Using standard JSON Schema to specify types and requirements
- **Predictable responses**: Agent always returns data matching your schema
- **Data validation**: Required fields, types, and descriptions

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

Try asking: "What is the weather in Tokyo?"

Expected response format:
```json
{
  "location": "Tokyo, Japan",
  "temperature": 72,
  "conditions": "Partly Cloudy",
  "forecast": [
    "Tomorrow: Sunny, 75°F",
    "Day 2: Cloudy, 68°F",
    "Day 3: Rain, 65°F"
  ]
}
```

## Why This Matters

**Structured output** is essential for:
- Building APIs that consume agent responses
- Integrating agents into larger systems
- Ensuring data consistency
- Parsing and processing responses programmatically

**Without structured output**:
```
"The weather in Tokyo is nice today, around 72 degrees..."
```
Hard to parse!

**With structured output**:
```json
{"location": "Tokyo, Japan", "temperature": 72, ...}
```
Easy to parse and use!

## Beginner's Explanation

**JSON Schema** = A contract for your data:
```json
{
  "type": "object",
  "properties": {
    "location": { "type": "string" },
    "temperature": { "type": "number" }
  },
  "required": ["location", "temperature"]
}
```

This says:
- Response must be an object
- Must have `location` (string) and `temperature` (number)
- Both fields are required

**How it works**:
1. You define the schema with `.output_schema()`
2. Agent receives your question
3. Model generates response matching the schema
4. You get predictable JSON every time

**Try modifying the schema**:
```rust
.output_schema(json!({
    "type": "object",
    "properties": {
        "city": { "type": "string" },
        "temp_celsius": { "type": "number" },
        "humidity": { "type": "number" }
    }
}))
```

Now the agent will return different fields!
