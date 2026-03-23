# Task 1.5: CLI 工具 - MVP

**Change ID**: task-1-5-cli-tool  
**状态**: 🚧 In Progress  
**创建日期**: 2026-03-23  
**优先级**: P0  
**父任务**: Phase 1 (Rust 核心 + Linux MVP)

---

## 📋 提案概述

创建 agently CLI 工具，提供 `agently run` 命令来运行 Agent。

**详细提案**: [proposal.md](proposal.md)

---

## 🏗 技术方案

**设计文档**: [design.md](design.md)

### 核心功能
- Workspace 配置
- `agently run <config>` 命令
- 配置加载集成
- 错误处理

### 交付物
- `cli/` - CLI 二进制
- `Cargo.toml` - Workspace 配置
- `cli/README.md` - 使用说明

---

## 📦 规格说明

**功能规格**: [specs/functional-spec.md](specs/functional-spec.md)

### 用户故事
1. 运行 Agent - 通过命令行运行配置
2. 查看帮助 - 了解命令用法
3. 错误处理 - 清晰的错误信息

---

## ✅ 任务清单

**任务详情**: [tasks.md](tasks.md)

### 当前进度

| Phase | 进度 |
|-------|------|
| Phase 1: Workspace 搭建 | ⏳ Todo |
| Phase 2: CLI 实现 | ⏳ Todo |
| Phase 3: 文档 + 测试 | ⏳ Todo |
| Phase 4: 提交归档 | ⏳ Todo |

---

## 📝 实现状态

### 已完成
- [x] 创建 proposal.md
- [x] 创建 specs/functional-spec.md
- [x] 创建 design.md
- [x] 创建 tasks.md
- [ ] Workspace 搭建
- [ ] CLI 实现
- [ ] 测试验证
- [ ] 提交推送

### 进行中
- 无

### 待开始
- 所有实现任务

---

## 🔗 依赖关系

**前置依赖**: 
- ✅ task-1-4-hello-world-agent

**后续依赖**:
- ⏳ task-4-1-more-cli-commands

---

## 📅 时间线

| 日期 | 事件 |
|------|------|
| 2026-03-23 | 创建任务，编写 artifacts |
| TBD | 开始实现 |
| TBD | 完成实现 |
| TBD | 归档 |

---

**提案作者**: 虾 (Xia)  
**审核状态**: ⏳ Pending  
**最后更新**: 2026-03-23
