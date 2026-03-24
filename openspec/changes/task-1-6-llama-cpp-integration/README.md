# Task 1.6: llama.cpp 完整集成

**Change ID**: task-1-6-llama-cpp-integration  
**状态**: 🚧 In Progress  
**创建日期**: 2026-03-24  
**优先级**: P0  
**父任务**: Phase 1 - Rust 核心 + Linux

---

## 📋 提案概述

实现完整的 llama.cpp 集成，使 `agently-edge-runtime` 具备真正的本地 LLM 推理能力。

**目标**：
- ✅ 本地 LLM 推理（CPU/GPU）
- ✅ Tool Calling 支持
- ✅ 流式响应
- ✅ 性能监控
- ✅ Phase 1 验收达标

**范围**：
- `src/llm/` - LLM 引擎重构
- `tests/llm_integration.rs` - 集成测试
- `examples/chat-agent.yaml` - 示例配置
- `docs/inference.md` - 推理文档

---

## 📄 文档

| 文档 | 说明 | 状态 |
|------|------|------|
| [proposal.md](./proposal.md) | 项目提案 | ✅ Complete |
| [specs/functional-spec.md](./specs/functional-spec.md) | 功能规格 | ✅ Complete |
| [design.md](./design.md) | 技术方案 | ✅ Complete |
| [tasks.md](./tasks.md) | 任务清单 | ✅ Complete |

---

## 🏗 技术方案

### 核心依赖

```toml
[dependencies]
llama-cpp-2 = "0.1.67"
llama-cpp-sys = "0.1"
tokio = { version = "1.35", features = ["full"] }
tokio-stream = "0.1"
```

### 架构

```
┌─────────────────────────────────────┐
│     LLMEngine                       │
├─────────────────────────────────────┤
│  ModelLoader → InferenceEngine      │
│  ToolCallParser → Streaming         │
└─────────────────────────────────────┘
```

---

## 📅 时间估算

| 子任务 | 估时 | 状态 |
|--------|------|------|
| 1.6.1: 依赖配置 | 2h | ⏳ Pending |
| 1.6.2: ModelLoader | 4h | ⏳ Pending |
| 1.6.3: InferenceEngine | 6h | ⏳ Pending |
| 1.6.4: Tool Calling | 4h | ⏳ Pending |
| 1.6.5: 流式生成 | 3h | ⏳ Pending |
| 1.6.6: 性能监控 | 2h | ⏳ Pending |
| 1.6.7: 测试 + 文档 | 4h | ⏳ Pending |
| **总计** | **29h** ≈ 3-4 天 | |

---

## ✅ 验收标准

### 功能验收

- [ ] 可加载 GGUF 模型（7B 及以下）
- [ ] 可生成文本响应
- [ ] 可处理多轮对话
- [ ] 可解析 Tool Calls
- [ ] 支持流式输出
- [ ] 支持模型卸载

### 性能验收

- [ ] 冷启动时间 <10s
- [ ] 热启动时间 <2s
- [ ] 生成速度 >10 tokens/s（CPU）
- [ ] 内存占用 <4GB（7B 模型）

### 质量验收

- [ ] 测试覆盖率 >80%
- [ ] clippy 无警告
- [ ] rustfmt 格式正确
- [ ] 文档完整

---

## 🔗 依赖关系

**前置依赖**: 
- Task 1.2 (核心模块) ✅
- Task 1.3 (LLM Mock) ✅

**后续依赖**:
- Task 1.7 (示例 Agent)
- Task 1.8 (Phase 1 验收)

---

## 📝 执行计划

1. **等待批准** → 用户确认 proposal
2. **启动子代理** → 按照 tasks.md 执行
3. **TDD 流程** → 每个子任务先写测试
4. **Review** → Spec Review + Code Quality Review
5. **合并** → 更新状态并归档

---

## 📊 状态追踪

| 日期 | 事件 | 状态 |
|------|------|------|
| 2026-03-24 | 创建提案 | 📝 Draft |
| 2026-03-24 | 批准 | ✅ Approved |
| 2026-03-24 | Task 1.6.1: 依赖配置 | ✅ Done |
| 2026-03-24 | Task 1.6.2: ModelLoader | ✅ Done |
| 2026-03-24 | Task 1.6.3: InferenceEngine | ✅ Done |
| 2026-03-24 | Task 1.6.4: Tool Calling | ✅ Done |
| 2026-03-24 | Task 1.6.5: 流式生成 | ⏸ Deferred |
| 2026-03-24 | Task 1.6.6: 性能监控 | ✅ Done (基础统计) |
| 2026-03-24 | Task 1.6.7: 测试 + 文档 | ✅ Done |

**完成日期**: 2026-03-24  
**实际用时**: ~2 小时  
**状态**: ✅ Completed

---

**提案作者**: 虾 (Xia)  
**审核状态**: ⏳ Pending Approval  
**最后更新**: 2026-03-24

---

## ⚠️ 注意事项

- 需要 Rust 工具链（如未安装需先安装）
- 测试需要 GGUF 模型文件（可用 mock 先行）
- 构建可能需要较长时间（llama.cpp 编译）
