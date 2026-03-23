//! 配置加载测试 - TDD 流程

use hello_world_agent::load_config;

/// 测试 1: 有效配置加载
#[test]
fn test_load_valid_config() {
    // 使用 agent.yaml 的路径
    let config = load_config("agent.yaml").expect("Failed to load config");
    assert_eq!(config.agent.id, "hello-world");
    assert_eq!(config.agent.name, "Hello World Agent");
}

/// 测试 2: 缺失文件错误
#[test]
fn test_load_missing_file() {
    let result = load_config("nonexistent.yaml");
    assert!(result.is_err());
}

/// 测试 3: 配置字段验证
#[test]
fn test_config_fields() {
    let config = load_config("agent.yaml").unwrap();
    assert_eq!(config.runtime.context_length, 4096);
    assert_eq!(config.runtime.temperature, 0.7);
    assert!(!config.prompts.system.is_empty());
}
