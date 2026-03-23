# Proposal: Task 1.6 - llama.cpp 完整集成

**Change ID**: task-1-6-llama-cpp-integration  
**创建日期**: 2026-03-24  
**优先级**: P0  
**父任务**: Phase 1 - Rust 核心 + Linux  
**预计周期**: 2-3 天

---

## 📋 问题陈述

当前 `agently-edge-runtime` 的 LLM 引擎仅实现了 Mock 响应，无法进行真正的本地推理。这导致：

1. ❌ 无法离线运行 Agent
2. ❌ 无法验证 Tool Calling 功能
3. ❌ 无法进行性能基准测试
4. ❌ Phase 1 验收标准未达成（目标：真正的 Edge AI 运行时）

---

## 🎯 目标

实现完整的 llama.cpp 集成，使 `agently-edge-runtime` 具备：

1. ✅ 本地 LLM 推理能力（CPU/GPU）
2. ✅ Tool Calling 支持
3. ✅ 流式响应支持
4. ✅ 模型热加载/卸载
5. ✅ 性能指标收集

---

## 📦 范围

### 包含内容

- [ ] Rust bindings 封装（使用 `llama-cpp-rs` 或 `llama-cpp-2`）
- [ ] 模型加载器（支持 GGUF 格式）
- [ ] 推理引擎（同步 + 异步）
- [ ] Tool Calling 解析器
- [ ] 流式响应支持
- [ ] 性能监控（tokens/s、内存占用）
- [ ] 单元测试 + 集成测试

### 不包含内容

- ❌ 模型量化（由用户自行准备 GGUF 模型）
- ❌ 多模型并发（留作 Phase 2）
- ❌ GPU 优化（基础 CUDA 支持即可）

---

## 🏗 技术方案概述

### 依赖选择

**推荐**: `llama-cpp-2` (Rust 绑定)

```toml
[dependencies]
llama-cpp-2 = "0.1"
llama-cpp-sys = "0.1"
```

**理由**:
- 活跃的 Rust 社区维护
- 完整的 llama.cpp 功能覆盖
- 支持 GGUF 格式
- 支持 Tool Calling

### 架构设计

```
┌─────────────────────────────────────┐
│     LLMEngine (src/llm.rs)          │
├─────────────────────────────────────┤
│  ┌─────────────────────────────────┐│
│  │  ModelLoader                    ││
│  │  - load_gguf(path)              ││
│  │  - unload()                     ││
│  └─────────────────────────────────┘│
│  ┌─────────────────────────────────┐│
│  │  InferenceEngine                ││
│  │  - generate(prompt)             ││
│  │  - chat(messages)               ││
│  │  - chat_with_tools(...)         ││
│  │  - stream_generate(...)         ││
│  └─────────────────────────────────┘│
│  ┌─────────────────────────────────┐│
│  │  ToolCallParser                 ││
│  │  - parse_tool_calls(response)   ││
│  └─────────────────────────────────┘│
└─────────────────────────────────────┘
```

---

## 📅 时间估算

| 子任务 | 估时 | 优先级 |
|--------|------|--------|
| 1.6.1: 依赖配置 + 构建系统 | 2 小时 | P0 |
| 1.6.2: ModelLoader 实现 | 4 小时 | P0 |
| 1.6.3: InferenceEngine 实现 | 6 小时 | P0 |
| 1.6.4: Tool Calling 支持 | 4 小时 | P0 |
| 1.6.5: 流式响应 | 3 小时 | P1 |
| 1.6.6: 性能监控 | 2 小时 | P1 |
| 1.6.7: 测试 + 文档 | 4 小时 | P0 |
| **总计** | **25 小时** ≈ 3 天 | |

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

- [ ] 冷启动时间 <10s（首次加载模型）
- [ ] 热启动时间 <2s（模型已加载）
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
- Task 1.2 (核心模块) ✅ 已完成
- Task 1.3 (LLM Mock) ✅ 已完成

**后续依赖**:
- Task 1.7 (示例 Agent)
- Task 1.8 (Phase 1 验收)

---

## ⚠️ 风险与缓解

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| llama-cpp-2 构建失败 | 高 | 备选：手动绑定 llama.cpp C API |
| 模型加载内存不足 | 中 | 限制支持 7B 及以下模型 |
| Tool Calling 格式不兼容 | 中 | 使用标准 JSON Schema |
| 性能不达标 | 中 | 提供量化模型推荐 |

---

## 📝 交付物

| 文件 | 说明 |
|------|------|
| `src/llm.rs` | 重构后的 LLM 引擎 |
| `src/model_loader.rs` | 模型加载器 |
| `src/tool_parser.rs` | Tool Call 解析器 |
| `tests/llm_integration.rs` | 集成测试 |
| `examples/chat-agent.yaml` | 示例配置 |
| `docs/inference.md` | 推理引擎文档 |

---

**提案作者**: 虾 (Xia)  
**审核状态**: ⏳ Pending Approval  
**最后更新**: 2026-03-24
