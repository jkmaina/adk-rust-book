use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use adk_tool::{Result as ToolResult, tool};
use schemars::JsonSchema;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

const APP_NAME: &str = "adk-rust-book";
const USER_ID_VALUE: &str = "book-user";
const SESSION_ID_VALUE: &str = "chapter5-multi-turn";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

#[derive(Deserialize, JsonSchema)]
struct ProductQuery {
    product: String,
}

#[tool]
async fn lookup_product(args: ProductQuery) -> ToolResult<serde_json::Value> {
    let info = match args.product.to_lowercase().as_str() {
        "macbook pro" | "macbook" => serde_json::json!({
            "product": "MacBook Pro 14\"",
            "price": 1999.00,
            "stock": 23,
            "specs": "M3 Pro, 18GB RAM, 512GB SSD",
            "category": "Laptops"
        }),
        "airpods" | "airpods pro" => serde_json::json!({
            "product": "AirPods Pro 2",
            "price": 249.00,
            "stock": 156,
            "specs": "Active Noise Cancellation, USB-C",
            "category": "Audio"
        }),
        "ipad" | "ipad air" => serde_json::json!({
            "product": "iPad Air M2",
            "price": 599.00,
            "stock": 0,
            "specs": "11\" Liquid Retina, M2 chip, 128GB",
            "category": "Tablets",
            "restock_date": "March 25, 2026"
        }),
        "keyboard" | "magic keyboard" => serde_json::json!({
            "product": "Magic Keyboard",
            "price": 299.00,
            "stock": 42,
            "specs": "Touch ID, Numeric Keypad, USB-C",
            "category": "Accessories"
        }),
        _ => serde_json::json!({
            "error": format!(
                "Product '{}' not found. Available: MacBook Pro, AirPods Pro, iPad Air, Magic Keyboard",
                args.product
            )
        }),
    };
    Ok(info)
}

#[derive(Deserialize, JsonSchema)]
struct CartItem {
    product: String,
    quantity: u64,
}

#[tool]
async fn add_to_cart(args: CartItem) -> ToolResult<serde_json::Value> {
    let unit_price = match args.product.to_lowercase().as_str() {
        "macbook pro" | "macbook" => 1999.00,
        "airpods" | "airpods pro" => 249.00,
        "ipad" | "ipad air" => 599.00,
        "keyboard" | "magic keyboard" => 299.00,
        _ => 0.00,
    };

    Ok(serde_json::json!({
        "product": args.product,
        "quantity": args.quantity,
        "unit_price": unit_price,
        "total": unit_price * args.quantity as f64,
        "cart_id": "CART-8821",
        "status": "added"
    }))
}

fn build_runner(
    agent: Arc<dyn Agent>,
    sessions: Arc<dyn SessionService>,
) -> anyhow::Result<Runner> {
    Ok(Runner::new(RunnerConfig {
        app_name: APP_NAME.into(),
        agent,
        session_service: sessions,
        artifact_service: None,
        memory_service: None,
        plugin_manager: None,
        run_config: None,
        compaction_config: None,
        context_cache_config: None,
        cache_capable: None,
        request_context: None,
        cancellation_token: None,
        intra_compaction_config: None,
        intra_compaction_summarizer: None,
    })?)
}

async fn create_session(
    sessions: &Arc<dyn SessionService>,
    session_id: &str,
) -> anyhow::Result<()> {
    sessions
        .create(CreateRequest {
            app_name: APP_NAME.into(),
            user_id: USER_ID_VALUE.into(),
            session_id: Some(session_id.into()),
            state: HashMap::new(),
        })
        .await?;
    Ok(())
}

async fn run_turn(
    runner: &Runner,
    user_id: &UserId,
    session_id: &SessionId,
    user_prompt: &str,
) -> anyhow::Result<()> {
    println!("User: {user_prompt}\n");
    let message = Content::new("user").with_text(user_prompt);
    let mut stream = runner
        .run(user_id.clone(), session_id.clone(), message)
        .await?;

    print!("Assistant: ");
    while let Some(event) = stream.next().await {
        let event = event?;
        if let Some(content) = &event.llm_response.content {
            for part in &content.parts {
                if let Some(text) = part.text() {
                    print!("{text}");
                }
            }
        }
    }
    println!("\n");
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, MODEL_NAME)?);
    let agent: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("shop_assistant")
            .instruction(
                "You are a helpful shopping assistant for an electronics store.\n\
                 - Use lookup_product to find product details, pricing, and stock levels.\n\
                 - Use add_to_cart to add items when the customer wants to buy.\n\
                 - Always check product availability before adding to cart.\n\
                 - If something is out of stock, mention the restock date if available.\n\
                 - Keep responses concise and helpful.",
            )
            .model(model)
            .tool(Arc::new(LookupProduct))
            .tool(Arc::new(AddToCart))
            .build()?,
    );

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;

    let runner = build_runner(agent, sessions)?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;

    run_turn(
        &runner,
        &user_id,
        &session_id,
        "I'm looking for a new laptop and some earbuds. What do you have?",
    )
    .await?;
    run_turn(
        &runner,
        &user_id,
        &session_id,
        "Nice! Add the laptop and 2 AirPods to my cart. Also, is the iPad available?",
    )
    .await?;
    run_turn(
        &runner,
        &user_id,
        &session_id,
        "What's my cart total so far?",
    )
    .await
}
