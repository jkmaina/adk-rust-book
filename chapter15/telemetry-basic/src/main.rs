use adk_telemetry::{debug, error, info, init_telemetry, instrument, trace, warn};

#[tokio::main]
async fn main() {
    init_telemetry("chapter15-telemetry-basic").expect("failed to init telemetry");

    println!("Telemetry Basic Example");
    println!("=======================\n");

    info!("Application started");
    debug!("Debug message (visible with RUST_LOG=debug)");
    trace!("Trace message (visible with RUST_LOG=trace)");

    info!(
        agent.name = "my_agent",
        session.id = "sess-123",
        user.id = "user-456",
        "Processing user request"
    );

    process_request("user-789", "Hello, agent!").await;

    warn!(rate_limit = 95, "Rate limit approaching");

    let err = std::io::Error::new(std::io::ErrorKind::NotFound, "Resource not found");
    error!(error = ?err, "Operation failed");

    info!("Application completed");

    println!("\nCheck the logs above for structured output.");
}

#[instrument]
async fn process_request(_user_id: &str, message: &str) {
    info!("Processing request");
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    validate_input(message).await;
    info!("Request processed successfully");
}

#[instrument(skip(input))]
async fn validate_input(input: &str) {
    debug!(input_length = input.len(), "Validating input");
}
