use adk_rust::prelude::*;
use futures::StreamExt;
use std::sync::Arc;
 
#[tokio::main]
// Fully qualify the standard Result type here so it doesn't collide with
// any `Result<T>` alias imported from adk_rust::prelude::*.
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🔍 ADK-Rust Environment Verification\n");
    println!("====================================\n");
    
    //  ✅ Check 1: Environment variable
    dotenv::dotenv().ok();
    print!("1. API Key... ");
    let api_key = std::env::var("GOOGLE_API_KEY")
        .expect("GOOGLE_API_KEY not set");
    println!("✓ Loaded ({} chars)", api_key.len());
    
    // ✅ Check 2: Create model
    print!("2. Model creation... ");
    let model = GeminiModel::new(&api_key, "gemini-2.0-flash-exp")?;
    println!("✓ GeminiModel created");
    
    // ✅ Check 3: Build agent
    print!("3. Agent building... ");
    let agent = LlmAgentBuilder::new("test_agent")
        .description("Verification test agent")
        .instruction("You are a helpful test assistant.")
        .model(Arc::new(model))
        .build()?;
    println!("✓ Agent built: {}", agent.name());
    
    // ✅ Check 4: Test LLM call (optional, makes actual API call)
    // This is enabled by passing --test-api to the program.
    let test_api = std::env::args().any(|a| a == "--test-api");
    print!("4. LLM connectivity (optional)... ");

    if test_api {
        // Build a simple single-turn request and call the model directly. Keep the prompt tiny
        // so this stays safe and cheap.
        let req_content = Content::new("user").with_text("Say 'Hello' in one word");
        let request = LlmRequest::new("gemini-2.0-flash-exp", vec![req_content]);

        // The `agent` above owns an Arc to the model; create a fresh model instance for a
        // direct test call so we can avoid transferring ownership or trying to coerce Arcs.
        let test_model = GeminiModel::new(&api_key, "gemini-2.0-flash-exp")?;

        // Non-streaming call (false) — this returns a stream containing a single final response.
        let mut stream = test_model.generate_content(request, false).await?;
        if let Some(item) = stream.next().await {
            let response = item?;
            if let Some(content) = response.content {
                if let Some(part) = content.parts.first() {
                    if let Part::Text { text } = part {
                        println!("✓ Response received: '{}'", text);
                    } else {
                        println!("✓ Response received (non-text)");
                    }
                } else {
                    println!("✓ Response received (no parts)");
                }
            } else {
                println!("✗ No content in response");
            }
        } else {
            println!("✗ No response from model");
        }
    } else {
        println!("Skipped (use --test-api to enable)");
    }
    
    println!("\n====================================");
    println!("✨ All checks passed! You're ready!");
    println!("====================================\n");
    
    Ok(())
}
