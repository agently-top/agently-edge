# Technical Design: llama.cpp 集成

**Task**: 1.6 - llama.cpp 完整集成  
**版本**: 1.0  
**日期**: 2026-03-24

---

## 1. 架构设计

### 1.1 模块结构

```
src/
├── llm/
│   ├── mod.rs              # 模块导出
│   ├── engine.rs           # LLMEngine 主结构
│   ├── loader.rs           # ModelLoader
│   ├── inference.rs        # InferenceEngine
│   ├── streaming.rs        # 流式生成
│   ├── tools/
│   │   ├── mod.rs
│   │   ├── definition.rs   # Tool 定义
│   │   └── parser.rs       # Tool Call 解析
│   └── stats.rs            # 性能统计
├── config/
│   └── model_config.rs     # 模型配置
└── error.rs                # 错误类型
```

### 1.2 类图

```
┌─────────────────────────────────────────┐
│           LLMEngine                     │
├─────────────────────────────────────────┤
│ - loader: ModelLoader                   │
│ - ctx: LlamaContext                     │
│ - stats: InferenceStats                 │
├─────────────────────────────────────────┤
│ + load(config) -> Result<()>            │
│ + unload() -> Result<()>                │
│ + generate(prompt, config) -> Result    │
│ + chat(messages, config) -> Result      │
│ + chat_with_tools(...) -> Result        │
│ + stream_generate(...) -> Stream        │
│ + get_stats() -> InferenceStats         │
└─────────────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────────────┐
│          ModelLoader                    │
├─────────────────────────────────────────┤
│ + load_gguf(path, config) -> Result     │
│ + validate_gguf(path) -> Result         │
│ + get_model_info(path) -> ModelInfo     │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│       ToolCallParser                    │
├─────────────────────────────────────────┤
│ + parse(response) -> Option<ToolCall>   │
│ + format_tools(tools) -> String         │
│ + inject_result(call_id, result)        │
└─────────────────────────────────────────┘
```

---

## 2. 依赖配置

### 2.1 Cargo.toml

```toml
[dependencies]
# llama.cpp Rust bindings
llama-cpp-2 = "0.1.67"
llama-cpp-sys = "0.1"

# 异步支持
tokio = { version = "1.35", features = ["full"] }
tokio-stream = "0.1"

# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 错误处理
anyhow = "1.0"
thiserror = "1.0"

# 日志
tracing = "0.1"

# JSON Schema
schemars = "0.8"
```

### 2.2 构建配置

```toml
[build-dependencies]
bindgen = "0.69"  # 如需自定义绑定
cc = "1.0"        # C 编译
```

---

## 3. 核心实现

### 3.1 ModelLoader

```rust
use llama_cpp_2::{LlamaModel, LlamaContext};
use std::path::Path;

pub struct ModelLoader {
    model: Option<LlamaModel>,
}

impl ModelLoader {
    pub fn new() -> Self {
        Self { model: None }
    }

    pub fn load_gguf(&mut self, path: &str, config: &ModelConfig) -> Result<ModelInfo> {
        // 1. 验证文件存在
        let path = Path::new(path);
        if !path.exists() {
            return Err(LLMError::ModelNotFound(path.display().to_string()));
        }

        // 2. 加载模型
        let model = LlamaModel::load_from_file(
            path,
            &llama_cpp_2::LlamaModelParameters {
                n_ctx: config.n_ctx,
                n_threads: config.n_threads,
                n_gpu_layers: config.n_gpu_layers,
                ..Default::default()
            },
        )?;

        // 3. 收集模型信息
        let info = ModelInfo {
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            size_mb: path.metadata()?.len() as f64 / 1024.0 / 1024.0,
            n_params: model.n_params(),
            n_ctx: config.n_ctx,
        };

        self.model = Some(model);
        Ok(info)
    }

    pub fn unload(&mut self) -> Result<()> {
        self.model = None;
        Ok(())
    }
}
```

---

### 3.2 LLMEngine

