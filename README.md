# ADK Rust Book - Learning Examples

**What:** A hands-on guide to learning Rust and building AI agents with adk-rust.

**Why:** Learn Rust fundamentals and agent development through practical, runnable examples.

## Project Structure

### Chapter 1: Rust Fundamentals
Core Rust concepts you need before building agents:

- [variables-example](chapter1/variables-example) - Variables, mutability, and shadowing
- [types-example](chapter1/types-example) - Rust's type system and inference
- [functions-example](chapter1/functions-example) - Defining and using functions
- [control-flow-example](chapter1/control-flow-example) - if/else, loops, and match
- [borrowing-example](chapter1/borrowing-example) - References and borrowing
- [structs-enums-example](chapter1/structs-enums-example) - Custom data types
- [result-example](chapter1/result-example) - Error handling with Result
- [prelude-example](chapter1/prelude-example) - What's available by default
- [async-example](chapter1/async-example) - Async/await for concurrent code
- [arc-example](chapter1/arc-example) - Shared ownership across threads
- [agent-trait-example](chapter1/agent-trait-example) - Traits and polymorphism

### Chapter 2: Agent Composition
Building modular agent systems:

- [agents-example](chapter2/agents-example) - Composing multiple agents with trait objects

### Chapter 3: First Agents
Your first working agents:

- [hello-rust](chapter3/hello-rust) - Hello World in Rust
- [my-first-agent](chapter3/my-first-agent) - Your first AI agent
- [agent-sandbox](chapter3/agent-sandbox) - Interactive agent playground

### Chapter 4: Advanced Agents
Agents with tools and capabilities:

- [my-minimal-agent](chapter4/my-minimal-agent) - Four agent examples (minimal, interactive, search, calculator)
- [structured_output](chapter4/structured_output) - Working with structured responses

### Chapter 5: Advanced Topics
Advanced agent patterns:

- [multilingual-agent](chapter5/multilingual-agent) - Multi-language support with session state
- [contact-extractor](chapter5/contact-extractor) - Extract structured contact info from text
- [text-summarizer](chapter5/text-summarizer) - Summarize text and save to session state
- [agent-setup-example](chapter5/agent-setup-example) - Configure multi-agent pipeline (setup only)
- [content-pipeline](chapter5/content-pipeline) - Multi-agent sequential pipeline (research → write → edit)
- [time-aware-agent](chapter5/time-aware-agent) - Dynamic instructions based on time and context

## Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs)
- **Google API Key**: Required for agent examples
  - Get one from [Google AI Studio](https://makersuite.google.com/app/apikey)
  - Set in `.env` file: `GOOGLE_API_KEY=your_api_key_here`

## Quick Start

1. Clone this repository
2. Set up your API key (see Prerequisites)
3. Navigate to any example directory
4. Run `cargo run`

## Learning Path

**New to Rust?** Start here:
1. Chapter 1 examples (in order)
2. Chapter 3: hello-rust
3. Chapter 3: my-first-agent

**Know Rust?** Jump to:
1. Chapter 3: my-first-agent
2. Chapter 2: agents-example
3. Chapter 4: my-minimal-agent

## Running Examples

Each example is a standalone Rust project:

```bash
# Navigate to any example
cd chapter1/variables-example

# Run it
cargo run

# Or run specific binary (for multi-binary projects)
cargo run --bin interactive
```

## Project Standards

All READMEs follow this structure:
- **What:** Brief description
- **Why:** Why this matters
- **What This Example Shows:** Key concepts
- **Run It:** How to run with expected output
- **Why This Matters:** Practical importance
- **Beginner's Explanation:** Simple analogies and breakdowns

## Contributing

This is a learning project. Feel free to:
- Add more examples
- Improve explanations
- Fix bugs
- Suggest better analogies

## Resources

- [Rust Book](https://doc.rust-lang.org/book/) - Official Rust documentation
- [adk-rust](https://github.com/anthropics/adk-rust) - Agent Development Kit
- [Gemini API](https://ai.google.dev/) - Google's AI model

## License

Educational use - check individual dependencies for their licenses.
