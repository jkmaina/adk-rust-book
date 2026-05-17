use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use adk_tool::ExitLoopTool;
use std::collections::HashMap;
use std::sync::Arc;

const APP_NAME: &str = "adk-rust-book-companion";
const USER_ID_VALUE: &str = "content-editor";
const SESSION_ID_VALUE: &str = "iterative-content-refiner-demo";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

// This example focuses on bounded improvement loops.
// The agent keeps refining until the content satisfies the quality bar or the
// maximum iteration count is reached. That makes the stopping condition part of
// the workflow design instead of an afterthought.

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
    state.insert("editor:audience".to_string(), "existing B2B customers".into());
    state.insert("editor:tone".to_string(), "clear, confident, and non-hyped".into());
    state.insert("editor:length".to_string(), "under 150 words".into());
    state.insert(
        "editor:quality_bar".to_string(),
        "must be specific, concise, and easy to skim".into(),
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

async fn print_turn(
    runner: &Runner,
    user_id: &UserId,
    session_id: &SessionId,
    label: &str,
    prompt: &str,
) -> anyhow::Result<()> {
    println!("{label}");
    println!("Draft Input:\n{prompt}\n");
    print!("Refined Output: ");

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

    let refiner = Arc::new(
        LlmAgentBuilder::new("content_refiner")
            .instruction(
                "You are refining content for {editor:audience}. \
                 Use a {editor:tone} tone and keep the result {editor:length}. \
                 The quality bar is: {editor:quality_bar}.\n\
                 On each pass, improve clarity, specificity, structure, and \
                 readability.\n\
                 If the content already meets the bar, call exit_loop with the \
                 final version.\n\
                 Otherwise, return an improved revision only.\n\
                 Never explain the workflow or mention iteration counts.",
            )
            .model(model)
            .tool(Arc::new(ExitLoopTool::new()))
            .build()?,
    ) as Arc<dyn Agent>;

    // `LoopAgent` gives us bounded retry/rework behavior. The maximum iteration
    // count matters because production systems need explicit stopping rules.
    let loop_agent: Arc<dyn Agent> = Arc::new(
        LoopAgent::new("iterative_content_refiner", vec![refiner]).with_max_iterations(5),
    );

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;

    let runner = build_runner(loop_agent, sessions)?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;

    println!("Iterative Content Refiner Demo\n");
    println!("Workflow: refine -> check quality -> exit or refine again\n");

    print_turn(
        &runner,
        &user_id,
        &session_id,
        "--- Example 1: Product Update Draft ---",
        "Hey everyone, we changed the reporting thing and it should be better now. \
         There are filters and exports and it is faster. Some old problems might \
         still be there but we think it helps a lot and customers should try it \
         when they can.",
    )
    .await?;

    print_turn(
        &runner,
        &user_id,
        &session_id,
        "--- Example 2: Follow-Up Variant ---",
        "Now refine a release note for the same audience: We added role based \
         access controls. Admins can do more stuff and people can limit what \
         others see. This should help security and teams with more complex needs.",
    )
    .await?;

    Ok(())
}
