//! LLM 推理引擎 - llama.cpp 集成 (简化 MVP 版)

use anyhow::Result;
use serde_json::Value as JsonValue;
use std::path::Path;
use std::time::Instant;

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

/// Tool 调用
#[derive(Debug, Clone)]
pub struct ToolCall {
    pub name: String,
    pub arguments: JsonValue,
    pub call_id: String,
}

/// Tool 定义
#[derive(Debug, Clone)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: JsonValue,
}

/// 模型配置
#[derive(Debug, Clone)]
pub struct ModelConfig {
    pub model_path: String,
    pub n_ctx: u32,
    pub n_threads: u32,
    pub n_gpu_layers: u32,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            model_path: String::new(),
            n_ctx: 2048,
            n_threads: num_cpus::get() as u32,
            n_gpu_layers: 0,
        }
    }
}

/// 模型信息
#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub size_mb: f64,
    pub n_params: u64,
    pub n_ctx: u32,
}

/// 生成配置
#[derive(Debug, Clone)]
pub struct GenerateConfig {
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
    pub stop_sequences: Vec<String>,
}

impl Default for GenerateConfig {
    fn default() -> Self {
        Self {
            max_tokens: 512,
            temperature: 0.7,
            top_p: 0.9,
            stop_sequences: vec![],
        }
    }
}

/// 生成结果
#[derive(Debug, Clone)]
pub struct GenerationResult {
    pub text: String,
    pub tokens_generated: u32,
    pub duration_ms: u64,
    pub tokens_per_second: f64,
}

/// 性能统计
#[derive(Debug, Clone, Default)]
pub struct InferenceStats {
    pub tokens_per_second: f64,
    pub time_to_first_token_ms: u64,
    pub memory_usage_mb: f64,
    pub total_tokens_generated: u64,
}

/// LLM 引擎
pub struct LLMEngine {
    config: ModelConfig,
    stats: InferenceStats,
    is_mock: bool,
    model_loaded: bool,
}

impl LLMEngine {
    /// 创建新引擎（未加载模型）
    pub fn new(config: ModelConfig) -> Result<Self> {
        Ok(Self {
            config,
            stats: InferenceStats::default(),
            is_mock: false,
            model_loaded: false,
        })
    }

    /// 创建 mock 引擎（测试用）
    pub fn mock() -> Self {
        Self {
            config: ModelConfig::default(),
            stats: InferenceStats::default(),
            is_mock: true,
            model_loaded: false,
        }
    }

    /// 加载 GGUF 模型
    pub async fn load_model(&mut self) -> Result<ModelInfo> {
        if self.is_mock {
            self.model_loaded = true;
            return Ok(ModelInfo {
                name: "mock".to_string(),
                size_mb: 0.0,
                n_params: 0,
                n_ctx: self.config.n_ctx,
            });
        }

        let path = Path::new(&self.config.model_path);
        if !path.exists() {
            anyhow::bail!("Model file not found: {}", self.config.model_path);
        }

        let file_size = path.metadata()?.len() as f64 / 1024.0 / 1024.0;

        // TODO: 实际加载 llama.cpp 模型
        // 目前 MVP 阶段仅验证文件存在
        
        self.model_loaded = true;

        Ok(ModelInfo {
            name: path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            size_mb: file_size,
            n_params: 7_000_000_000, // 假设 7B 模型
            n_ctx: self.config.n_ctx,
        })
    }

    /// 卸载模型
    pub fn unload_model(&mut self) {
        self.model_loaded = false;
    }

    /// 生成文本
    pub async fn generate(&mut self, prompt: &str, config: &GenerateConfig) -> Result<GenerationResult> {
        if !self.model_loaded {
            anyhow::bail!("Model not loaded");
        }

        if self.is_mock {
            return Ok(self.mock_generate(prompt, config));
        }

        // TODO: 实际 llama.cpp 推理
        // 目前返回 mock 响应
        
        let start = Instant::now();
        let text = format!("Response to: {}", prompt);
        let tokens = text.len() as u32 / 4;
        let duration = start.elapsed();

        let tokens_per_second = if duration.as_secs_f64() > 0.0 {
            tokens as f64 / duration.as_secs_f64()
        } else {
            0.0
        };

        self.stats.tokens_per_second = tokens_per_second;
        self.stats.total_tokens_generated += tokens as u64;

        Ok(GenerationResult {
            text,
            tokens_generated: tokens,
            duration_ms: duration.as_millis() as u64,
            tokens_per_second,
        })
    }

    /// Mock 生成（测试用）
    fn mock_generate(&self, prompt: &str, config: &GenerateConfig) -> GenerationResult {
        let text = format!("Mock response to: {} (max_tokens: {})", prompt, config.max_tokens);
        let tokens = text.len() as u32 / 4;
        GenerationResult {
            text,
            tokens_generated: tokens,
            duration_ms: 100,
            tokens_per_second: 10.0,
        }
    }

