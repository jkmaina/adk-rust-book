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
const SESSION_ID_VALUE: &str = "chapter6-multi-tools";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

#[derive(Deserialize, JsonSchema)]
struct WeatherArgs {
    city: String,
}

#[tool]
async fn get_weather(args: WeatherArgs) -> ToolResult<serde_json::Value> {
    Ok(serde_json::json!({
        "city": args.city,
        "temperature": "22°C",
        "condition": "sunny"
    }))
}

#[derive(Deserialize, JsonSchema)]
struct CalcArgs {
    a: f64,
    b: f64,
    operation: String,
}

#[tool]
async fn calculate(args: CalcArgs) -> ToolResult<serde_json::Value> {
    let result = match args.operation.as_str() {
        "add" => args.a + args.b,
        "subtract" => args.a - args.b,
        "multiply" => args.a * args.b,
        "divide" if args.b != 0.0 => args.a / args.b,
        _ => 0.0,
    };
    Ok(serde_json::json!({ "result": result }))
}

#[derive(Deserialize, JsonSchema)]
struct ConvertArgs {
    value: f64,
    from: String,
    to: String,
}

#[tool]
async fn convert_units(args: ConvertArgs) -> ToolResult<serde_json::Value> {
    let result = match (args.from.as_str(), args.to.as_str()) {
        ("celsius", "fahrenheit") => args.value * 9.0 / 5.0 + 32.0,
        ("fahrenheit", "celsius") => (args.value - 32.0) * 5.0 / 9.0,
        ("km", "miles") => args.value * 0.621371,
        ("miles", "km") => args.value / 0.621371,
        ("kg", "lbs") => args.value * 2.20462,
        ("lbs", "kg") => args.value / 2.20462,
        _ => args.value,
    };

    Ok(serde_json::json!({
        "value": args.value,
        "from": args.from,
        "to": args.to,
        "result": result
    }))
}

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

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, MODEL_NAME)?);
    let agent: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("multi_tool_agent")
            .instruction(
                "You are a helpful assistant with multiple tools:\n\
                 - get_weather: weather lookups\n\
                 - calculate: arithmetic operations\n\
                 - convert_units: unit conversions (celsius/fahrenheit, km/miles, kg/lbs)\n\
                 Use the appropriate tool for each part of the user's request.",
            )
            .model(model)
            .tool(Arc::new(GetWeather))
            .tool(Arc::new(Calculate))
            .tool(Arc::new(ConvertUnits))
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
        "What's the weather in Tokyo? Convert 22°C to Fahrenheit. Also what's 15% of 250?",
    )
    .await
}
