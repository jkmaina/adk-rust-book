use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, GetRequest, SessionService};
use adk_rust::tool::AgentTool;
use adk_rust::{SessionId, UserId};
use adk_tool::{Result as ToolResult, tool};
use schemars::JsonSchema;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

const APP_NAME: &str = "adk-rust-book-companion";
const USER_ID_VALUE: &str = "sam-rivera";
const SESSION_ID_VALUE: &str = "customer-support-agent-demo";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

// This example intentionally stays in one file so a new reader can trace the
// full runtime path without jumping between modules:
// typed tools -> specialist agents -> coordinator -> runner -> streamed turns.

// This typed input is part of the Chapter 6 lesson: tools should accept a
// narrow, explicit contract rather than an unstructured blob.
#[derive(Deserialize, JsonSchema)]
struct OrderLookupArgs {
    order_id: String,
}

#[tool]
async fn lookup_order(args: OrderLookupArgs) -> ToolResult<serde_json::Value> {
    // In a real system this would query a database or service. Here we keep the
    // data local so the example teaches ADK-Rust concepts instead of I/O setup.
    let order = match args.order_id.as_str() {
        "1001" => serde_json::json!({
            "order_id": "1001",
            "item": "Noise-cancelling headphones",
            "status": "delivered",
            "issue": "arrived_damaged",
            "total_usd": 89.00,
            "ordered_on": "2026-05-02",
            "eligible_for_refund": true
        }),
        "2002" => serde_json::json!({
            "order_id": "2002",
            "item": "Team collaboration headset bundle",
            "status": "delivered",
            "issue": "customer_requests_high_value_refund",
            "total_usd": 249.00,
            "ordered_on": "2026-05-10",
            "eligible_for_refund": true
        }),
        _ => serde_json::json!({
            "order_id": args.order_id,
            "status": "not_found",
            "message": "No order matched that ID."
        }),
    };

    Ok(order)
}

#[derive(Deserialize, JsonSchema)]
struct RefundArgs {
    order_id: String,
    amount_usd: f64,
    reason: String,
}

#[tool]
async fn issue_refund(args: RefundArgs) -> ToolResult<serde_json::Value> {
    // Business rules belong in code-level tools or services, not only in the
    // prompt. That makes the agent's allowed actions explicit and testable.
    if args.amount_usd > 100.0 {
        return Ok(serde_json::json!({
            "order_id": args.order_id,
            "status": "manager_approval_required",
            "amount_usd": args.amount_usd,
            "reason": args.reason,
            "message": "Refunds above $100 require manager approval before completion."
        }));
    }

    Ok(serde_json::json!({
        "order_id": args.order_id,
        "status": "approved",
        "amount_usd": args.amount_usd,
        "reason": args.reason,
        "refund_id": format!("RF-{}", args.order_id),
        "eta": "3-5 business days"
    }))
}

#[derive(Deserialize, JsonSchema)]
struct TicketArgs {
    order_id: String,
    issue_summary: String,
    priority: String,
}

#[tool]
async fn create_support_ticket(args: TicketArgs) -> ToolResult<serde_json::Value> {
    Ok(serde_json::json!({
        "ticket_id": format!("TKT-{}", args.order_id),
        "order_id": args.order_id,
        "priority": args.priority,
        "queue": "billing-review",
        "issue_summary": args.issue_summary,
        "next_step": "A manager will review the request."
    }))
}

fn build_runner(
    agent: Arc<dyn Agent>,
    sessions: Arc<dyn SessionService>,
) -> anyhow::Result<Runner> {
    // The builder keeps this example aligned with the published crate surface.
    // Feature-gated fields such as artifacts or plugins simply do not appear
    // unless the crate was compiled with those features enabled.
    Ok(Runner::builder()
        .app_name(APP_NAME)
        .agent(agent)
        .session_service(sessions)
        .build()?)
}

