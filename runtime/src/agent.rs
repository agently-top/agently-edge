//! Agent 管理模块

/// Agent 配置
pub struct AgentConfig {
    pub id: String,
    pub name: String,
}

/// Agent
pub struct Agent {
    config: AgentConfig,
}

impl Agent {
    /// 创建新 Agent
    pub fn new(config: AgentConfig) -> Self {
        Self { config }
    }

    /// 获取 Agent ID
    pub fn id(&self) -> &str {
        &self.config.id
    }
}
