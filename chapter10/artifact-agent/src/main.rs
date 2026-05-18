use adk_rust::artifact::{
    ArtifactService, InMemoryArtifactService, ListRequest, LoadRequest, SaveRequest,
};
use adk_rust::futures::StreamExt;
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
const SESSION_ID_VALUE: &str = "chapter10-artifact-agent";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

static ARTIFACT_SVC: OnceLock<Arc<InMemoryArtifactService>> = OnceLock::new();

#[derive(Deserialize, JsonSchema)]
struct SaveArgs {
    file_name: String,
    content: String,
}

#[tool]
async fn save_artifact(args: SaveArgs) -> ToolResult<serde_json::Value> {
    let Some(service) = ARTIFACT_SVC.get() else {
        return Ok(serde_json::json!({ "error": "artifact service not initialized" }));
    };

    match service
        .save(SaveRequest {
            app_name: APP_NAME.into(),
            user_id: USER_ID_VALUE.into(),
            session_id: SESSION_ID_VALUE.into(),
            file_name: args.file_name.clone(),
            part: Part::Text { text: args.content },
            version: None,
        })
        .await
    {
        Ok(response) => Ok(serde_json::json!({
            "saved": args.file_name,
            "version": response.version,
        })),
        Err(error) => Ok(serde_json::json!({ "error": error.to_string() })),
    }
}

#[derive(Deserialize, JsonSchema)]
struct LoadArgs {
    file_name: String,
    version: Option<i64>,
}

#[tool]
async fn load_artifact(args: LoadArgs) -> ToolResult<serde_json::Value> {
    let Some(service) = ARTIFACT_SVC.get() else {
        return Ok(serde_json::json!({ "error": "artifact service not initialized" }));
    };

    match service
        .load(LoadRequest {
            app_name: APP_NAME.into(),
            user_id: USER_ID_VALUE.into(),
            session_id: SESSION_ID_VALUE.into(),
            file_name: args.file_name.clone(),
            version: args.version,
        })
        .await
    {
        Ok(response) => {
            let text = match &response.part {
                Part::Text { text } => text.clone(),
                _ => "(binary data)".into(),
            };

            Ok(serde_json::json!({
                "file": args.file_name,
                "content": text,
                "version": args.version,
            }))
        }
        Err(error) => Ok(serde_json::json!({ "error": error.to_string() })),
    }
}

#[tool]
async fn list_artifacts() -> ToolResult<serde_json::Value> {
    let Some(service) = ARTIFACT_SVC.get() else {
        return Ok(serde_json::json!({ "error": "artifact service not initialized" }));
    };

    match service
        .list(ListRequest {
            app_name: APP_NAME.into(),
            user_id: USER_ID_VALUE.into(),
            session_id: SESSION_ID_VALUE.into(),
        })
        .await
    {
        Ok(response) => Ok(serde_json::json!({
            "files": response.file_names,
        })),
        Err(error) => Ok(serde_json::json!({ "error": error.to_string() })),
    }
}

fn build_runner(
    agent: Arc<dyn Agent>,
    sessions: Arc<dyn SessionService>,
    artifact_service: Arc<InMemoryArtifactService>,
) -> anyhow::Result<Runner> {
    Ok(Runner::builder()
        .app_name(APP_NAME)
        .agent(agent)
        .session_service(sessions)
        .artifact_service(artifact_service)
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

    println!("=== Artifact-Powered Agent ===\n");

    let artifact_service = Arc::new(InMemoryArtifactService::new());
    let _ = ARTIFACT_SVC.set(artifact_service.clone());

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, MODEL_NAME)?);
    let agent: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("artifact_agent")
            .instruction(
                "You are a writing assistant with artifact storage. \
                 When asked to write something, save it as an artifact. \
                 When asked to revise, load the prior version, improve it, and save a new one. \
                 Always confirm what you saved and the version number.",
            )
            .model(model)
            .tool(Arc::new(SaveArtifact))
            .tool(Arc::new(LoadArtifact))
            .tool(Arc::new(ListArtifacts))
            .build()?,
    );

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;

    let runner = build_runner(agent, sessions, artifact_service)?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;

    let query = "Write a short haiku about Rust programming and save it as poem.txt";
    println!("User: {query}\n");
    print!("Agent: ");
    print_streamed_response(&runner, &user_id, &session_id, query).await
}
