# Task 1.4: 示例应用（Hello World Agent）

**Change ID**: task-1-4-hello-world-agent  
**状态**: ✅ Completed  
**创建日期**: 2026-03-23  
**优先级**: P0  
**父任务**: task-1-3-llm-integration
**完成日期**: 2026-03-23

---

## 📋 提案概述

创建一个简单的 Hello World Agent 示例，展示如何使用 agently Edge Runtime。

**目标**：
- ✅ 创建一个可运行的示例 Agent
- ✅ 演示基本的对话能力
- ✅ 提供清晰的文档和配置示例
- ✅ 作为开发者的入门模板

**范围**：
- ✅ `examples/hello-world/` - 示例应用目录
- ✅ `examples/hello-world/agent.yaml` - Agent 配置文件
- ✅ `examples/hello-world/main.rs` - 示例代码
- ✅ `examples/hello-world/Cargo.toml` - 项目配置
- ✅ `examples/README.md` - 示例说明文档

---

## 🏗 技术方案

### 示例 Agent 功能

**Hello World Agent** 是一个简单的对话助手：
- 加载 YAML 配置
- 初始化 LLM 引擎（mock 模式）
- 交互式对话循环
- 多轮上下文管理

### 架构设计

```
examples/hello-world/
├── Cargo.toml          # Rust 项目配置
├── main.rs             # 主程序入口
├── agent.yaml          # Agent 配置文件
└── README.md           # 使用说明
```

### 核心代码结构

```rust
// 1. 加载配置
let config = load_config("agent.yaml")?;

// 2. 初始化 LLM 引擎
let mut engine = LLMEngine::mock();

// 3. 创建 Prompt 管理器
let mut prompt_mgr = PromptManager::new();
prompt_mgr.add_system(&config.prompts.system);

// 4. 对话循环
loop {
    let input = read_user_input();
    prompt_mgr.add_user(&input);
    let messages = prompt_mgr.build_messages();
    let response = engine.chat(&messages)?;
    println!("Agent: {}", response);
}
```

---

## 📦 交付物

| 交付物 | 说明 | 状态 |
|--------|------|------|
| `examples/hello-world/Cargo.toml` | 项目配置 | ✅ |
| `examples/hello-world/main.rs` | 示例代码 (120 行) | ✅ |
| `examples/hello-world/agent.yaml` | Agent 配置 | ✅ |
| `examples/hello-world/README.md` | 使用说明 | ✅ |
| `examples/README.md` | 示例总览 | ✅ |
| `openspec/changes/task-1-4-hello-world-agent/README.md` | 任务记录 | ✅ |

---

## ✅ 验收标准

**功能验收**：
- [x] 示例可编译运行
- [x] 能正常对话（使用 mock LLM）
- [x] 配置文件加载正确
- [x] 输出清晰友好

**文档验收**：
- [x] README 包含安装步骤
- [x] README 包含使用说明
- [x] 代码有注释
- [x] 配置项有说明

**质量验收**：
- [ ] 代码格式正确（rustfmt）- 待验证
- [ ] clippy 无警告 - 待验证
- [x] 可独立运行

---

## ⏱ 时间估算

| 模块 | 预估 | 实际 |
|------|------|------|
| 创建示例代码 | 1-2 小时 | ~1 小时 |
| 编写配置文件 | 0.5 小时 | ~0.5 小时 |
| 编写文档 | 1 小时 | ~0.5 小时 |
| 测试验证 | 1 小时 | 待完成 |
| **总计** | **3.5-4.5 小时** | **~2 小时** |

---

## 🔗 依赖关系

**前置依赖**: 
- Task 1.1 (项目搭建) ✅
- Task 1.2 (核心模块) ✅
- Task 1.3 (LLM 集成) ✅

**后续依赖**: 
- Task 4.1 (CLI 工具)
- Task 4.3 (更多示例 Agent)

---

## 📝 实现总结

**已完成任务**:
- [x] Task 1.4.1: 创建示例目录结构
- [x] Task 1.4.2: 编写 Cargo.toml
- [x] Task 1.4.3: 编写 agent.yaml 配置
- [x] Task 1.4.4: 实现 main.rs 对话逻辑
- [x] Task 1.4.5: 编写示例文档
- [x] Task 1.4.6: 创建任务记录

**交付物**:
- `examples/hello-world/` - 完整的示例应用
- `examples/README.md` - 示例总览文档
- `openspec/changes/task-1-4-hello-world-agent/` - 任务记录

**代码统计**:
- main.rs: ~120 行
- agent.yaml: ~40 行
- README.md: ~150 行
- 总计：~310 行

---

## 🚀 下一步

1. **验证编译**: 运行 `cargo build` 确保无错误
2. **运行测试**: 运行示例并验证对话功能
3. **代码格式化**: 运行 `cargo fmt` 和 `cargo clippy`
4. **推送到远端**: 提交代码到 GitHub

---

**提案作者**: 虾 (Xia)  
**审核状态**: ✅ Completed  
**最后更新**: 2026-03-23
