use adk_auth::{AccessControl, Permission, Role};
use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use adk_tool::FunctionTool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

const APP_NAME: &str = "adk-rust-book-companion";
const SESSION_ID_VALUE: &str = "role-aware-operations-assistant-demo";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

static ACCESS_CONTROL: OnceLock<Arc<AccessControl>> = OnceLock::new();

struct UserProfile {
    user_id: &'static str,
    role: &'static str,
}

const USERS: &[UserProfile] = &[
    UserProfile {
        user_id: "viewer@northstar.example",
        role: "viewer",
    },
    UserProfile {
        user_id: "analyst@northstar.example",
        role: "analyst",
    },
    UserProfile {
        user_id: "admin@northstar.example",
        role: "admin",
    },
];

#[derive(Serialize, Deserialize, JsonSchema)]
struct SearchArgs {
    query: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct SummarizeArgs {
    text: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct RestartArgs {
    service: String,
    reason: String,
}

fn access_control() -> &'static Arc<AccessControl> {
    ACCESS_CONTROL.get().expect("access control initialized")
}

fn permission_error(user_id: &str, tool: &str, error: impl std::fmt::Display) -> serde_json::Value {
    serde_json::json!({
        "status": "denied",
        "user_id": user_id,
        "tool": tool,
        "reason": error.to_string(),
    })
}

fn build_runner(
    agent: Arc<dyn Agent>,
    sessions: Arc<dyn SessionService>,
) -> anyhow::Result<Runner> {
    Ok(Runner::builder()
        .app_name(APP_NAME)
        .agent(agent)
        .session_service(sessions)
        .build()?)
}

async fn create_session(
    sessions: &Arc<dyn SessionService>,
    profile: &UserProfile,
) -> anyhow::Result<()> {
    let mut state = HashMap::new();
    state.insert("ops:team".to_string(), "production operations".into());
    state.insert("ops:style".to_string(), "direct, operational, and explicit about denials".into());
    state.insert("ops:role".to_string(), profile.role.into());

    sessions
        .create(CreateRequest {
            app_name: APP_NAME.into(),
            user_id: profile.user_id.into(),
            session_id: Some(format!("{SESSION_ID_VALUE}-{}", profile.role)),
            state,
        })
        .await?;

    Ok(())
}

async fn print_turn(
    runner: &Runner,
    user_id: &UserId,
    session_id: &SessionId,
    label: &str,
    prompt: &str,
) -> anyhow::Result<()> {
    println!("{label}");
    println!("User Prompt: {prompt}\n");
    print!("Assistant: ");

    let message = Content::new("user").with_text(prompt);
    let mut stream = runner
        .run(user_id.clone(), session_id.clone(), message)
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
    Ok(())
}

fn build_access_control() -> anyhow::Result<AccessControl> {
    let admin = Role::new("admin")
        .allow(Permission::Tool("search_runbook".into()))
        .allow(Permission::Tool("summarize_incident".into()))
        .allow(Permission::Tool("restart_service".into()));

    let analyst = Role::new("analyst")
        .allow(Permission::Tool("search_runbook".into()))
        .allow(Permission::Tool("summarize_incident".into()))
        .deny(Permission::Tool("restart_service".into()));

    let viewer = Role::new("viewer")
        .allow(Permission::Tool("search_runbook".into()))
        .deny(Permission::Tool("summarize_incident".into()))
        .deny(Permission::Tool("restart_service".into()));

    AccessControl::builder()
        .role(admin)
        .role(analyst)
        .role(viewer)
        .assign("viewer@northstar.example", "viewer")
        .assign("analyst@northstar.example", "analyst")
        .assign("admin@northstar.example", "admin")
        .build()
        .map_err(Into::into)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let ac = Arc::new(build_access_control()?);
    let _ = ACCESS_CONTROL.set(ac.clone());

    println!("Role-Aware Operations Assistant Demo\n");
    println!("Role matrix:");
    for (user, tool) in [
        ("viewer@northstar.example", "search_runbook"),
        ("viewer@northstar.example", "summarize_incident"),
        ("analyst@northstar.example", "summarize_incident"),
        ("analyst@northstar.example", "restart_service"),
        ("admin@northstar.example", "restart_service"),
    ] {
        let allowed = ac.check(user, &Permission::Tool(tool.into())).is_ok();
        println!("  {user} -> {tool}: {}", if allowed { "ALLOWED" } else { "DENIED" });
    }
    println!();

    let search_tool = FunctionTool::new(
        "search_runbook",
        "Search incident runbooks and operational notes",
        |ctx, args| async move {
            let user_id = ctx.user_id().to_string();
            let ac = access_control();
            if let Err(error) = ac.check(&user_id, &Permission::Tool("search_runbook".into())) {
                return Ok(permission_error(&user_id, "search_runbook", error));
            }

            let query = args
                .get("query")
                .and_then(|value| value.as_str())
                .unwrap_or_default();
            Ok(serde_json::json!({
                "status": "ok",
                "query": query,
                "results": [
                    {
                        "title": "Database saturation playbook",
                        "summary": "Scale read replicas, inspect slow queries, and monitor connection pool exhaustion."
                    },
                    {
                        "title": "API latency incident guide",
                        "summary": "Check p95 latency, upstream dependencies, and deploy health before remediation."
                    }
                ]
            }))
        },
    )
    .with_parameters_schema::<SearchArgs>()
    .with_read_only(true)
    .with_concurrency_safe(true);

    let summarize_tool = FunctionTool::new(
        "summarize_incident",
        "Summarize incident details into a concise operational note",
        |ctx, args| async move {
            let user_id = ctx.user_id().to_string();
            let ac = access_control();
            if let Err(error) = ac.check(&user_id, &Permission::Tool("summarize_incident".into()))
            {
                return Ok(permission_error(&user_id, "summarize_incident", error));
            }

            let text = args
                .get("text")
                .and_then(|value| value.as_str())
                .unwrap_or_default();
            let shortened = text.chars().take(100).collect::<String>();
            Ok(serde_json::json!({
                "status": "ok",
                "summary": format!("Operational summary: {shortened}"),
            }))
        },
    )
    .with_parameters_schema::<SummarizeArgs>()
    .with_read_only(true)
    .with_concurrency_safe(true);

    let restart_tool = FunctionTool::new(
        "restart_service",
        "Restart a production service when the caller is authorized",
        |ctx, args| async move {
            let user_id = ctx.user_id().to_string();
            let ac = access_control();
            if let Err(error) = ac.check(&user_id, &Permission::Tool("restart_service".into())) {
                return Ok(permission_error(&user_id, "restart_service", error));
            }

            let service = args
                .get("service")
                .and_then(|value| value.as_str())
                .unwrap_or("unknown");
            let reason = args
                .get("reason")
                .and_then(|value| value.as_str())
                .unwrap_or("unspecified");
            Ok(serde_json::json!({
                "status": "ok",
                "service": service,
                "action": "restart_initiated",
                "reason": reason,
            }))
        },
    )
    .with_parameters_schema::<RestartArgs>();

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, MODEL_NAME)?);
    let agent: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("role_aware_operations_assistant")
            .instruction(
                "You assist {ops:team}. The current user role is {ops:role}. \
                 Use a {ops:style} tone. \
                 You may search runbooks, summarize incident details, and request \
                 service restarts, but tool permissions are enforced at runtime. \
                 If a tool returns denied status, explain that clearly and do not \
                 pretend the action succeeded.",
            )
            .model(model)
            .tool(Arc::new(search_tool))
            .tool(Arc::new(summarize_tool))
            .tool(Arc::new(restart_tool))
            .build()?,
    );

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    for profile in USERS {
        create_session(&sessions, profile).await?;
    }

    let runner = build_runner(agent, sessions)?;

    let prompt = "Search the runbook for database saturation guidance, summarize the response, \
                  then restart the orders-api service if needed because of elevated latency.";

    for profile in USERS {
        let user_id = UserId::new(profile.user_id)?;
        let session_id = SessionId::new(&format!("{SESSION_ID_VALUE}-{}", profile.role))?;
        let label = format!("--- {} ({}) ---", profile.user_id, profile.role);
        print_turn(&runner, &user_id, &session_id, &label, prompt).await?;
    }

    Ok(())
}
