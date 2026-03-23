use agently_edge::config;
use std::io::Write;

#[test]
fn test_load_valid_config() {
    // 创建一个临时 YAML 配置文件
    let config_content = r#"
agent:
  id: test-agent
  name: Test Agent
  version: 0.1.0
  description: "Test agent"
runtime:
  model_path: /models/test.gguf
  context_length: 4096
  temperature: 0.7
prompts:
  system: "You are a helpful assistant."
  welcome: "Hello!"
"#;

    let temp_file = tempfile::NamedTempFile::new().unwrap();
    temp_file
        .as_file()
        .write_all(config_content.as_bytes())
        .unwrap();
    let temp_path = temp_file.path().to_str().unwrap();

    // 加载配置
    let config = config::load_config(temp_path).expect("Failed to load config");

    // 验证配置
    assert_eq!(config.agent.id, "test-agent");
    assert_eq!(config.agent.name, "Test Agent");
    assert_eq!(config.runtime.model_path, "/models/test.gguf");
    assert_eq!(config.runtime.context_length, 4096);
}

#[test]
fn test_load_invalid_config() {
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    temp_file
        .as_file()
        .write_all(b"invalid yaml content")
        .unwrap();
    let temp_path = temp_file.path().to_str().unwrap();

    let result = config::load_config(temp_path);
    assert!(result.is_err());
}

#[test]
fn test_load_nonexistent_file() {
    let result = config::load_config("/nonexistent/path.yaml");
    assert!(result.is_err());
}
