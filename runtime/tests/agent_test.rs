use agently_edge::agent::Agent;
use agently_edge::config::{AgentConfig, AgentInfo, PromptConfig, RuntimeConfig};

#[test]
fn test_agent_load() {
    let config = AgentConfig {
        agent: AgentInfo {
            id: "test-agent".to_string(),
            name: "Test Agent".to_string(),
            version: "0.1.0".to_string(),
            description: "Test agent".to_string(),
        },
        runtime: RuntimeConfig {
            model_path: "/models/test.gguf".to_string(),
            context_length: 4096,
            temperature: 0.7,
            max_tokens: 512,
            n_threads: 4,
        },
        prompts: PromptConfig {
            system: "You are a helpful assistant.".to_string(),
            welcome: "Hello!".to_string(),
        },
        logging: None,
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
        agent: AgentInfo {
            id: "test-agent".to_string(),
            name: "Test Agent".to_string(),
            version: "0.1.0".to_string(),
            description: "Test agent".to_string(),
        },
        runtime: RuntimeConfig {
            model_path: "/models/test.gguf".to_string(),
            context_length: 4096,
            temperature: 0.7,
            max_tokens: 512,
            n_threads: 4,
        },
        prompts: PromptConfig {
            system: "You are a helpful assistant.".to_string(),
            welcome: "Hello!".to_string(),
        },
        logging: None,
    }
}
