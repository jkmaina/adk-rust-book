use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use std::collections::HashMap;
use std::sync::Arc;

const APP_NAME: &str = "adk-rust-book-companion";
const USER_ID_VALUE: &str = "editorial-lead";
const SESSION_ID_VALUE: &str = "research-writing-workflow-demo";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

// This example demonstrates a workflow choice rather than a tool choice.
// Each stage has one job:
// research -> draft -> edit.
// The agent graph is simple enough to read, but realistic enough to show how
// staged content generation differs from a single prompt.

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
    // Session-backed state lets the workflow stay generic while the publication
    // target and audience remain configurable.
    let mut state = HashMap::new();
    state.insert("publication:name".to_string(), "Systems Engineering Weekly".into());
    state.insert("publication:audience".to_string(), "engineering managers and senior ICs".into());
    state.insert("publication:voice".to_string(), "clear, analytical, and non-hyped".into());
    state.insert("publication:length_target".to_string(), "roughly 3 short paragraphs".into());

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
    println!("Prompt: {prompt}\n");
    print!("Final Output: ");

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

    let researcher = Arc::new(
        LlmAgentBuilder::new("researcher")
            .description("Collects the core facts, tensions, and evidence for the topic.")
            .instruction(
                "You are the research stage for {publication:name}. \
                 The audience is {publication:audience}. \
                 For the given topic, produce concise research notes with: \
                 1. the core thesis, \
                 2. 3-4 key supporting points, \
                 3. one realistic tradeoff or caution. \
                 Focus on usable technical substance rather than marketing language. \
                 Do not write the final article.",
            )
            .model(model.clone())
            .build()?,
    ) as Arc<dyn Agent>;

    let writer = Arc::new(
        LlmAgentBuilder::new("writer")
            .description("Turns research notes into a coherent article draft.")
            .instruction(
                "You are the drafting stage for {publication:name}. \
                 Turn the prior research notes into a readable article draft for \
                 {publication:audience}. Keep the voice {publication:voice}. \
                 Aim for {publication:length_target}. Preserve the technical tradeoff \
                 instead of turning the piece into pure advocacy. \
                 Output a draft only, not editorial commentary about the process.",
            )
            .model(model.clone())
            .build()?,
    ) as Arc<dyn Agent>;

    let editor = Arc::new(
        LlmAgentBuilder::new("editor")
            .description("Edits the draft for clarity, structure, and publication quality.")
            .instruction(
                "You are the editorial stage for {publication:name}. \
                 Edit the draft for clarity, structure, precision, and tone. \
                 Keep the voice {publication:voice}. Remove repetition, keep the \
                 strongest technical points, and output the final publishable version only. \
                 Never mention the workflow, missing publication information, or what tone \
                 you selected. Use the publication profile already provided in session state.",
            )
            .model(model)
            .build()?,
    ) as Arc<dyn Agent>;

    // `SequentialAgent` makes the workflow boundary explicit: each stage runs in
    // order and hands its output to the next stage.
    let workflow: Arc<dyn Agent> = Arc::new(SequentialAgent::new(
        "research_writing_workflow",
        vec![researcher, writer, editor],
    ));

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;

    let runner = build_runner(workflow, sessions)?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;

    println!("Research Writing Workflow Demo\n");
    println!("Pipeline: researcher -> writer -> editor\n");

    print_turn(
        &runner,
        &user_id,
        &session_id,
        "--- Example 1: Main Article ---",
        "Write a short article on whether Rust reduces operational risk in backend services.",
    )
    .await?;

    // A follow-up prompt in the same session shows that the workflow can reuse
    // the same publication profile and recent context for a derivative piece.
    print_turn(
        &runner,
        &user_id,
        &session_id,
        "--- Example 2: Follow-Up Variant ---",
        "Now turn the same topic into a shorter executive briefing for leaders deciding whether to fund a migration.",
    )
    .await?;

    Ok(())
}
