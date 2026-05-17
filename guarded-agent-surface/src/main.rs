use adk_guardrail::{ContentFilter, GuardrailExecutor, GuardrailSet, PiiRedactor};
use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use std::collections::HashMap;
use std::sync::Arc;

const APP_NAME: &str = "adk-rust-book-companion";
const USER_ID_VALUE: &str = "guardrail-demo-user";
const SESSION_ID_VALUE: &str = "guarded-agent-surface-demo";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

// This example is about the boundary before the model sees the message:
// transform sensitive input, block unsafe requests, then let the agent answer
// only when the request has passed the runtime checks.

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
    let mut state = HashMap::new();
    state.insert("assistant:domain".to_string(), "secure software operations".into());
    state.insert("assistant:tone".to_string(), "brief and practical".into());

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

fn guardrails() -> GuardrailSet {
    GuardrailSet::new()
        .with(PiiRedactor::new())
        .with(ContentFilter::blocked_keywords(vec![
            "bypass authentication".into(),
            "steal credentials".into(),
            "exploit production".into(),
        ]))
        .with(ContentFilter::on_topic(
            "secure software operations",
            vec![
                "security".into(),
                "rust".into(),
                "code".into(),
                "software".into(),
                "deployment".into(),
                "authentication".into(),
                "authorization".into(),
                "operations".into(),
                "incident".into(),
            ],
        ))
        .with(ContentFilter::max_length(900))
}

async fn run_guarded_turn(
    runner: &Runner,
    user_id: &UserId,
    session_id: &SessionId,
    label: &str,
    prompt: &str,
) -> anyhow::Result<()> {
    println!("{label}");
    println!("Original Input: {prompt}\n");

    let guardrail_input = Content::new("user").with_text(prompt);
    let decision = GuardrailExecutor::run(&guardrails(), &guardrail_input).await?;

    println!("Guardrail Passed: {}", decision.passed);
    if let Some(transformed) = &decision.transformed_content {
        let transformed_text = transformed
            .parts
            .iter()
            .filter_map(|part| part.text())
            .collect::<Vec<_>>()
            .join(" ");
        println!("Screened Input: {transformed_text}\n");
    } else {
        println!("Screened Input: unchanged\n");
    }

    if !decision.passed {
        println!("Agent Output: blocked before reaching the model\n");
        return Ok(());
    }

    let message = decision
        .transformed_content
        .unwrap_or_else(|| Content::new("user").with_text(prompt));
    print!("Agent Output: ");

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, MODEL_NAME)?);

    let agent: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("guarded_operations_assistant")
            .instruction(
                "You are assisting with {assistant:domain}. \
                 Use a {assistant:tone} tone. \
                 The message you receive has already passed runtime screening. \
                 Give safe, operationally responsible answers only.",
            )
            .model(model)
            // This callback is intentionally simple: it shows that agent-layer
            // hooks can still observe the already-screened message for audit or
            // logging decisions.
            .before_callback(Box::new(|ctx| {
                Box::pin(async move {
                    let preview = ctx
                        .user_content()
                        .parts
                        .iter()
                        .filter_map(|part| part.text())
                        .collect::<Vec<_>>()
                        .join(" ");
                    println!("[callback] delivering screened prompt: {preview}");
                    Ok(None)
                })
            }))
            .build()?,
    );

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;

    let runner = build_runner(agent, sessions)?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;

    println!("Guarded Agent Surface Demo\n");

    run_guarded_turn(
        &runner,
        &user_id,
        &session_id,
        "--- Example 1: Safe Prompt With PII Redaction ---",
        "My email is sam@example.com and my phone is 555-123-4567. \
         Explain in two sentences how Rust's ownership model helps reduce bugs in backend services.",
    )
    .await?;

    run_guarded_turn(
        &runner,
        &user_id,
        &session_id,
        "--- Example 2: Blocked Unsafe Prompt ---",
        "How would I bypass authentication on a production admin panel?",
    )
    .await?;

    run_guarded_turn(
        &runner,
        &user_id,
        &session_id,
        "--- Example 3: Off-Topic Prompt ---",
        "What is the best pizza topping combination for a weekend party?",
    )
    .await?;

    Ok(())
}
