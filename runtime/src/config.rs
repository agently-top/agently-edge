//! 配置加载模块

use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct AgentConfig {
    pub agent: AgentInfo,
    pub runtime: RuntimeConfig,
    pub prompts: PromptConfig,
    #[serde(default)]
    pub logging: Option<LoggingConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AgentInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RuntimeConfig {
    pub model_path: String,
    pub context_length: u32,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    #[serde(default = "default_threads")]
    pub n_threads: u32,
}

fn default_temperature() -> f32 {
    0.7
}

fn default_max_tokens() -> u32 {
    512
}

fn default_threads() -> u32 {
    4
}

#[derive(Debug, Clone, Deserialize)]
pub struct PromptConfig {
    pub system: String,
    pub welcome: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

impl AgentConfig {
    /// 从 YAML 文件加载配置
    pub fn load<P: AsRef<Path>>(path: P) -> Result<AgentConfig> {
        let content = fs::read_to_string(path.as_ref())?;
        let config: AgentConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}

/// 从 YAML 文件加载配置（兼容旧 API）
pub fn load_config<P: AsRef<Path>>(path: P) -> Result<AgentConfig> {
    AgentConfig::load(path)
}
