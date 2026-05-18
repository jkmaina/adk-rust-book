use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{
    CreateRequest, DeleteRequest, GetRequest, ListRequest, PostgresSessionService, SessionService,
};
use adk_rust::{SessionId, UserId};
use std::collections::HashMap;
use std::sync::Arc;

const APP_NAME: &str = "pg-demo";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("POSTGRES_URL")
        .unwrap_or_else(|_| "postgres://adk:adk_playground@localhost:5433/adk_sessions".into());

    println!("# PostgreSQL Session Backend\n");
    println!("Connecting to `{db_url}`...\n");

    let service = PostgresSessionService::new(&db_url).await?;
    service.migrate().await?;
    println!("Connected and migrated.\n");

    let mut state1 = HashMap::new();
    state1.insert("app:version".to_string(), serde_json::json!("2.1.0"));
    state1.insert("user:plan".to_string(), serde_json::json!("enterprise"));
    state1.insert("context".to_string(), serde_json::json!("onboarding"));

    let session1 = service
        .create(CreateRequest {
            app_name: APP_NAME.into(),
            user_id: "alice".into(),
            session_id: Some("pg-session-1".into()),
            state: state1,
        })
        .await?;
    println!("Session 1: {}", session1.id());

    let mut state2 = HashMap::new();
    state2.insert("app:version".to_string(), serde_json::json!("2.1.0"));
    state2.insert("user:plan".to_string(), serde_json::json!("free"));
    state2.insert("context".to_string(), serde_json::json!("support"));

    let session2 = service
        .create(CreateRequest {
            app_name: APP_NAME.into(),
            user_id: "bob".into(),
            session_id: Some("pg-session-2".into()),
            state: state2,
        })
        .await?;
    println!("Session 2: {}\n", session2.id());

    let alice_sessions = service
        .list(ListRequest {
            app_name: APP_NAME.into(),
            user_id: "alice".into(),
            limit: None,
            offset: None,
        })
        .await?;
    println!("Alice: {} session(s)\n", alice_sessions.len());

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, MODEL_NAME)?);
    let agent: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("pg_agent")
            .instruction(
                "You are a helpful assistant. Remember context from previous messages. Be concise.",
            )
            .model(model)
            .build()?,
    );

    let sessions_arc: Arc<dyn SessionService> = Arc::new(service);
    let runner = Runner::builder()
        .app_name(APP_NAME)
        .agent(agent)
        .session_service(sessions_arc.clone())
        .build()?;

    let msg1 =
        Content::new("user").with_text("My name is Alice and I love PostgreSQL. Remember that!");
    print!("User: My name is Alice and I love PostgreSQL.\n\nAgent: ");
    let mut stream = runner
        .run(UserId::new("alice")?, SessionId::new("pg-session-1")?, msg1)
        .await?;
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

    let msg2 = Content::new("user").with_text("What's my name and what database do I love?");
    print!("User: What's my name and what database do I love?\n\nAgent: ");
    let mut stream2 = runner
        .run(UserId::new("alice")?, SessionId::new("pg-session-1")?, msg2)
        .await?;
    while let Some(event) = stream2.next().await {
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

    let retrieved = sessions_arc
        .get(GetRequest {
            app_name: APP_NAME.into(),
            user_id: "alice".into(),
            session_id: "pg-session-1".into(),
            num_recent_events: None,
            after: None,
        })
        .await?;
    println!(
        "Session `{}` has {} persisted events.",
        retrieved.id(),
        retrieved.events().len()
    );

    sessions_arc
        .delete(DeleteRequest {
            app_name: APP_NAME.into(),
            user_id: "alice".into(),
            session_id: "pg-session-1".into(),
        })
        .await?;
    sessions_arc
        .delete(DeleteRequest {
            app_name: APP_NAME.into(),
            user_id: "bob".into(),
            session_id: "pg-session-2".into(),
        })
        .await?;
    println!("Cleanup complete.");

    Ok(())
}
