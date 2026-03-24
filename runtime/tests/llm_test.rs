use agently_edge::llm::{LLMEngine, Message, Role, ModelConfig, GenerateConfig};

#[tokio::test]
async fn test_llm_mock_load() {
    let mut engine = LLMEngine::mock();
    let result = engine.load_model().await;
    assert!(result.is_ok());
    let info = result.unwrap();
    assert_eq!(info.name, "mock");
}

#[tokio::test]
async fn test_llm_generate() {
    let mut engine = LLMEngine::mock();
    engine.load_model().await.unwrap();

    let config = GenerateConfig::default();
    let result = engine.generate("Hello", &config).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.text.is_empty());
    assert!(response.tokens_generated > 0);
}

#[tokio::test]
async fn test_llm_chat() {
    let mut engine = LLMEngine::mock();
    engine.load_model().await.unwrap();

    let messages = vec![
        Message {
            role: Role::System,
            content: "You are a helpful assistant.".to_string(),
        },
        Message {
            role: Role::User,
            content: "Hello!".to_string(),
        },
    ];

    let config = GenerateConfig::default();
    let response = engine.chat(&messages, &config).await;
    assert!(response.is_ok());
    let result = response.unwrap();
    assert!(!result.text.is_empty());
}

#[tokio::test]
async fn test_tool_call_parser() {
    use agently_edge::llm::ToolCallParser;
    
    // 测试直接 JSON 解析
    let json_response = r#"{"name": "search", "arguments": {"query": "test"}}"#;
    let call = ToolCallParser::parse(json_response);
    assert!(call.is_some());
    let call = call.unwrap();
    assert_eq!(call.name, "search");
    
    // 测试 markdown 代码块解析
    let md_response = r#"Here's the tool call:
```json
{"name": "search", "arguments": {"query": "test"}}
```"#;
    let call = ToolCallParser::parse(md_response);
    assert!(call.is_some());
}

#[test]
fn test_model_config_default() {
    let config = ModelConfig::default();
    assert_eq!(config.n_ctx, 2048);
    assert!(config.n_threads > 0);
}
