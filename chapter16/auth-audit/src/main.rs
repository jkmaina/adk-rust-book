use adk_auth::{AccessControl, AuditEvent, AuditOutcome, AuditSink, AuthError, Permission, Role};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

struct MemoryAuditSink {
    events: Mutex<Vec<AuditEvent>>,
}

impl MemoryAuditSink {
    fn new() -> Self {
        Self {
            events: Mutex::new(Vec::new()),
        }
    }

    fn events(&self) -> Vec<AuditEvent> {
        self.events.lock().expect("events lock").clone()
    }
}

#[async_trait]
impl AuditSink for MemoryAuditSink {
    async fn log(&self, event: AuditEvent) -> std::result::Result<(), AuthError> {
        self.events.lock().expect("events lock").push(event);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("=== RBAC + Audit Trail ===\n");

    let admin = Role::new("admin")
        .allow(Permission::AllTools)
        .allow(Permission::AllAgents);
    let analyst = Role::new("analyst")
        .allow(Permission::Tool("search_records".into()))
        .deny(Permission::Tool("delete_record".into()))
        .deny(Permission::Tool("transfer_funds".into()));
    let finance = Role::new("finance")
        .allow(Permission::Tool("search_records".into()))
        .allow(Permission::Tool("transfer_funds".into()))
        .deny(Permission::Tool("delete_record".into()));

    let ac = AccessControl::builder()
        .role(admin)
        .role(analyst)
        .role(finance)
        .assign("alice@company.com", "admin")
        .assign("bob@company.com", "analyst")
        .assign("carol@company.com", "finance")
        .build()?;

    let users = ["alice@company.com", "bob@company.com", "carol@company.com"];
    let tools = ["search_records", "delete_record", "transfer_funds"];

    println!("Permission matrix:");
    print!("  {:15}", "");
    for tool in &tools {
        print!("{:18}", tool);
    }
    println!();
    print!("  {:15}", "");
    println!("{}", "-".repeat(54));

    for user in &users {
        let name = user.split('@').next().unwrap_or(user);
        print!("  {:15}", name);
        for tool in &tools {
            let allowed = ac.check(user, &Permission::Tool((*tool).into())).is_ok();
            print!("{:18}", if allowed { "ALLOW" } else { "DENY" });
        }
        println!();
    }
    println!();

    let audit = Arc::new(MemoryAuditSink::new());
    let attempts = [
        ("alice", "search_records"),
        ("alice", "delete_record"),
        ("alice", "transfer_funds"),
        ("bob", "search_records"),
        ("bob", "delete_record"),
        ("bob", "transfer_funds"),
        ("carol", "search_records"),
        ("carol", "delete_record"),
        ("carol", "transfer_funds"),
    ];

    for (user, tool_name) in &attempts {
        let full_user = format!("{user}@company.com");
        let outcome = if ac
            .check(&full_user, &Permission::Tool((*tool_name).into()))
            .is_ok()
        {
            AuditOutcome::Allowed
        } else {
            AuditOutcome::Denied
        };
        audit
            .log(AuditEvent::tool_access(*user, *tool_name, outcome))
            .await?;
    }

    println!("Audit events:");
    for event in audit.events() {
        println!(
            "  user={} resource={} outcome={:?}",
            event.user, event.resource, event.outcome
        );
    }

    println!("\nProduction note: pair runtime enforcement with append-only audit sinks.");
    println!(
        "Runtime enforcement is demonstrated in `chapter16-auth-rbac` and `chapter16-auth-sso`."
    );
    Ok(())
}
