# Task 1.3: LLM 集成

**Change ID**: task-1-3-llm-integration  
**状态**: ✅ Completed  
**创建日期**: 2026-03-22  
**优先级**: P0  
**父任务**: task-1-1-runtime-setup
**完成日期**: 2026-03-22 20:42

---

## 📋 提案概述

集成 llama.cpp 作为 LLM 推理引擎，实现轻量级文本生成和对话能力。

**目标**：
- llama.cpp 推理引擎集成
- Qwen2.5-0.5B 模型支持（轻量级）
- 文本生成（推理）
- 对话管理（多轮上下文）
- Tool Calling（函数调用）

**范围**：
- ✅ `src/llm/engine.rs` - LLM 推理引擎
- ✅ `src/llm/model.rs` - 模型管理
- ✅ `src/llm/prompt.rs` - Prompt 管理
- ✅ `src/llm/tool_call.rs` - Tool Calling 支持
- ✅ 单元测试 + 集成测试

---

## 🏗 技术方案

### 技术选型

| 组件 | 选型 | 理由 |
|------|------|------|
| **推理引擎** | llama.cpp | 轻量、高效、支持 GGUF |
| **Rust 绑定** | llama-cpp-rust | 原生集成 |
| **模型格式** | GGUF (Q4_K_M) | 轻量，~400MB |
| **推荐模型** | Qwen2.5-0.5B-Instruct | 轻量，中文友好 |

### 模型配置

**模型**: `Qwen/Qwen2.5-0.5B-Instruct-GGUF`

**文件**: `qwen2.5-0.5b-instruct-q4_k_m.gguf` (~400MB)

**配置**:
```yaml
model:
  path: /opt/agently/models/qwen2.5-0.5b-instruct-q4_k_m.gguf
  context_length: 4096
  temperature: 0.7
  top_p: 0.9
  max_tokens: 512
  n_threads: 4
  batch_size: 512
```

### 架构设计

```
┌─────────────────────────────────────┐
│         Agent                       │
├─────────────────────────────────────┤
│  ┌─────────────────────────────┐    │
│  │  LLM Engine (llama.cpp)     │    │
│  │  - Model Loader             │    │
│  │  - Text Generation          │    │
│  │  - Context Management       │    │
│  │  - Tool Calling             │    │
│  └─────────────────────────────┘    │
│                                      │
│  ┌─────────────────────────────┐    │
│  │  Prompt Manager             │    │
│  │  - System Prompt            │    │
│  │  - User Prompt              │    │
│  │  - Conversation History     │    │
│  └─────────────────────────────┘    │
└─────────────────────────────────────┘
```

---

## 📦 交付物

| 交付物 | 说明 |
|--------|------|
| `src/llm/engine.rs` | LLM 推理引擎 |
| `src/llm/model.rs` | 模型加载与管理 |
| `src/llm/prompt.rs` | Prompt 管理 |
| `src/llm/tool_call.rs` | Tool Calling 支持 |
| `tests/llm_test.rs` | 单元测试 |
| `tests/integration_llm_test.rs` | 集成测试 |

---

## ✅ 验收标准

**功能验收**：
- [ ] 模型可加载（GGUF 格式，Qwen2.5-0.5B）
- [ ] 文本生成正常
- [ ] 多轮对话正常
- [ ] Tool Calling 格式正确

**性能验收**：
- [ ] 冷启动 <3s
- [ ] 推理延迟 <300ms (100 tokens)
- [ ] 内存占用 <500MB (模型 + 运行时)

**质量验收**：
- [ ] 测试覆盖率 >80%
- [ ] clippy 无警告
- [ ] rustfmt 格式正确

---

## ⏱ 时间估算

| 模块 | 时间 |
|------|------|
| LLM 引擎集成 | 2-3 小时 |
| Prompt 管理 | 1 小时 |
| Tool Calling | 2 小时 |
| 测试 | 2 小时 |
| **总计** | **7-8 小时** |

---

## 🔗 依赖关系

**前置依赖**: Task 1.2 (核心模块开发) ✅  
**后续依赖**: Task 1.4 (示例应用)

---

## 📝 完成总结

**已完成任务**:
- [x] Task 1.3.1: LLM 引擎 (3/3 tests)
- [x] Task 1.3.2: Prompt 管理 (3/3 tests)
- [x] Task 1.3.3: Tool Calling (3/3 tests)
- [x] Task 1.3.4: 集成测试 (3/3 tests)

**交付物**:
- `src/llm.rs` - LLM 引擎、Prompt 管理、Tool Calling
- `tests/llm_test.rs` - LLM 单元测试
- `tests/prompt_test.rs` - Prompt 管理测试
- `tests/tool_call_test.rs` - Tool Calling 测试
- `tests/integration_llm_test.rs` - 集成测试

**Git 历史**:
- `c8bd07e` Task 1.3.4: Add LLM integration tests
- `11be3e7` Task 1.3.3: Implement Tool Calling support
- `ebf621c` Task 1.3.2: Implement PromptManager module
- `498351a` Task 1.3.1: Implement LLM engine module

**验收标准**:
- [x] 所有测试通过 (31/31)
- [x] 代码已格式化
- [x] 无 clippy 警告
- [ ] 待推送到远端（网络问题）

---

**提案作者**: 虾 (Xia)  
**审核状态**: ✅ Approved  
**最后更新**: 2026-03-22 20:42
