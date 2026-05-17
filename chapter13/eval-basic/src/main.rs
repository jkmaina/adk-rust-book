use adk_eval::schema::ContentData;
use adk_eval::{EvalCase, IntermediateData, TestFile, ToolUse, Turn};
use serde_json::json;

fn main() -> anyhow::Result<()> {
    println!("=== Evaluation Doc-Test: Basic ===\n");

    let test_json = r#"{
      "eval_set_id": "weather_agent_tests",
      "name": "Weather Agent Tests",
      "description": "Test weather agent functionality",
      "eval_cases": [
        {
          "eval_id": "test_current_weather",
          "conversation": [
            {
              "invocation_id": "inv_001",
              "user_content": {
                "parts": [{"text": "What's the weather in NYC?"}],
                "role": "user"
              },
              "final_response": {
                "parts": [{"text": "The weather in NYC is 65°F and sunny."}],
                "role": "model"
              },
              "intermediate_data": {
                "tool_uses": [
                  {
                    "name": "get_weather",
                    "args": {"location": "NYC"}
                  }
                ]
              }
            }
          ]
        }
      ]
    }"#;

    let test_file: TestFile = serde_json::from_str(test_json)?;
    println!("Parsed test file: {}", test_file.name);
    println!("  eval_set_id: {}", test_file.eval_set_id);
    println!("  cases: {}", test_file.eval_cases.len());

    let case = &test_file.eval_cases[0];
    assert_eq!(case.eval_id, "test_current_weather");
    println!("Verified eval_id");

    let turn = &case.conversation[0];
    assert_eq!(turn.invocation_id, "inv_001");
    println!("Verified invocation_id");

    let tool_uses = turn.intermediate_data.as_ref().expect("tool uses");
    assert_eq!(tool_uses.tool_uses[0].name, "get_weather");
    println!("Verified tool_uses");

    let programmatic_file = TestFile {
        eval_set_id: "my_tests".to_string(),
        name: "My Test Suite".to_string(),
        description: "Tests created programmatically".to_string(),
        eval_cases: vec![EvalCase {
            eval_id: "test_1".to_string(),
            description: "Simple test".to_string(),
            conversation: vec![Turn {
                invocation_id: "turn_1".to_string(),
                user_content: ContentData::text("Hello"),
                final_response: Some(ContentData::model_response("Hi there!")),
                intermediate_data: Some(IntermediateData {
                    tool_uses: vec![ToolUse::new("greet").with_args(json!({"name": "user"}))],
                    ..Default::default()
                }),
            }],
            session_input: Default::default(),
            tags: vec!["basic".to_string()],
        }],
    };

    let json_output = serde_json::to_string_pretty(&programmatic_file)?;
    let _parsed_back: TestFile = serde_json::from_str(&json_output)?;
    println!("Verified programmatic creation and round-trip");

    println!("\nAll basic evaluation checks passed.");
    Ok(())
}
