use adk_auth::{AccessControl, Permission, Role};
use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use adk_tool::tool;
use schemars::JsonSchema;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

static ACCESS_CONTROL: OnceLock<Arc<AccessControl>> = OnceLock::new();
static CURRENT_USER: &str = "analyst@company.com";

fn live_smoke_requested() -> bool {
    matches!(std::env::var("BOOK_RUN_LIVE_SMOKE").as_deref(), Ok("1"))
}

#[derive(Deserialize, JsonSchema)]
struct SearchArgs {
    query: String,
}

#[tool]
async fn search_database(args: SearchArgs) -> adk_tool::Result<serde_json::Value> {
    let ac = ACCESS_CONTROL.get().expect("access control");
    if let Err(error) = ac.check(CURRENT_USER, &Permission::Tool("search_database".into())) {
        return Ok(serde_json::json!({ "error": format!("ACCESS DENIED: {error}") }));
    }
    Ok(serde_json::json!({
        "query": args.query,
        "results": [
            {"id": 1, "title": "Q1 Revenue Report", "summary": "Revenue grew 15% YoY"},
            {"id": 2, "title": "Q2 Forecast", "summary": "Projected 12% growth"},
        ]
    }))
}

#[derive(Deserialize, JsonSchema)]
struct DeleteArgs {
    record_id: u32,
}

#[tool]
async fn admin_delete(args: DeleteArgs) -> adk_tool::Result<serde_json::Value> {
    let ac = ACCESS_CONTROL.get().expect("access control");
    if let Err(error) = ac.check(CURRENT_USER, &Permission::Tool("admin_delete".into())) {
        return Ok(serde_json::json!({ "error": format!("ACCESS DENIED: {error}") }));
    }
    Ok(serde_json::json!({
        "deleted": args.record_id,
        "status": "success"
    }))
}

#[derive(Deserialize, JsonSchema)]
struct SummarizeArgs {
    text: String,
}

#[tool]
async fn summarize(args: SummarizeArgs) -> adk_tool::Result<serde_json::Value> {
    let ac = ACCESS_CONTROL.get().expect("access control");
    if let Err(error) = ac.check(CURRENT_USER, &Permission::Tool("summarize".into())) {
        return Ok(serde_json::json!({ "error": format!("ACCESS DENIED: {error}") }));
    }
    Ok(serde_json::json!({
        "summary": format!("Summary of {} chars of text", args.text.len()),
        "status": "success"
    }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    println!("=== RBAC Agent ===\n");

    let admin = Role::new("admin")
        .allow(Permission::AllTools)
        .allow(Permission::AllAgents);
    let analyst = Role::new("analyst")
        .allow(Permission::Tool("search_database".into()))
        .allow(Permission::Tool("summarize".into()))
        .deny(Permission::Tool("admin_delete".into()));
    let viewer = Role::new("viewer")
        .allow(Permission::Tool("search_database".into()))
        .deny(Permission::Tool("admin_delete".into()))
        .deny(Permission::Tool("summarize".into()));

    let ac = AccessControl::builder()
        .role(admin)
        .role(analyst)
        .role(viewer)
        .assign("admin@company.com", "admin")
        .assign("analyst@company.com", "analyst")
        .assign("viewer@company.com", "viewer")
        .build()?;

    println!("Permission checks for analyst@company.com:");
    let checks = [
        (
            "search_database",
            ac.check(
                "analyst@company.com",
                &Permission::Tool("search_database".into()),
            ),
        ),
        (
            "summarize",
            ac.check("analyst@company.com", &Permission::Tool("summarize".into())),
        ),
        (
            "admin_delete",
            ac.check(
                "analyst@company.com",
                &Permission::Tool("admin_delete".into()),
            ),
        ),
    ];
    for (tool_name, result) in &checks {
        println!(
            "  {tool_name} -> {}",
            if result.is_ok() { "ALLOWED" } else { "DENIED" }
        );
    }
    println!();

    let ac = Arc::new(ac);
    let _ = ACCESS_CONTROL.set(ac);

    if !live_smoke_requested() {
        println!(
            "Skipping live RBAC agent run. Set BOOK_RUN_LIVE_SMOKE=1 and GOOGLE_API_KEY to continue."
        );
        return Ok(());
    }

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, "gemini-3.1-flash-lite-preview")?);
    let agent = Arc::new(
        LlmAgentBuilder::new("rbac_agent")
            .instruction(
                "The current user is an analyst with access to search_database and summarize, but not admin_delete. Explain permission denials clearly.",
            )
            .model(model)
            .tool(Arc::new(SearchDatabase))
            .tool(Arc::new(AdminDelete))
            .tool(Arc::new(Summarize))
            .build()?,
    );

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    sessions
        .create(CreateRequest {
            app_name: "playground".into(),
            user_id: "user".into(),
            session_id: Some("s1".into()),
            state: HashMap::new(),
        })
        .await?;

    let runner = Runner::builder()
        .app_name("playground")
        .agent(agent)
        .session_service(sessions)
        .build()?;

    let query = "Search for revenue reports, summarize the results, then delete record #1.";
    println!("User: {query}\n");
    print!("Agent: ");
    let message = Content::new("user").with_text(query);
    let mut stream = runner
        .run(UserId::new("user")?, SessionId::new("s1")?, message)
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
    println!();

    Ok(())
}
