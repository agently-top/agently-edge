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

/// Prompt 管理器
pub struct PromptManager {
    messages: Vec<Message>,
    system_template: Option<String>,
    variables: std::collections::HashMap<String, String>,
}

impl PromptManager {
    /// 创建新管理器
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            system_template: None,
            variables: std::collections::HashMap::new(),
        }
    }

    /// 添加系统消息
    pub fn add_system(&mut self, content: &str) {
        self.messages.push(Message {
            role: Role::System,
            content: content.to_string(),
        });
    }

    /// 添加用户消息
    pub fn add_user(&mut self, content: &str) {
        self.messages.push(Message {
            role: Role::User,
            content: content.to_string(),
        });
    }

    /// 添加助手消息
    pub fn add_assistant(&mut self, content: &str) {
        self.messages.push(Message {
            role: Role::Assistant,
            content: content.to_string(),
        });
    }

    /// 设置系统模板
    pub fn set_system_template(&mut self, template: &str) {
        self.system_template = Some(template.to_string());
    }

    /// 添加变量
    pub fn add_variable(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }

    /// 构建消息列表
    pub fn build_messages(&mut self) -> Vec<Message> {
        let mut messages = Vec::new();

        // 处理系统模板
        if let Some(ref template) = self.system_template {
            let mut content = template.clone();
            for (key, value) in &self.variables {
                content = content.replace(&format!("{{{}}}", key), value);
            }
            messages.push(Message {
                role: Role::System,
                content,
            });
        }

        // 添加其他消息
        messages.extend(self.messages.clone());

        messages
    }
}

impl Default for PromptManager {
    fn default() -> Self {
        Self::new()
    }
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
