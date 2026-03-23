# Tasks: llama.cpp 集成

**Task ID**: task-1-6-llama-cpp-integration  
**创建日期**: 2026-03-24  
**状态**: ⏳ Pending Approval

---

## 任务分解

### Task 1.6.1: 依赖配置 + 构建系统

**估时**: 2 小时  
**优先级**: P0

**步骤**:
1. [ ] 更新 `runtime/Cargo.toml` 添加 llama-cpp-2 依赖
2. [ ] 运行 `cargo fetch` 验证依赖可下载
3. [ ] 配置 build.rs（如需自定义绑定）
4. [ ] 验证 `cargo check` 通过

**验收**:
- [ ] cargo check 无错误
- [ ] 依赖版本锁定

---

### Task 1.6.2: ModelLoader 实现

**估时**: 4 小时  
**优先级**: P0

**步骤**:
1. [ ] 创建 `src/llm/loader.rs`
2. [ ] 实现 `load_gguf()` 方法
3. [ ] 实现 `unload()` 方法
4. [ ] 实现 `validate_gguf()` 方法
5. [ ] 编写单元测试（mock 文件）
6. [ ] 通过 clippy 和 rustfmt

**验收**:
- [ ] 可加载测试模型
- [ ] 错误处理完整
- [ ] 测试覆盖率 >80%

---

### Task 1.6.3: InferenceEngine 实现

**估时**: 6 小时  
**优先级**: P0

**步骤**:
1. [ ] 创建 `src/llm/inference.rs`
2. [ ] 实现 `generate()` 方法
3. [ ] 实现 `chat()` 方法
4. [ ] 实现 tokenization/detokenization
5. [ ] 实现采样器配置（temperature, top_p）
6. [ ] 编写单元测试
7. [ ] 性能基准测试

**验收**:
- [ ] 可生成连贯文本
- [ ] 多轮对话正常
- [ ] 性能指标正确收集

---

### Task 1.6.4: Tool Calling 支持

**估时**: 4 小时  
**优先级**: P0

**步骤**:
1. [ ] 创建 `src/llm/tools/` 目录
2. [ ] 实现 `ToolDefinition` 结构
3. [ ] 实现 `ToolCallParser::format_tools()`
4. [ ] 实现 `ToolCallParser::parse()`
5. [ ] 实现 `chat_with_tools()` 方法
6. [ ] 实现 `submit_tool_result()` 方法
7. [ ] 编写集成测试

**验收**:
- [ ] 可正确解析 tool calls
- [ ] 可注入 tool results
- [ ] 支持多轮 tool 对话

---

### Task 1.6.5: 流式生成

**估时**: 3 小时  
**优先级**: P1

**步骤**:
1. [ ] 创建 `src/llm/streaming.rs`
2. [ ] 实现 `stream_generate()` 方法
3. [ ] 实现 `StreamChunk` 枚举
4. [ ] 集成 tokio-stream
5. [ ] 编写流式测试

**验收**:
- [ ] 可流式输出 tokens
- [ ] 支持取消流
- [ ] 最后返回统计信息

---

### Task 1.6.6: 性能监控

**估时**: 2 小时  
**优先级**: P1

**步骤**:
1. [ ] 创建 `src/llm/stats.rs`
2. [ ] 实现 `InferenceStats` 结构
3. [ ] 实现 `get_stats()` 方法
4. [ ] 添加性能日志
5. [ ] 编写基准测试

**验收**:
- [ ] 可查询 tokens/s
- [ ] 可查询内存占用
- [ ] 日志包含性能信息

---

### Task 1.6.7: 测试 + 文档

**估时**: 4 小时  
**优先级**: P0

**步骤**:
1. [ ] 编写集成测试 `tests/llm_integration.rs`
2. [ ] 创建示例配置 `examples/chat-agent.yaml`
3. [ ] 编写文档 `docs/inference.md`
4. [ ] 更新 README.md
5. [ ] 运行完整测试套件
6. [ ] 修复 clippy 警告
7. [ ] 运行 rustfmt

**验收**:
- [ ] 所有测试通过
- [ ] 文档完整
- [ ] 代码质量达标

---

## 依赖关系

```
Task 1.6.1 (依赖配置)
    ↓
Task 1.6.2 (ModelLoader)
    ↓
Task 1.6.3 (InferenceEngine)
    ↓
Task 1.6.4 (Tool Calling)
    ↓
Task 1.6.5 (流式生成) ───┐
    ↓                    │
Task 1.6.6 (性能监控) ───┤
    ↓                    │
Task 1.6.7 (测试 + 文档) ←┘
```

---

## 总估时

| 阶段 | 时间 |
|------|------|
| 基础设施 | 2 小时 |
| 核心实现 | 14 小时 |
| Tool Calling | 4 小时 |
| 高级功能 | 5 小时 |
| 测试 + 文档 | 4 小时 |
| **总计** | **29 小时** ≈ 3-4 天 |

---

## 风险缓冲

- 构建问题：+4 小时
- API 不兼容：+4 小时
- 性能优化：+4 小时

**最坏情况**: 41 小时 ≈ 5 天

---

**创建者**: 虾 (Xia)  
**最后更新**: 2026-03-24
