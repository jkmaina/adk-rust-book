# Arc (Atomic Reference Counting) Example

Master **safe shared ownership** across threads.

## What This Example Shows

- **Arc<T>**: Atomically reference-counted pointer for thread-safe sharing
- **Arc::clone()**: Creates a new reference (doesn't clone the data)
- **Thread spawning**: Multiple threads accessing the same data safely
- **Reference counting**: Tracking how many owners exist

## Run It

```bash
cargo run
```

## Key Concepts

- `Arc` allows multiple threads to own the same data (unlike regular Rust ownership)
- `Arc::strong_count()` shows how many owners currently exist
- Thread-safe but data must be immutable (use `Arc<Mutex<T>>` for shared mutability)
- Ideal for sharing expensive resources (like models, connections) across threads
