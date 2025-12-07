use adk_rust::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY must be set");
    let model = Arc::new(GeminiModel::new(&api_key, "gemini-2.0-flash-exp")?);
    
    // Step 1: Research agent - receives raw user input directly
    // NO template variables in the first agent
    let researcher = LlmAgentBuilder::new("researcher")
        .description("Gathers information on the topic the user asks about")
        .instruction(
            "The user will provide a topic. Research that topic thoroughly and provide key facts, \
             recent developments, and important context. Be comprehensive but concise."
        )
        .model(model.clone())
        .output_key("research_findings")
        .build()?;
    
    // Step 2: Writer agent - uses research findings via template
    let writer = LlmAgentBuilder::new("writer")
        .description("Writes articles based on research")
        .instruction(
            "Write a well-structured blog post based on the following research findings:\n\n\
             {research_findings}\n\n\
             Create an engaging article with an introduction, main body sections, and conclusion. \
             Use clear headings and maintain a professional yet accessible tone."
        )
        .model(model.clone())
        .output_key("draft_article")
        .build()?;
    
    // Step 3: Editor agent - polishes the draft
    let editor = LlmAgentBuilder::new("editor")
        .description("Polishes articles for clarity and engagement")
        .instruction(
            "Edit and polish the following article for clarity, engagement, and professionalism:\n\n\
             {draft_article}\n\n\
             Fix any grammar issues, improve flow, enhance readability, and ensure the article \
             is compelling and well-structured. Return only the final polished version."
        )
        .model(model.clone())
        .output_key("final_article")
        .build()?;
    
    // Create sequential pipeline
    let pipeline = SequentialAgent::new(
        "content_pipeline",
        vec![
            Arc::new(researcher),
            Arc::new(writer),
            Arc::new(editor),
        ],
    );
    
    println!("=== Content Pipeline Agent ===\n");
    println!("This pipeline has 3 stages:");
    println!("1. 🔍 Researcher → Gathers information");
    println!("2. ✍️  Writer → Creates draft article");  
    println!("3. ✨ Editor → Polishes final version\n");
    println!("Just type your topic (e.g., 'AI Agents' or 'Rust programming')\n");
    
    use adk_rust::Launcher;
    Launcher::new(Arc::new(pipeline)).run().await?;
    
    Ok(())
}