    /// 对话
    pub async fn chat(&mut self, messages: &[Message], config: &GenerateConfig) -> Result<GenerationResult> {
        let prompt = self.format_chat(messages);
        self.generate(&prompt, config).await
    }

    /// 格式化对话为 prompt
    fn format_chat(&self, messages: &[Message]) -> String {
        let mut prompt = String::new();
        
        for msg in messages {
            match msg.role {
                Role::System => prompt.push_str(&format!("System: {}\n", msg.content)),
                Role::User => prompt.push_str(&format!("User: {}\n", msg.content)),
                Role::Assistant => prompt.push_str(&format!("Assistant: {}\n", msg.content)),
            }
        }
        
        prompt.push_str("Assistant: ");
        prompt
    }

    /// 带工具的对话
    pub async fn chat_with_tools(
        &mut self,
        messages: &[Message],
        tools: &[Tool],
        config: &GenerateConfig,
    ) -> Result<(String, Option<ToolCall>)> {
        if self.is_mock {
            return Ok(self.mock_chat_with_tools(messages, tools));
        }

        // 构建带 tool definitions 的 prompt
        let tools_json = serde_json::to_string_pretty(&tools.iter().map(|t| {
            json!({
                "type": "function",
                "function": {
                    "name": t.name,
                    "description": t.description,
                    "parameters": t.parameters
                }
            })
        }).collect::<Vec<_>>())?;

        let system_prompt = format!(
            "You are a helpful assistant with access to these tools:\n{}\n\n\
             If you need to use a tool, respond with JSON: {{\"name\": \"tool_name\", \"arguments\": {{...}}}}\n\
             If no tool is needed, just respond normally.",
            tools_json
        );

        let mut all_messages = vec![Message {
            role: Role::System,
            content: system_prompt,
        }];
        all_messages.extend_from_slice(messages);

        let result = self.chat(&all_messages, config).await?;
        
        // 尝试解析 tool call
        if let Some(call) = ToolCallParser::parse(&result.text) {
            Ok((result.text, Some(call)))
        } else {
            Ok((result.text, None))
        }
    }

    /// Mock 带工具对话
    fn mock_chat_with_tools(&self, messages: &[Message], tools: &[Tool]) -> (String, Option<ToolCall>) {
        let last_user_msg = messages
            .iter()
            .rev()
            .find(|m| m.role == Role::User)
            .map(|m| m.content.as_str())
            .unwrap_or("Hello");

        let lower_input = last_user_msg.to_lowercase();
        if lower_input.contains("what") || lower_input.contains("how") {
            let tool_call = ToolCall {
                name: tools.first().map(|t| t.name.clone()).unwrap_or_else(|| "unknown".to_string()),
                arguments: JsonValue::String(last_user_msg.to_string()),
                call_id: uuid::Uuid::new_v4().to_string(),
            };
            return ("I'll help you with that.".to_string(), Some(tool_call));
        }
        ("I understand.".to_string(), None)
    }

    /// 获取统计信息
    pub fn get_stats(&self) -> &InferenceStats {
        &self.stats
    }
    
    /// 检查模型是否已加载
    pub fn is_loaded(&self) -> bool {
        self.model_loaded
    }
}

/// Tool Call 解析器
pub struct ToolCallParser;

impl ToolCallParser {
    /// 解析 tool call
    pub fn parse(response: &str) -> Option<ToolCall> {
        // 尝试直接解析 JSON
        if let Ok(value) = serde_json::from_str::<JsonValue>(response) {
            if let Some(obj) = value.as_object() {
                if let (Some(name), Some(args)) = (
                    obj.get("name").and_then(|v| v.as_str()),
                    obj.get("arguments"),
                ) {
                    return Some(ToolCall {
                        name: name.to_string(),
                        arguments: args.clone(),
                        call_id: uuid::Uuid::new_v4().to_string(),
                    });
                }
            }
        }

        // 尝试解析 markdown 代码块
        if let Some(start) = response.find("```json") {
            if let Some(end) = response[start..].find("```") {
                let json_str = &response[start + 7..start + end];
                if let Ok(value) = serde_json::from_str::<JsonValue>(json_str) {
                    if let Some(obj) = value.as_object() {
                        if let (Some(name), Some(args)) = (
                            obj.get("name").and_then(|v| v.as_str()),
                            obj.get("arguments"),
                        ) {
                            return Some(ToolCall {
                                name: name.to_string(),
                                arguments: args.clone(),
                                call_id: uuid::Uuid::new_v4().to_string(),
                            });
                        }
                    }
                }
            }
        }

        None
    }

    /// 格式化 tool result
    pub fn format_result(call_id: &str, result: &JsonValue) -> String {
        format!(
            "Tool result for call {}:\n```json\n{}\n```",
            call_id,
            serde_json::to_string_pretty(result).unwrap_or_default()
        )
    }
}

// 辅助宏
#[macro_export]
macro_rules! json {
    ($($tt:tt)*) => {
        serde_json::json!($($tt)*)
    };
}

use serde_json::json;
