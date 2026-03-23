# agently Edge Core Architecture Design

**日期**: 2026-03-23  
**阶段**: Phase 1 (Rust 核心 + Linux MVP)  
**优先级**: B. 架构优先  
**方案**: A. 分层架构

---

## 1. 架构概述

### 1.1 分层设计

```
┌─────────────────────────────────────────────┐
│           CLI / SDK Layer                   │  ← 外部接口 (clap + public API)
├─────────────────────────────────────────────┤
│           Agent Runtime Layer               │  ← Agent 生命周期/上下文管理
├─────────────────────────────────────────────┤
│           Core Services Layer               │  ← 消息总线/工具注册表/配置加载
├─────────────────────────────────────────────┤
│           LLM Engine Layer                  │  ← llama.cpp 封装
└─────────────────────────────────────────────┘
```

### 1.2 FFI 边界

```
┌─────────────────────────────────────────────┐
│     Rust: Core Services Layer               │
│  - Message Bus                              │
│  - Tool Registry                            │
│  - Config Loader                            │
│                                             │
│  #[ffi_export] → C ABI                      │
└──────────────────┬──────────────────────────┘
                   │ C FFI
┌──────────────────▼──────────────────────────┐
│     C++: llama.cpp                          │
│  - LLM inference                            │
│  - Model loading                            │
└─────────────────────────────────────────────┘
```

**FFI 导出位置**: Core Services 层  
**理由**: 
- 上层 (Agent Runtime, CLI) 可保持纯 Rust
- 下层 (LLM Engine) 可替换（如未来换其他推理引擎）
- C ABI 稳定，易于 Android/鸿蒙调用

---

## 2. 核心组件设计

### 2.1 Agent Runtime Layer

**职责**: Agent 生命周期管理、对话上下文维护

```rust
// core/agent/mod.rs
pub struct Agent {
    id: String,
    config: AgentConfig,
    context: ConversationContext,
    tools: Vec<Box<dyn Tool>>,
}

pub trait AgentLifecycle {
    fn init(&mut self) -> Result<()>;
    fn process(&mut self, input: Message) -> Result<Message>;
    fn shutdown(&mut self) -> Result<()>;
}

pub struct ConversationContext {
    history: Vec<Message>,
    max_turns: usize,
}
```

**关键设计**:
- Agent 无状态，状态由 Context 管理
- 支持多轮对话（可配置最大轮数）
- 工具注入（依赖注入模式）

---

### 2.2 Core Services Layer

**职责**: 消息传递、工具管理、配置加载

#### 2.2.1 消息总线

```rust
// core/services/message_bus.rs
pub struct MessageBus {
    tx: Sender<Message>,
    rx: Receiver<Message>,
}

// 使用 async-channel（轻量，无 tokio 依赖）
impl MessageBus {
    pub fn new() -> (Self, Sender<Message>, Receiver<Message>);
    pub async fn send(&self, msg: Message) -> Result<()>;
    pub async fn recv(&self) -> Result<Message>;
}
```

**设计决策**: 
- 单通道，顺序处理（简化 MVP）
- 未来可扩展为多通道（按 Agent ID 路由）

#### 2.2.2 工具注册表

```rust
// core/services/tool_registry.rs
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn execute(&self, input: Value) -> Result<Value>;
}

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn register(&mut self, tool: Box<dyn Tool>);
    pub fn get(&self, name: &str) -> Option<&dyn Tool>;
}
```

**内置工具** (MVP):
- `http_request` — HTTP 调用
- `file_read` / `file_write` — 文件操作
- `shell_exec` — Shell 命令（受限）

#### 2.2.3 配置加载

```rust
// core/services/config.rs
#[derive(Deserialize)]
pub struct AgentConfig {
    pub agent_id: String,
    pub name: String,
    pub model: ModelConfig,
    pub tools: Vec<ToolConfig>,
    pub prompts: PromptConfig,
}

// 使用 serde_yaml
pub fn load_config(path: &Path) -> Result<AgentConfig>;
```

**配置格式**: YAML  
**理由**: 开发者友好，生态常用

---

### 2.3 LLM Engine Layer

**职责**: llama.cpp 封装，提供统一推理接口

```rust
// core/llama/engine.rs
pub struct LlamaEngine {
    // FFI handle to llama.cpp
    ctx: *mut llama_context,
}

impl LlamaEngine {
    pub fn load(model_path: &str, config: ModelConfig) -> Result<Self>;
    pub fn generate(&mut self, prompt: &str) -> Result<String>;
    pub fn chat(&mut self, messages: &[Message]) -> Result<String>;
}

// 内部调用 llama.cpp C API
// 不直接暴露给上层
```

**内存管理**:
- 模型加载时分配，Engine 生命周期内复用
- 通过 `Drop` trait 自动释放

---

### 2.4 CLI / SDK Layer

**职责**: 命令行接口，公共 API 导出

