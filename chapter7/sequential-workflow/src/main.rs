use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use std::collections::HashMap;
use std::sync::Arc;

const APP_NAME: &str = "adk-rust-book";
const USER_ID_VALUE: &str = "book-user";
const SESSION_ID_VALUE: &str = "chapter7-sequential";
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

    let researcher = Arc::new(
        LlmAgentBuilder::new("researcher")
            .instruction("Research the given topic. Identify 3 key points with evidence.")
            .model(model.clone())
            .build()?,
    ) as Arc<dyn Agent>;

    let writer = Arc::new(
        LlmAgentBuilder::new("writer")
            .instruction("Take the research and write a polished 2-paragraph summary.")
            .model(model.clone())
            .build()?,
    ) as Arc<dyn Agent>;

    let editor = Arc::new(
        LlmAgentBuilder::new("editor")
            .instruction("Edit for clarity and conciseness. Fix any issues. Output final version.")
            .model(model)
            .build()?,
    ) as Arc<dyn Agent>;

    let pipeline: Arc<dyn Agent> = Arc::new(SequentialAgent::new(
        "research_pipeline",
        vec![researcher, writer, editor],
    ));

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;
    let runner = build_runner(pipeline, sessions)?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;

    println!("Running 3-stage pipeline: researcher -> writer -> editor\n");
    print_streamed_response(
        &runner,
        &user_id,
        &session_id,
        "The impact of Rust on systems programming",
    )
    .await
}
