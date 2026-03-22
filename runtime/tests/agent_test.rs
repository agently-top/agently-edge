use agently_edge::agent::Agent;
use agently_edge::config::{AgentConfig, ModelConfig, PromptConfig};

#[test]
fn test_agent_load() {
    let config = AgentConfig {
        agent_id: "test-agent".to_string(),
        name: "Test Agent".to_string(),
        model: ModelConfig {
            path: "/models/test.gguf".to_string(),
            context_length: 4096,
            temperature: 0.7,
        },
        tools: vec![],
        prompts: PromptConfig {
            system: "You are a helpful assistant.".to_string(),
            greeting: "Hello!".to_string(),
        },
    };

    let agent = Agent::load(config).expect("Failed to load agent");
    assert_eq!(agent.id(), "test-agent");
    assert_eq!(agent.name(), "Test Agent");
}

#[test]
fn test_agent_start_stop() {
    let config = create_test_config();
    let mut agent = Agent::load(config).expect("Failed to load agent");

    // 启动 Agent
    agent.start().expect("Failed to start agent");
    assert!(agent.is_running());

    // 停止 Agent
    agent.stop().expect("Failed to stop agent");
    assert!(!agent.is_running());
}

#[test]
fn test_agent_process_message() {
    let config = create_test_config();
    let mut agent = Agent::load(config).expect("Failed to load agent");
    agent.start().expect("Failed to start agent");

    // 处理消息
    let response = agent
        .process_message("Hello!")
        .expect("Failed to process message");
    assert!(!response.text.is_empty());
}

fn create_test_config() -> AgentConfig {
    AgentConfig {
        agent_id: "test-agent".to_string(),
        name: "Test Agent".to_string(),
        model: ModelConfig {
            path: "/models/test.gguf".to_string(),
            context_length: 4096,
            temperature: 0.7,
        },
        tools: vec![],
        prompts: PromptConfig {
            system: "You are a helpful assistant.".to_string(),
            greeting: "Hello!".to_string(),
        },
    }
}