```rust
// cli/src/main.rs
#[derive(Parser)]
enum Command {
    Run { config: PathBuf },
    Create { name: String },
    Deploy { target: String },
}

// SDK 导出（供未来 Python 绑定使用）
#[ffi_export]
pub fn agent_create(config: &AgentConfig) -> *mut Agent;
#[ffi_export]
pub fn agent_process(agent: *mut Agent, input: &str) -> *mut c_char;
#[ffi_export]
pub fn agent_destroy(agent: *mut Agent);
```

---

## 3. 数据流

### 3.1 请求处理流程

```
用户输入 (CLI)
    ↓
CLI Layer → 解析命令
    ↓
Agent Runtime → 创建/获取 Agent
    ↓
Agent.process() → 构建 Prompt
    ↓
Core Services → 消息总线
    ↓
LLM Engine → llama.cpp 推理
    ↓
LLM Engine → 返回响应
    ↓
Agent Runtime → 更新 Context
    ↓
CLI Layer → 输出响应
```

### 3.2 工具调用流程

```
LLM 生成工具调用请求
    ↓
Agent Runtime → 解析工具名 + 参数
    ↓
Core Services → ToolRegistry.get(name)
    ↓
Tool.execute(params)
    ↓
返回结果给 LLM
    ↓
LLM 生成最终响应
```

---

## 4. 错误处理

### 4.1 错误类型

```rust
// core/error.rs
#[derive(thiserror::Error, Debug)]
pub enum AgentlyError {
    #[error("Config load failed: {0}")]
    ConfigError(#[from] ConfigError),
    
    #[error("LLM inference failed: {0}")]
    InferenceError(String),
    
    #[error("Tool execution failed: {0}")]
    ToolError(String),
    
    #[error("Message bus error: {0}")]
    BusError(#[from] async_channel::SendError<Message>),
}

pub type Result<T> = std::result::Result<T, AgentlyError>;
```

### 4.2 错误传播

- 底层错误 → 转换为 `AgentlyError` → 向上传播
- CLI 层捕获并友好提示
- 日志记录完整堆栈（debug 模式）

---

## 5. 测试策略

### 5.1 单元测试

- 每层独立测试（通过 trait mock）
- 核心逻辑 100% 覆盖
- 命令：`cargo test --lib`

### 5.2 集成测试

- Agent 端到端流程
- 工具调用集成
- 命令：`cargo test --test integration`

### 5.3 FFI 测试

- C ABI 稳定性测试
- 跨语言调用测试（未来 Python/Android）

---

## 6. 目录结构

```
agently-edge/
├── runtime/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs              # 库入口
│   │   ├── agent/              # Agent Runtime Layer
│   │   │   ├── mod.rs
│   │   │   ├── base.rs
│   │   │   └── context.rs
│   │   ├── services/           # Core Services Layer
│   │   │   ├── mod.rs
│   │   │   ├── message_bus.rs
│   │   │   ├── tool_registry.rs
│   │   │   └── config.rs
│   │   ├── llama/              # LLM Engine Layer
│   │   │   ├── mod.rs
│   │   │   └── engine.rs
│   │   ├── error.rs
│   │   └── ffi.rs              # FFI 导出
│   └── tests/
│       ├── unit/
│       └── integration/
├── cli/
│   ├── Cargo.toml
│   └── src/main.rs
├── examples/
│   └── hello-world/
│       ├── agent.yaml
│       └── main.rs
└── docs/
    └── plans/
        └── 2026-03-23-core-architecture-design.md
```

---

## 7. 里程碑

| 里程碑 | 交付物 | 验收标准 |
|--------|--------|----------|
| M1 | 分层架构搭建完成 | `cargo build` 通过 |
| M2 | Agent 生命周期实现 | `agent.init()` / `process()` / `shutdown()` 可用 |
| M3 | 消息总线 + 工具注册表 | 工具可注册、可调用 |
| M4 | llama.cpp 集成 | 可加载模型并生成文本 |
| M5 | Hello World Agent | `agently run agent.yaml` 可对话 |

---

## 8. 风险与应对

| 风险 | 概率 | 影响 | 应对 |
|------|------|------|------|
| llama.cpp FFI 绑定复杂 | 中 | 高 | 先用 Rust binding 库（llama-cpp-rs） |
| async-channel 性能瓶颈 | 低 | 中 | MVP 够用，后续可换 tokio |
| 内存泄漏（FFI 边界） | 中 | 高 | 严格测试 + valgrind 检查 |

---

## 9. 下一步

1. ✅ 设计文档批准（当前）
2. ⏳ 创建 OpenSpec 正式提案
3. ⏳ 拆解 Task 1.4 (Hello World Agent) 为详细计划
4. ⏳ Subagent-Driven 执行

---

**设计作者**: 虾 (Xia)  
**批准状态**: ✅ 用户已批准方案 A  
**批准日期**: 2026-03-23
