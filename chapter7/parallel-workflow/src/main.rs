use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use std::collections::HashMap;
use std::sync::Arc;

const APP_NAME: &str = "adk-rust-book";
const USER_ID_VALUE: &str = "book-user";
const SESSION_ID_VALUE: &str = "chapter7-parallel";
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

    let technical = Arc::new(
        LlmAgentBuilder::new("technical_analyst")
            .instruction("Analyze from a technical perspective. Be specific about implementation.")
            .model(model.clone())
            .build()?,
    ) as Arc<dyn Agent>;

    let business = Arc::new(
        LlmAgentBuilder::new("business_analyst")
            .instruction(
                "Analyze from a business and market perspective. Focus on ROI and strategy.",
            )
            .model(model.clone())
            .build()?,
    ) as Arc<dyn Agent>;

    let user_exp = Arc::new(
        LlmAgentBuilder::new("ux_analyst")
            .instruction("Analyze from a user experience perspective. Focus on usability.")
            .model(model)
            .build()?,
    ) as Arc<dyn Agent>;

    let parallel: Arc<dyn Agent> = Arc::new(ParallelAgent::new(
        "multi_perspective_analysis",
        vec![technical, business, user_exp],
    ));

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;
    let runner = build_runner(parallel, sessions)?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;

    println!("Running 3 analysts in parallel...\n");
    print_streamed_response(
        &runner,
        &user_id,
        &session_id,
        "Should a startup adopt WebAssembly for their web app?",
    )
    .await
}
