# Time-Aware Agent

**What:** Build an agent that adapts its greeting based on the current time of day.

**Why:** Learn how to create time-aware agents that adjust their behavior dynamically.

## What This Example Shows

- **Time-based logic**: Using chrono to get current time
- **Dynamic instruction**: Building instructions at startup based on time
- **Pattern matching**: Using match expressions for time ranges
- **String formatting**: Creating personalized greetings

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

The agent will greet you differently based on the time of day:
- 5 AM - 11 AM: "Good morning!"
- 12 PM - 5 PM: "Good afternoon!"
- 6 PM - 9 PM: "Good evening!"
- 10 PM - 4 AM: "Hello!"

## Why This Matters

**Time-aware agents** enable:
- **Contextual greetings**: Different greetings for different times
- **Appropriate tone**: Morning energy vs. evening calm
- **User experience**: More natural, human-like interactions
- **Dynamic behavior**: Adapting to real-world context

## Beginner's Explanation

**Time-based greeting** = Check time at startup:

```
Startup:
  1. Get current hour with chrono::Local::now().hour()
  2. Match hour to time range (5-11, 12-17, 18-21)
  3. Select appropriate greeting
  4. Build instruction with greeting
  5. Create agent with that instruction
```

**The flow**:
```
User sends message at 9 AM
  ↓
instruction_provider runs
  ↓
Gets current hour: 9
  ↓
Matches 5..=11 → "Good morning"
  ↓
Generates: "Good morning! You are a helpful assistant. The user's ID is user_123..."
  ↓
Agent uses this instruction
  ↓
Agent responds with morning-appropriate tone
```

**Real-world use cases**:
1. **Customer service**: Different greetings for business hours vs. after hours
2. **Health apps**: Morning motivation vs. evening relaxation
3. **Productivity tools**: Energetic morning vs. wind-down evening
4. **Educational apps**: Adapt learning style to time of day

**Try modifying** for day of week:
```rust
use chrono::Datelike;

let day = chrono::Local::now().weekday();
let is_weekend = matches!(day, chrono::Weekday::Sat | chrono::Weekday::Sun);

let tone = if is_weekend {
    "casual and friendly"
} else {
    "professional and efficient"
};

let instruction = format!(
    "You are a {} assistant. Help the user with their questions.",
    tone
);
```

This pattern creates more natural, context-aware interactions!
