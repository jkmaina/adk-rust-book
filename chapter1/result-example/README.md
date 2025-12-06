# Result & Error Handling Example

**What:** Learn how Rust handles things that might fail (like opening a file).

**Why:** Real programs have errors—files don't exist, permissions are denied, etc. Rust makes you handle these safely.

## What This Example Shows

- **Result type**: "This operation might succeed with a value, or fail with an error"
- **Using `match`**: Handling both success and failure explicitly
- **The `?` operator**: A shorthand to handle errors cleanly
- **File operations**: Trying to open a file and read it

## Run It

```bash
cargo run
```

Make sure `hello.txt` exists in the same directory, or you'll see a safe error message.

## Why This Matters

**Other languages**: Errors crash your program silently.

**Rust**: Forces you to handle errors explicitly. This makes programs more reliable.

## Beginner's Explanation

**Result** is like asking a question:
```
"Can you open this file?"
Either: "Yes, here it is" (Ok)
Or: "No, it doesn't exist" (Err)
```

**match** lets you respond to both:
```
match result {
    Ok(file) => "Great! I got the file",
    Err(error) => "Oops! The file didn't work"
}
```

**?** is shorthand:
```
File::open("file.txt")?
// If error, return it immediately
// If Ok, keep the value and continue
```
