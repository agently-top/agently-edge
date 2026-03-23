# Task 1.4: Hello World Agent 示例应用

**Change ID**: task-1-4-hello-world-agent  
**状态**: 📝 Proposed  
**创建日期**: 2026-03-23  
**优先级**: P0  
**父任务**: task-1-3-llm-integration

---

## 📋 提案概述

创建一个简单的 Hello World Agent 示例应用，展示如何使用 agently Edge Runtime 构建对话助手。

### 目标

- 创建一个可运行的示例 Agent
- 演示基本的配置加载和对话功能
- 提供清晰的文档作为开发者入门模板
- 验证 runtime 核心模块的可用性

### 范围

**包含**：
- `examples/hello-world/` 示例应用目录
- `examples/hello-world/Cargo.toml` 项目配置
- `examples/hello-world/main.rs` 主程序
- `examples/hello-world/agent.yaml` 配置文件
- `examples/hello-world/README.md` 使用说明
- `examples/README.md` 示例总览

**不包含**：
- 真实模型部署（使用 mock 模式）
- 复杂工具调用
- 生产级错误处理

---

## 🎯 成功标准

### 功能标准
- [ ] 示例可编译运行
- [ ] 能进行至少 3 轮对话
- [ ] 配置文件正确加载
- [ ] 输出友好清晰

### 文档标准
- [ ] README 包含安装步骤
- [ ] README 包含使用说明
- [ ] 代码有充分注释
- [ ] 配置项有详细说明

### 质量标准
- [ ] 代码通过 `cargo fmt`
- [ ] 无 `cargo clippy` 警告
- [ ] 可独立于主项目运行

---

## 🔗 依赖关系

**前置依赖**：
- ✅ task-1-1-runtime-setup (项目搭建)
- ✅ task-1-2-core-modules (核心模块)
- ✅ task-1-3-llm-integration (LLM 集成)

**后续依赖**：
- ⏳ task-4-1-cli-tool (CLI 工具)
- ⏳ task-4-3-more-examples (更多示例)

---

## ⏱ 时间估算

| 模块 | 预估时间 |
|------|----------|
| 创建示例代码 | 1-2 小时 |
| 编写配置文件 | 0.5 小时 |
| 编写文档 | 1 小时 |
| 测试验证 | 1 小时 |
| **总计** | **3.5-4.5 小时** |

---

**提案作者**: 虾 (Xia)  
**审核状态**: ✅ Approved  
**批准日期**: 2026-03-23  
**最后更新**: 2026-03-23
