use adk_plugin::{Plugin, PluginBuilder, PluginConfig, PluginManager};
use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use std::collections::HashMap;
use std::sync::Arc;

const APP_NAME: &str = "adk-rust-book";
const USER_ID_VALUE: &str = "book-user";
const SESSION_ID_VALUE: &str = "chapter9-plugin-system";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

fn build_runner(
    agent: Arc<dyn Agent>,
    sessions: Arc<dyn SessionService>,
    plugin_manager: Arc<PluginManager>,
) -> anyhow::Result<Runner> {
    Ok(Runner::builder()
        .app_name(APP_NAME)
        .agent(agent)
        .session_service(sessions)
        .plugin_manager(plugin_manager)
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

    println!("=== Plugin System: Live Agent Lifecycle ===\n");

    let logging = Plugin::new(PluginConfig {
        name: "logging".to_string(),
        on_user_message: Some(Box::new(|_ctx, content| {
            Box::pin(async move {
                let text = content
                    .parts
                    .iter()
                    .filter_map(|part| part.text())
                    .collect::<Vec<_>>()
                    .join(" ");
                println!("  [log] User message: \"{}\"", &text[..text.len().min(60)]);
                Ok(None)
            })
        })),
        on_event: Some(Box::new(|_ctx, event| {
            Box::pin(async move {
                println!("  [log] Event '{}' by {}", event.id, event.author);
                Ok(None)
            })
        })),
        ..Default::default()
    });
    println!("Registered logging plugin");

    let metrics = Plugin::new(PluginConfig {
        name: "metrics".to_string(),
        before_run: Some(Box::new(|_ctx| {
            Box::pin(async move {
                println!("  [metrics] Run starting");
                Ok(None)
            })
        })),
        after_run: Some(Box::new(|_ctx| {
            Box::pin(async move {
                println!("  [metrics] Run completed");
            })
        })),
        ..Default::default()
    });
    println!("Registered metrics plugin");

    let interceptor = PluginBuilder::new("model-interceptor")
        .before_model(Box::new(|_ctx, request| {
            Box::pin(async move {
                println!("  [interceptor] LLM request intercepted; passing through");
                Ok(BeforeModelResult::Continue(request))
            })
        }))
        .after_model(Box::new(|_ctx, _response| {
            Box::pin(async move {
                println!("  [interceptor] LLM response received");
                Ok(None)
            })
        }))
        .build();
    println!("Registered model interceptor");

    let manager = Arc::new(PluginManager::new(vec![logging, metrics, interceptor]));
    println!("Composed PluginManager with 3 plugins\n");

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, MODEL_NAME)?);
    let agent: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("plugged_agent")
            .instruction("You are a helpful assistant. Be concise in 2-3 sentences.")
            .model(model)
            .build()?,
    );

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    create_session(&sessions, SESSION_ID_VALUE).await?;

    let runner = build_runner(agent, sessions, manager)?;
    let user_id = UserId::new(USER_ID_VALUE)?;
    let session_id = SessionId::new(SESSION_ID_VALUE)?;

    println!("--- Running agent with plugins active ---\n");
    print!("Agent: ");
    print_streamed_response(
        &runner,
        &user_id,
        &session_id,
        "What are the benefits of plugin architectures in software systems?",
    )
    .await?;

    println!("\n--- Plugin lifecycle complete ---");
    Ok(())
}
