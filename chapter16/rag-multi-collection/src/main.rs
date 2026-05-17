use adk_rag::{
    Document, EmbeddingProvider, FixedSizeChunker, InMemoryVectorStore, RagConfig, RagPipeline,
};
use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use adk_tool::tool;
use schemars::JsonSchema;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

static PIPELINE: OnceLock<Arc<RagPipeline>> = OnceLock::new();

fn live_smoke_requested() -> bool {
    matches!(std::env::var("BOOK_RUN_LIVE_SMOKE").as_deref(), Ok("1"))
}

struct HashEmbedder;

#[async_trait::async_trait]
impl EmbeddingProvider for HashEmbedder {
    async fn embed(&self, text: &str) -> adk_rag::Result<Vec<f32>> {
        let hash = text.bytes().fold(0u64, |acc, byte| {
            acc.wrapping_mul(31).wrapping_add(byte as u64)
        });
        let mut vector = vec![0.0f32; 64];
        for (index, value) in vector.iter_mut().enumerate() {
            *value = ((hash.wrapping_add(index as u64)) as f32).sin();
        }
        let norm: f32 = vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            vector.iter_mut().for_each(|x| *x /= norm);
        }
        Ok(vector)
    }

    fn dimensions(&self) -> usize {
        64
    }
}

#[derive(Deserialize, JsonSchema)]
struct SearchArgs {
    collection: String,
    query: String,
}

#[tool]
async fn search_knowledge_base(args: SearchArgs) -> adk_tool::Result<serde_json::Value> {
    let pipeline = PIPELINE.get().expect("rag pipeline");
    let results = match pipeline.query(&args.collection, &args.query).await {
        Ok(results) => results,
        Err(error) => return Ok(serde_json::json!({ "error": error.to_string() })),
    };

    let hits: Vec<_> = results
        .iter()
        .take(3)
        .map(|result| {
            serde_json::json!({
                "document_id": result.chunk.document_id,
                "text": result.chunk.text,
                "score": result.score,
            })
        })
        .collect();

    Ok(serde_json::json!({
        "collection": args.collection,
        "query": args.query,
        "results": hits,
    }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    println!("=== Multi-Collection RAG Agent ===\n");

    let pipeline = Arc::new(
        RagPipeline::builder()
            .config(
                RagConfig::builder()
                    .chunk_size(512)
                    .chunk_overlap(100)
                    .top_k(3)
                    .build()?,
            )
            .embedding_provider(Arc::new(HashEmbedder))
            .vector_store(Arc::new(InMemoryVectorStore::new()))
            .chunker(Arc::new(FixedSizeChunker::new(512, 100)))
            .build()?,
    );

    pipeline.create_collection("engineering").await?;
    pipeline.create_collection("hr-policies").await?;

    pipeline
        .ingest_batch(
            "engineering",
            &[
                Document {
                    id: "arch".into(),
                    text: "Our microservices use gRPC for inter-service communication. Each service owns its database and exposes a protobuf API.".into(),
                    metadata: HashMap::from([("team".into(), "platform".into())]),
                    source_uri: None,
                },
                Document {
                    id: "deploy".into(),
                    text: "Deployments use blue-green strategy with automatic rollback if error rate exceeds 1 percent. Canary releases start at 5 percent traffic.".into(),
                    metadata: HashMap::from([("team".into(), "devops".into())]),
                    source_uri: None,
                },
            ],
        )
        .await?;
    pipeline
        .ingest_batch(
            "hr-policies",
            &[
                Document {
                    id: "pto".into(),
                    text: "Employees receive 20 days PTO per year. Unused PTO carries over up to 5 days.".into(),
                    metadata: HashMap::from([("category".into(), "benefits".into())]),
                    source_uri: None,
                },
                Document {
                    id: "remote".into(),
                    text: "Remote work policy allows up to 3 days per week from home. Full remote requires VP approval.".into(),
                    metadata: HashMap::from([("category".into(), "workplace".into())]),
                    source_uri: None,
                },
            ],
        )
        .await?;

    let _ = PIPELINE.set(pipeline.clone());
    println!("Collections ingested successfully.\n");

    let engineering_results = pipeline.query("engineering", "deployment strategy").await?;
    let hr_results = pipeline.query("hr-policies", "PTO days").await?;
    println!(
        "Engineering top hit: {}",
        engineering_results
            .first()
            .map(|result| result.chunk.document_id.as_str())
            .unwrap_or("none")
    );
    println!(
        "HR top hit: {}",
        hr_results
            .first()
            .map(|result| result.chunk.document_id.as_str())
            .unwrap_or("none")
    );

    if !live_smoke_requested() {
        println!(
            "\nSkipping live RAG agent run. Set BOOK_RUN_LIVE_SMOKE=1 and GOOGLE_API_KEY to continue."
        );
        pipeline.delete_collection("engineering").await?;
        pipeline.delete_collection("hr-policies").await?;
        return Ok(());
    }

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, "gemini-3.1-flash-lite-preview")?);
    let agent = Arc::new(
        LlmAgentBuilder::new("rag_agent")
            .instruction(
                "You are a company knowledge assistant. Use the search_knowledge_base tool before answering and cite the collection you used.",
            )
            .model(model)
            .tool(Arc::new(SearchKnowledgeBase))
            .build()?,
    );

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    sessions
        .create(CreateRequest {
            app_name: "playground".into(),
            user_id: "user".into(),
            session_id: Some("s1".into()),
            state: HashMap::new(),
        })
        .await?;

    let runner = Runner::new(RunnerConfig {
        app_name: "playground".into(),
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
    })?;

    let query = "What's our deployment strategy and how many PTO days do employees get?";
    println!("\nUser: {query}\n");
    print!("Agent: ");
    let message = Content::new("user").with_text(query);
    let mut stream = runner
        .run(UserId::new("user")?, SessionId::new("s1")?, message)
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

    pipeline.delete_collection("engineering").await?;
    pipeline.delete_collection("hr-policies").await?;
    Ok(())
}
