use agently_edge::llm::{LLMEngine, Tool, ToolCall};

#[test]
fn test_tool_call_parsing() {
    let tool_call = ToolCall {
        name: "query_product".to_string(),
        arguments: serde_json::json!({"keyword": "phone"}),
    };

    assert_eq!(tool_call.name, "query_product");
    assert!(tool_call.arguments.get("keyword").is_some());
}

#[test]
fn test_tool_definition() {
    let tool = Tool {
        name: "query_product".to_string(),
        description: "Query product information".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "keyword": {"type": "string"}
            }
        }),
    };

    assert_eq!(tool.name, "query_product");
}

#[test]
fn test_llm_with_tools() {
    let mut engine = LLMEngine::mock();

    let tools = vec![Tool {
        name: "test_tool".to_string(),
        description: "A test tool".to_string(),
        parameters: serde_json::json!({}),
    }];

    let result = engine.chat_with_tools("What products do you have?", &tools);
    assert!(result.is_ok());
}