```rust
pub struct LLMEngine {
    loader: ModelLoader,
    ctx: Option<LlamaContext>,
    stats: InferenceStats,
    config: ModelConfig,
}

impl LLMEngine {
    pub fn new(config: ModelConfig) -> Self {
        Self {
            loader: ModelLoader::new(),
            ctx: None,
            stats: InferenceStats::default(),
            config,
        }
    }

    pub async fn load(&mut self) -> Result<ModelInfo> {
        let info = self.loader.load_gguf(&self.config.model_path, &self.config)?;
        
        // 创建上下文
        let model = self.loader.model.as_ref().unwrap();
        self.ctx = Some(model.new_context()?);
        
        Ok(info)
    }

    pub fn generate(&mut self, prompt: &str, config: &GenerateConfig) -> Result<GenerationResult> {
        let ctx = self.ctx.as_mut()
            .ok_or_else(|| LLMError::InferenceFailed("Model not loaded".into()))?;

        let start = std::time::Instant::now();
        
        // Tokenize prompt
        let tokens = ctx.tokenize(prompt.as_bytes())?;
        let prompt_tokens = tokens.len();

        // 创建采样器
        let mut sampler = ctx.create_sampler(
            config.temperature,
            config.top_p,
            110, // repetition penalty
        )?;

        // 自回归生成
        let mut output_tokens = Vec::new();
        ctx.decode(&tokens)?;

        for _ in 0..config.max_tokens {
            let token = sampler.sample(ctx, &output_tokens)?;
            
            if ctx.token_is_eog(token) {
                break;
            }

            output_tokens.push(token);
            ctx.decode(&[token])?;
        }

        // 解码输出
        let text = ctx.detokenize(&output_tokens);
        let text = String::from_utf8_lossy(&text).to_string();

        // 统计
        let duration = start.elapsed();
        let tokens_generated = output_tokens.len() as u32;
        
        self.stats.update(GenerationStats {
            tokens_generated,
            duration_ms: duration.as_millis() as u64,
            tokens_per_second: tokens_generated as f64 / duration.as_secs_f64(),
        });

        Ok(GenerationResult {
            text,
            tokens_generated,
            duration_ms: duration.as_millis() as u64,
            tokens_per_second: tokens_generated as f64 / duration.as_secs_f64(),
        })
    }
}
```

---

### 3.3 ToolCallParser

```rust
use serde_json::{json, Value};

pub struct ToolCallParser;

impl ToolCallParser {
    /// 格式化 tools 为 system prompt
    pub fn format_tools(tools: &[ToolDefinition]) -> String {
        let tools_json: Vec<Value> = tools.iter().map(|t| {
            json!({
                "type": "function",
                "function": {
                    "name": t.name,
                    "description": t.description,
                    "parameters": t.parameters
                }
            })
        }).collect();

        format!(
            "You are a helpful assistant with access to these tools:\n{}\n\n\
             Use the format: {{\"name\": \"tool_name\", \"arguments\": {{...}}}}\n\
             If no tool is needed, just respond normally.",
            serde_json::to_string_pretty(&tools_json).unwrap()
        )
    }

    /// 解析 tool call
    pub fn parse(response: &str) -> Option<ToolCallRequest> {
        // 尝试解析 JSON
        if let Ok(value) = serde_json::from_str::<Value>(response) {
            if let Some(name) = value.get("name").and_then(|v| v.as_str()) {
                if let Some(args) = value.get("arguments") {
                    return Some(ToolCallRequest {
                        tool_name: name.to_string(),
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
                if let Ok(value) = serde_json::from_str::<Value>(json_str) {
                    if let Some(name) = value.get("name").and_then(|v| v.as_str()) {
                        if let Some(args) = value.get("arguments") {
                            return Some(ToolCallRequest {
                                tool_name: name.to_string(),
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
    pub fn format_result(call_id: &str, result: &ToolResult) -> String {
        match result {
            ToolResult::Success(value) => {
                format!(
                    "Tool result for call {}:\n```json\n{}\n```",
                    call_id,
                    serde_json::to_string_pretty(value).unwrap()
                )
            }
            ToolResult::Error(msg) => {
                format!("Tool call {} failed: {}", call_id, msg)
            }
        }
    }
}
```

---

### 3.4 流式生成

