use agently_edge::llm::{LLMEngine, Message, Role};

#[test]
fn test_llm_engine_load() {
    // MVP 阶段：使用空模型路径测试接口
    let engine = LLMEngine::load("/models/test.gguf");
    // 预期失败（模型文件不存在）
    assert!(engine.is_err());
}

#[test]
fn test_llm_generate() {
    let mut engine = create_test_engine();

    let result = engine.generate("Hello", 50);
    assert!(result.is_ok());
    assert!(!result.unwrap().is_empty());
}

#[test]
fn test_llm_chat() {
    let mut engine = create_test_engine();

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

    let response = engine.chat(&messages);
    assert!(response.is_ok());
    assert!(!response.unwrap().is_empty());
}

fn create_test_engine() -> LLMEngine {
    // MVP 阶段：返回 mock 引擎
    LLMEngine::mock()
}
