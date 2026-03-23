# Hello World Agent

一个简单的示例 Agent，展示如何使用 agently Edge Runtime 创建对话助手。

## 📋 功能

- ✅ 加载 YAML 配置文件
- ✅ 初始化 LLM 引擎
- ✅ 交互式对话
- ✅ 多轮上下文管理
- ✅ Mock 模式（无需真实模型即可测试）

## 🚀 快速开始

### 1. 编译运行

```bash
# 进入示例目录
cd examples/hello-world

# 编译并运行
cargo run
```

### 2. 开始对话

运行后会看到欢迎信息：

```
╔══════════════════════════════════════════════════════════╗
║  Hello World Agent v0.1.0                                ║
║  A friendly hello world assistant                        ║
╚══════════════════════════════════════════════════════════╝

👋 Hello! I'm your Hello World Agent.
I'm running on agently Edge Runtime.
How can I help you today?

Type 'quit' or 'exit' to stop.

👤 You: Hello!
🤖 Agent: Hello! How can I help you today?

👤 You: What can you do?
🤖 Agent: I'm a simple example agent...

👤 You: quit
👋 Goodbye! Thanks for using Hello World Agent.
```

## 📁 项目结构

```
hello-world/
├── Cargo.toml          # Rust 项目配置
├── main.rs             # 主程序入口
├── agent.yaml          # Agent 配置文件
└── README.md           # 本文档
```

## ⚙️ 配置说明

### agent.yaml

```yaml
agent:
  id: hello-world           # Agent 唯一标识
  name: Hello World Agent   # 显示名称
  version: 0.1.0            # 版本号
  description: "..."        # 描述

runtime:
  model_path: ./models/...  # 模型文件路径
  context_length: 4096      # 上下文长度
  temperature: 0.7          # 生成温度
  max_tokens: 512           # 最大生成 token 数
  n_threads: 4              # 推理线程数

prompts:
  system: |                 # 系统提示词
    You are a friendly...
  welcome: |                # 欢迎语
    Hello! I'm your...
```

## 🔧 自定义

### 修改系统提示词

编辑 `agent.yaml` 中的 `prompts.system`：

```yaml
prompts:
  system: |
    You are a helpful shopping assistant.
    Help users find products and answer questions.
```

### 添加更多功能

在 `main.rs` 中添加：

```rust
// 添加工具调用
let tools = vec![Tool {
    name: "get_weather".to_string(),
    // ...
}];
let (response, tool_call) = engine.chat_with_tools(&messages, &tools)?;
```

## 📝 下一步

- 尝试修改配置文件，创建你自己的 Agent
- 查看其他示例：`examples/README.md`
- 阅读完整文档：https://agently-top.github.io/agently-edge/

## 🛠️ 开发

### 编译

```bash
cargo build --release
```

### 测试

```bash
cargo test
```

### 格式化

```bash
cargo fmt
```

### 代码检查

```bash
cargo clippy
```

## 📄 许可证

Apache 2.0 - 与 agently Edge 主项目一致
