use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::tool::AgentTool;
use adk_rust::{SessionId, UserId};
use adk_tool::{Result as ToolResult, tool};
use schemars::JsonSchema;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

const APP_NAME: &str = "adk-rust-book";
const USER_ID_VALUE: &str = "book-user";
const SESSION_ID_VALUE: &str = "chapter7-customer-service";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

#[derive(Deserialize, JsonSchema)]
struct AccountLookup {
    customer_id: String,
}

#[tool]
async fn lookup_account(args: AccountLookup) -> ToolResult<serde_json::Value> {
    Ok(serde_json::json!({
        "customer_id": args.customer_id,
        "name": "Alex Johnson",
        "plan": "Business ($79/mo)",
        "status": "active",
        "member_since": "2024-03-15",
        "recent_charges": [
            { "date": "2026-03-01", "amount": 79.00, "description": "Business Plan - March 2026" },
            { "date": "2026-03-01", "amount": 79.00, "description": "Business Plan - March 2026 (DUPLICATE)" },
            { "date": "2026-02-01", "amount": 79.00, "description": "Business Plan - February 2026" }
        ],
        "payment_method": "Visa ending in 4242"
    }))
}

#[derive(Deserialize, JsonSchema)]
struct RefundRequest {
    customer_id: String,
    amount: f64,
    reason: String,
}

#[tool]
async fn process_refund(args: RefundRequest) -> ToolResult<serde_json::Value> {
    if args.amount > 50.0 {
        return Ok(serde_json::json!({
            "status": "pending_approval",
            "refund_id": "REF-20260320-001",
            "amount": args.amount,
            "reason": args.reason,
            "message": "Refund exceeds $50 limit. Manager approval required.",
            "requires": "manager_approval"
        }));
    }

    Ok(serde_json::json!({
        "customer_id": args.customer_id,
        "status": "approved",
        "refund_id": "REF-20260320-001",
        "amount": args.amount,
        "reason": args.reason,
        "eta": "3-5 business days"
    }))
}

#[derive(Deserialize, JsonSchema)]
struct ApprovalDecision {
    refund_id: String,
    approved: bool,
    note: String,
}

#[tool]
async fn approve_refund(args: ApprovalDecision) -> ToolResult<serde_json::Value> {
    if args.approved {
        Ok(serde_json::json!({
            "refund_id": args.refund_id,
            "status": "approved",
            "approved_by": "Manager",
            "note": args.note,
            "eta": "3-5 business days",
            "confirmation": "Customer will receive email confirmation within 1 hour."
        }))
    } else {
        Ok(serde_json::json!({
            "refund_id": args.refund_id,
            "status": "denied",
            "denied_by": "Manager",
            "note": args.note,
            "next_steps": "Customer may appeal via support ticket."
        }))
    }
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

async fn print_streamed_response(
    runner: &Runner,
    user_id: &UserId,
    session_id: &SessionId,
    prompt: &str,
) -> anyhow::Result<()> {
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

    println!();
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, MODEL_NAME)?);

    let billing_agent = LlmAgentBuilder::new("billing_agent")
        .description("Handles billing inquiries: account lookup, charges, and refund initiation. Cannot approve refunds over $50.")
        .instruction(
            "You are a billing specialist. Your job:\n\
             1. Look up customer accounts with lookup_account\n\
             2. Process refunds with process_refund\n\n\
             CRITICAL: If process_refund returns status 'pending_approval', you MUST respond with \
             EXACTLY this format: 'ESCALATION REQUIRED: Refund [refund_id] for $[amount] needs \
             manager approval. Reason: [reason]'. Do NOT resolve the issue yourself. \
             Do NOT tell the customer the refund is complete. Just report the escalation need.",
        )
        .model(model.clone())
        .tool(Arc::new(LookupAccount))
        .tool(Arc::new(ProcessRefund))
        .build()?;

    let manager_agent = LlmAgentBuilder::new("manager_agent")
        .description("Manager who reviews and approves or denies escalated refund requests over $50. Must use approve_refund tool.")
        .instruction(
            "You are a customer service manager. You MUST use the approve_refund tool to \
             make your decision.\n\n\
             When you receive an escalation:\n\
             1. Use approve_refund with the refund_id, set approved=true for duplicate charges\n\
             2. Always include a note explaining your decision\n\n\
             You MUST call approve_refund. Do not just respond with text.",
        )
        .model(model.clone())
        .tool(Arc::new(ApproveRefund))
        .build()?;

    let billing_tool = AgentTool::new(Arc::new(billing_agent)).timeout(Duration::from_secs(30));
    let manager_tool = AgentTool::new(Arc::new(manager_agent)).timeout(Duration::from_secs(30));

    let coordinator: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("coordinator")
            .instruction(
                "You coordinate customer service by delegating to your team. You NEVER handle \
                 issues directly. You ALWAYS delegate to the appropriate agent.\n\n\
                 Your team:\n\
                 - billing_agent: Looks up accounts and initiates refunds\n\
                 - manager_agent: Approves or denies refunds over $50\n\n\
                 MANDATORY WORKFLOW:\n\
                 Step 1: Call billing_agent with the customer's issue\n\
                 Step 2: If billing_agent mentions ESCALATION REQUIRED or pending_approval, \
                 you MUST call manager_agent with the refund details\n\
                 Step 3: After manager_agent responds, summarize the full resolution to the customer.\n\n\
                 IMPORTANT: You must call BOTH billing_agent AND manager_agent for refunds over $50.",
            )
            .model(model)
            .tool(Arc::new(billing_tool))
            .tool(Arc::new(manager_tool))
            .max_iterations(10)
            .build()?,
    );

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;
    let runner = build_runner(coordinator, sessions)?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;

    println!("Customer Service Escalation Demo\n");
    print_streamed_response(
        &runner,
        &user_id,
        &session_id,
        "I see two charges of $79 on March 1st for my Business Plan. My account is alex@example.com. Please refund the duplicate charge and get it fully approved so I know it's done.",
    )
    .await
}
