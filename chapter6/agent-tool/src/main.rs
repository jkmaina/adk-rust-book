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
const SESSION_ID_VALUE: &str = "chapter6-agent-tool";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

#[derive(Deserialize, JsonSchema)]
struct CalcArgs {
    operation: String,
    a: f64,
    b: f64,
}

#[tool]
async fn calculator(args: CalcArgs) -> ToolResult<serde_json::Value> {
    let result = match args.operation.as_str() {
        "add" => args.a + args.b,
        "subtract" => args.a - args.b,
        "multiply" => args.a * args.b,
        "divide" if args.b != 0.0 => args.a / args.b,
        "divide" => return Ok(serde_json::json!({ "error": "division by zero" })),
        _ => return Ok(serde_json::json!({ "error": "unknown operation" })),
    };
    Ok(serde_json::json!({ "result": result }))
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

    let math_agent = LlmAgentBuilder::new("math_expert")
        .description("Solves math problems using a calculator tool")
        .instruction("You are a math expert. Use the calculator for arithmetic. Show your work.")
        .model(model.clone())
        .tool(Arc::new(Calculator))
        .build()?;

    let trivia_agent = LlmAgentBuilder::new("trivia_expert")
        .description("Answers trivia and general knowledge questions")
        .instruction("You are a trivia expert. Answer accurately and concisely.")
        .model(model.clone())
        .build()?;

    let math_tool = AgentTool::new(Arc::new(math_agent)).timeout(Duration::from_secs(30));
    let trivia_tool = AgentTool::new(Arc::new(trivia_agent)).timeout(Duration::from_secs(30));

    let coordinator: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("coordinator")
            .instruction(
                "Route questions to the right specialist:\n\
                 - Math/calculations -> math_expert\n\
                 - Trivia/facts -> trivia_expert\n\
                 Call each specialist ONCE, then summarize their responses for the user.\n\
                 Do NOT call the same specialist more than once.",
            )
            .model(model)
            .tool(Arc::new(math_tool))
            .tool(Arc::new(trivia_tool))
            .max_iterations(10)
            .build()?,
    );

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;

    let runner = build_runner(coordinator, sessions)?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;

    print_streamed_response(
        &runner,
        &user_id,
        &session_id,
        "What is 15% of 250, and who invented the percentage symbol?",
    )
    .await
}
