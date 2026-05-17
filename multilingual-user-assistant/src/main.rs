use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_rust::{SessionId, UserId};
use std::collections::HashMap;
use std::sync::Arc;

const APP_NAME: &str = "adk-rust-book-companion";
const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

// This example demonstrates the lighter side of the framework:
// session-backed prompt templating, typed runtime identity, and multi-turn
// continuity without introducing tools or workflow orchestration too early.

struct UserProfile {
    user_id: &'static str,
    session_id: &'static str,
    name: &'static str,
    language: &'static str,
    expertise: &'static str,
    tone: &'static str,
    response_format: &'static str,
    analogy_preference: &'static str,
}

const PROFILES: &[UserProfile] = &[
    UserProfile {
        user_id: "sofia-martinez",
        session_id: "multilingual-spanish-beginner",
        name: "Sofia",
        language: "Spanish",
        expertise: "beginner",
        tone: "warm and encouraging",
        response_format: "short paragraphs followed by a bullet recap",
        analogy_preference: "simple everyday analogies",
    },
    UserProfile {
        user_id: "amelie-leroux",
        session_id: "multilingual-french-intermediate",
        name: "Amelie",
        language: "French",
        expertise: "intermediate",
        tone: "concise and practical",
        response_format: "two short paragraphs",
        analogy_preference: "only when it clarifies a technical point",
    },
    UserProfile {
        user_id: "daniel-cho",
        session_id: "multilingual-english-advanced",
        name: "Daniel",
        language: "English",
        expertise: "advanced",
        tone: "direct and technical",
        response_format: "compact technical explanation",
        analogy_preference: "avoid analogies unless explicitly requested",
    },
];

fn build_runner(
    agent: Arc<dyn Agent>,
    sessions: Arc<dyn SessionService>,
) -> anyhow::Result<Runner> {
    // The builder keeps the example aligned with the public crate surface and
    // avoids hand-constructing feature-gated fields.
    Ok(Runner::builder()
        .app_name(APP_NAME)
        .agent(agent)
        .session_service(sessions)
        .build()?)
}

async fn create_profile_session(
    sessions: &Arc<dyn SessionService>,
    profile: &UserProfile,
) -> anyhow::Result<()> {
    // Session state is the key concept here: the assistant prompt is parameterized
    // by user metadata rather than being rebuilt for each person by hand.
    let mut state = HashMap::new();
    state.insert("user:name".to_string(), profile.name.into());
    state.insert("user:language".to_string(), profile.language.into());
    state.insert("user:expertise".to_string(), profile.expertise.into());
    state.insert("user:tone".to_string(), profile.tone.into());
    state.insert("user:format".to_string(), profile.response_format.into());
    state.insert(
        "user:analogy_preference".to_string(),
        profile.analogy_preference.into(),
    );

    sessions
        .create(CreateRequest {
            app_name: APP_NAME.into(),
            user_id: profile.user_id.into(),
            session_id: Some(profile.session_id.into()),
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
    println!("User: {prompt}\n");
    print!("Assistant: ");

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

    let assistant: Arc<dyn Agent> = Arc::new(
        LlmAgentBuilder::new("multilingual_user_assistant")
            .instruction(
                "You are assisting {user:name}.\n\
                 Always reply in {user:language}.\n\
                 The user's technical expertise is {user:expertise}.\n\
                 Use a {user:tone} tone.\n\
                 Format the reply as {user:format}.\n\
                 Follow this analogy preference: {user:analogy_preference}.\n\
                 Keep explanations accurate, adapt the depth to the expertise level, \
                 and preserve continuity across follow-up questions in the same session.",
            )
            .model(model)
            .build()?,
    );

    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    for profile in PROFILES {
        create_profile_session(&sessions, profile).await?;
    }

    let runner = build_runner(assistant, sessions)?;

    println!("Multilingual User Assistant Demo\n");

    let shared_prompt =
        "Explain what an API is and when a frontend application should call one.";

    // The same question is asked under three different profile sessions so the
    // reader can compare how session state changes the answer without changing
    // the underlying agent implementation.
    for profile in PROFILES {
        let user_id = UserId::new(profile.user_id)?;
        let session_id = SessionId::new(profile.session_id)?;

        let label = format!(
            "--- Profile: {} ({}, {}) ---",
            profile.name, profile.language, profile.expertise
        );
        print_turn(&runner, &user_id, &session_id, &label, shared_prompt).await?;
    }

    // One follow-up turn demonstrates that the same session can preserve both
    // user profile state and conversation history.
    let sofia_user_id = UserId::new(PROFILES[0].user_id)?;
    let sofia_session_id = SessionId::new(PROFILES[0].session_id)?;
    print_turn(
        &runner,
        &sofia_user_id,
        &sofia_session_id,
        "--- Follow-up: Sofia Keeps The Same Profile ---",
        "Now explain the same idea with a restaurant analogy and keep it beginner friendly.",
    )
    .await?;

    Ok(())
}
