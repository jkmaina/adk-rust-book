use adk_rust::prelude::*;
use serde_json::json;
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let api_key = std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY must be set");
    let model = GeminiModel::new(&api_key, "gemini-2.0-flash-exp")?;

    let extractor = LlmAgentBuilder::new("contact_extractor")
        .description("Extracts contact information from text")
        .instruction("Extract name, email, and phone from the provided text.")
        .model(Arc::new(model))
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
    println!("Extracts structured contact information from text.\n");

    // Example inputs
    let examples = vec![
        "Contact John Doe at john@example.com or call 555-1234",
        "Reach out to Alice Smith via alice.smith@company.com",
        "Bob Johnson, phone: (555) 987-6543",
        "No contact information here",
    ];

    for (i, input) in examples.iter().enumerate() {
        println!("Example {}:", i + 1);
        println!("Input: \"{}\"", input);
        
        let content = Content::new("user").with_text(input);
        let response = extractor.run(content).await?;
        
        if let Some(text) = response.content.and_then(|c| c.text()) {
            println!("Output: {}\n", text);
        }
    }

    Ok(())
}
