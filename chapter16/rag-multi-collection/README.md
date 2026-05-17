# Chapter 16 RAG Multi Collection

This crate adapts `../adk-playground/playground/backend/examples/rag_multi_collection.rs` into the book workspace.

## What It Demonstrates

- separate collections for separate business domains
- an offline-safe hash embedder for deterministic examples
- an optional live agent run over those collections

## ADK-Rust 0.8.2 Connection

In local `0.8.2`, RAG sits in the specialist surface, but the governance lesson here is broader than one crate: retrieval boundaries are part of security architecture too. This example keeps collection scoping explicit so the reader sees how knowledge access can be partitioned instead of dumped into one undifferentiated store.

## Run

```bash
cargo run -p chapter16-rag-multi-collection
```
