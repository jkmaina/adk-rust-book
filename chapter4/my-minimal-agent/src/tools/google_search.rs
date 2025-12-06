use adk_core::{Result, Tool, ToolContext};
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

/// A very small stub GoogleSearch tool that returns a mocked result.
///
/// This provides a working example so the `search` binary compiles and runs
/// without users needing a real search API key. Replace the body of
/// `execute` with a real search call if you have an API you want to use.
#[derive(Debug, Default)]
pub struct GoogleSearchTool {}

impl GoogleSearchTool {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Tool for GoogleSearchTool {
    fn name(&self) -> &str {
        "google_search"
    }

    fn description(&self) -> &str {
        "Performs a web search (stub implementation, replace with a real API)"
    }

    async fn execute(&self, _ctx: Arc<dyn ToolContext>, args: Value) -> Result<Value> {
        // Read query from args (expected `{ "query": "..." }`)
        let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");

        let mock = serde_json::json!({
            "query": query,
            "top_results": [
                {
                    "title": format!("Mocked result for '{}'", query),
                    "snippet": "This is a mocked search result created for local testing.",
                    "url": "https://example.com/mock"
                }
            ]
        });

        Ok(mock)
    }
}
