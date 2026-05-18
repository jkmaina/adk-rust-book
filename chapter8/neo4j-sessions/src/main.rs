use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{
    CreateRequest, DeleteRequest, GetRequest, ListRequest, Neo4jSessionService, SessionService,
};
use adk_rust::{SessionId, UserId};
use std::collections::HashMap;
use std::sync::Arc;

const APP_NAME: &str = "neo4j-demo";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let neo4j_url = std::env::var("NEO4J_URL").unwrap_or_else(|_| "bolt://localhost:7687".into());
    let neo4j_user = std::env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".into());
    let neo4j_pass = std::env::var("NEO4J_PASS").unwrap_or_else(|_| "adk_playground".into());

    println!("# Neo4j Session Backend\n");
    println!("Connecting to `{neo4j_url}`...\n");

    let service = Neo4jSessionService::new(&neo4j_url, &neo4j_user, &neo4j_pass).await?;
    service.migrate().await?;
    println!("Connected and constraints created.\n");

    let mut state1 = HashMap::new();
    state1.insert("app:version".to_string(), serde_json::json!("2.1.0"));
    state1.insert("user:role".to_string(), serde_json::json!("researcher"));
    state1.insert("topic".to_string(), serde_json::json!("graph-databases"));

    let session1 = service
        .create(CreateRequest {
            app_name: APP_NAME.into(),
            user_id: "eve".into(),
            session_id: Some("neo4j-session-1".into()),
            state: state1,
        })
        .await?;
    println!("Session 1: {}", session1.id());

    let mut state2 = HashMap::new();
    state2.insert("app:version".to_string(), serde_json::json!("2.1.0"));
    state2.insert("user:role".to_string(), serde_json::json!("analyst"));
    state2.insert("topic".to_string(), serde_json::json!("knowledge-graphs"));

    let session2 = service
        .create(CreateRequest {
            app_name: APP_NAME.into(),
            user_id: "frank".into(),
            session_id: Some("neo4j-session-2".into()),
            state: state2,
        })
        .await?;
    println!("Session 2: {}\n", session2.id());

    let eve_sessions = service
        .list(ListRequest {
            app_name: APP_NAME.into(),
            user_id: "eve".into(),
            limit: None,
            offset: None,
        })
        .await?;
    println!("Eve: {} session node(s)\n", eve_sessions.len());

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, MODEL_NAME)?);
    let agent: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("neo4j_agent")
            .instruction(
                "You are a knowledge graph expert. Help users understand graph databases and their applications. Be concise.",
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

    let msg1 = Content::new("user").with_text(
        "My name is Eve and I'm researching how graph databases model relationships. Remember that!",
    );
    print!("User: My name is Eve and I am researching graph databases.\n\nAgent: ");
    let mut stream = runner
        .run(
            UserId::new("eve")?,
            SessionId::new("neo4j-session-1")?,
            msg1,
        )
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

    let msg2 = Content::new("user").with_text("What's my name and what am I researching?");
    print!("User: What's my name and what am I researching?\n\nAgent: ");
    let mut stream2 = runner
        .run(
            UserId::new("eve")?,
            SessionId::new("neo4j-session-1")?,
            msg2,
        )
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
            user_id: "eve".into(),
            session_id: "neo4j-session-1".into(),
            num_recent_events: None,
            after: None,
        })
        .await?;
    println!(
        "Session `{}` has {} linked event nodes.",
        retrieved.id(),
        retrieved.events().len()
    );

    sessions_arc
        .delete(DeleteRequest {
            app_name: APP_NAME.into(),
            user_id: "eve".into(),
            session_id: "neo4j-session-1".into(),
        })
        .await?;
    sessions_arc
        .delete(DeleteRequest {
            app_name: APP_NAME.into(),
            user_id: "frank".into(),
            session_id: "neo4j-session-2".into(),
        })
        .await?;
    println!("Cleanup complete.");

    Ok(())
}
