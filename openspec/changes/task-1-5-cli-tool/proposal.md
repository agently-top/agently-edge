# Task 1.5: CLI 工具 - MVP

**Change ID**: task-1-5-cli-tool  
**状态**: 📝 Proposed  
**创建日期**: 2026-03-23  
**优先级**: P0  
**父任务**: Phase 1 (Rust 核心 + Linux MVP)

---

## 📋 提案概述

创建 agently CLI 工具，提供 `agently run` 命令来运行 Agent。

### 目标

- 实现最小可行 CLI (`agently run <config>`)
- 集成 runtime 配置加载功能
- 提供友好的错误信息
- 为后续 `create`/`deploy` 命令奠定基础

### 范围

**包含**：
- `cli/` 目录创建
- Workspace 配置 (`Cargo.toml`)
- `agently run` 命令实现
- 基础帮助文档
- 单元测试

**不包含**：
- `create`/`deploy`/`status` 命令（后续任务）
- 复杂日志系统（MVP 后用 tracing）
- 自动完成脚本

---

## 🎯 成功标准

### 功能标准
- [ ] `agently run <config.yaml>` 可运行
- [ ] `agently --help` 显示帮助
- [ ] `agently run --help` 显示命令帮助
- [ ] 错误配置有清晰错误信息
- [ ] 缺失参数有友好提示

### 质量标准
- [ ] 代码通过 `cargo fmt`
- [ ] 无 `cargo clippy` 警告
- [ ] 单元测试通过
- [ ] Workspace 编译通过

### 文档标准
- [ ] CLI README 包含使用说明
- [ ] 命令帮助完整清晰

---

## 🔗 依赖关系

**前置依赖**：
- ✅ task-1-4-hello-world-agent (配置加载 API)
- ✅ runtime 核心模块

**后续依赖**：
- ⏳ task-4-1-more-cli-commands (扩展命令)
- ⏳ task-4-2-deployment (部署打包)

---

## ⏱ 时间估算

| 模块 | 预估时间 |
|------|----------|
| Workspace 搭建 | 0.5 小时 |
| CLI 命令实现 | 1 小时 |
| 配置加载集成 | 0.5 小时 |
| 错误处理 | 0.5 小时 |
| 测试验证 | 0.5 小时 |
| **总计** | **3 小时** |

---

## 📐 技术方案

**架构**: Rust workspace + clap v4

**结构**:
```
agently-edge/
├── Cargo.toml (workspace)
├── runtime/ (library)
└── cli/ (binary)
```

**命令设计**:
```bash
agently run <config-path> [--verbose]
```

详细设计见：`docs/plans/2026-03-23-cli-design.md`

---

**提案作者**: 虾 (Xia)  
**审核状态**: ⏳ Pending  
**最后更新**: 2026-03-23
