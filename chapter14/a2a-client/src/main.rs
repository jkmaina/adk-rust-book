use adk_server::a2a::{A2aClient, Message, Part, Role};

const DEFAULT_BASE_URL: &str = "http://localhost:8090";

fn live_smoke_requested() -> bool {
    matches!(std::env::var("BOOK_RUN_LIVE_SMOKE").as_deref(), Ok("1"))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let base_url = std::env::var("A2A_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());

    println!("=== A2A Client Packaging Example ===\n");
    println!("Target base URL: {base_url}");

    if !live_smoke_requested() {
        println!("Offline compile validation is complete.");
        println!("Set BOOK_RUN_LIVE_SMOKE=1 to connect to a running A2A server.");
        return Ok(());
    }

    let client = A2aClient::from_url(&base_url).await?;
    let card = client.agent_card();
    println!("Fetched agent card:");
    println!("  name: {}", card.name);
    println!("  description: {}", card.description);
    println!("  streaming: {}", card.capabilities.streaming);

    let message = Message {
        role: Role::User,
        parts: vec![Part::text("What is 7 * 8?".to_string())],
        message_id: uuid::Uuid::new_v4().to_string(),
        context_id: None,
        task_id: None,
        metadata: None,
    };

    let response = client.send_message(message).await?;
    println!("\nResponse:");
    if let Some(result) = response.result {
        println!("{}", serde_json::to_string_pretty(&result)?);
    }
    if let Some(error) = response.error {
        println!("Error: {} (code: {})", error.message, error.code);
    }

    Ok(())
}
