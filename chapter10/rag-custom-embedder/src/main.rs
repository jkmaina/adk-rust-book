use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::rag::{
    Document, EmbeddingProvider, FixedSizeChunker, InMemoryVectorStore, RagConfig, RagPipeline,
};
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use adk_tool::{Result as ToolResult, tool};
use schemars::JsonSchema;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

const APP_NAME: &str = "adk-rust-book";
const USER_ID_VALUE: &str = "book-user";
const SESSION_ID_VALUE: &str = "chapter10-rag-custom-embedder";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

static PIPELINE: OnceLock<Arc<RagPipeline>> = OnceLock::new();

struct TfIdfEmbedder {
    dims: usize,
}

impl TfIdfEmbedder {
    fn word_to_dim(&self, word: &str) -> usize {
        word.bytes().fold(0usize, |acc, byte| {
            acc.wrapping_mul(31).wrapping_add(byte as usize)
        }) % self.dims
    }
}

#[adk_rust::async_trait]
impl EmbeddingProvider for TfIdfEmbedder {
    async fn embed(&self, text: &str) -> adk_rust::rag::Result<Vec<f32>> {
        let mut vector = vec![0.0f32; self.dims];
        let words: Vec<&str> = text
            .split_whitespace()
            .map(|word| word.trim_matches(|ch: char| !ch.is_alphanumeric()))
            .filter(|word| word.len() > 2)
            .collect();

        let total = words.len() as f32;
        if total == 0.0 {
            return Ok(vector);
        }

        let mut freq: HashMap<usize, f32> = HashMap::new();
        for word in &words {
            *freq
                .entry(self.word_to_dim(&word.to_lowercase()))
                .or_default() += 1.0;
        }

        for (dim, count) in freq {
            vector[dim] = count / total;
        }

        let norm = vector.iter().map(|value| value * value).sum::<f32>().sqrt();
        if norm > 0.0 {
            vector.iter_mut().for_each(|value| *value /= norm);
        }

        Ok(vector)
    }

    fn dimensions(&self) -> usize {
        self.dims
    }
}

#[derive(Deserialize, JsonSchema)]
struct SearchArgs {
    query: String,
}

#[tool]
async fn search_docs(args: SearchArgs) -> ToolResult<serde_json::Value> {
    let Some(pipeline) = PIPELINE.get() else {
        return Ok(serde_json::json!({ "error": "RAG pipeline not initialized" }));
    };

    match pipeline.query("languages", &args.query).await {
        Ok(results) => {
            let hits: Vec<_> = results
                .iter()
                .take(3)
                .map(|result| {
                    serde_json::json!({
                        "document_id": result.chunk.document_id,
                        "text": result.chunk.text,
                        "score": format!("{:.3}", result.score),
                    })
                })
                .collect();

            Ok(serde_json::json!({ "results": hits }))
        }
        Err(error) => Ok(serde_json::json!({ "error": error.to_string() })),
    }
}

fn build_runner(
    agent: Arc<dyn Agent>,
    sessions: Arc<dyn SessionService>,
) -> anyhow::Result<Runner> {
    Ok(Runner::new(RunnerConfig {
        app_name: APP_NAME.into(),
        agent,
        session_service: sessions,
        artifact_service: None,
        memory_service: None,
        plugin_manager: None,
        run_config: None,
        compaction_config: None,
        context_cache_config: None,
        cache_capable: None,
        request_context: None,
        cancellation_token: None,
        intra_compaction_config: None,
        intra_compaction_summarizer: None,
    })?)
}

async fn create_session(
    sessions: &Arc<dyn SessionService>,
    session_id: &str,
) -> anyhow::Result<()> {
    sessions
        .create(CreateRequest {
            app_name: APP_NAME.into(),
            user_id: USER_ID_VALUE.into(),
            session_id: Some(session_id.into()),
            state: HashMap::new(),
        })
        .await?;
    Ok(())
}

