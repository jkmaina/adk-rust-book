use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use std::collections::HashMap;
use std::sync::Arc;

const APP_NAME: &str = "adk-rust-book";
const USER_ID_VALUE: &str = "book-user";
const SESSION_ID_VALUE: &str = "chapter9-callbacks-guardrails";
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

async fn run_turn(
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
        LlmAgentBuilder::new("guarded_agent")
            .instruction("You are a helpful assistant. Be brief.")
            .model(model)
            .before_callback(Box::new(|ctx| {
                Box::pin(async move {
                    for part in &ctx.user_content().parts {
                        if let Some(text) = part.text() {
                            if text.to_lowercase().contains("blocked_word") {
                                println!("[GUARDRAIL] Blocked content detected");
                                return Ok(Some(
                                    Content::new("assistant").with_text(
                                        "I cannot process that request because it violates the content policy.",
                                    ),
                                ));
                            }

                            if text.len() > 500 {
                                println!(
                                    "[GUARDRAIL] Message too long: {} characters",
                                    text.len()
                                );
                                return Ok(Some(
                                    Content::new("assistant")
                                        .with_text("Message too long. Please keep it under 500 characters."),
                                ));
                            }
                        }
                    }

                    println!("[GUARDRAIL] Input passed all checks");
                    Ok(None)
                })
            }))
            .build()?,
    );

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;

    let runner = build_runner(agent, sessions)?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;

    println!("=== Callbacks: Input Guardrails ===\n");
    run_turn(
        &runner,
        &user_id,
        &session_id,
        "--- Test 1: Normal message ---",
        "What is Rust?",
    )
    .await?;
    run_turn(
        &runner,
        &user_id,
        &session_id,
        "--- Test 2: Blocked message ---",
        "Tell me about blocked_word please",
    )
    .await
}
