use adk_rag::{
    Document, EmbeddingProvider, FixedSizeChunker, InMemoryVectorStore, RagConfig, RagPipeline,
};
use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::tool::AgentTool;
use adk_rust::{SessionId, UserId};
use adk_tool::{Result as ToolResult, tool};
use schemars::JsonSchema;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

const APP_NAME: &str = "adk-rust-book-companion";
const USER_ID_VALUE: &str = "maya-owens";
const SESSION_ID_VALUE: &str = "internal-knowledge-assistant-demo";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

// This example keeps the full architecture in one file so the learning path is
// easy to follow:
// local documents -> RAG pipeline -> typed tools -> specialist agents ->
// coordinator -> runner -> streamed answers.

static PIPELINE: OnceLock<Arc<RagPipeline>> = OnceLock::new();

struct KnowledgePdfSource {
    id: &'static str,
    collection: &'static str,
    file_name: &'static str,
}

const PDF_SOURCES: &[KnowledgePdfSource] = &[
    KnowledgePdfSource {
        id: "deployments",
        collection: "engineering-docs",
        file_name: "engineering-deployments.pdf",
    },
    KnowledgePdfSource {
        id: "pto-policy",
        collection: "hr-policies",
        file_name: "hr-pto-policy.pdf",
    },
    KnowledgePdfSource {
        id: "sev1-playbook",
        collection: "ops-playbooks",
        file_name: "ops-sev1-playbook.pdf",
    },
];

struct HashEmbedder;

#[adk_rust::async_trait]
impl EmbeddingProvider for HashEmbedder {
    async fn embed(&self, text: &str) -> adk_rag::Result<Vec<f32>> {
        let mut vector = vec![0.0f32; 96];
        for word in text
            .split_whitespace()
            .map(|word| word.trim_matches(|ch: char| !ch.is_alphanumeric()).to_lowercase())
            .filter(|word| word.len() > 2)
        {
            let dim = word.bytes().fold(0usize, |acc, byte| {
                acc.wrapping_mul(31).wrapping_add(byte as usize)
            }) % vector.len();
            vector[dim] += 1.0;
        }

        let norm = vector.iter().map(|value| value * value).sum::<f32>().sqrt();
        if norm > 0.0 {
            vector.iter_mut().for_each(|value| *value /= norm);
        }

        Ok(vector)
    }

    fn dimensions(&self) -> usize {
        96
    }
}

#[derive(Deserialize, JsonSchema)]
struct KnowledgeQueryArgs {
    query: String,
}

fn pipeline() -> Option<&'static Arc<RagPipeline>> {
    PIPELINE.get()
}

fn search_results_to_json(results: Vec<adk_rag::SearchResult>, collection: &str) -> serde_json::Value {
    let hits: Vec<_> = results
        .iter()
        .take(3)
        .map(|result| {
            serde_json::json!({
                "collection": collection,
                "document_id": result.chunk.document_id,
                "text": result.chunk.text,
                "score": format!("{:.3}", result.score),
            })
        })
        .collect();

    serde_json::json!({ "collection": collection, "results": hits })
}

fn policy_assets_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("policies")
}

fn extract_pdf_text(path: &Path) -> anyhow::Result<String> {
    // `pdf_extract` gives us a practical local fixture path for this example.
    // In production, teams often replace this with a richer ingestion service
    // that handles OCR, metadata extraction, and document lifecycle events.
    let raw_text = pdf_extract::extract_text(path)?;
    let normalized = raw_text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    Ok(normalized)
}

#[tool]
async fn search_engineering_docs(args: KnowledgeQueryArgs) -> ToolResult<serde_json::Value> {
    // Each tool is intentionally scoped to one collection. This keeps the
    // retrieval boundary obvious and prevents a single free-form tool from
    // quietly searching everything.
    let Some(pipeline) = pipeline() else {
        return Ok(serde_json::json!({ "error": "RAG pipeline not initialized" }));
    };

    match pipeline.query("engineering-docs", &args.query).await {
        Ok(results) => Ok(search_results_to_json(results, "engineering-docs")),
        Err(error) => Ok(serde_json::json!({ "error": error.to_string() })),
    }
}

