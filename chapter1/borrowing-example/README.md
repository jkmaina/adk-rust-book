# Borrowing Example

**What:** Learn how to share data temporarily without giving it away.

**Why:** Rust makes sure your data is safe. This example shows how to "lend" data to functions without losing it.

## What This Example Shows

- **Creating a String**: Making text data
- **References** (`&`): Lending data to a function instead of handing it over
- **Calling functions with references**: Passing data safely
- **Keeping your data**: The original string stays valid after the function uses it

## Run It

```bash
cargo run
```

Output:
```
The length of 'hello' is 5.
```

## Why This Matters

In many languages, when you pass data to a function, you might lose it. Rust's **borrowing** system lets you:
- **Lend** your data to a function (using `&`)
- **Keep ownership** after the function returns
- **Stay safe** because Rust prevents data accidents

## Beginner's Explanation

Think of borrowing like lending a book:
- **`let s1 = String::from("hello")`**: You own the book
- **`&s1`**: You lend the book to someone (but you still own it)
- **Function uses it**: Someone reads your book
- **After function**: Book comes back to you, you still own it

No copying required, and you don't lose anything!
