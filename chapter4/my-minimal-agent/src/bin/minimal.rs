use adk_rust::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();
    
    // Get AP key
    let api_key = std::env::var("GOOGLE_API_KEY")
        .expect("GOOGLE_API_KEY not found in environment");
    
    // Create the model
    let model = GeminiModel::new(&api_key, "gemini-2.0-flash-exp")?;
    
    // Build the agent
    let agent = LlmAgentBuilder::new("assistant")
        .model(Arc::new(model))
        .build()?;
    
    println!("Agent '{}' created successfully!", agent.name());
    
    Ok(())
}
