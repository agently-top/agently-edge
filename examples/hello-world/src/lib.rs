//! Hello World Agent Library
//!
//! 配置加载和核心功能模块

use serde::Deserialize;
use std::fs;

pub use anyhow;

/// Agent 配置结构
#[derive(Debug, Deserialize)]
pub struct AgentConfig {
    pub agent: AgentInfo,
    pub runtime: RuntimeConfig,
    pub prompts: PromptConfig,
    pub logging: Option<LoggingConfig>,
}

/// Agent 基本信息
#[derive(Debug, Deserialize)]
pub struct AgentInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
}

/// 运行时配置
#[derive(Debug, Deserialize)]
pub struct RuntimeConfig {
    pub model_path: String,
    pub context_length: u32,
    pub temperature: f32,
    pub max_tokens: u32,
    pub n_threads: u32,
}

/// Prompt 配置
#[derive(Debug, Deserialize)]
pub struct PromptConfig {
    pub system: String,
    pub welcome: String,
}

/// 日志配置
#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

/// 加载配置文件
pub fn load_config(path: &str) -> anyhow::Result<AgentConfig> {
    let content = fs::read_to_string(path)?;
    let config: AgentConfig = serde_yaml::from_str(&content)?;
    Ok(config)
}

/// 对话上下文管理
pub struct ConversationContext {
    history: Vec<String>,
    #[allow(dead_code)]
    max_turns: usize,
}

impl ConversationContext {
    pub fn new(max_turns: usize) -> Self {
        ConversationContext {
            history: Vec::new(),
            max_turns,
        }
    }

    pub fn add_user(&mut self, message: String) {
        self.history.push(format!("👤 You: {}", message));
    }

    pub fn add_assistant(&mut self, message: String) {
        self.history.push(format!("🤖 Agent: {}", message));
    }

    pub fn get_history(&self) -> &[String] {
        &self.history
    }
}

/// 检查是否应该退出
pub fn should_exit(input: &str) -> bool {
    let normalized = input.trim().to_lowercase();
    normalized == "quit" || normalized == "exit"
}

/// 构建欢迎信息
pub fn build_welcome(welcome_template: &str) -> String {
    welcome_template.to_string()
}
