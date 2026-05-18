use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use adk_tool::{Result as ToolResult, tool};
use schemars::JsonSchema;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

const APP_NAME: &str = "adk-rust-book";
const USER_ID_VALUE: &str = "book-user";
const SESSION_ID_VALUE: &str = "chapter6-function-tool";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

#[derive(Deserialize, JsonSchema)]
struct WeatherArgs {
    city: String,
}

#[tool]
async fn get_weather(args: WeatherArgs) -> ToolResult<serde_json::Value> {
    Ok(serde_json::json!({
        "city": args.city,
        "temp_c": 22,
        "condition": "Sunny",
        "humidity": "45%"
    }))
}

#[derive(Deserialize, JsonSchema)]
struct TimeArgs {
    timezone: String,
}

#[tool]
async fn get_time(args: TimeArgs) -> ToolResult<serde_json::Value> {
    Ok(serde_json::json!({
        "timezone": args.timezone,
        "time": "14:30",
        "date": "2026-03-17"
    }))
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
    let agent: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("weather_time_agent")
            .instruction(
                "You help users check weather and time. Use get_weather for weather \
                 and get_time for time queries. Be concise.",
            )
            .model(model)
            .tool(Arc::new(GetWeather))
            .tool(Arc::new(GetTime))
            .build()?,
    );

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;

    let runner = build_runner(agent, sessions)?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;

    print_streamed_response(
        &runner,
        &user_id,
        &session_id,
        "What's the weather in Tokyo and what time is it there?",
    )
    .await
}