#[tool]
async fn search_hr_policies(args: KnowledgeQueryArgs) -> ToolResult<serde_json::Value> {
    let Some(pipeline) = pipeline() else {
        return Ok(serde_json::json!({ "error": "RAG pipeline not initialized" }));
    };

    match pipeline.query("hr-policies", &args.query).await {
        Ok(results) => Ok(search_results_to_json(results, "hr-policies")),
        Err(error) => Ok(serde_json::json!({ "error": error.to_string() })),
    }
}

#[tool]
async fn search_ops_playbooks(args: KnowledgeQueryArgs) -> ToolResult<serde_json::Value> {
    let Some(pipeline) = pipeline() else {
        return Ok(serde_json::json!({ "error": "RAG pipeline not initialized" }));
    };

    match pipeline.query("ops-playbooks", &args.query).await {
        Ok(results) => Ok(search_results_to_json(results, "ops-playbooks")),
        Err(error) => Ok(serde_json::json!({ "error": error.to_string() })),
    }
}

fn build_runner(
    agent: Arc<dyn Agent>,
    sessions: Arc<dyn SessionService>,
) -> anyhow::Result<Runner> {
    Ok(Runner::builder()
        .app_name(APP_NAME)
        .agent(agent)
        .session_service(sessions)
        .build()?)
}

async fn create_session(
    sessions: &Arc<dyn SessionService>,
    session_id: &str,
) -> anyhow::Result<()> {
    let mut initial_state = HashMap::new();
    initial_state.insert("employee:name".to_string(), "Maya Owens".into());
    initial_state.insert("employee:team".to_string(), "platform engineering".into());
    initial_state.insert("employee:style".to_string(), "concise and practical".into());

    sessions
        .create(CreateRequest {
            app_name: APP_NAME.into(),
            user_id: USER_ID_VALUE.into(),
            session_id: Some(session_id.into()),
            state: initial_state,
        })
        .await?;

    Ok(())
}

async fn create_pipeline() -> anyhow::Result<Arc<RagPipeline>> {
    let pipeline = Arc::new(
        RagPipeline::builder()
            .config(
                RagConfig::builder()
                    .chunk_size(320)
                    .chunk_overlap(64)
                    .top_k(3)
                    .build()?,
            )
            .embedding_provider(Arc::new(HashEmbedder))
            .vector_store(Arc::new(InMemoryVectorStore::new()))
            .chunker(Arc::new(FixedSizeChunker::new(320, 64)))
            .build()?,
    );

    pipeline.create_collection("engineering-docs").await?;
    pipeline.create_collection("hr-policies").await?;
    pipeline.create_collection("ops-playbooks").await?;

    let mut grouped: HashMap<&'static str, Vec<Document>> = HashMap::new();
    for source in PDF_SOURCES {
        let path = policy_assets_dir().join(source.file_name);
        let text = extract_pdf_text(&path)?;
        grouped.entry(source.collection).or_default().push(Document {
            id: source.id.into(),
            text,
            metadata: HashMap::from([
                ("format".into(), "pdf".into()),
                ("owner".into(), source.collection.into()),
            ]),
            source_uri: Some(path.to_string_lossy().into_owned()),
        });
    }

    // The RAG pipeline still uses small local fixtures, but now the source of
    // truth is an actual document file instead of an inline Rust string.
    for (collection, documents) in grouped {
        pipeline.ingest_batch(collection, &documents).await?;
    }

    Ok(pipeline)
}

