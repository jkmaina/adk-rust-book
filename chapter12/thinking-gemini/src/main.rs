use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

const APP_NAME: &str = "adk-rust-book";
const USER_ID_VALUE: &str = "book-user";
const SESSION_ID_VALUE: &str = "chapter12-thinking-gemini";
const MODEL_NAME: &str = "gemini-2.5-flash";

#[derive(JsonSchema, Serialize, Deserialize)]
struct CalculateArgs {
    expression: String,
}

#[derive(JsonSchema, Serialize, Deserialize)]
struct ConvertArgs {
    value: f64,
    from_unit: String,
    to_unit: String,
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

async fn print_turn(
    runner: &Runner,
    user_id: &UserId,
    session_id: &SessionId,
    label: &str,
    prompt: &str,
) -> anyhow::Result<()> {
    println!("{label}\n");
    println!("User: {prompt}\n");

    let message = Content::new("user").with_text(prompt);
    let mut stream = runner
        .run(user_id.clone(), session_id.clone(), message)
        .await?;

    let mut thinking_blocks = 0;
    let mut tool_calls = 0;
    let mut thought_signatures = 0;

    while let Some(event) = stream.next().await {
        let event = event?;
        if let Some(content) = &event.llm_response.content {
            for part in &content.parts {
                match part {
                    Part::Thinking { thinking, .. } => {
                        thinking_blocks += 1;
                        println!("[thinking]\n{thinking}\n");
                    }
                    Part::FunctionCall {
                        name,
                        args,
                        thought_signature,
                        ..
                    } => {
                        tool_calls += 1;
                        println!("[tool call] {name}({args})");
                        if let Some(signature) = thought_signature {
                            thought_signatures += 1;
                            let preview = signature.chars().take(40).collect::<String>();
                            println!(
                                "[thought_signature] {preview}... ({} chars)",
                                signature.len()
                            );
                        }
                        println!();
                    }
                    Part::FunctionResponse {
                        function_response, ..
                    } => {
                        println!("[tool response] {}\n", function_response.response);
                    }
                    _ => {
                        if let Some(text) = part.text() {
                            print!("{text}");
                        }
                    }
                }
            }
        }

        if event.llm_response.turn_complete
            && let Some(usage) = &event.llm_response.usage_metadata
            && let Some(tokens) = usage.thinking_token_count
        {
            println!("\n\nThinking tokens used: {tokens}");
        }
    }

    println!(
        "\nSummary: {thinking_blocks} thinking blocks, {tool_calls} tool calls, {thought_signatures} thought signatures\n"
    );
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let calc_tool = FunctionTool::new(
        "calculate",
        "Evaluate a mathematical expression and return the numeric result",
        |_ctx, args| async move {
            let expr = args
                .get("expression")
                .and_then(|value| value.as_str())
                .unwrap_or("0");
            let result: f64 = match expr {
                "120 / 2.25" | "120/2.25" => 53.33,
                "53.33 * 0.621371" | "53.33*0.621371" => 33.14,
                "53.33 * 1.60934" | "53.33*1.60934" => 85.84,
                "330 - 60" => 270.0,
                "270 / 150" => 1.8,
                "60 / 33.14" => 1.81,
                _ => expr.parse().unwrap_or(0.0),
            };
            Ok(serde_json::json!({ "expression": expr, "result": result }))
        },
    )
    .with_parameters_schema::<CalculateArgs>();

    let convert_tool = FunctionTool::new(
        "unit_convert",
        "Convert a value between units",
        |_ctx, args| async move {
            let value = args
                .get("value")
                .and_then(|value| value.as_f64())
                .unwrap_or(0.0);
            let from = args
                .get("from_unit")
                .and_then(|value| value.as_str())
                .unwrap_or("?");
            let to = args
                .get("to_unit")
                .and_then(|value| value.as_str())
                .unwrap_or("?");
            let result = match (from, to) {
                ("km/h", "mph") => value * 0.621371,
                ("mph", "km/h") => value * 1.60934,
                ("km", "miles") => value * 0.621371,
                ("miles", "km") => value * 1.60934,
                ("celsius", "fahrenheit") => value * 9.0 / 5.0 + 32.0,
                ("fahrenheit", "celsius") => (value - 32.0) * 5.0 / 9.0,
                ("kg", "lbs") => value * 2.20462,
                ("lbs", "kg") => value * 0.453592,
                _ => value,
            };

            Ok(serde_json::json!({
                "value": value,
                "from": from,
                "to": to,
                "result": format!("{result:.2}"),
            }))
        },
    )
    .with_parameters_schema::<ConvertArgs>();

    let model = Arc::new(GeminiModel::new(&api_key, MODEL_NAME)?);
    let agent: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("math_thinker")
            .instruction(
                "You are a precise math assistant. Think through problems carefully. \
                 Use calculate for arithmetic and unit_convert for conversions. \
                 Show your reasoning and verify results.",
            )
            .model(model)
            .tool(Arc::new(calc_tool))
            .tool(Arc::new(convert_tool))
            .build()?,
    );

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;

    let runner = build_runner(agent, sessions)?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;

    println!("=== Gemini Thinking and Thought Signatures ===\n");
    print_turn(
        &runner,
        &user_id,
        &session_id,
        "Turn 1: Multi-step calculation",
        "A train travels 120 km in 2 hours and 15 minutes. What is its average speed in both km/h and mph?",
    )
    .await?;
    print_turn(
        &runner,
        &user_id,
        &session_id,
        "Turn 2: Follow-up on preserved context",
        "Now convert that speed to a pace in minutes per mile for a runner comparison.",
    )
    .await
}
