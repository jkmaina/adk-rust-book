use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{Launcher, SessionId, UserId};
use anyhow::bail;
use std::collections::HashMap;
use std::sync::Arc;

const APP_NAME: &str = "deployment-launcher-app";
const USER_ID_VALUE: &str = "book-user";
const SESSION_ID_VALUE: &str = "chapter14-deployment-launcher";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

fn live_smoke_requested() -> bool {
    matches!(std::env::var("BOOK_RUN_LIVE_SMOKE").as_deref(), Ok("1"))
}

fn explicit_launcher_mode() -> bool {
    std::env::args().nth(1).is_some()
}

fn build_agent(api_key: &str) -> anyhow::Result<Arc<dyn Agent>> {
    let model = Arc::new(GeminiModel::new(api_key, MODEL_NAME)?);
    let agent = LlmAgentBuilder::new("deployment_launcher")
        .description("Launcher-friendly assistant for deployment examples")
        .instruction("You are a helpful assistant. Reply in one concise sentence.")
        .model(model)
        .build()?;

    Ok(Arc::new(agent))
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

async fn run_smoke_prompt(
    runner: &Runner,
    user_id: &UserId,
    session_id: &SessionId,
) -> anyhow::Result<String> {
    let prompt = "Reply with one short sentence confirming the packaged launcher path is ready.";
    let message = Content::new("user").with_text(prompt);
    let mut stream = runner
        .run(user_id.clone(), session_id.clone(), message)
        .await?;
    let mut response = String::new();

    while let Some(event) = stream.next().await {
        let event = event?;
        if let Some(content) = &event.llm_response.content {
            for part in &content.parts {
                if let Some(text) = part.text() {
                    response.push_str(text);
                }
            }
        }
    }

    if response.trim().is_empty() {
        bail!("deployment launcher smoke returned no text");
    }

    Ok(response)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    println!("=== Deployment Launcher ===\n");
    println!("This crate adapts a validated launcher example into a package-friendly workflow.");

    if !explicit_launcher_mode() && !live_smoke_requested() {
        println!("Offline compile validation is handled by `cargo check --workspace`.");
        println!("Set BOOK_RUN_LIVE_SMOKE=1 and GOOGLE_API_KEY for a one-prompt smoke run.");
        println!("Or run with explicit launcher args such as `chat` or `serve --port 8080`.");
        return Ok(());
    }

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let agent = build_agent(&api_key)?;
    let launcher = Launcher::new(agent.clone()).app_name(APP_NAME);

    if explicit_launcher_mode() {
        return Ok(launcher.run().await?);
    }

    println!("Validated launcher construction for both chat and serve entrypoints.");

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;

    let runner = build_runner(agent, sessions)?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;
    let response = run_smoke_prompt(&runner, &user_id, &session_id).await?;

    println!("\nLive smoke response:\n{response}");
    Ok(())
}
