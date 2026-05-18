use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use adk_telemetry::{
    LlmUsage, add_context_attributes, agent_run_span, callback_span, debug, info, init_telemetry,
    llm_generate_span, record_llm_usage, tool_execute_span, warn,
};
use anyhow::bail;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::Instrument;

const APP_NAME: &str = "telemetry-demo-app";
const USER_ID_VALUE: &str = "book-user";
const SESSION_ID_VALUE: &str = "chapter15-telemetry-demo";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

fn live_smoke_requested() -> bool {
    matches!(std::env::var("BOOK_RUN_LIVE_SMOKE").as_deref(), Ok("1"))
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

async fn run_live_prompt(
    runner: &Runner,
    user_id: &UserId,
    session_id: &SessionId,
) -> anyhow::Result<String> {
    let prompt = "Reply with one short sentence confirming telemetry is active.";
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
        bail!("telemetry demo live prompt returned no text");
    }

    Ok(response)
}

async fn demo_context_span() {
    let span = agent_run_span("telemetry-agent", "inv-telemetry-001");
    async {
        add_context_attributes("user-42", "session-telemetry");
        info!("Agent execution started");
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        info!("Agent execution completed");
    }
    .instrument(span)
    .await;
}

async fn demo_tool_and_callback_spans() {
    let tool_span = tool_execute_span("weather_lookup");
    async {
        info!(
            tool.name = "weather_lookup",
            city = "Nairobi",
            "Tool executed"
        );
    }
    .instrument(tool_span)
    .await;

    let callback = callback_span("before_model");
    let _enter = callback.enter();
    info!("Callback executed before model invocation");
}

async fn demo_manual_usage_recording() {
    let span = llm_generate_span("manual-provider", "offline-demo-model", false);
    async {
        record_llm_usage(&LlmUsage {
            input_tokens: 150,
            output_tokens: 42,
            total_tokens: 192,
            cache_read_tokens: Some(10),
            thinking_tokens: Some(5),
            ..Default::default()
        });
        debug!("Recorded synthetic LLM usage for telemetry demonstration");
    }
    .instrument(span)
    .await;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    init_telemetry("chapter15-telemetry-demo")?;

    println!("Telemetry Demo");
    println!("==============\n");

    info!("Starting telemetry demo");
    warn!(mode = "offline-first", "Live model path is opt-in");

    demo_context_span().await;
    demo_tool_and_callback_spans().await;
    demo_manual_usage_recording().await;

    if !live_smoke_requested() {
        println!("Offline telemetry demo completed.");
        println!("Set BOOK_RUN_LIVE_SMOKE=1 and GOOGLE_API_KEY to run the live model path.");
        return Ok(());
    }

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, MODEL_NAME)?);
    let agent = Arc::new(
        LlmAgentBuilder::new("telemetry_agent")
            .instruction("You are a concise assistant used to demonstrate telemetry.")
            .model(model)
            .build()?,
    );

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;

    let runner = build_runner(agent, sessions)?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;
    let response = run_live_prompt(&runner, &user_id, &session_id).await?;
    println!("\nLive telemetry response:\n{response}");

    Ok(())
}
