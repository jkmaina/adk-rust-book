use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use std::collections::HashMap;
use std::sync::Arc;

const APP_NAME: &str = "adk-rust-book";
const USER_ID_VALUE: &str = "book-user";
const SESSION_ID_VALUE: &str = "chapter11-conditional-router";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

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

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, MODEL_NAME)?);

    let tech_agent: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("tech_expert")
            .instruction(
                "You are a senior software engineer. Answer with code examples, technical depth, and best practices.",
            )
            .model(model.clone())
            .build()?,
    );

    let general_agent: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("general_helper")
            .instruction(
                "You are a friendly general assistant. Explain things simply and avoid jargon where possible.",
            )
            .model(model.clone())
            .build()?,
    );

    let creative_agent: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("creative_writer")
            .instruction(
                "You are a creative writer. Be imaginative, expressive, and engaging. Use vivid language.",
            )
            .model(model.clone())
            .build()?,
    );

    let router: Arc<dyn Agent> = Arc::new(
        LlmConditionalAgent::builder("smart_router", model)
            .instruction(
                "Classify the user's question as exactly one of: technical, general, or creative. \
                 Respond with only the category name.",
            )
            .route("technical", tech_agent)
            .route("general", general_agent.clone())
            .route("creative", creative_agent)
            .default_route(general_agent)
            .build()?,
    );

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;

    let runner = build_runner(router, sessions)?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;

    println!("=== LLM Conditional Router ===");
    println!("Routes: technical | general | creative\n");
    print_streamed_response(
        &runner,
        &user_id,
        &session_id,
        "Write me a short poem about a Rust programmer who finally defeated the borrow checker.",
    )
    .await
}
