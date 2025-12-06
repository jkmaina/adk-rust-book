# Arc (Atomic Reference Counting) Example

**What:** Learn how multiple threads can safely share the same data.

**Why:** Sometimes you need to give data to multiple threads. Normal Rust doesn't allow this. `Arc` makes it safe.

## What This Example Shows

- **Arc**: "Atomic Reference Counter"—lets multiple threads own the same data
- **Arc::clone()**: Creates a new reference without copying data
- **Threads**: Multiple workers doing work simultaneously
- **Reference counting**: Keeps track of how many threads are using the data

## Run It

```bash
cargo run
```

Watch the counter go up and down as threads start and finish.

## Why This Matters

**Normal Rust ownership**: "Only one owner" (safe but restrictive)

**Threads**: Multiple workers need access to the same thing

**Arc**: Lets multiple threads own data safely by reference-counting

## Beginner's Explanation

Think of Arc like a shared document:
- You have 1 shared model (expensive to copy)
- 3 threads need to use it simultaneously
- Instead of copying (3 copies), you create 3 references
- Each thread can read the model safely
- When a thread finishes, it releases its reference
- When all threads finish, the model is freed

**Arc::strong_count()** tells you: "How many threads are using this right now?"

```
Main creates Arc → count = 1
Thread 1 gets Arc → count = 2
Thread 2 gets Arc → count = 3
Thread 3 gets Arc → count = 4
Thread 1 finishes → count = 3
Thread 2 finishes → count = 2
Thread 3 finishes → count = 1
Main finishes → count = 0 (freed!)
```
