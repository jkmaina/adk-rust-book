# Structs & Enums Example

**What:** Learn how to create your own custom data types.

**Why:** Sometimes you need to bundle different pieces of data together, or represent choices. This example shows how.

## What This Example Shows

- **Structs**: Bundle related information together (like a template for a user profile)
- **Methods**: Add functions to your custom types (like `User::new()` to create users)
- **Enums**: Represent a choice between multiple options (like different types of messages)
- **Pattern matching**: Handle each choice differently using `match`

## Run It

```bash
cargo run
```

## Why This Matters

**Structs** help you organize data:
- Instead of loose variables, bundle them together
- Example: `User { username: "alice", email: "alice@example.com" }`

**Enums** represent choices:
- A value can be one of several types
- Example: `Message::Quit` or `Message::Move { x: 10, y: 20 }`

## Beginner's Explanation

**Structs** are like a form:
```
User Form:
- username: [__________]
- email: [__________]
```

**Enums** are like multiple choice:
```
What do you want to do?
A) Quit
B) Move (x, y)
C) Write (text)
```

**Methods** let you do things with your types:
```
User::new("alice", "alice@example.com")  // Create a user easily
```