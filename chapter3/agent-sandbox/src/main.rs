use adk_rust::prelude::*;
use adk_rust::Launcher;
use std::sync::Arc;
 
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = GeminiModel::new(&api_key, "gemini-2.0-flash-exp")?;
 
    let agent = LlmAgentBuilder::new("assistant")
        .instruction("You are a helpful assistant.")
        .model(Arc::new(model))
        .build()?;
 
    Launcher::new(Arc::new(agent)).run().await?;
    Ok(())
}
