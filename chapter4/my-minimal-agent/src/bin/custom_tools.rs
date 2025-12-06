use serde_json::json;
use std::sync::Arc;
use adk_rust::prelude::*;
use adk_rust::Launcher;
 
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let api_key = std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY environment variable not set");
    let model = GeminiModel::new(&api_key, "gemini-2.0-flash-exp")?;
    
    // Create custom calculator tool
    let calculator = FunctionTool::new(
        "calculate",
        "Perform precise arithmetic calculations",
        |_ctx, args| async move {
            // Normalize incoming args — some callers put the real object under
            // `arguments` or `args`, or pass a JSON string. Accept those shapes.
            let mut obj = match &args {
                serde_json::Value::Object(_) => args.clone(),
                serde_json::Value::String(s) => serde_json::from_str::<serde_json::Value>(s)
                    .unwrap_or_else(|_| json!({"expression": s})),
                _ => args.clone(),
            };

            // Unwrap nested wrappers like {"arguments": {...}} or {"args": {...}}
            if let Some(wrapped) = obj.get("arguments").or_else(|| obj.get("args")) {
                obj = wrapped.clone();
            }

            // Extract numbers (accept numeric values OR numeric strings)
            let a = obj.get("a").and_then(|v| v.as_f64()).or_else(|| obj.get("a").and_then(|v| v.as_str()).and_then(|s| s.parse::<f64>().ok())).unwrap_or(0.0);
            let b = obj.get("b").and_then(|v| v.as_f64()).or_else(|| obj.get("b").and_then(|v| v.as_str()).and_then(|s| s.parse::<f64>().ok())).unwrap_or(0.0);
            let op = obj.get("operation").and_then(|v| v.as_str()).unwrap_or("add");
            
            let result = match op {
                "add" => a + b,
                "subtract" => a - b,
                "multiply" => a * b,
                "divide" if b != 0.0 => a / b,
                _ => 0.0,
            };
            
            Ok(json!(
                {
                "result": result,
                "expression": format!("{} {} {} = {}", a, op, b, result)
            }))
        },
    );
    
    // Agent with calculator
    let agent = LlmAgentBuilder::new("math_assistant")
        .description("A math assistant with precise calculations")
        .instruction("You help with math. Use the calculate tool for precise arithmetic.")
        .model(Arc::new(model))
        .tool(Arc::new(calculator))
        .build()?;
    
    Launcher::new(Arc::new(agent)).run().await?;
    Ok(())
}
