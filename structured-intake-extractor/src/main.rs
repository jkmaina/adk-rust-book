use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

const APP_NAME: &str = "adk-rust-book-companion";
const USER_ID_VALUE: &str = "intake-analyst";
const SESSION_ID_VALUE: &str = "structured-intake-extractor-demo";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";
const TEAM_QUEUE_PREFIX: &str = "ops";

// This example is intentionally tool-free. The core lesson is that the model
// can turn messy input into a stable JSON contract that downstream systems can
// validate and consume.

fn intake_ticket_schema() -> Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "requester_name": { "type": "string" },
            "organization": { "type": "string" },
            "contact_email": { "type": "string" },
            "contact_phone": { "type": "string" },
            "issue_category": {
                "type": "string",
                "enum": ["billing", "access", "bug", "outage", "feature_request", "other"]
            },
            "severity": {
                "type": "string",
                "enum": ["low", "medium", "high", "critical"]
            },
            "product_area": { "type": "string" },
            "summary": { "type": "string" },
            "business_impact": { "type": "string" },
            "customer_requested_action": { "type": "string" },
            "recommended_queue": { "type": "string" },
            "follow_up_required": { "type": "boolean" },
            "missing_information": {
                "type": "array",
                "items": { "type": "string" }
            }
        },
        "required": [
            "requester_name",
            "organization",
            "contact_email",
            "contact_phone",
            "issue_category",
            "severity",
            "product_area",
            "summary",
            "business_impact",
            "customer_requested_action",
            "recommended_queue",
            "follow_up_required",
            "missing_information"
        ]
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
    session_id: &str,
) -> anyhow::Result<()> {
    // Session-backed state lets the extractor inherit operational preferences
    // without rewriting the instruction string in code each time.
    let mut state = HashMap::new();
    state.insert("team:name".to_string(), "Customer Operations".into());
    state.insert("team:queue_prefix".to_string(), TEAM_QUEUE_PREFIX.into());
    state.insert(
        "team:triage_policy".to_string(),
        "Be conservative. Escalate outages and multi-user incidents aggressively.".into(),
    );

    sessions
        .create(CreateRequest {
            app_name: APP_NAME.into(),
            user_id: USER_ID_VALUE.into(),
            session_id: Some(session_id.into()),
            state,
        })
        .await?;

    Ok(())
}

async fn extract_and_print(
    runner: &Runner,
    user_id: &UserId,
    session_id: &SessionId,
    label: &str,
    prompt: &str,
) -> anyhow::Result<()> {
    println!("{label}");
    println!("Intake Source:\n{prompt}\n");

    let message = Content::new("user").with_text(prompt);
    let mut stream = runner
        .run(user_id.clone(), session_id.clone(), message)
        .await?;

    let mut response_text = String::new();
    while let Some(event) = stream.next().await {
        let event = event?;
        if let Some(content) = &event.llm_response.content {
            for part in &content.parts {
                if let Some(text) = part.text() {
                    response_text.push_str(text);
                }
            }
        }
    }

    // Pretty-print the model output so the reader can inspect the stable schema
    // instead of a raw minified JSON blob.
    let mut parsed: Value = serde_json::from_str(&response_text)?;
    normalize_ticket(&mut parsed);
    println!(
        "Structured Ticket:\n{}\n",
        serde_json::to_string_pretty(&parsed)?
    );

    Ok(())
}

fn normalize_ticket(ticket: &mut Value) {
    // This is the downstream integration layer: the schema guarantees shape,
    // then application code can enforce tenant-specific conventions such as
    // queue naming or fallback cleanup rules.
    let Some(queue) = ticket
        .get("recommended_queue")
        .and_then(Value::as_str)
        .map(str::trim)
    else {
        return;
    };

    if queue.starts_with(&format!("{TEAM_QUEUE_PREFIX}-")) {
        return;
    }

    let slug = queue
        .chars()
        .map(|ch| match ch {
            'A'..='Z' => ch.to_ascii_lowercase(),
            'a'..='z' | '0'..='9' => ch,
            _ => '-',
        })
        .collect::<String>()
        .trim_matches('-')
        .split('-')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    if let Some(object) = ticket.as_object_mut() {
        object.insert(
            "recommended_queue".to_string(),
            Value::String(format!("{TEAM_QUEUE_PREFIX}-{slug}")),
        );
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, MODEL_NAME)?);
    let extractor: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("structured_intake_extractor")
            .instruction(
                "You are a structured intake analyst for {team:name}.\n\
                 Always return valid JSON matching the provided schema.\n\
                 The recommended_queue value MUST begin with {team:queue_prefix}-.\n\
                 Follow this triage policy: {team:triage_policy}\n\
                 If information is missing, keep the field as an empty string \
                 when appropriate and list the missing details in missing_information.\n\
                 Do not add prose outside the JSON object.",
            )
            .model(model)
            .output_schema(intake_ticket_schema())
            .build()?,
    );

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;

    let runner = build_runner(extractor, sessions)?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;

    println!("Structured Intake Extractor Demo\n");

    extract_and_print(
        &runner,
        &user_id,
        &session_id,
        "--- Example 1: Payment Incident ---",
        "Hi team, this is Priya Nair from Acme Retail. Since about 8:15 AM UTC our \
         finance staff have seen duplicate charges in the dashboard for at least 27 \
         customers. We paused manual refunds because we are not sure if the issue is \
         still spreading. Please call me at +44 7700 900123 or email \
         priya.nair@acmeretail.example. This is affecting our EU checkout operations \
         and we need an urgent update.",
    )
    .await?;

    extract_and_print(
        &runner,
        &user_id,
        &session_id,
        "--- Example 2: Access Request With Gaps ---",
        "Hello, my name is Marcus. I joined the customer success team yesterday and I \
         still cannot log into the analytics portal. I can access email, but the portal \
         says my account does not exist. Could someone help me get access before the \
         onboarding session this afternoon?",
    )
    .await?;

    Ok(())
}
