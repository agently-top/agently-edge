# Functional Specifications: llama.cpp 集成

**Task**: 1.6 - llama.cpp 完整集成  
**版本**: 1.0  
**日期**: 2026-03-24

---

## 1. 模型加载

### 1.1 GGUF 模型加载

**功能**: 加载 GGUF 格式的量化模型

**输入**:
```rust
pub struct ModelConfig {
    pub model_path: String,      // GGUF 文件路径
    pub n_ctx: u32,              // 上下文长度 (默认 2048)
    pub n_threads: u32,          // 推理线程数 (默认 CPU 核心数)
    pub n_gpu_layers: u32,       // GPU 卸载层数 (默认 0)
}
```

**输出**:
```rust
pub enum ModelLoadResult {
    Success { model_info: ModelInfo },
    Error { message: String },
}

pub struct ModelInfo {
    pub name: String,
    pub size_mb: f64,
    pub n_params: u64,
    pub n_ctx: u32,
}
```

**行为**:
- 检查文件是否存在
- 验证 GGUF 格式
- 加载模型到内存
- 返回模型信息

**错误处理**:
- 文件不存在 → `ModelLoadError::NotFound`
- 格式错误 → `ModelLoadError::InvalidFormat`
- 内存不足 → `ModelLoadError::OutOfMemory`

---

### 1.2 模型卸载

**功能**: 释放模型占用的内存

**接口**:
```rust
pub fn unload(&mut self) -> Result<()>;
```

**行为**:
- 释放模型权重
- 清理上下文
- 重置状态

---

## 2. 文本生成

### 2.1 基础生成

**功能**: 根据 prompt 生成文本

**接口**:
```rust
pub fn generate(&mut self, prompt: &str, config: &GenerateConfig) -> Result<GenerationResult>;

pub struct GenerateConfig {
    pub max_tokens: u32,      // 最大生成长度
    pub temperature: f32,     // 温度 (0.0-2.0)
    pub top_p: f32,           // Top-P 采样
    pub stop_sequences: Vec<String>,
}

pub struct GenerationResult {
    pub text: String,
    pub tokens_generated: u32,
    pub duration_ms: u64,
    pub tokens_per_second: f64,
}
```

**行为**:
- Tokenize prompt
- 自回归生成
- 应用采样策略
- 检测停止条件
- 返回结果和性能指标

---

### 2.2 流式生成

**功能**: 流式输出生成的 tokens

**接口**:
```rust
pub fn stream_generate(
    &mut self,
    prompt: &str,
    config: &GenerateConfig,
) -> Result<impl Stream<Item = Result<StreamChunk>>>;

pub enum StreamChunk {
    Token(String),
    Done(GenerationStats),
}

pub struct GenerationStats {
    pub tokens_generated: u32,
    pub duration_ms: u64,
    pub tokens_per_second: f64,
}
```

**行为**:
- 每生成一个 token 立即返回
- 最后返回统计信息
- 支持中途取消

---

## 3. 对话

### 3.1 多轮对话

**功能**: 处理对话历史

**接口**:
```rust
pub fn chat(&mut self, messages: &[Message], config: &ChatConfig) -> Result<ChatResponse>;

pub struct Message {
    pub role: Role,        // System/User/Assistant
    pub content: String,
}

pub struct ChatConfig {
    pub max_tokens: u32,
    pub temperature: f32,
    // ... 同 GenerateConfig
}

pub struct ChatResponse {
    pub message: Message,
    pub usage: UsageInfo,
}

pub struct UsageInfo {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
```

**行为**:
- 格式化对话历史为 prompt
- 调用生成接口
- 解析响应
- 更新对话历史

---

## 4. Tool Calling

### 4.1 Tool 定义

**功能**: 定义可用的工具

**接口**:
```rust
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: JsonSchema,
}

pub struct JsonSchema {
    #[serde(rename = "type")]
    pub schema_type: String,
    pub properties: HashMap<String, JsonProperty>,
    pub required: Vec<String>,
}
```

**示例**:
```json
{
  "name": "search_product",
  "description": "Search for products by name or category",
  "parameters": {
    "type": "object",
    "properties": {
      "query": {
        "type": "string",
        "description": "Search query"
      },
      "category": {
        "type": "string",
        "description": "Product category"
      }
    },
    "required": ["query"]
  }
}
```

---

### 4.2 Tool Call 解析

**功能**: 从响应中解析工具调用

**接口**:
```rust
pub fn chat_with_tools(
    &mut self,
    messages: &[Message],
    tools: &[ToolDefinition],
    config: &ChatConfig,
) -> Result<ToolChatResponse>;

pub enum ToolChatResponse {
    Text(Message),
    ToolCall(ToolCallRequest),
}

pub struct ToolCallRequest {
    pub tool_name: String,
    pub arguments: serde_json::Value,
    pub call_id: String,
}
```

**行为**:
- 注入 tool definitions 到 system prompt
- 解析模型输出的 tool call
- 返回结构化请求

---

### 4.3 Tool Result 注入

**功能**: 将工具执行结果注入对话

**接口**:
```rust
pub fn submit_tool_result(
    &mut self,
    call_id: &str,
    result: &ToolResult,
) -> Result<ChatResponse>;

pub enum ToolResult {
    Success(serde_json::Value),
    Error(String),
}
```

**行为**:
- 添加工具结果到对话历史
- 继续生成响应
- 返回最终回复

---

## 5. 性能监控

### 5.1 指标收集

**功能**: 收集推理性能指标

**指标**:
- `tokens_per_second`: 生成速度
- `time_to_first_token`: 首 token 延迟
- `memory_usage_mb`: 内存占用
- `gpu_memory_usage_mb`: GPU 显存占用

**接口**:
```rust
pub fn get_stats(&self) -> InferenceStats;

pub struct InferenceStats {
    pub tokens_per_second: f64,
    pub time_to_first_token_ms: u64,
    pub memory_usage_mb: f64,
    pub total_tokens_generated: u64,
}
```

---

## 6. 错误处理

### 6.1 错误类型

```rust
#[derive(Debug, thiserror::Error)]
pub enum LLMError {
    #[error("Model not found: {0}")]
    ModelNotFound(String),
    
    #[error("Invalid model format: {0}")]
    InvalidModelFormat(String),
    
    #[error("Out of memory: {0}")]
    OutOfMemory(String),
    
    #[error("Inference failed: {0}")]
    InferenceFailed(String),
    
    #[error("Tool call parse error: {0}")]
    ToolCallParseError(String),
    
    #[error("Context length exceeded")]
    ContextLengthExceeded,
}
```

---

## 7. 配置

### 7.1 YAML 配置示例

```yaml
llm:
  model_path: "/models/qwen2.5-7b-instruct-q4_k_m.gguf"
  n_ctx: 4096
  n_threads: 8
  n_gpu_layers: 35
  temperature: 0.7
  top_p: 0.9
  max_tokens: 2048
```

---

**审核状态**: ⏳ Pending  
**最后更新**: 2026-03-24
