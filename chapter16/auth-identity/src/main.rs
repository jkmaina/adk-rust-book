use adk_core::{
    AdkIdentity, AppName, ExecutionIdentity, IdentityError, InvocationId, SessionId, UserId,
};
use adk_rust::futures::StreamExt;
use adk_rust::prelude::*;
use adk_rust::session::{CreateRequest, SessionService};
use adk_session::{AppendEventRequest, Event, GetRequest, InMemorySessionService};
use std::collections::HashMap;
use std::sync::Arc;

fn live_smoke_requested() -> bool {
    matches!(std::env::var("BOOK_RUN_LIVE_SMOKE").as_deref(), Ok("1"))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    println!("=== Typed Identity - Multi-Tenant Safety ===\n");

    let app = AppName::try_from("secure-app")?;
    let user = UserId::try_from("tenant:alice@example.com")?;
    let session = SessionId::try_from("session-abc-123")?;
    let invocation = InvocationId::generate();

    println!("AppName:      {}", app.as_ref());
    println!("UserId:       {}", user.as_ref());
    println!("SessionId:    {}", session.as_ref());
    println!("InvocationId: {invocation}");

    let identity = AdkIdentity::new(app.clone(), user.clone(), session.clone());
    let exec = ExecutionIdentity {
        adk: identity.clone(),
        invocation_id: invocation,
        branch: "main".to_string(),
        agent_name: "planner".to_string(),
    };
    println!(
        "ExecutionIdentity: agent={}, branch={}",
        exec.agent_name, exec.branch
    );

    let json = serde_json::to_string(&identity)?;
    let deser: AdkIdentity = serde_json::from_str(&json)?;
    assert_eq!(identity, deser);
    println!("Serde round-trip succeeded.\n");

    let attacks: Vec<(&str, &str, std::result::Result<(), IdentityError>)> = vec![
        ("Empty string", "\"\"", AppName::try_from("").map(|_| ())),
        (
            "Null byte",
            "\"bad\\0name\"",
            AppName::try_from("bad\0name").map(|_| ()),
        ),
        (
            "Overflow (513 bytes)",
            "\"x\" x 513",
            AppName::try_from("x".repeat(513).as_str()).map(|_| ()),
        ),
    ];

    println!("-- Injection attack prevention --");
    for (name, input, result) in &attacks {
        match result {
            Ok(_) => println!("  {name} ({input}) was unexpectedly accepted"),
            Err(error) => println!("  {name} blocked: {error}"),
        }
    }
    assert!(AppName::try_from("a".repeat(512).as_str()).is_ok());
    println!("  Max length of 512 bytes is accepted.\n");

    println!("-- Multi-tenant session isolation --");
    let service = InMemorySessionService::new();
    let shared_sid = "shared-session-42";
    service
        .create(CreateRequest {
            app_name: "secure-app".into(),
            user_id: "alice".into(),
            session_id: Some(shared_sid.into()),
            state: HashMap::new(),
        })
        .await?;
    service
        .create(CreateRequest {
            app_name: "secure-app".into(),
            user_id: "bob".into(),
            session_id: Some(shared_sid.into()),
            state: HashMap::new(),
        })
        .await?;

    let alice_id = AdkIdentity::new(
        AppName::try_from("secure-app")?,
        UserId::try_from("alice")?,
        SessionId::try_from(shared_sid)?,
    );
    service
        .append_event_for_identity(AppendEventRequest {
            identity: alice_id,
            event: Event::new("inv-alice"),
        })
        .await?;

    let alice_events = service
        .get(GetRequest {
            app_name: "secure-app".into(),
            user_id: "alice".into(),
            session_id: shared_sid.into(),
            num_recent_events: None,
            after: None,
        })
        .await?
        .events()
        .len();
    let bob_events = service
        .get(GetRequest {
            app_name: "secure-app".into(),
            user_id: "bob".into(),
            session_id: shared_sid.into(),
            num_recent_events: None,
            after: None,
        })
        .await?
        .events()
        .len();

    println!("  Alice events: {alice_events}");
    println!("  Bob events:   {bob_events}");
    assert_eq!(alice_events, 1);
    assert_eq!(bob_events, 0);
    println!("  Isolation confirmed.\n");

    if !live_smoke_requested() {
        println!(
            "Skipping live identity-scoped agent run. Set BOOK_RUN_LIVE_SMOKE=1 and GOOGLE_API_KEY to continue."
        );
        return Ok(());
    }

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, "gemini-3.1-flash-lite-preview")?);
    let agent = Arc::new(
        LlmAgentBuilder::new("identity_agent")
            .instruction(
                "Explain briefly why typed identity prevents injection attacks in multi-tenant AI systems.",
            )
            .model(model)
            .build()?,
    );

    let validated_user = UserId::try_from("alice")?;
    let validated_session = SessionId::try_from("identity-demo")?;
    let sessions: Arc<dyn SessionService> = Arc::new(InMemorySessionService::new());
    sessions
        .create(CreateRequest {
            app_name: "playground".into(),
            user_id: validated_user.as_ref().to_string(),
            session_id: Some(validated_session.as_ref().to_string()),
            state: HashMap::new(),
        })
        .await?;

    let runner = Runner::builder()
        .app_name("playground")
        .agent(agent)
        .session_service(sessions)
        .build()?;

    let message = Content::new("user").with_text(
        "How does typed identity prevent injection attacks in multi-tenant AI systems? Be concise.",
    );
    let mut stream = runner
        .run(validated_user, validated_session, message)
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
