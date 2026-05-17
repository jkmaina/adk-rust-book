use adk_telemetry::{
    Span, add_context_attributes, agent_run_span, callback_span, info, init_telemetry,
    model_call_span, tool_execute_span,
};

#[tokio::main]
async fn main() {
    init_telemetry("chapter15-telemetry-spans").expect("failed to init telemetry");

    println!("Telemetry Spans Example");
    println!("=======================\n");

    println!("1. Agent execution span:");
    {
        let span = agent_run_span("support_agent", "inv-abc-123");
        let _enter = span.enter();
        info!("Agent starting execution");
        info!("Agent completed");
    }

    println!("\n2. Model call span:");
    {
        let span = model_call_span("gemini-3.1-flash-lite-preview");
        let _enter = span.enter();
        info!("Calling LLM");
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        info!("LLM response received");
    }

    println!("\n3. Tool execution span:");
    {
        let span = tool_execute_span("weather_tool");
        let _enter = span.enter();
        info!(location = "Tokyo", "Fetching weather data");
        info!(temp = 22, condition = "sunny", "Weather retrieved");
    }

    println!("\n4. Callback span:");
    {
        let span = callback_span("before_model");
        let _enter = span.enter();
        info!("Executing before_model callback");
    }

    println!("\n5. Context attributes:");
    {
        let span = tracing::info_span!(
            "custom_operation",
            user.id = tracing::field::Empty,
            session.id = tracing::field::Empty
        );
        let _enter = span.enter();
        add_context_attributes("user-456", "sess-789");
        info!("Operation with context");
    }

    println!("\n6. Dynamic span attributes:");
    {
        let span = tracing::info_span!(
            "data_processing",
            operation.r#type = "batch",
            result.count = tracing::field::Empty
        );
        let _enter = span.enter();
        info!("Processing data");
        let count = 42;
        Span::current().record("result.count", count);
        info!(count = count, "Processing complete");
    }

    println!("\nAll span examples demonstrated.");
}
