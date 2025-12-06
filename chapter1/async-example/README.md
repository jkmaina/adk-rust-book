# Async/Await Example

Explore Rust's **asynchronous programming** with Tokio.

## What This Example Shows

- **async functions**: Non-blocking code that returns a Future
- **#[tokio::main]**: Sets up the async runtime
- **.await**: Suspends execution until the Future completes
- **Futures are lazy**: Calling an async fn doesn't execute it—you must `.await`

## Run It

```bash
cargo run
```

## Key Concepts

- `async fn` creates a function that returns a Future (not executed immediately)
- `.await` blocks until the Future completes (but doesn't block other tasks)
- `#[tokio::main]` macro handles runtime setup
- Async enables concurrent execution without threads (lightweight tasks)
