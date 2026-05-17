use adk_core::SingleAgentLoader;
use adk_rust::prelude::*;
use adk_server::{ServerConfig, create_app_with_a2a};
use adk_session::InMemorySessionService;
use std::sync::Arc;

const DEFAULT_BASE_URL: &str = "http://localhost:8090";

fn live_smoke_requested() -> bool {
    matches!(std::env::var("BOOK_RUN_LIVE_SMOKE").as_deref(), Ok("1"))
}

fn serve_requested() -> bool {
    matches!(std::env::var("BOOK_RUN_A2A_SERVER").as_deref(), Ok("1"))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let base_url = std::env::var("A2A_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());

    println!("=== A2A Server Packaging Example ===\n");
    println!("This crate prepares an A2A-capable server around an ADK agent.");

    if !live_smoke_requested() {
        println!("Offline compile validation is complete.");
        println!("Set BOOK_RUN_LIVE_SMOKE=1 and GOOGLE_API_KEY to construct the live server app.");
        println!("Set BOOK_RUN_A2A_SERVER=1 as well if you want to bind and serve on {base_url}.");
        return Ok(());
    }

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, "gemini-3.1-flash-lite-preview")?);

    let agent = LlmAgentBuilder::new("math_helper")
        .description("Helps with math questions")
        .instruction("You are a math assistant. Answer math questions concisely.")
        .model(model)
        .build()?;

    let config = ServerConfig::new(
        Arc::new(SingleAgentLoader::new(Arc::new(agent))),
        Arc::new(InMemorySessionService::new()),
    );

    let app = create_app_with_a2a(config, Some(base_url.as_str()));

    println!("Constructed A2A application successfully.");
    println!("  Agent card: {base_url}/.well-known/agent.json");
    println!("  A2A endpoint: {base_url}/a2a");

    if !serve_requested() {
        println!("Serve mode is disabled. Set BOOK_RUN_A2A_SERVER=1 to bind and serve.");
        drop(app);
        return Ok(());
    }

    let bind_target = base_url
        .strip_prefix("http://")
        .or_else(|| base_url.strip_prefix("https://"))
        .unwrap_or(base_url.as_str())
        .to_string();
    let listener = tokio::net::TcpListener::bind(&bind_target).await?;

    println!("Serving on {base_url}. Press Ctrl+C to stop.");
    axum::serve(listener, app).await?;
    Ok(())
}
