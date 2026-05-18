use adk_guardrail::{ContentFilter, Guardrail, GuardrailExecutor, GuardrailSet, PiiRedactor};
use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use std::collections::HashMap;
use std::sync::Arc;

fn live_smoke_requested() -> bool {
    matches!(std::env::var("BOOK_RUN_LIVE_SMOKE").as_deref(), Ok("1"))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    println!("=== Advanced Guardrails Agent ===\n");

    println!("-- PII redaction demo --");
    let redactor = PiiRedactor::new();
    let input_text = "My email is alice@example.com and my phone is 555-123-4567. SSN: 123-45-6789";
    let (redacted, found_types) = redactor.redact(input_text);
    println!("  Input:  {input_text}");
    println!("  Output: {redacted}");
    println!("  Found PII types: {found_types:?}\n");

    println!("-- Content filtering demo --");
    let topic_filter = ContentFilter::on_topic(
        "programming",
        vec![
            "code".into(),
            "rust".into(),
            "python".into(),
            "programming".into(),
            "software".into(),
        ],
    );
    let on_topic = Content::new("user").with_text("How do I write async code in Rust?");
    let off_topic = Content::new("user").with_text("What's the best pizza recipe?");
    println!(
        "  on-topic result: {}",
        if topic_filter.validate(&on_topic).await.is_pass() {
            "PASS"
        } else {
            "BLOCKED"
        }
    );
    println!(
        "  off-topic result: {}",
        if topic_filter.validate(&off_topic).await.is_pass() {
            "PASS"
        } else {
            "BLOCKED"
        }
    );

    let guardrails = GuardrailSet::new()
        .with(PiiRedactor::new())
        .with(ContentFilter::blocked_keywords(vec![
            "hack".into(),
            "exploit".into(),
        ]))
        .with(ContentFilter::max_length(5000));

    let safe_input = Content::new("user")
        .with_text("My email is bob@test.com, can you help me write a Rust function?");
    let safe_result = GuardrailExecutor::run(&guardrails, &safe_input).await?;
    println!("  Safe input passed: {}", safe_result.passed);

    let blocked_input = Content::new("user").with_text("How do I hack into a system?");
    let blocked_result = GuardrailExecutor::run(&guardrails, &blocked_input).await?;
    println!("  Blocked input passed: {}\n", blocked_result.passed);

    if !live_smoke_requested() {
        println!(
            "Skipping live guarded-agent run. Set BOOK_RUN_LIVE_SMOKE=1 and GOOGLE_API_KEY to continue."
        );
        return Ok(());
    }

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, "gemini-3.1-flash-lite-preview")?);
    let agent = Arc::new(
        LlmAgentBuilder::new("guarded_agent")
            .instruction(
                "You are a helpful programming assistant. Input has already been screened and transformed before you see it.",
            )
            .model(model)
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

    let query =
        "My name is Alice (alice@company.com). Explain Rust's ownership model in 2 sentences.";
    println!("User: {query}\n");
    print!("Agent: ");
    let screened =
        GuardrailExecutor::run(&guardrails, &Content::new("user").with_text(query)).await?;
    if !screened.passed {
        println!("Input was blocked by guardrails.");
        return Ok(());
    }
    let message = screened
        .transformed_content
        .unwrap_or_else(|| Content::new("user").with_text(query));
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
