use adk_rust::futures::StreamExt;
use adk_rust::memory::{InMemoryMemoryService, MemoryEntry, MemoryService, SearchRequest};
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use adk_tool::{Result as ToolResult, tool};
use schemars::JsonSchema;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

const APP_NAME: &str = "adk-rust-book";
const USER_ID_VALUE: &str = "book-user";
const SESSION_ID_VALUE: &str = "chapter10-memory-agent";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

static MEMORY_SVC: OnceLock<Arc<InMemoryMemoryService>> = OnceLock::new();

#[derive(Deserialize, JsonSchema)]
struct RecallArgs {
    query: String,
}

#[tool]
async fn recall_memory(args: RecallArgs) -> ToolResult<serde_json::Value> {
    let Some(service) = MEMORY_SVC.get() else {
        return Ok(serde_json::json!({ "error": "memory service not initialized" }));
    };

    match service
        .search(SearchRequest {
            query: args.query.clone(),
            user_id: USER_ID_VALUE.into(),
            app_name: APP_NAME.into(),
            project_id: None,
            limit: Some(5),
            min_score: None,
        })
        .await
    {
        Ok(response) => {
            let memories: Vec<_> = response
                .memories
                .iter()
                .map(|memory| {
                    let text = memory
                        .content
                        .parts
                        .iter()
                        .filter_map(|part| part.text())
                        .collect::<Vec<_>>()
                        .join(" ");
                    serde_json::json!({
                        "author": memory.author,
                        "text": text,
                    })
                })
                .collect();

            Ok(serde_json::json!({
                "query": args.query,
                "found": memories.len(),
                "memories": memories,
            }))
        }
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

async fn preload_memory(service: &InMemoryMemoryService) -> anyhow::Result<()> {
    service
        .add_session(
            APP_NAME,
            USER_ID_VALUE,
            "old-session-1",
            vec![
                MemoryEntry {
                    content: Content::new("user")
                        .with_text("I'm building a web scraper in Rust using reqwest and tokio."),
                    author: "user".into(),
                    timestamp: chrono::Utc::now(),
                },
                MemoryEntry {
                    content: Content::new("assistant").with_text(
                        "Great choice. Reqwest with tokio gives you async HTTP, and select! helps with concurrency.",
                    ),
                    author: "assistant".into(),
                    timestamp: chrono::Utc::now(),
                },
            ],
        )
        .await?;

    service
        .add_session(
            APP_NAME,
            USER_ID_VALUE,
            "old-session-2",
            vec![
                MemoryEntry {
                    content: Content::new("user").with_text(
                        "My favorite language is Rust and I prefer async/await over threads.",
                    ),
                    author: "user".into(),
                    timestamp: chrono::Utc::now(),
                },
                MemoryEntry {
                    content: Content::new("assistant").with_text(
                        "Noted. Async/await with tokio is a strong fit for your I/O-heavy work.",
                    ),
                    author: "assistant".into(),
                    timestamp: chrono::Utc::now(),
                },
            ],
        )
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

    println!("=== Memory-Enhanced Agent ===\n");

    let memory_service = Arc::new(InMemoryMemoryService::new());
    preload_memory(memory_service.as_ref()).await?;
    println!("Loaded 2 prior sessions into memory.\n");
    let _ = MEMORY_SVC.set(memory_service);

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, MODEL_NAME)?);
    let agent: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("memory_agent")
            .instruction(
                "You are a helpful coding assistant with long-term memory. \
                 Use the recall_memory tool before answering so you can remember the user's \
                 preferences, projects, and past discussions. If memory is relevant, mention it naturally.",
            )
            .model(model)
            .tool(Arc::new(RecallMemory))
            .build()?,
    );

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;

    let runner = build_runner(agent, sessions)?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;

    let query = "I need help with error handling in my project. What approach would suit me best?";
    println!("User: {query}\n");
    print!("Agent: ");
    print_streamed_response(&runner, &user_id, &session_id, query).await
}