```rust
use tokio_stream::Stream;
use std::pin::Pin;
use tokio::sync::mpsc;

pub enum StreamChunk {
    Token(String),
    Done(GenerationStats),
}

pub async fn stream_generate(
    ctx: &mut LlamaContext,
    prompt: &str,
    config: &GenerateConfig,
) -> impl Stream<Item = Result<StreamChunk>> {
    let (tx, rx) = mpsc::channel(32);

    tokio::spawn(async move {
        // Tokenize
        let tokens = ctx.tokenize(prompt.as_bytes()).unwrap();
        ctx.decode(&tokens).unwrap();

        let mut sampler = ctx.create_sampler(
            config.temperature,
            config.top_p,
            110.0,
        ).unwrap();

        let start = std::time::Instant::now();
        let mut tokens_generated = 0;

        for _ in 0..config.max_tokens {
            let token = sampler.sample(ctx, &[]).unwrap();

            if ctx.token_is_eog(token) {
                break;
            }

            let text = ctx.detokenize(&[token]);
            let text = String::from_utf8_lossy(&text).to_string();

            tx.send(Ok(StreamChunk::Token(text))).await.unwrap();
            tokens_generated += 1;

            ctx.decode(&[token]).unwrap();
        }

        let duration = start.elapsed();
        tx.send(Ok(StreamChunk::Done(GenerationStats {
            tokens_generated,
            duration_ms: duration.as_millis() as u64,
            tokens_per_second: tokens_generated as f64 / duration.as_secs_f64(),
        }))).await.unwrap();
    });

    tokio_stream::wrappers::ReceiverStream::new(rx)
}
```

---

## 4. 测试策略

### 4.1 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_call_parser() {
        let response = r#"{"name": "search_product", "arguments": {"query": "phone"}}"#;
        let call = ToolCallParser::parse(response).unwrap();
        
        assert_eq!(call.tool_name, "search_product");
        assert_eq!(call.arguments["query"], "phone");
    }

    #[tokio::test]
    async fn test_mock_generation() {
        let mut engine = LLMEngine::mock();
        let result = engine.generate("Hello", &GenerateConfig::default()).await.unwrap();
        
        assert!(!result.text.is_empty());
        assert!(result.tokens_generated > 0);
    }
}
```

### 4.2 集成测试

```rust
#[tokio::test]
#[ignore] // 需要真实模型文件
async fn test_real_model_load() {
    let config = ModelConfig {
        model_path: "/models/test-q4_k_m.gguf".into(),
        n_ctx: 2048,
        n_threads: 4,
        n_gpu_layers: 0,
    };

    let mut engine = LLMEngine::new(config);
    let info = engine.load().await.unwrap();

    assert!(info.n_params > 0);
    assert!(info.size_mb > 0);
}
```

---

## 5. 性能优化

### 5.1 内存优化

- 使用量化模型（Q4_K_M 推荐）
- 限制上下文长度（默认 2048）
- 及时卸载不用的模型

### 5.2 推理优化

- 使用 GPU 卸载（如有 CUDA）
- 调整线程数匹配 CPU 核心
- KV Cache 复用（多轮对话）

---

## 6. 文件清单

| 文件 | 说明 | 行数估 |
|------|------|--------|
| `src/llm/mod.rs` | 模块导出 | 20 |
| `src/llm/engine.rs` | LLMEngine 主结构 | 300 |
| `src/llm/loader.rs` | ModelLoader | 150 |
| `src/llm/inference.rs` | 推理逻辑 | 200 |
| `src/llm/streaming.rs` | 流式生成 | 100 |
| `src/llm/tools/mod.rs` | Tool 模块 | 20 |
| `src/llm/tools/definition.rs` | Tool 定义 | 80 |
| `src/llm/tools/parser.rs` | Tool Call 解析 | 150 |
| `src/llm/stats.rs` | 性能统计 | 60 |
| `src/config/model_config.rs` | 模型配置 | 80 |
| `tests/llm_integration.rs` | 集成测试 | 200 |
| **总计** | | **~1360** |

---

**审核状态**: ⏳ Pending  
**最后更新**: 2026-03-24
