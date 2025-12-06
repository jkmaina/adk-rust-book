# Async/Await Example

**What:** Learn how to write code that doesn't block—it can do multiple things "at once."

**Why:** Modern programs need to handle many tasks at the same time. Async lets you do this efficiently.

## What This Example Shows

- **async functions**: Functions that can pause and resume
- **#[tokio::main]**: Magic line that sets up the environment for async code
- **.await**: "Wait here until this finishes, then continue"
- **Non-blocking**: While waiting for one thing, you can do other things

## Run It

```bash
cargo run
```

Output:
```
Doing something async
Finished async work
```

## Why This Matters

**Without async** (blocking):
```
1. Start task A
2. Wait for A to finish (stuck here!)
3. Start task B
```

**With async** (non-blocking):
```
1. Start task A (can pause if waiting)
2. While A is waiting, start task B
3. A finishes, B finishes
```

## Beginner's Explanation

Think of cooking dinner:
- **Blocking**: Make pasta (10 min) → then make sauce (5 min) → then vegetables (3 min) = 18 min total
- **Async**: Start pasta (10 min) → while waiting, start sauce (5 min) → while waiting, prep vegetables (3 min) = 10 min total

Async lets your program multitask like a good chef!
