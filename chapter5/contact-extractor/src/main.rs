use adk_rust::Launcher;
use adk_rust::prelude::*;
use serde_json::json;
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY must be set");
    let model = GeminiModel::new(&api_key, "gemini-2.0-flash-exp")?;

    let extractor = LlmAgentBuilder::new("contact_extractor")
        .description("Extracts contact information from text")
        .model(Arc::new(model))
        .instruction("Extract name, email, and phone from the provided text.")
        .output_schema(json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "email": { "type": "string", "format": "email" },
                "phone": { "type": "string" },
                "found": { "type": "boolean" }
            },
            "required": ["name", "found"]
        }))
        .build()?;

    println!("Contact Extractor Agent: {}", extractor.name());
    println!("This agent extracts structured contact information from text.");
    println!("Try: 'Contact John Doe at john@example.com or call 555-1234'\n");

    Launcher::new(Arc::new(extractor)).run().await?;

    Ok(())
}
