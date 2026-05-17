# Chapter 10 RAG Custom Embedder

This crate is the book adaptation of `../adk-playground/playground/backend/examples/rag_custom_embedder.rs`.

## What It Demonstrates

- implementing a custom `EmbeddingProvider`
- building a local `RagPipeline` with in-memory storage
- exposing retrieval through a typed tool that the agent can call during reasoning

## ADK-Rust 0.8.0 Connection

The local `0.8.0` docs now present `RagTool` as the standard high-level wrapper for agentic retrieval, but this crate intentionally teaches one layer lower. It makes the embedder, vector store, pipeline, and retrieval contract visible so the reader understands what the higher-level abstraction is actually doing.

## Run

```bash
export GOOGLE_API_KEY=your-api-key
cargo run -p chapter10-rag-custom-embedder
```

## Expected Behavior

The program builds a tiny language-doc knowledge base, demonstrates cosine similarity for related texts, and then has the agent compare Rust and Go using the retrieval tool.
