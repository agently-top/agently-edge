# agently Edge

**Edge AI Agent Runtime for Commercial Display Devices**

让每一台商显设备都能运行 AI Agent

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20Android%20%7C%20Harmony-green.svg)](https://github.com/agently-top/agently-edge)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://agently-top.github.io/agently-edge/)

---

## 🎯 产品定位

**agently Edge** 是一个运行在边缘设备上的 AI Agent 运行时平台，专为商显设备设计：

- 📱 **小屏广告机** (10-32 寸) — 门店橱窗、收银台、电梯间
- 🖥️ **户外大屏商显** (43-98 寸) — 商场中庭、楼宇外墙、广场大屏
- 🎤 **会议一体机** (55-86 寸) — 企业会议室、政府会议厅、培训室

---

## 💡 核心价值

### 对硬件厂商
> **同样的硬件，溢价 50%，还能收持续服务费**

- ✅ 预装即支持 AI Agent（语音交互、智能推荐、自动响应）
- ✅ 差异化卖点，提升产品竞争力
- ✅ 云端服务持续收费（设备管理、模型调用）

### 对应用开发者
> **一次开发，部署 10000+ 台设备，延迟降低 90%**

- ✅ 统一 SDK，无需适配硬件差异
- ✅ 本地推理，低延迟 (<50ms)、数据不出设备
- ✅ 通过 agently 设备网络直接触达客户

### 对终端客户
> **把"显示设备"变成"智能助手"**

- ✅ 人力成本降低（自动服务）
- ✅ 转化效率提升（智能推荐）
- ✅ 管理效率提升（远程管控）

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

### 技术栈

| 层级 | 技术 | 说明 |
|------|------|------|
| **核心运行时** | Rust | 跨平台编译，高性能，内存安全 |
| **LLM 引擎** | llama.cpp (C++) | 直接集成 |
| **CLI (Linux)** | Rust (clap) | 单二进制，无依赖 |
| **Android 封装** | Kotlin + JNI | APK 部署 |
| **鸿蒙封装** | ArkTS + NDK | HAP 部署 |

---

## 📦 设备要求

| 等级 | CPU | 内存 | 存储 | 可运行模型 |
|------|-----|------|------|-----------|
| **L1 入门** | 4 核 ARM | 1GB | 8GB | Qwen2.5-0.5B (Q4) |
| **L2 主流** | 8 核 ARM | 2GB | 16GB | Qwen2.5-1.5B (Q4) |
| **L3 高端** | 8 核+ | 4GB | 32GB | Qwen2.5-7B (Q4) |

**目标设备**：L2 主流 (2GB 内存，8 核 ARM)

**核心运行时内存占用**：
- Linux: <50MB (Rust 原生)
- Android: <200MB (含 JVM)
- 鸿蒙：<200MB (含 ArkTS 运行时)

---

## 🚀 快速开始

### 1. 安装 (Linux)

```bash
# 下载二进制
wget https://github.com/agently-top/agently-edge/releases/download/v1.0.0/agently-linux-x64
chmod +x agently-linux-x64
sudo mv agently-linux-x64 /usr/local/bin/agently

# 验证安装
agently --version
```

### 2. 准备模型

```bash
# 下载 Qwen2.5-1.5B-Instruct (Q4_K_M)
wget https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF/resolve/main/qwen2.5-1.5b-instruct-q4_k_m.gguf
sudo mkdir -p /opt/agently/models
sudo mv qwen2.5-1.5b-instruct-q4_k_m.gguf /opt/agently/models/
```

### 3. 创建 Agent 配置

创建 `agent.yaml`：

```yaml
agent_id: hello-world
name: Hello World Agent
model:
  path: /opt/agently/models/qwen2.5-1.5b-instruct-q4_k_m.gguf
  context_length: 4096
  temperature: 0.7

prompts:
  system: |
    你是一个友好的 AI 助手。
    用简洁、专业的语言回答问题。
  greeting: 你好！我是 agently Edge Agent，有什么可以帮你的吗？

output:
  type: text
```

### 4. 运行 Agent

```bash
agently run agent.yaml
```

**详细文档**: https://agently-top.github.io/agently-edge/

---

## 📚 文档

- [安装指南](https://agently-top.github.io/agently-edge/installation.html)
- [快速开始](https://agently-top.github.io/agently-edge/quickstart.html)
- [API 参考](https://agently-top.github.io/agently-edge/api.html)

---

## 🗺️ 路线图

### Phase 1 (2026 Q2): MVP - Rust 核心 + Linux
**Week 1-4**
- [x] Rust 核心运行时（Agent 调度 + 工具调用 + C FFI）
- [x] CLI 工具（Rust，单二进制）
- [x] Sphinx 文档系统
- [x] GitHub Actions CI/CD
- [ ] 示例应用（Hello World）

### Phase 2 (2026 Q3): Android + 鸿蒙
**Week 5-10**
- [ ] Android 封装（Kotlin + JNI，APK）
- [ ] 鸿蒙封装（ArkTS + NDK，HAP）
- [ ] 会议助手 Agent
- [ ] 展厅讲解 Agent

### Phase 3 (2026 Q4): 生态
**Week 11-12+**
- [ ] Python 绑定（PyO3，可选）
- [ ] 开发者文档完善
- [ ] 应用市场（MVP）
- [ ] 云端管理平台（MVP）

---

## 🤝 开源协议

Apache 2.0 License - 允许商业使用，需保留版权声明

---

## 📬 联系方式

- GitHub: [@agently-top](https://github.com/agently-top)
- 文档：https://agently-top.github.io/agently-edge/
- 社区：https://discord.gg/clawd

---

**Made with 🦐 by agently Team**
