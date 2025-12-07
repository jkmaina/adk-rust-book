use adk_rust::prelude::*;
use chrono::Timelike;
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY must be set");
    let model = GeminiModel::new(&api_key, "gemini-2.0-flash-exp")?;

    // Get current time-based greeting
    let hour = chrono::Local::now().hour();
    let greeting = match hour {
        5..=11 => "Good morning",
        12..=17 => "Good afternoon",
        18..=21 => "Good evening",
        _ => "Hello",
    };

    let time_aware = LlmAgentBuilder::new("time_assistant")
        .description("Assistant aware of time and context")
        .model(Arc::new(model))
        .instruction(&format!(
            "{}! You are a helpful assistant. \
             Respond in a tone appropriate for this time of day. \
             Be friendly and helpful.",
            greeting
        ))
        .build()?;

    println!("Time-Aware Assistant: {}", time_aware.name());
    println!("Current greeting: {}", greeting);
    println!("This agent adapts its greeting based on the current time.\n");

    use adk_rust::Launcher;
    Launcher::new(Arc::new(time_aware)).run().await?;

    Ok(())
}
