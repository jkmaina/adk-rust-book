use adk_rust::graph::{AgentNode, END, ExecutionConfig, NodeOutput, START, StateGraph};
use adk_rust::prelude::*;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

const MODEL_NAME: &str = "gemini-3.1-flash-lite-preview";

fn extract_text(events: &[Event]) -> String {
    events
        .iter()
        .filter_map(|event| event.content())
        .flat_map(|content| content.parts.iter())
        .filter_map(|part| part.text())
        .collect::<Vec<_>>()
        .join("")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let model = Arc::new(GeminiModel::new(&api_key, MODEL_NAME)?);

    println!("=== Supervisor Routing: Task Delegation ===\n");

    let supervisor = Arc::new(
        LlmAgentBuilder::new("supervisor")
            .model(model.clone())
            .instruction(
                "You are a task supervisor. Route to researcher for research tasks, writer for content tasks, coder for technical tasks, or done if the task is complete. Reply with only one of: researcher, writer, coder, done.",
            )
            .build()?,
    ) as Arc<dyn Agent>;

    let researcher = Arc::new(
        LlmAgentBuilder::new("researcher")
            .model(model.clone())
            .instruction(
                "You are a research specialist. Provide concise findings in under 3 sentences.",
            )
            .build()?,
    ) as Arc<dyn Agent>;

    let writer = Arc::new(
        LlmAgentBuilder::new("writer")
            .model(model.clone())
            .instruction("You are a writing specialist. Produce clear and engaging prose in under 3 sentences.")
            .build()?,
    ) as Arc<dyn Agent>;

    let coder = Arc::new(
        LlmAgentBuilder::new("coder")
            .model(model)
            .instruction(
                "You are a coding specialist. Provide technical solutions in under 3 sentences.",
            )
            .build()?,
    ) as Arc<dyn Agent>;

    let supervisor_node = AgentNode::new(supervisor)
        .with_input_mapper(|state| {
            let task = state.get("task").and_then(|value| value.as_str()).unwrap_or("");
            let history = state
                .get("work_done")
                .and_then(|value| value.as_str())
                .unwrap_or("");

            let prompt = if history.is_empty() {
                format!("Task: {task}")
            } else {
                format!(
                    "Task: {task}\nWork done so far: {history}\nReply with only: researcher, writer, coder, or done."
                )
            };

            Content::new("user").with_text(&prompt)
        })
        .with_output_mapper(|events| {
            let mut updates = HashMap::new();
            let text = extract_text(events).to_lowercase();
            let next = if text.contains("researcher") {
                "researcher"
            } else if text.contains("writer") {
                "writer"
            } else if text.contains("coder") {
                "coder"
            } else {
                "done"
            };

            println!("Supervisor -> {next}");
            updates.insert("next_agent".to_string(), json!(next));
            updates
        });

    let make_specialist_node = |agent: Arc<dyn Agent>, label: &'static str| {
        AgentNode::new(agent)
            .with_input_mapper(|state| {
                let task = state
                    .get("task")
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                Content::new("user").with_text(task)
            })
            .with_output_mapper(move |events| {
                let mut updates = HashMap::new();
                let text = extract_text(events);
                if !text.is_empty() {
                    let preview = text.chars().take(100).collect::<String>();
                    println!("{label} result: {preview}");
                    updates.insert("work_done".to_string(), json!(text));
                }
                updates
            })
    };

    let researcher_node = make_specialist_node(researcher, "researcher");
    let writer_node = make_specialist_node(writer, "writer");
    let coder_node = make_specialist_node(coder, "coder");

    let graph = StateGraph::with_channels(&["task", "next_agent", "work_done", "iteration"])
        .add_node(supervisor_node)
        .add_node(researcher_node)
        .add_node(writer_node)
        .add_node(coder_node)
        .add_node_fn("counter", |ctx| async move {
            let current = ctx
                .get("iteration")
                .and_then(|value| value.as_i64())
                .unwrap_or(0);
            Ok(NodeOutput::new().with_update("iteration", json!(current + 1)))
        })
        .add_edge(START, "counter")
        .add_edge("counter", "supervisor")
        .add_conditional_edges(
            "supervisor",
            |state| {
                let next = state
                    .get("next_agent")
                    .and_then(|value| value.as_str())
                    .unwrap_or("done");
                let iteration = state
                    .get("iteration")
                    .and_then(|value| value.as_i64())
                    .unwrap_or(0);

                if iteration >= 3 {
                    END.to_string()
                } else {
                    next.to_string()
                }
            },
            [
                ("researcher", "researcher"),
                ("writer", "writer"),
                ("coder", "coder"),
                ("done", END),
                (END, END),
            ],
        )
        .add_edge("researcher", "counter")
        .add_edge("writer", "counter")
        .add_edge("coder", "counter")
        .compile()?;

    let mut input = HashMap::new();
    input.insert(
        "task".to_string(),
        json!("Research the benefits of WebAssembly and write a brief summary."),
    );

    let result = graph
        .invoke(
            input,
            ExecutionConfig::new("task-1").with_recursion_limit(10),
        )
        .await?;

    let final_text = result
        .get("work_done")
        .and_then(|value| value.as_str())
        .unwrap_or("");
    println!("\nFinal: {final_text}");
    Ok(())
}
