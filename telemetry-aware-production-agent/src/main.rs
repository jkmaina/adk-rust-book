use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use adk_telemetry::{
    LlmUsage, Span, add_context_attributes, agent_run_span, info, init_telemetry,
    llm_generate_span, record_llm_usage, tool_execute_span, warn,
};
use anyhow::bail;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::Instrument;

const APP_NAME: &str = "northstar-observability-agent";
const USER_ID_VALUE: &str = "sre-lead@northstar.example";
const SESSION_ID_VALUE: &str = "telemetry-aware-production-agent-demo";
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
    let mut state = HashMap::new();
    state.insert(
        "ops:service".to_string(),
        serde_json::Value::String("checkout-api".to_string()),
    );
    state.insert(
        "ops:style".to_string(),
        serde_json::Value::String("brief, operational, and measurable".to_string()),
    );
    state.insert(
        "ops:slo".to_string(),
        serde_json::Value::String("p99 latency under 250ms".to_string()),
    );

    sessions
        .create(CreateRequest {
            app_name: APP_NAME.into(),
            user_id: USER_ID_VALUE.into(),
            session_id: Some(session_id.into()),
            state,
        })
        .await?;
    Ok(())
}

async fn run_live_prompt(
    runner: &Runner,
    user_id: &UserId,
    session_id: &SessionId,
) -> anyhow::Result<String> {
    let prompt = "Checkout latency has climbed to 420ms and error rate is 0.8%. Give me a short operator summary with likely next actions.";
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
        bail!("telemetry-aware production agent returned no text");
    }

    Ok(response)
}

// This simulates the kind of structured ingress logging a production service
// would emit before the model is called.
async fn record_request_ingress() {
    let span = agent_run_span("observability_agent", "inv-ops-001");
    async {
        add_context_attributes(USER_ID_VALUE, SESSION_ID_VALUE);
        info!(
            service.name = "checkout-api",
            incident.severity = "high",
            "Received operator request"
        );
    }
    .instrument(span)
    .await;
}

// Tools and integrations should be observable separately so operators can tell
// whether time was spent in the model or in downstream dependencies.
async fn record_dependency_lookup() {
    let span = tool_execute_span("query_metrics_backend");
    async {
        info!(
            backend = "prometheus",
            metric = "http_request_duration_p99",
            "Fetched latency signal"
        );
        info!(
            backend = "prometheus",
            metric = "http_error_rate",
            "Fetched error-rate signal"
        );
    }
    .instrument(span)
    .await;
}

// Manual usage recording is still useful when you want predictable examples or
// when part of the workflow happens outside provider-managed accounting.
async fn record_synthetic_usage() {
    let span = llm_generate_span("manual-demo", MODEL_NAME, false);
    async {
        record_llm_usage(&LlmUsage {
            input_tokens: 320,
            output_tokens: 96,
            total_tokens: 416,
            cache_read_tokens: Some(48),
            thinking_tokens: Some(12),
            ..Default::default()
        });
        info!(usage.kind = "synthetic-demo", "Recorded usage sample");
    }
    .instrument(span)
    .await;
}

async fn record_decision_summary() {
    let span = tracing::info_span!(
        "operational_summary",
        recommendation.primary = tracing::field::Empty,
        recommendation.secondary = tracing::field::Empty
    );
    let _enter = span.enter();
    Span::current().record("recommendation.primary", "rollback canary");
    Span::current().record("recommendation.secondary", "notify incident channel");
    warn!(latency_ms = 420, error_rate = 0.8, "SLO breach requires attention");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    init_telemetry("telemetry-aware-production-agent")?;

    println!("=== Telemetry-Aware Production Agent ===\n");

    record_request_ingress().await;
    record_dependency_lookup().await;
    record_synthetic_usage().await;
    record_decision_summary().await;

    if !live_smoke_requested() {
        println!("Offline telemetry path completed.");
        println!("Set BOOK_RUN_LIVE_SMOKE=1 and GOOGLE_API_KEY for a live runner-backed prompt.");
        return Ok(());
    }

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, MODEL_NAME)?);

    let agent: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("observability_agent")
            .description("Operational assistant with explicit telemetry coverage")
            .instruction(
                "You are an SRE-facing assistant. Summarize production signals clearly, \
                 name the likely risk, and recommend concrete next steps.",
            )
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
