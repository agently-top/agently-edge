use agently_edge::llm::{LLMEngine, Message, Role, Tool, GenerateConfig};
use serde_json::json;

#[tokio::test]
async fn test_full_llm_workflow() {
    // 1. 创建 mock 引擎
    let mut engine = LLMEngine::mock();
    engine.load_model().await.unwrap();

    // 2. 创建消息
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

    // 3. 对话
    let config = GenerateConfig::default();
    let response = engine.chat(&messages, &config).await;
    assert!(response.is_ok());
    assert!(!response.unwrap().text.is_empty());
}

#[tokio::test]
async fn test_llm_with_tools_workflow() {
    // 1. 创建 mock 引擎
    let mut engine = LLMEngine::mock();
    engine.load_model().await.unwrap();

    // 2. 定义工具
    let tools = vec![Tool {
        name: "query_product".to_string(),
        description: "Query product information".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "keyword": {"type": "string"}
            }
        }),
    }];

    // 3. 带工具对话
    let messages = vec![Message {
        role: Role::User,
        content: "What products do you have?".to_string(),
    }];
    
    let config = GenerateConfig::default();
    let (response, tool_call) = engine
        .chat_with_tools(&messages, &tools, &config)
        .await
        .expect("Failed to chat with tools");

    assert!(!response.is_empty());
    // Mock 模式会返回 tool call
    assert!(tool_call.is_some());
}

#[tokio::test]
async fn test_conversation_history() {
    let mut engine = LLMEngine::mock();
    engine.load_model().await.unwrap();

    // 多轮对话
    let messages = vec![
        Message {
            role: Role::System,
            content: "You are a helpful assistant.".to_string(),
        },
        Message {
            role: Role::User,
            content: "Question 1".to_string(),
        },
        Message {
            role: Role::Assistant,
            content: "Answer 1".to_string(),
        },
        Message {
            role: Role::User,
            content: "Question 2".to_string(),
        },
    ];
    
    assert_eq!(messages.len(), 4);

    let config = GenerateConfig::default();
    let response = engine.chat(&messages, &config).await;
    assert!(response.is_ok());
}