async fn create_session(
    sessions: &Arc<dyn SessionService>,
    session_id: &str,
) -> anyhow::Result<()> {
    // This state demonstrates the Chapter 5 pattern: explicit keyed state can
    // personalize later instructions without requiring a separate prompt per user.
    let mut initial_state = HashMap::new();
    initial_state.insert("customer:name".to_string(), "Sam Rivera".into());
    initial_state.insert("customer:tier".to_string(), "premium".into());
    initial_state.insert("customer:style".to_string(), "clear and practical".into());

    sessions
        .create(CreateRequest {
            app_name: APP_NAME.into(),
            user_id: USER_ID_VALUE.into(),
            session_id: Some(session_id.into()),
            state: initial_state,
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
    println!("Customer: {prompt}\n");
    print!("Agent: ");

    let message = Content::new("user").with_text(prompt);
    let mut stream = runner
        .run(user_id.clone(), session_id.clone(), message)
        .await?;

    // The runner returns an event stream. We print only text parts here so the
    // beginner can focus on the main control flow before exploring richer events.
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

    // Chapter 3 baseline: make the model dependency explicit instead of hiding
    // it behind convenience helpers too early.
    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, MODEL_NAME)?);

    // Specialist 1: reads order facts using a typed tool.
    let order_specialist = LlmAgentBuilder::new("order_specialist")
        .description("Checks orders and explains what happened to the order.")
        .instruction(
            "You are an order support specialist. Always use lookup_order before \
             answering questions about an order. Report the order status, item, \
             issue, and total clearly.",
        )
        .model(model.clone())
        .tool(Arc::new(LookupOrder))
        .build()?;

    // Specialist 2: handles refund decisions and escalations.
    let billing_specialist = LlmAgentBuilder::new("billing_specialist")
        .description("Handles refunds and creates tickets when manager approval is needed.")
        .instruction(
            "You are a billing specialist.\n\
             1. Use issue_refund when a refund is requested.\n\
             2. If the refund status is manager_approval_required, you MUST call \
             create_support_ticket.\n\
             3. Summarize whether the refund was approved immediately or escalated.",
        )
        .model(model.clone())
        .tool(Arc::new(IssueRefund))
        .tool(Arc::new(CreateSupportTicket))
        .build()?;

    // Chapter 6 and 7 bridge: specialist agents are exposed as tools so a
    // coordinator can delegate instead of trying to do every job itself.
    let order_tool = AgentTool::new(Arc::new(order_specialist)).timeout(Duration::from_secs(20));
    let billing_tool =
        AgentTool::new(Arc::new(billing_specialist)).timeout(Duration::from_secs(20));

    let coordinator: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("support_coordinator")
            .instruction(
                "You are the support coordinator for {customer:name}, a {customer:tier} \
                 customer who prefers {customer:style} answers.\n\
                 Always delegate order questions to order_specialist.\n\
                 Delegate refund actions to billing_specialist.\n\
                 Give the customer one final, easy-to-follow answer that explains \
                 what was checked, what action was taken, and what happens next.",
            )
            .model(model)
            .tool(Arc::new(order_tool))
            .tool(Arc::new(billing_tool))
            .max_iterations(8)
            .build()?,
    );

    // This demo uses the simplest session backend so the reader can understand
    // the runtime flow first. The book later swaps this for persistent stores.
    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;

    let runner = build_runner(coordinator, sessions.clone())?;

    // Typed identity is one of the book's recurring lessons: runtime identity
    // is explicit and validated, not just passed around as loose strings.
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;

    println!("Customer Support Agent Demo\n");

    // Turn 1 shows the straight-through path: look up the order and approve a
    // small refund immediately.
    print_turn(
        &runner,
        &user_id,
        &session_id,
        "--- Turn 1: Immediate Refund ---",
        "My order 1001 arrived damaged. Please check the order and refund it if possible.",
    )
    .await?;

    // Turn 2 shows the escalation path: a higher-value refund triggers a ticket.
    print_turn(
        &runner,
        &user_id,
        &session_id,
        "--- Turn 2: Escalation Path ---",
        "Now please also check order 2002. I want a refund on that order too, and if it needs manager approval, start the escalation for me.",
    )
    .await?;

    // Print one small session detail so the reader can see that both turns
    // really shared the same session rather than acting as disconnected calls.
    let session = sessions
        .get(GetRequest {
            app_name: APP_NAME.into(),
            user_id: USER_ID_VALUE.into(),
            session_id: SESSION_ID_VALUE.into(),
            num_recent_events: None,
            after: None,
        })
        .await?;

    println!("Session ID: {}", session.id());
    println!("Stored events: {}", session.events().len());

    Ok(())
}
