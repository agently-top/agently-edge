use agently_edge::llm::{LLMEngine, Tool, ToolCall, GenerateConfig};
use serde_json::json;

#[test]
fn test_tool_call_parsing() {
    use agently_edge::llm::ToolCallParser;
    
    let json_response = r#"{"name": "query_product", "arguments": {"keyword": "phone"}}"#;
    let call = ToolCallParser::parse(json_response);
    assert!(call.is_some());
    let call = call.unwrap();
    assert_eq!(call.name, "query_product");
    assert!(call.arguments.get("keyword").is_some());
}

#[test]
fn test_tool_definition() {
    let tool = Tool {
        name: "query_product".to_string(),
        description: "Query product information".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "keyword": {"type": "string"}
            }
        }),
    };

    assert_eq!(tool.name, "query_product");
}

#[tokio::test]
async fn test_llm_with_tools() {
    let mut engine = LLMEngine::mock();
    engine.load_model().await.unwrap();

    let tools = vec![Tool {
        name: "test_tool".to_string(),
        description: "A test tool".to_string(),
        parameters: json!({}),
    }];

    let config = GenerateConfig::default();
    let result = engine.chat_with_tools(&[], &tools, &config).await;
    assert!(result.is_ok());
}
