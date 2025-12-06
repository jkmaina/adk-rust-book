# Variables Example

**What:** Learn how to store and manage data in variables.

**Why:** Programs store information (scores, names, counts). Variables let you name and use that information.

## What This Example Shows

- **Creating variables**: Using `let`
- **Immutability**: Variables can't change by default (safe!)
- **Mutability**: Using `let mut` to allow changes
- **Shadowing**: Reusing variable names
- **Scope**: Where variables exist
- **Type inference**: Rust figures out the type from the value

## Run It

```bash
cargo run
```

## Why This Matters

**Rust's default: "Variables can't change"**

Why? Safety. If you don't explicitly say "let mut", Rust assumes you don't want surprises.

## Beginner's Explanation

**Immutable variable** (can't change):
```rust
let score = 10;  // score is 10
score = 20;      // ERROR! Can't change score
```

**Mutable variable** (can change):
```rust
let mut score = 10;  // score is 10
score = 20;          // OK! Now score is 20
```

**Shadowing** (reuse the name):
```rust
let x = 5;        // x is 5
let x = x + 1;    // x is now 6 (shadowed the old x)
let x = "hello";  // x is now a String! Different type!
```

Think of shadowing like:
- You named a variable `x` = 5
- You decide to create a NEW variable also called `x` = 6
- The old `x` is gone, the new `x` is what matters now

**Scope** (where variables live):
```rust
{
    let name = "Alice";  // name exists here
    println!("{}", name); // OK!
}
println!("{}", name);     // ERROR! name is gone (outside scope)
```

Variables die when their `{ }` block ends.