async fn print_streamed_response(
    runner: &Runner,
    user_id: &UserId,
    session_id: &SessionId,
    prompt: &str,
) -> anyhow::Result<()> {
    let message = Content::new("user").with_text(prompt);
    let mut stream = runner
        .run(user_id.clone(), session_id.clone(), message)
        .await?;

    while let Some(event) = stream.next().await {
        let event = event?;
        if let Some(content) = &event.llm_response.content {
            for part in &content.parts {
                if let Some(text) = part.text() {
                    print!("{text}");
                }
            }
        }
    }

    println!();
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    println!("=== Custom Embedder: TF-IDF RAG Agent ===\n");

    let embedder = Arc::new(TfIdfEmbedder { dims: 128 });
    let v1 = embedder.embed("Rust programming language safety").await?;
    let v2 = embedder
        .embed("Rust programming language performance")
        .await?;
    let v3 = embedder.embed("cooking recipes for Italian pasta").await?;
    let sim_12: f32 = v1.iter().zip(&v2).map(|(a, b)| a * b).sum();
    let sim_13: f32 = v1.iter().zip(&v3).map(|(a, b)| a * b).sum();

    println!("Cosine similarity demo:");
    println!("  rust+safety vs rust+perf = {:.3}", sim_12);
    println!("  rust+safety vs cooking   = {:.3}\n", sim_13);

    let pipeline = Arc::new(
        RagPipeline::builder()
            .config(
                RagConfig::builder()
                    .chunk_size(256)
                    .chunk_overlap(50)
                    .top_k(3)
                    .build()?,
            )
            .embedding_provider(embedder)
            .vector_store(Arc::new(InMemoryVectorStore::new()))
            .chunker(Arc::new(FixedSizeChunker::new(256, 50)))
            .build()?,
    );

    pipeline.create_collection("languages").await?;
    pipeline
        .ingest_batch(
            "languages",
            &[
                Document {
                    id: "rust".into(),
                    text: "Rust is a systems programming language focused on safety, speed, and concurrency. It achieves memory safety without garbage collection through ownership and borrowing. Rust's borrow checker prevents data races at compile time.".into(),
                    metadata: HashMap::from([("paradigm".into(), "systems".into())]),
                    source_uri: None,
                },
                Document {
                    id: "python".into(),
                    text: "Python is a high-level interpreted language popular for data science, web development, and scripting. It emphasizes readability and has a rich ecosystem including NumPy, Pandas, and TensorFlow.".into(),
                    metadata: HashMap::from([("paradigm".into(), "scripting".into())]),
                    source_uri: None,
                },
                Document {
                    id: "go".into(),
                    text: "Go is a statically typed language designed at Google for simplicity and efficiency. It features goroutines for lightweight concurrency, a built-in garbage collector, and fast compilation. Go is strong for network services and CLIs.".into(),
                    metadata: HashMap::from([("paradigm".into(), "systems".into())]),
                    source_uri: None,
                },
            ],
        )
        .await?;
    println!("Loaded 3 language documents into the RAG pipeline.\n");

    let _ = PIPELINE.set(pipeline.clone());

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, MODEL_NAME)?);
    let agent: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("embedder_agent")
            .instruction(
                "You are a programming language expert with a retrieval tool. \
                 Use the search_docs tool before answering. Compare languages objectively and cite retrieved facts.",
            )
            .model(model)
            .tool(Arc::new(SearchDocs))
            .build()?,
    );

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;

    let runner = build_runner(agent, sessions)?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;

    let query = "Which language is better for building safe concurrent systems, Rust or Go? Search the docs and compare.";
    println!("User: {query}\n");
    print!("Agent: ");
    print_streamed_response(&runner, &user_id, &session_id, query).await?;

    pipeline.delete_collection("languages").await?;
    println!(
        "\nProduction note: swap the custom embedder for GeminiEmbeddingProvider or OpenAIEmbeddingProvider in a real deployment."
    );
    Ok(())
}
