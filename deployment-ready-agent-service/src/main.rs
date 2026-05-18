use adk_rust::artifact::InMemoryArtifactService;
use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{Launcher, SessionId, StreamingMode, UserId};
use anyhow::bail;
use axum::{Json, Router, routing::get};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

const APP_NAME: &str = "northstar-release-assistant";
const USER_ID_VALUE: &str = "release-manager@northstar.example";
const SESSION_ID_VALUE: &str = "deployment-ready-agent-service-demo";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";
const DEFAULT_BASE_URL: &str = "http://localhost:8094";

fn live_smoke_requested() -> bool {
    matches!(std::env::var("BOOK_RUN_LIVE_SMOKE").as_deref(), Ok("1"))
}

fn serve_requested() -> bool {
    matches!(std::env::var("BOOK_RUN_AGENT_SERVICE").as_deref(), Ok("1"))
}

fn a2a_requested() -> bool {
    matches!(std::env::args().nth(1).as_deref(), Some("a2a"))
}

fn launcher_mode_requested() -> bool {
    matches!(std::env::args().nth(1).as_deref(), Some("chat" | "serve"))
}

fn build_agent(api_key: &str) -> anyhow::Result<Arc<dyn Agent>> {
    let model = Arc::new(GeminiModel::new(api_key, MODEL_NAME)?);

    let agent = LlmAgentBuilder::new("release_assistant")
        .description("Helps release managers prepare safe production rollouts")
        .instruction(
            "You are a release engineering assistant. Answer in a concise, operational style. \
             When asked for rollout guidance, include readiness checks, rollback advice, and \
             communication expectations.",
        )
        .model(model)
        .build()?;

    Ok(Arc::new(agent))
}

fn build_runner(
    agent: Arc<dyn Agent>,
    sessions: Arc<dyn SessionService>,
    artifacts: Arc<InMemoryArtifactService>,
) -> anyhow::Result<Runner> {
    Ok(Runner::builder()
        .app_name(APP_NAME)
        .agent(agent)
        .session_service(sessions)
        .artifact_service(artifacts)
        .build()?)
}

fn build_launcher(
    agent: Arc<dyn Agent>,
    sessions: Arc<dyn SessionService>,
    artifacts: Arc<InMemoryArtifactService>,
    base_url: &str,
) -> Launcher {
    Launcher::new(agent)
        .app_name(APP_NAME)
        .with_session_service(sessions)
        .with_artifact_service(artifacts)
        .with_streaming_mode(StreamingMode::SSE)
        .with_a2a_base_url(base_url)
}

async fn create_session(
    sessions: &Arc<dyn SessionService>,
    session_id: &str,
) -> anyhow::Result<()> {
    let mut state = HashMap::new();
    state.insert(
        "release:team".to_string(),
        serde_json::Value::String("platform engineering".to_string()),
    );
    state.insert(
        "release:style".to_string(),
        serde_json::Value::String("brief, risk-aware, and operational".to_string()),
    );
    state.insert(
        "release:environment".to_string(),
        serde_json::Value::String("production".to_string()),
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

async fn run_smoke_prompt(
    runner: &Runner,
    user_id: &UserId,
    session_id: &SessionId,
) -> anyhow::Result<String> {
    let prompt = "We are deploying a billing API patch in 30 minutes. Give me a short go-live checklist with rollback and communication steps.";
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
        bail!("deployment smoke run returned no text");
    }

    Ok(response)
}

fn with_ops_routes(app: Router) -> Router {
    app.route("/ops/ready", get(ops_ready))
}

async fn ops_ready() -> Json<serde_json::Value> {
    Json(json!({
        "service": APP_NAME,
        "status": "ready",
        "deployment_mode": "composed-axum-router"
    }))
}

async fn maybe_serve(app: Router, base_url: &str) -> anyhow::Result<()> {
    if !serve_requested() {
        println!("Serve mode is disabled. Set BOOK_RUN_AGENT_SERVICE=1 to bind the router.");
        return Ok(());
    }

    let bind_target = base_url
        .strip_prefix("http://")
        .or_else(|| base_url.strip_prefix("https://"))
        .unwrap_or(base_url)
        .to_string();
    let listener = tokio::net::TcpListener::bind(&bind_target).await?;
    println!("Serving on {base_url}. Press Ctrl+C to stop.");
    axum::serve(listener, app).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    println!("=== Deployment-Ready Agent Service ===\n");

    if !launcher_mode_requested() && !live_smoke_requested() {
        println!("This example shows one ADK agent exposed through three delivery surfaces:");
        println!("  1. explicit Runner smoke checks");
        println!("  2. Launcher-based CLI or HTTP serving");
        println!("  3. composed Axum app construction with optional A2A routes");
        println!();
        println!("Offline validation:");
        println!("  cargo check -p deployment-ready-agent-service");
        println!();
        println!("Live smoke:");
        println!("  export GOOGLE_API_KEY=...");
        println!("  BOOK_RUN_LIVE_SMOKE=1 cargo run -p deployment-ready-agent-service");
        println!();
        println!("Launcher surfaces:");
        println!("  cargo run -p deployment-ready-agent-service -- chat");
        println!("  cargo run -p deployment-ready-agent-service -- serve --port 8094");
        println!();
        println!("Composed router surfaces:");
        println!("  BOOK_RUN_LIVE_SMOKE=1 cargo run -p deployment-ready-agent-service -- app");
        println!("  BOOK_RUN_LIVE_SMOKE=1 cargo run -p deployment-ready-agent-service -- a2a");
        println!(
            "  BOOK_RUN_LIVE_SMOKE=1 BOOK_RUN_AGENT_SERVICE=1 cargo run -p deployment-ready-agent-service -- a2a"
        );
        return Ok(());
    }

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let base_url = std::env::var("A2A_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());

    let agent = build_agent(&api_key)?;
    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    let artifacts = Arc::new(InMemoryArtifactService::new());

    if launcher_mode_requested() {
        let launcher = build_launcher(agent, sessions, artifacts, &base_url);
        return Ok(launcher.run().await?);
    }

    create_session(&sessions, SESSION_ID_VALUE).await?;
    let runner = build_runner(agent.clone(), sessions.clone(), artifacts.clone())?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;
    let response = run_smoke_prompt(&runner, &user_id, &session_id).await?;

    println!("Runner smoke response:\n{response}\n");

    let launcher = build_launcher(agent, sessions, artifacts, &base_url);
    let app = if a2a_requested() {
        println!("Constructing composed router with A2A routes enabled.");
        println!("Expected agent card: {base_url}/.well-known/agent.json");
        println!("Expected A2A endpoint: {base_url}/a2a");
        with_ops_routes(launcher.build_app_with_a2a(base_url.as_str())?)
    } else {
        println!("Constructing composed router without A2A routes.");
        with_ops_routes(launcher.build_app()?)
    };

    println!("Custom route available: {base_url}/ops/ready");
    maybe_serve(app, &base_url).await
}
