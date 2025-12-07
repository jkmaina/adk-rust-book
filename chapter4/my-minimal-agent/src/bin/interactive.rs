use adk_rust::prelude::*;
use adk_rust::Launcher;
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();
    
    // Get API key
    let api_key = std::env::var("GOOGLE_API_KEY")
        .expect("GOOGLE_API_KEY environment variable not set");
    
    // Create the Gemini model
    let model = GeminiModel::new(&api_key, "gemini-2.0-flash-exp")?;
    
    // Build the agent with personality
    let agent = LlmAgentBuilder::new("assistant")
        .description("A helpful AI assistant for general queries")
        .instruction("You are a friendly and knowledgeable assistant. \
                      Answer questions clearly and concisely. \
                      Be conversational but informative. \
                      If you don't know something, admit it honestly.")
        .model(Arc::new(model))
        .build()?;
    
    // Launch interactive CLI
    println!("🤖 Starting your AI assistant...\n");
    Launcher::new(Arc::new(agent)).run().await?;
    
    Ok(())
}
