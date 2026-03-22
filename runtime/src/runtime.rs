//! Runtime 核心模块

use anyhow::Result;

/// 运行时配置
#[derive(Debug)]
pub struct RuntimeConfig {
    pub agent_id: String,
    pub model_path: String,
}

/// 运行时
pub struct Runtime {
    config: RuntimeConfig,
}

impl Runtime {
    /// 创建新运行时
    pub fn new(config: RuntimeConfig) -> Self {
        Self { config }
    }
    
    /// 启动运行时
    pub fn start(&self) -> Result<()> {
        tracing::info!("Starting runtime with config: {:?}", self.config);
        Ok(())
    }
}
