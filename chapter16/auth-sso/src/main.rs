use adk_auth::sso::{
    AzureADProvider, ClaimsMapper, GoogleProvider, JwtValidator, OidcProvider, OktaProvider,
    TokenClaims, TokenError,
};
use adk_auth::{AccessControl, Permission, Role};

fn live_smoke_requested() -> bool {
    matches!(std::env::var("BOOK_RUN_LIVE_SMOKE").as_deref(), Ok("1"))
}

fn create_claims(email: &str, groups: Vec<&str>) -> TokenClaims {
    TokenClaims {
        sub: format!("user-{}", email.split('@').next().unwrap_or(email)),
        email: Some(email.to_string()),
        email_verified: Some(true),
        groups: groups.into_iter().map(String::from).collect(),
        ..Default::default()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("=== SSO and JWT for Enterprise Identity ===\n");

    let _google = GoogleProvider::new("your-google-client-id");
    let _azure = AzureADProvider::new("your-tenant-id", "your-client-id");
    let _okta = OktaProvider::new("your-domain.okta.com", "your-client-id");
    let _oidc = OidcProvider::new(
        "https://keycloak.example.com/realms/main",
        "your-client-id",
        "https://keycloak.example.com/realms/main/protocol/openid-connect/certs",
    );
    println!("Built Google, Azure AD, Okta, and generic OIDC provider configs.");

    let validator = JwtValidator::builder()
        .issuer("https://accounts.google.com")
        .jwks_uri("https://www.googleapis.com/oauth2/v3/certs")
        .audience("api://my-agent-app")
        .build();
    println!(
        "JWT validator build status: {}",
        if validator.is_ok() {
            "ready"
        } else {
            "not ready"
        }
    );

    let claims = TokenClaims {
        sub: "user-12345".into(),
        iss: "https://accounts.google.com".into(),
        email: Some("alice@company.com".into()),
        name: Some("Alice Smith".into()),
        groups: vec!["Engineering".into(), "Admins".into()],
        exp: 1_735_700_000,
        iat: 1_735_696_400,
        ..Default::default()
    };

    println!("Claims example:");
    println!("  issuer: {}", claims.iss);
    println!("  groups: {:?}", claims.groups);
    println!("  expired: {}\n", claims.is_expired());

    let errors = [
        TokenError::Expired,
        TokenError::InvalidSignature,
        TokenError::InvalidIssuer {
            expected: "https://expected.com".into(),
            actual: "https://actual.com".into(),
        },
        TokenError::MissingClaim("email".into()),
    ];
    println!("Handled token errors:");
    for error in &errors {
        println!("  {error}");
    }
    println!();

    let mapper = ClaimsMapper::builder()
        .map_group("Admins", "admin")
        .map_group("Engineering", "developer")
        .map_group("DataAnalysts", "analyst")
        .user_id_from_email()
        .default_role("viewer")
        .build();

    let test_users = vec![
        create_claims("alice@company.com", vec!["Admins", "Engineering"]),
        create_claims("bob@company.com", vec!["Engineering"]),
        create_claims("carol@company.com", vec!["DataAnalysts"]),
        create_claims("guest@external.com", vec![]),
    ];
    println!("Resolved roles:");
    for claims in &test_users {
        let roles = mapper.map_to_roles(claims);
        let name = claims
            .email
            .as_deref()
            .unwrap_or("?")
            .split('@')
            .next()
            .unwrap_or("?");
        println!("  {name:10} -> {roles:?}");
    }
    println!();

    let ac = AccessControl::builder()
        .role(
            Role::new("admin")
                .allow(Permission::AllTools)
                .allow(Permission::AllAgents),
        )
        .role(
            Role::new("developer")
                .allow(Permission::Tool("search_data".into()))
                .allow(Permission::Tool("deploy_service".into()))
                .deny(Permission::Tool("admin_action".into())),
        )
        .role(
            Role::new("analyst")
                .allow(Permission::Tool("search_data".into()))
                .deny(Permission::Tool("deploy_service".into()))
                .deny(Permission::Tool("admin_action".into())),
        )
        .role(Role::new("viewer").allow(Permission::Tool("search_data".into())))
        .build()?;

    println!("Group-derived permission matrix:");
    for claims in &test_users {
        let roles = mapper.map_to_roles(claims);
        let name = claims
            .email
            .as_deref()
            .unwrap_or("?")
            .split('@')
            .next()
            .unwrap_or("?");
        let can_search = roles.iter().any(|role| {
            ac.get_role(role)
                .map(|role| role.can_access(&Permission::Tool("search_data".into())))
                .unwrap_or(false)
        });
        let can_deploy = roles.iter().any(|role| {
            ac.get_role(role)
                .map(|role| role.can_access(&Permission::Tool("deploy_service".into())))
                .unwrap_or(false)
        });
        let can_admin = roles.iter().any(|role| {
            ac.get_role(role)
                .map(|role| role.can_access(&Permission::Tool("admin_action".into())))
                .unwrap_or(false)
        });
        println!(
            "  {name:10} search={} deploy={} admin={}",
            can_search, can_deploy, can_admin
        );
    }

    if !live_smoke_requested() {
        println!(
            "\nSkipping networked OIDC discovery. Set BOOK_RUN_LIVE_SMOKE=1 to try provider discovery."
        );
        return Ok(());
    }

    println!("\nAttempting Google OIDC discovery...");
    match OidcProvider::from_discovery("https://accounts.google.com", "example-client-id").await {
        Ok(_) => println!("Discovery succeeded."),
        Err(error) => println!("Discovery failed: {error}"),
    }

    Ok(())
}