async fn print_turn(
    runner: &Runner,
    user_id: &UserId,
    session_id: &SessionId,
    label: &str,
    prompt: &str,
) -> anyhow::Result<()> {
    println!("{label}");
    println!("Employee: {prompt}\n");
    print!("Assistant: ");

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

    println!("\n");
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let pipeline = create_pipeline().await?;
    let _ = PIPELINE.set(pipeline.clone());

    println!("Internal Knowledge Assistant Demo\n");
    println!("Loaded PDF collections: engineering-docs, hr-policies, ops-playbooks\n");

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, MODEL_NAME)?);

    let engineering_specialist = LlmAgentBuilder::new("engineering_specialist")
        .description("Answers deployment, architecture, API ownership, and engineering documentation questions.")
        .instruction(
            "You answer questions using engineering-docs only. Always call \
             search_engineering_docs before responding. Summarize the policy \
             clearly and mention when the answer came from engineering-docs. \
             This includes deployment rollout, rollback, architecture review, \
             service ownership, and engineering operating rules.",
        )
        .model(model.clone())
        .tool(Arc::new(SearchEngineeringDocs))
        .build()?;

    let hr_specialist = LlmAgentBuilder::new("hr_specialist")
        .description("Answers HR, PTO, workplace, and employee policy questions.")
        .instruction(
            "You answer questions using hr-policies only. Always call \
             search_hr_policies before responding. Keep the answer factual and \
             mention when the answer came from hr-policies. This includes PTO, \
             remote work, benefits, and workplace policy questions.",
        )
        .model(model.clone())
        .tool(Arc::new(SearchHrPolicies))
        .build()?;

    let ops_specialist = LlmAgentBuilder::new("ops_specialist")
        .description("Answers incident response, on-call, outage, and operations playbook questions.")
        .instruction(
            "You answer questions using ops-playbooks only. Always call \
             search_ops_playbooks before responding. Explain the operational \
             sequence and mention when the answer came from ops-playbooks. \
             This includes Sev-1 response, bridges, paging, outages, \
             communications, and status updates.",
        )
        .model(model.clone())
        .tool(Arc::new(SearchOpsPlaybooks))
        .build()?;

    // Each specialist becomes a tool for the coordinator. This mirrors a real
    // internal assistant more closely than exposing every retrieval tool to one
    // giant prompt.
    let engineering_tool =
        AgentTool::new(Arc::new(engineering_specialist)).timeout(Duration::from_secs(20));
    let hr_tool = AgentTool::new(Arc::new(hr_specialist)).timeout(Duration::from_secs(20));
    let ops_tool = AgentTool::new(Arc::new(ops_specialist)).timeout(Duration::from_secs(20));

    let coordinator: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("knowledge_coordinator")
            .instruction(
                "You are the internal knowledge assistant for {employee:name} on \
                 {employee:team}. The user prefers {employee:style} responses.\n\
                 Delegate deployment, rollback, architecture, API ownership, and \
                 engineering documentation questions to engineering_specialist.\n\
                 Delegate PTO, leave, benefits, remote work, and workplace policy \
                 questions to hr_specialist.\n\
                 Delegate incident response, on-call, bridge, outage, paging, and \
                 status update questions to ops_specialist.\n\
                 Return one clear final answer. Do not invent policies that were \
                 not found in the retrieved material.",
            )
            .model(model)
            .tool(Arc::new(engineering_tool))
            .tool(Arc::new(hr_tool))
            .tool(Arc::new(ops_tool))
            .max_iterations(8)
            .build()?,
    );

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;

    let runner = build_runner(coordinator, sessions)?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;

    print_turn(
        &runner,
        &user_id,
        &session_id,
        "--- Turn 1: Engineering Docs ---",
        "What is our rollback rule if a deployment starts increasing the error rate?",
    )
    .await?;

    print_turn(
        &runner,
        &user_id,
        &session_id,
        "--- Turn 2: HR Policy ---",
        "How many PTO days can carry into next year?",
    )
    .await?;

    print_turn(
        &runner,
        &user_id,
        &session_id,
        "--- Turn 3: Ops Playbook ---",
        "For a Sev-1 incident, who must join first and how often are updates posted?",
    )
    .await?;

    pipeline.delete_collection("engineering-docs").await?;
    pipeline.delete_collection("hr-policies").await?;
    pipeline.delete_collection("ops-playbooks").await?;

    Ok(())
}
