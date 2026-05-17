use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use std::collections::HashMap;
use std::sync::Arc;

const APP_NAME: &str = "adk-rust-book-companion";
const USER_ID_VALUE: &str = "review-chair";
const SESSION_ID_VALUE: &str = "parallel-review-system-demo";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

// This example demonstrates the workflow value of parallel review:
// the same proposal is examined concurrently by different specialists so the
// final discussion starts with breadth instead of one narrow perspective.

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
    state.insert("review:company".to_string(), "Northstar Collaboration".into());
    state.insert("review:audience".to_string(), "product leadership and engineering leads".into());
    state.insert("review:stage".to_string(), "pre-beta feature review".into());
    state.insert("review:style".to_string(), "direct, concrete, and decision-oriented".into());

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

async fn print_turn(
    runner: &Runner,
    user_id: &UserId,
    session_id: &SessionId,
    label: &str,
    prompt: &str,
) -> anyhow::Result<()> {
    println!("{label}");
    println!("Proposal:\n{prompt}\n");
    print!("Parallel Review: ");

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, MODEL_NAME)?);

    let technical = Arc::new(
        LlmAgentBuilder::new("technical_reviewer")
            .description("Reviews implementation risk, architecture, and operational constraints.")
            .instruction(
                "You are the technical reviewer for {review:company}. \
                 The audience is {review:audience}. \
                 Evaluate the proposal from an engineering perspective during a \
                 {review:stage}. Use a {review:style} tone. \
                 Focus on implementation complexity, system risk, dependencies, \
                 and rollout concerns. Keep the review concise and actionable. \
                 Output only a section titled TECHNICAL REVIEW with exactly three \
                 bullets: strengths, risks, and pre-beta requirement.",
            )
            .model(model.clone())
            .build()?,
    ) as Arc<dyn Agent>;

    let product = Arc::new(
        LlmAgentBuilder::new("product_reviewer")
            .description("Reviews value, prioritization, and business tradeoffs.")
            .instruction(
                "You are the product reviewer for {review:company}. \
                 Evaluate the proposal for user value, prioritization, strategic fit, \
                 and launch tradeoffs. Use a {review:style} tone and keep the review \
                 concise, concrete, and decision-oriented. \
                 Output only a section titled PRODUCT REVIEW with exactly three \
                 bullets: value, tradeoff, and pre-beta requirement.",
            )
            .model(model.clone())
            .build()?,
    ) as Arc<dyn Agent>;

    let ux = Arc::new(
        LlmAgentBuilder::new("ux_reviewer")
            .description("Reviews usability, clarity, and interaction risk.")
            .instruction(
                "You are the UX reviewer for {review:company}. \
                 Evaluate the proposal for usability, discoverability, clarity, \
                 and likely user friction. Use a {review:style} tone and provide \
                 concise recommendations. \
                 Output only a section titled UX REVIEW with exactly three \
                 bullets: usability risk, clarity issue, and pre-beta requirement.",
            )
            .model(model)
            .build()?,
    ) as Arc<dyn Agent>;

    // `ParallelAgent` runs all three perspectives concurrently. The main lesson
    // is that breadth can be designed into the workflow instead of relying on a
    // single answer to cover every decision dimension well.
    let review_system: Arc<dyn Agent> = Arc::new(ParallelAgent::new(
        "parallel_review_system",
        vec![technical, product, ux],
    ));

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;

    let runner = build_runner(review_system, sessions)?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;

    println!("Parallel Review System Demo\n");
    println!("Reviewers: technical + product + UX\n");

    print_turn(
        &runner,
        &user_id,
        &session_id,
        "--- Example 1: Feature Proposal Review ---",
        "We want to add an AI-generated meeting summary panel to our B2B \
         collaboration product. After each recorded meeting, users would see a \
         summary, action items, and decisions. The first release would support \
         English only and run on our existing transcript pipeline. Review this \
         proposal for a pre-beta launch.",
    )
    .await?;

    print_turn(
        &runner,
        &user_id,
        &session_id,
        "--- Example 2: Follow-Up Decision Prompt ---",
        "Given the same feature, what would each reviewer want fixed before a \
         broader customer beta?",
    )
    .await?;

    Ok(())
}
