//! 配置加载模块

use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct AgentConfig {
    pub agent_id: String,
    pub name: String,
    pub model: ModelConfig,
    #[serde(default)]
    pub tools: Vec<ToolConfig>,
    pub prompts: PromptConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelConfig {
    pub path: String,
    pub context_length: usize,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
}

fn default_temperature() -> f32 {
    0.7
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToolConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub tool_type: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PromptConfig {
    pub system: String,
    pub greeting: String,
}

/// 从 YAML 文件加载配置
pub fn load_config<P: AsRef<Path>>(path: P) -> Result<AgentConfig> {
    let content = fs::read_to_string(path.as_ref())?;
    let config: AgentConfig = serde_yaml::from_str(&content)?;
    Ok(config)
}
