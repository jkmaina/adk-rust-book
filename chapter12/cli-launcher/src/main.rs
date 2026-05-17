use adk_rust::artifact::InMemoryArtifactService;
use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{Launcher, SessionId, StreamingMode, UserId};
use std::collections::HashMap;
use std::sync::Arc;

const APP_NAME: &str = "devops-assistant";
const USER_ID_VALUE: &str = "book-user";
const SESSION_ID_VALUE: &str = "chapter12-cli-launcher";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

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

    println!("=== CLI Launcher: Agent Deployment ===\n");

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, MODEL_NAME)?);
    let agent: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("deploy-agent")
            .instruction(
                "You are a DevOps assistant that helps with deployment questions. Be concise and practical.",
            )
            .model(model)
            .build()?,
    );

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    let artifacts = Arc::new(InMemoryArtifactService::new());

    let _launcher = Launcher::new(agent.clone())
        .app_name(APP_NAME)
        .with_session_service(sessions.clone())
        .with_artifact_service(artifacts)
        .with_streaming_mode(StreamingMode::SSE);

    println!("Launcher configured:");
    println!("  app_name:  {APP_NAME}");
    println!("  sessions:  InMemorySessionService");
    println!("  artifacts: InMemoryArtifactService");
    println!("  streaming: SSE\n");

    println!("Deployment modes:");
    println!("  cargo run -p chapter12-cli-launcher");
    println!("  cargo run -p chapter12-cli-launcher -- serve");
    println!("  cargo run -p chapter12-cli-launcher -- serve --port 8080\n");

    println!("Serve endpoints:");
    println!("  POST /run");
    println!("  POST /run/stream");
    println!("  GET  /health\n");

    create_session(&sessions, SESSION_ID_VALUE).await?;
    let runner = build_runner(agent, sessions)?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;

    let query = "What's the simplest way to containerize a Rust web service with Docker? Give me a minimal Dockerfile.";
    println!("User: {query}\n");
    print!("Agent: ");
    print_streamed_response(&runner, &user_id, &session_id, query).await
}
