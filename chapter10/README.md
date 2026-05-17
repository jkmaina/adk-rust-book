# Chapter 10 Examples

These crates are the active Chapter 10 ADK-Rust examples for memory, artifacts, and RAG.

They are deliberately chosen to keep three different persistence problems separate. In local `0.8.2`, ADK-Rust offers richer context accessors and higher-level retrieval helpers than the early drafts of the book assumed, but these examples keep memory recall, artifact storage, and document retrieval visible so readers learn the boundaries before they learn every convenience layer.

## Included Crates

- `chapter10-memory-agent`
- `chapter10-artifact-agent`
- `chapter10-rag-custom-embedder`

## Running from the Repo Root

```bash
cargo run -p chapter10-memory-agent
cargo run -p chapter10-artifact-agent
cargo run -p chapter10-rag-custom-embedder
```

All three crates compile offline, but live execution requires `GOOGLE_API_KEY`.

## How This Chapter Fits ADK-Rust 0.8.2

- `chapter10-memory-agent` teaches cross-session recall as an explicit retrieval decision, not vague prompt magic.
- `chapter10-artifact-agent` teaches that durable generated output belongs in the artifact subsystem, not hidden in chat history.
- `chapter10-rag-custom-embedder` teaches the pipeline underneath agentic retrieval before readers move on to higher-level wrappers like `RagTool`.

One practical feature-tier note matters here: memory and artifact workflows fit comfortably inside the standard `0.8.2` stack, while RAG belongs to the wider specialist feature surface.
