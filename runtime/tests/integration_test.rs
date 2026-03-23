use agently_edge::{
    agent::Agent,
    config::{AgentConfig, AgentInfo, RuntimeConfig, PromptConfig},
    message_bus::MessageBus,
    tools::{http::HttpTool, shell::ShellTool, ToolRegistry},
};
use std::io::Write;
use tempfile::NamedTempFile;

// Helper function to create temporary config file
fn write_temp_config(content: &str) -> (NamedTempFile, String) {
    let mut temp = NamedTempFile::new().unwrap();
    temp.write_all(content.as_bytes()).unwrap();
    let path = temp.path().to_str().unwrap().to_string();
    (temp, path)
}

#[test]
fn test_full_agent_workflow() {
    // 1. 创建配置文件
    let config_content = r#"
agent:
  id: integration-test
  name: Integration Test Agent
  version: 0.1.0
  description: "Test agent"
runtime:
  model_path: /models/test.gguf
  context_length: 4096
  temperature: 0.7
prompts:
  system: "You are a helpful assistant."
  welcome: "Hello! I'm your agent."
"#;

    let (temp_file, temp_path) = write_temp_config(config_content);

    // 2. 加载配置
    let config = agently_edge::config::load_config(std::path::Path::new(&temp_path))
        .expect("Failed to load config");
    let _ = temp_file;

    // 3. 创建 Agent
    let mut agent = Agent::load(config).expect("Failed to load agent");

    // 4. 启动 Agent
    agent.start().expect("Failed to start agent");
    assert!(agent.is_running());

    // 5. 处理消息
    let response = agent
        .process_message("Hello!")
        .expect("Failed to process message");
    assert!(!response.text.is_empty());

    // 6. 停止 Agent
    agent.stop().expect("Failed to stop agent");
    assert!(!agent.is_running());
}

#[test]
fn test_agent_with_tools() {
    // 创建 Agent 配置
    let config = AgentConfig {
        agent: AgentInfo {
            id: "tool-agent".to_string(),
            name: "Tool Agent".to_string(),
            version: "0.1.0".to_string(),
            description: "Test agent with tools".to_string(),
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

    let mut agent = Agent::load(config).expect("Failed to load agent");
    agent.start().expect("Failed to start agent");

    // 创建工具注册表并注册工具
    let mut registry = ToolRegistry::new();
    registry
        .register(Box::new(HttpTool::new()))
        .expect("Failed to register");
    registry
        .register(Box::new(ShellTool::new()))
        .expect("Failed to register");

    // 执行工具
    let args = serde_json::json!({"command": "echo test"});
    let result = registry.execute("shell", &args).expect("Failed to execute");
    assert!(result.is_string());
}

#[test]
fn test_message_bus_integration() {
    let bus = MessageBus::new();

    // 订阅
    let rx = bus.subscribe("integration-test");

    // 发布
    use agently_edge::message_bus::Message;
    let msg = Message {
        topic: "integration-test".to_string(),
        payload: "Test message".to_string(),
    };
    bus.publish("integration-test", msg)
        .expect("Failed to publish");

    // 接收
    let received = rx
        .recv_timeout(std::time::Duration::from_millis(100))
        .expect("Failed to receive");
    assert_eq!(received.payload, "Test message");
}
