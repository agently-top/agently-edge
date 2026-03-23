//! Agent 管理模块

use crate::config::AgentConfig;
use anyhow::Result;

/// Agent 状态
#[derive(Debug, Clone, PartialEq)]
pub enum AgentState {
    Loading,
    Running,
    Paused,
    Stopped,
}

/// Agent 响应
#[derive(Debug, Clone)]
pub struct AgentResponse {
    pub text: String,
}

/// Agent
pub struct Agent {
    id: String,
    name: String,
    config: AgentConfig,
    state: AgentState,
}

impl Agent {
    /// 加载 Agent
    pub fn load(config: AgentConfig) -> Result<Self> {
        Ok(Self {
            id: config.agent.id.clone(),
            name: config.agent.name.clone(),
            config,
            state: AgentState::Loading,
        })
    }

    /// 获取 Agent ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// 获取 Agent 名称
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 启动 Agent
    pub fn start(&mut self) -> Result<()> {
        self.state = AgentState::Running;
        Ok(())
    }

    /// 停止 Agent
    pub fn stop(&mut self) -> Result<()> {
        self.state = AgentState::Stopped;
        Ok(())
    }

    /// 检查是否运行中
    pub fn is_running(&self) -> bool {
        self.state == AgentState::Running
    }

    /// 处理消息
    pub fn process_message(&mut self, message: &str) -> Result<AgentResponse> {
        // MVP 阶段：简单返回 welcome
        let response = if message == "Hello!" || message.starts_with("你好") {
            self.config.prompts.welcome.clone()
        } else {
            "I'm a simple agent. LLM integration coming soon.".to_string()
        };

        Ok(AgentResponse { text: response })
    }
}
