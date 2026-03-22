# Task 1.1: Runtime 子项目基础设施搭建

**Change ID**: task-1-1-runtime-setup  
**状态**: ✅ Completed  
**创建日期**: 2026-03-22  
**优先级**: P0 (Phase 1 第一个任务)
**完成日期**: 2026-03-22 17:38
**合并 Commit**: 13a1745

---

## 📋 提案概述

在当前 `agently-edge` 仓库下创建 `runtime/` 子项目，作为 Rust 核心运行时的代码载体。

**目标**：
- 建立 Rust 项目基础结构
- 配置 Cargo workspace
- 配置 CI/CD 流程
- 配置 Sphinx 文档系统

**范围**：
- ✅ 创建 `runtime/` 目录结构
- ✅ 配置 Rust 项目 (Cargo.toml)
- ✅ 配置 Sphinx 文档
- ✅ 配置 GitHub Actions CI
- ✅ 更新根目录配置

---

## 🎯 业务目标

1. **可运行的 Rust 项目** - 开发者可以 `cargo build` 编译
2. **CI 自动化** - 每次提交自动构建测试
3. **文档基础** - Sphinx 文档站点可访问
4. **团队协同** - 代码规范、分支策略明确

---

## 🏗 技术方案

### 目录结构

```
agently-edge/
├── runtime/                    # Rust 核心运行时
│   ├── Cargo.toml              # Rust 项目配置
│   ├── src/
│   │   ├── lib.rs              # 库入口
│   │   ├── runtime.rs          # Runtime 核心
│   │   ├── agent.rs            # Agent 管理
│   │   ├── message_bus.rs      # 消息总线
│   │   ├── tools/              # 工具系统
│   │   │   ├── mod.rs
│   │   │   ├── registry.rs
│   │   │   ├── http.rs
│   │   │   └── shell.rs
│   │   └── ffi.rs              # C FFI 导出
│   └── tests/
│       ├── runtime_test.rs
│       └── tools_test.rs
├── docs/                       # Sphinx 文档
│   ├── source/
│   │   ├── conf.py
│   │   ├── index.rst
│   │   ├── installation.rst
│   │   └── quickstart.rst
│   └── requirements.txt
├── .github/
│   └── workflows/
│       ├── ci.yml              # Rust CI
│       └── docs.yml            # 文档构建
├── .gitignore                  # 更新 (添加 Rust)
└── README.md                   # 更新 (添加 Runtime 说明)
```

### 技术栈

| 组件 | 技术 | 版本 |
|------|------|------|
| Rust | 编程语言 | 1.75+ |
| Cargo | 包管理 | - |
| tokio | 异步运行时 | 1.35+ |
| serde | 序列化 | 1.0+ |
| clap | CLI 框架 | 4.4+ |
| Sphinx | 文档生成 | 7.0+ |
| GitHub Actions | CI/CD | - |

---

## 📦 交付物

| 交付物 | 说明 | 验收标准 |
|--------|------|----------|
| `runtime/Cargo.toml` | Rust 项目配置 | `cargo build` 成功 |
| `runtime/src/lib.rs` | 库入口 | 编译无警告 |
| `docs/source/index.rst` | 文档首页 | `make html` 成功 |
| `.github/workflows/ci.yml` | CI 配置 | PR 自动触发 |
| `.gitignore` | 更新 | 包含 Rust 规则 |

---

## ✅ 验收标准

### 功能验收
- [ ] `cd runtime && cargo build` 成功
- [ ] `cd runtime && cargo test` 通过
- [ ] `cd docs && make html` 生成文档
- [ ] GitHub Actions CI 自动运行

### 质量验收
- [ ] 代码无 clippy 警告
- [ ] 代码通过 rustfmt 格式化
- [ ] 文档无语法错误

### 文档验收
- [ ] README 包含项目概述
- [ ] 安装指南完整
- [ ] 快速开始可操作

---

## ⏱ 时间估算

| 任务 | 时间 |
|------|------|
| 目录结构创建 | 30 分钟 |
| Cargo 配置 | 1 小时 |
| Sphinx 配置 | 1 小时 |
| CI/CD 配置 | 1 小时 |
| 文档编写 | 1 小时 |
| **总计** | **4.5 小时** |

---

## 🔗 依赖关系

**前置依赖**: 无  
**后续依赖**: Task 1.2 (核心模块开发)

---

## 📝 备注

- 使用 Cargo workspace 管理多 crate（为后续 CLI、bindings 预留）
- Sphinx 文档部署到 GitHub Pages
- CI 使用 GitHub Actions（免费额度足够）

---

## 📝 完成总结

**已完成任务**:
- [x] Task 1.1.1: 创建目录结构
- [x] Task 1.1.2: 配置 Rust 项目
- [x] FFI 安全修复：空指针检查、销毁函数、线程安全、内存泄漏

**交付物**:
- `runtime/Cargo.toml` - Rust 项目配置
- `runtime/src/` - 10 个 Rust 源文件
- `docs/source/` - 文档目录
- `.github/workflows/` - CI 目录
- `.gitignore` - Git 忽略规则

**Git 历史**:
- `13a1745` - Merge task-1-1-runtime-setup
- 已合并到 master 分支

---

**提案作者**: 虾 (Xia)  
**审核状态**: ✅ Approved  
**最后更新**: 2026-03-22 17:38
