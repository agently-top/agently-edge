//! LLM 推理引擎

use anyhow::Result;

/// 消息角色
#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    System,
    User,
    Assistant,
}

/// 消息
#[derive(Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

/// LLM 引擎
pub struct LLMEngine {
    #[allow(dead_code)]
    model_path: String,
    is_mock: bool,
}

impl LLMEngine {
    /// 加载模型
    pub fn load(model_path: &str) -> Result<Self> {
        // MVP 阶段：检查文件是否存在
        let path = std::path::Path::new(model_path);
        if !path.exists() {
            return Err(anyhow::anyhow!("Model file not found: {}", model_path));
        }

        Ok(Self {
            model_path: model_path.to_string(),
            is_mock: false,
        })
    }

    /// 创建 mock 引擎（测试用）
    pub fn mock() -> Self {
        Self {
            model_path: String::new(),
            is_mock: true,
        }
    }

    /// 生成文本
    pub fn generate(&mut self, prompt: &str, max_tokens: usize) -> Result<String> {
        if self.is_mock {
            // MVP 阶段：返回 mock 响应
            return Ok(format!(
                "Mock response to: {} (max_tokens: {})",
                prompt, max_tokens
            ));
        }

        // TODO: 集成 llama.cpp
        Ok("Response from llama.cpp".to_string())
    }

    /// 对话
    pub fn chat(&mut self, messages: &[Message]) -> Result<String> {
        if self.is_mock {
            // MVP 阶段：返回 mock 响应
            let last_user_msg = messages
                .iter()
                .rev()
                .find(|m| m.role == Role::User)
                .map(|m| m.content.as_str())
                .unwrap_or("Hello");
            return Ok(format!("Mock response to: {}", last_user_msg));
        }

        // TODO: 集成 llama.cpp
        Ok("Response from llama.cpp".to_string())
    }
}
