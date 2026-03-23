# Hello World Agent 技术设计

## 1. 架构设计

### 1.1 整体架构

```
┌─────────────────────────────────────────┐
│         examples/hello-world            │
├─────────────────────────────────────────┤
│                                         │
│  ┌─────────────┐    ┌────────────────┐ │
│  │  agent.yaml │───▶│  Config Loader │ │
│  └─────────────┘    └───────┬────────┘ │
│                             │          │
│                             ▼          │
│  ┌─────────────┐    ┌────────────────┐ │
│  │   User      │───▶│  PromptManager │ │
│  │   Input     │    │  + LLM Engine  │ │
│  └─────────────┘    └───────┬────────┘ │
│                             │          │
│                             ▼          │
│                    ┌────────────────┐  │
│                    │    Response    │  │
│                    │    Output      │  │
│                    └────────────────┘  │
└─────────────────────────────────────────┘
```

### 1.2 模块结构

```
hello-world/
├── Cargo.toml          # 项目配置
├── main.rs             # 主程序入口
├── agent.yaml          # 配置文件
└── README.md           # 使用说明
```

---

## 2. 技术选型

### 2.1 依赖选择

| 依赖 | 版本 | 用途 |
|------|------|------|
| agently-edge-runtime | path | 核心运行时 |
| anyhow | 1.0 | 错误处理 |
| tokio | 1.35 | 异步运行时 |
| serde | 1.0 | 序列化 |
| serde_yaml | 0.9 | YAML 解析 |

### 2.2 设计决策

#### 决策 1: Mock 模式
**问题**: 是否需要真实模型？  
**选项**:
- A. 使用真实模型（需要下载 ~400MB）
- B. 使用 Mock 模式（快速测试）

**决策**: **选项 B**  
**理由**: 
- 示例目的是展示 API 用法，不是性能测试
- 降低使用门槛（无需下载模型）
- 加快 CI/CD 速度

#### 决策 2: 交互式 vs 单次运行
**问题**: 对话模式？  
**选项**:
- A. 交互式循环（REPL）
- B. 单次对话（命令行参数）

**决策**: **选项 A**  
**理由**:
- 更符合实际使用场景
- 能展示多轮对话能力
- 用户体验更好

#### 决策 3: 配置方式
**问题**: 如何配置？  
**选项**:
- A. 硬编码
- B. 环境变量
- C. YAML 配置文件

**决策**: **选项 C**  
**理由**:
- 灵活易修改
- 符合行业惯例
- 便于版本控制

---

## 3. 数据结构

### 3.1 配置结构

```rust
#[derive(Debug, Deserialize)]
struct AgentConfig {
    agent: AgentInfo,
    runtime: RuntimeConfig,
    prompts: Prompts,
}

#[derive(Debug, Deserialize)]
struct AgentInfo {
    id: String,
    name: String,
    version: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct RuntimeConfig {
    model_path: String,
    context_length: u32,
    temperature: f32,
    max_tokens: u32,
    n_threads: u32,
}

#[derive(Debug, Deserialize)]
struct Prompts {
    system: String,
    welcome: String,
}
```

### 3.2 对话流程

```rust
// 1. 加载配置
let config = load_config("agent.yaml")?;

// 2. 初始化 LLM
let mut engine = LLMEngine::mock();

// 3. 创建 Prompt 管理器
let mut prompt_mgr = PromptManager::new();
prompt_mgr.add_system(&config.prompts.system);

// 4. 对话循环
loop {
    let input = read_user_input();
    if input == "quit" { break; }
    
    prompt_mgr.add_user(&input);
    let messages = prompt_mgr.build_messages();
    let response = engine.chat(&messages)?;
    
    prompt_mgr.add_assistant(&response);
    println!("Agent: {}", response);
}
```

---

## 4. 错误处理

### 4.1 错误类型

| 错误 | 处理方式 |
|------|----------|
| 配置文件不存在 | 打印错误并退出 |
| YAML 解析失败 | 打印错误详情并退出 |
| LLM 调用失败 | 打印错误并继续 |
| 用户输入错误 | 忽略并继续 |

### 4.2 错误信息

```
❌ Error: Configuration file not found: agent.yaml
   → Please create agent.yaml in the current directory

❌ Error: Failed to parse YAML: invalid type at line 5
   → Please check your agent.yaml syntax

⚠️  Warning: LLM chat failed: mock error
   → Continuing...
```

---

## 5. 性能考虑

### 5.1 启动时间

- 配置加载：<10ms
- LLM 初始化（mock）：<1ms
- 总启动时间：<100ms

### 5.2 内存占用

- 基础占用：<10MB
- 对话历史：动态增长
- 目标上限：<50MB

---

## 6. 安全考虑

### 6.1 输入验证

- 限制单行输入长度（<1000 字符）
- 过滤控制字符
- 处理 UTF-8 编码

### 6.2 资源限制

- 对话历史限制（最近 20 条）
- 最大 token 数限制（512）

---

**设计版本**: 1.0  
**创建日期**: 2026-03-23  
**审核状态**: ⏳ Pending
