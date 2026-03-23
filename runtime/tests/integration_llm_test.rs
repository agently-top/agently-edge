use agently_edge::llm::{LLMEngine, PromptManager, Tool};

#[test]
fn test_full_llm_workflow() {
    // 1. 创建 mock 引擎
    let mut engine = LLMEngine::mock();

    // 2. 创建 Prompt 管理器
    let mut prompt_mgr = PromptManager::new();
    prompt_mgr.add_system("You are a helpful assistant.");
    prompt_mgr.add_user("Hello!");

    let messages = prompt_mgr.build_messages();

    // 3. 对话
    let response = engine.chat(&messages);
    assert!(response.is_ok());
    assert!(!response.unwrap().is_empty());
}

#[test]
fn test_llm_with_tools_workflow() {
    // 1. 创建 mock 引擎
    let mut engine = LLMEngine::mock();

    // 2. 定义工具
    let tools = vec![Tool {
        name: "query_product".to_string(),
        description: "Query product information".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "keyword": {"type": "string"}
            }
        }),
    }];

    // 3. 带工具对话
    let (response, tool_call) = engine
        .chat_with_tools("What products do you have?", &tools)
        .expect("Failed to chat with tools");

    assert!(!response.is_empty());
    assert!(tool_call.is_some());
    assert_eq!(tool_call.unwrap().name, "query_product");
}

#[test]
fn test_conversation_history() {
    let mut engine = LLMEngine::mock();
    let mut prompt_mgr = PromptManager::new();

    // 多轮对话
    prompt_mgr.add_system("You are a helpful assistant.");
    prompt_mgr.add_user("Question 1");
    prompt_mgr.add_assistant("Answer 1");
    prompt_mgr.add_user("Question 2");

    let messages = prompt_mgr.build_messages();
    assert_eq!(messages.len(), 4);

    let response = engine.chat(&messages);
    assert!(response.is_ok());
}
