# Internal Knowledge Assistant

Beginner-friendly internal knowledge assistant that answers company questions over scoped PDF document collections instead of one unbounded knowledge source.

## What This Example Teaches

- Chapter 3 concepts: explicit model, session, runner, content, and streamed responses
- Chapter 5 concepts: session-backed personalization and multi-turn continuity
- Chapter 6 concepts: typed retrieval tools with narrow operational boundaries
- Chapter 7 concepts: coordinator plus specialist delegation through `AgentTool`
- Chapter 10 concepts: local RAG ingestion, chunking, embeddings, and retrieval
- Chapter 16 habits: scope control so engineering, HR, and ops knowledge stay separated

## Real-World Shift

This example now ingests real PDF fixtures from `assets/policies/` before building the assistant. That is closer to how internal knowledge systems work in practice: the knowledge base starts as documents, not inline strings inside the application.

The example still keeps the pipeline local and simple:

- PDFs are extracted at startup
- extracted text is ingested into scoped collections
- specialists retrieve only from their assigned collection

In production, teams usually extend this with OCR, document versioning, background ingestion jobs, metadata indexing, and access control per collection.

## What the Assistant Does

The example builds a small internal assistant with three PDF-backed knowledge domains:

- `engineering-docs` for deployment rules
- `hr-policies` for PTO policy answers
- `ops-playbooks` for incident response procedures

The coordinator does not search all collections directly. Instead, it delegates to a specialist for the relevant domain, and each specialist can only search its own collection.

## Why This Architecture Matters

This is a realistic step up from a single retrieval demo:

- the retrieval boundary is explicit
- prompts stay smaller and easier to reason about
- company policy domains do not silently bleed into each other
- a reader can see how RAG and agent delegation fit together in one application

## How to Read the Code

If you are studying the implementation, read `src/main.rs` in this order:

1. `HashEmbedder`, `PDF_SOURCES`, and `create_pipeline`
2. the typed retrieval tools
3. `create_session`
4. the three specialist agents
5. the coordinator agent
6. `build_runner` and `print_turn`

That progression follows the book: retrieval setup first, then tools, then agent composition, then runtime execution.

## Run It

```bash
cargo run -p internal-knowledge-assistant
```

You will need:

- `GOOGLE_API_KEY` in your environment or `.env`

The program runs three turns in the same session:

1. an engineering deployment question
2. an HR PTO question
3. an operations incident-response question

## What to Notice

- The collections are separate by design. Retrieval scope is part of the system design, not just a prompt suggestion.
- The specialists own domain retrieval. The coordinator owns conversation flow.
- Session state personalizes the final response style without rebuilding the agent each time.
- The example uses a local in-memory vector store and simple embedder so the reader can focus on ADK-Rust concepts first.
- The source documents are actual PDFs, which makes the retrieval flow more realistic even though the ingestion pipeline is still small.
