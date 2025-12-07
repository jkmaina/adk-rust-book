use adk_rust::prelude::*;
use adk_rust::Launcher;
use my_minimal_agent::tools::GoogleSearchTool;
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    
    let api_key = std::env::var("GOOGLE_API_KEY")
        .expect("GOOGLE_API_KEY environment variable not set");
    
    let model = GeminiModel::new(&api_key, "gemini-2.0-flash-exp")?;
    
    // Build agent with search capability
    let agent = LlmAgentBuilder::new("search_assistant")
        .description("An AI assistant that can search for current information")
        .instruction("You are a helpful assistant with access to web search. \
                      When asked about current events, recent information, or anything \
                      you're uncertain about, use the search tool. \
                      Synthesize search results into clear, helpful answers.")
        .model(Arc::new(model))
        .tool(Arc::new(GoogleSearchTool::new()))  // Add search!
        .build()?;
    
    println!("🤖 Starting search-enabled assistant...\n");
    Launcher::new(Arc::new(agent)).run().await?;
    
    Ok(())
}
