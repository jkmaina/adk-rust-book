use adk_rust::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY must be set");
    let model = Arc::new(GeminiModel::new(&api_key, "gemini-2.0-flash-exp")?);
    
    // Step 1: Research agent
    let researcher = LlmAgentBuilder::new("researcher")
        .description("Gathers information on a topic")
        .instruction("Research {topic} and provide key facts.")
        .model(model.clone())
        .output_key("research_findings")  // Save findings
        .build()?;
    
    // Step 2: Writer agent (uses research via template)
    let writer = LlmAgentBuilder::new("writer")
        .description("Writes articles based on research")
        .instruction(
            "Write a blog post about {topic}. \
             Use these research findings: {research_findings}"
        )
        .model(model.clone())
        .output_key("draft_article")  // Save draft
        .build()?;
    
    // Step 3: Editor agent (uses draft via template)
    let editor = LlmAgentBuilder::new("editor")
        .description("Polishes articles")
        .instruction("Edit this article for clarity: {draft_article}")
        .model(model.clone())
        .output_key("final_article")  // Save final version
        .build()?;
    
    // In real usage, you'd run these sequentially via SequentialAgent
    // or manually orchestrate them
    
    println!("Content pipeline agents created!");
    println!("1. {} saves to 'research_findings'", researcher.name());
    println!("2. {} uses findings, saves to 'draft_article'", writer.name());
    println!("3. {} uses draft, saves to 'final_article'", editor.name());
    
    Ok(())
}
