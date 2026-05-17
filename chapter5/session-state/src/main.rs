use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, GetRequest, SessionService};
use adk_rust::{SessionId, UserId};
use std::collections::HashMap;
use std::sync::Arc;

const APP_NAME: &str = "adk-rust-book";
const USER_ID_VALUE: &str = "alice";
const SESSION_ID_VALUE: &str = "chapter5-session-state";
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
    let mut initial_state = HashMap::new();
    initial_state.insert("user_preference".to_string(), "concise answers".into());

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

async fn print_turn(
    runner: &Runner,
    user_id: &UserId,
    session_id: &SessionId,
    label: &str,
    prompt: &str,
) -> anyhow::Result<()> {
    println!("{label}");
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

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, MODEL_NAME)?);
    let agent: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("stateful_assistant")
            .instruction(
                "You are a helpful assistant. Remember context from previous messages. \
                 Be concise.",
            )
            .model(model)
            .build()?,
    );

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;

    let runner = build_runner(agent, sessions.clone())?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;

    print_turn(
        &runner,
        &user_id,
        &session_id,
        "--- Turn 1 ---",
        "My name is Alice and I love Rust.",
    )
    .await?;

    print_turn(
        &runner,
        &user_id,
        &session_id,
        "--- Turn 2 ---",
        "What's my name and what language do I like?",
    )
    .await?;

    let session = sessions
        .get(GetRequest {
            app_name: APP_NAME.into(),
            user_id: USER_ID_VALUE.into(),
            session_id: SESSION_ID_VALUE.into(),
            num_recent_events: None,
            after: None,
        })
        .await?;

    println!("Session ID: {}", session.id());
    println!("Events count: {}", session.events().len());
    Ok(())
}
