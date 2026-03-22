# agently Edge 开发计划

**版本**: 2.0 (Rust 核心架构)  
**更新日期**: 2026-03-22  
**状态**: Approved

---

## 📋 项目概述

**目标**：构建跨平台 Edge AI Agent 运行时，支持 Linux、Android、鸿蒙 Next 三平台。

**核心技术栈**：
- 核心运行时：Rust
- LLM 引擎：llama.cpp (C++)
- Linux CLI：Rust (clap)
- Android 封装：Kotlin + JNI
- 鸿蒙封装：ArkTS + NDK

**开发周期**：12 周

---

## 🏗 技术架构

```
┌─────────────────────────────────────────────────┐
│              agently Edge Runtime               │
├─────────────────────────────────────────────────┤
│                                                  │
│  ┌─────────────────────────────────────────┐    │
│  │   核心运行时 (Rust)                      │    │
│  │   - Agent 调度                           │    │
│  │   - 消息总线                             │    │
│  │   - 工具执行                             │    │
│  │   - 资源管理                             │    │
│  │                                          │    │
│  │   导出 C ABI 接口 (ffi_export)          │    │
│  └─────────────────┬───────────────────────┘    │
│                   │ C FFI                        │
│  ┌────────────────▼───────────────────────┐    │
│  │   LLM 引擎 (llama.cpp C++)             │    │
│  └────────────────────────────────────────┘    │
└─────────────────────────────────────────────────┘
                   │
     ┌─────────────┼─────────────┐
     ▼             ▼             ▼
┌─────────┐  ┌──────────┐  ┌──────────┐
│ Linux   │  │ Android  │  │ 鸿蒙 Next │
│ - Rust  │  │ - Kotlin │  │ - ArkTS  │
│   CLI   │  │ + JNI    │  │ + NDK    │
│ - Binary│  │ - APK    │  │ - HAP    │
└─────────┘  └──────────┘  └──────────┘
```

---

## 📅 Phase 1: Rust 核心 + Linux (Week 1-4)

### 目标
- Rust 核心运行时完成
- CLI 工具可运行
- Linux 完整部署
- 用户文档发布

### Task 列表

#### Week 1: 项目搭建
- [ ] **Task 1.1**: 创建仓库 `agently-edge-runtime`
- [ ] **Task 1.2**: 配置 Rust 项目结构 (Cargo workspace)
- [ ] **Task 1.3**: 配置 CI/CD (GitHub Actions)
- [ ] **Task 1.4**: 配置 Sphinx 文档站点

#### Week 2: 核心模块
- [ ] **Task 2.1**: Agent 生命周期管理
- [ ] **Task 2.2**: 消息总线 (async)
- [ ] **Task 2.3**: 配置加载 (YAML)

#### Week 3: 工具系统
- [ ] **Task 3.1**: 工具注册表
- [ ] **Task 3.2**: HTTP 工具
- [ ] **Task 3.3**: Shell/文件工具
- [ ] **Task 3.4**: C FFI 导出

#### Week 4: CLI + 集成
- [ ] **Task 4.1**: CLI 工具 (clap)
- [ ] **Task 4.2**: llama.cpp 集成
- [ ] **Task 4.3**: 示例 Agent (Hello World)
- [ ] **Task 4.4**: 文档完善 + 发布

### 交付物
- [ ] `libagently.so` (Linux 共享库)
- [ ] `agently` CLI (单二进制)
- [ ] Sphinx 文档站点
- [ ] 示例 Agent (shopping-assistant)

### 验收标准
- [ ] `agently run agent.yaml` 可运行
- [ ] Agent 能调用 HTTP 工具
- [ ] 文档完整（安装、快速开始、API）
- [ ] 测试覆盖率 >80%

---

## 📅 Phase 2: Android + 鸿蒙 (Week 5-10)

### 目标
- Android APK 可部署
- 鸿蒙 HAP 可部署
- 三平台一致 API

### Task 列表

#### Week 5-6: Android
- [ ] **Task 5.1**: Android 项目搭建
- [ ] **Task 5.2**: JNI 绑定
- [ ] **Task 5.3**: Kotlin 封装层
- [ ] **Task 5.4**: APK 打包

#### Week 7-8: 鸿蒙
- [ ] **Task 6.1**: 鸿蒙项目搭建 (DevEco Studio)
- [ ] **Task 6.2**: NDK 绑定
- [ ] **Task 6.3**: ArkTS 封装层
- [ ] **Task 6.4**: HAP 打包

#### Week 9-10: 集成测试
- [ ] **Task 7.1**: 三平台一致性测试
- [ ] **Task 7.2**: 性能优化
- [ ] **Task 7.3**: 部署文档

### 交付物
- [ ] `agently-android.apk`
- [ ] `agently-harmony.hap`
- [ ] 三平台部署指南

### 验收标准
- [ ] Android 设备可安装运行
- [ ] 鸿蒙设备可安装运行
- [ ] 三平台 API 一致

---

## 📅 Phase 3: 生态 (Week 11-12+)

### 目标
- 开发者友好
- 生态工具完善

### Task 列表

#### Week 11: Python 绑定 (可选)
- [ ] **Task 8.1**: PyO3 绑定
- [ ] **Task 8.2**: Python SDK
- [ ] **Task 8.3**: pip 发布

#### Week 12: 生态工具
- [ ] **Task 9.1**: 应用市场 (MVP)
- [ ] **Task 9.2**: 云端管理 (MVP)
- [ ] **Task 9.3**: 开发者社区

### 交付物
- [ ] Python SDK (可选)
- [ ] 应用市场 MVP
- [ ] 云端管理 MVP

---

## 📊 关键指标

### 技术指标

| 指标 | 目标 | 测量方式 |
|------|------|----------|
| 冷启动时间 | <5s | 启动到可响应 |
| 对话响应延迟 | <500ms | 消息到响应 |
| 内存占用 (核心) | <50MB | RSS |
| 二进制大小 | <50MB | Linux CLI |
| 测试覆盖率 | >80% | cargo tarpaulin |

### 业务指标 (Phase 1)

| 指标 | 目标 |
|------|------|
| 合作硬件厂商 | 2-3 家 |
| 预装设备数 | 1000+ |
| GitHub Stars | 500+ |
| 开发者社区 | 100+ |

---

## 🎯 第一个 Task: Task 1.1

**名称**: 创建仓库 `agently-edge-runtime`

**内容**:
1. 在 GitHub 创建新仓库
2. 配置仓库基本信息 (README, License, .gitignore)
3. 配置分支策略 (main + develop)
4. 配置保护分支

**验收**:
- [ ] 仓库可访问
- [ ] README 包含项目概述
- [ ] License (Apache 2.0)
- [ ] .gitignore (Rust + IDE)

---

**文档维护**: agently Team  
**最后更新**: 2026-03-22
