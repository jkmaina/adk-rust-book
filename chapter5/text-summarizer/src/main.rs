use adk_rust::prelude::*;
use std::sync::Arc;
use adk_rust::session::{CreateRequest, SessionService, InMemorySessionService};
use adk_rust::runner::{Runner, RunnerConfig};
use adk_rust::futures::StreamExt;
use std::collections::HashMap;
use std::io::{self, Write, BufRead};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let api_key = std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY must be set");
    let model = GeminiModel::new(&api_key, "gemini-2.0-flash-exp")?;

    let summarizer = LlmAgentBuilder::new("summarizer")
        .model(Arc::new(model))
        .instruction("Summarize the text concisely.")
        .output_key("summary")  // Saves response to state["summary"]
        .build()?;

    println!("Text Summarizer Agent: {}", summarizer.name());
    println!("This agent summarizes text and saves it to session state.\n");

   

    let app_name = "summarizer_app";
    let user_id = "user_123";
    let session_service = Arc::new(InMemorySessionService::new());

    // Create session with empty initial state
    let session = session_service.create(CreateRequest {
        app_name: app_name.to_string(),
        user_id: user_id.to_string(),
        session_id: None,
        state: HashMap::new(),
    }).await?;

    let session_id = session.id().to_string();

    let runner = Runner::new(RunnerConfig {
        app_name: app_name.to_string(),
        agent: Arc::new(summarizer),
        session_service,
        artifact_service: None,
        memory_service: None,
    })?;

    println!("🤖 Agent ready! Paste text to summarize (or 'exit' to quit).\n");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("You: ");
        stdout.flush()?;

        let mut input = String::new();
        let bytes_read = stdin.lock().read_line(&mut input)?;

        if bytes_read == 0 { break; }

        let input = input.trim();
        if input == "exit" || input == "quit" { break; }
        if input.is_empty() { continue; }

        let content = Content::new("user").with_text(input);
        let mut events = runner.run(user_id.to_string(), session_id.clone(), content).await?;

        print!("Summary: ");
        stdout.flush()?;

        while let Some(event) = events.next().await {
            match event {
                Ok(evt) => {
                    if let Some(content) = evt.llm_response.content {
                        for part in content.parts {
                            if let Some(text) = part.text() {
                                print!("{}", text);
                                stdout.flush()?;
                            }
                        }
                    }
                }
                Err(e) => eprintln!("\nError: {}", e),
            }
        }
        println!("\n");

        println!("\n(Summary saved to session state['summary'])\n");
    }

    Ok(())
}
