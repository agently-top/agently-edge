# agently Edge

**Edge AI Agent Runtime for Commercial Display Devices**

让每一台商显设备都能运行 AI Agent

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20Android%20%7C%20Harmony-green.svg)](https://github.com/agently-top/agently-edge)

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

## 🚀 四大价值维度

| 维度 | 传统方案 | agently Edge |
|------|---------|-------------|
| 💬 **人机互动** | 被动播放，无交互 | 语音/触摸交互，主动问候和推荐 |
| 🎨 **内容生成** | 人工设计素材，周期长 | AI 自动生成文案，千人千面 |
| 📁 **内容管理** | U 盘手动更新，单设备管理 | 云端统一下发，智能投放 |
| 🔧 **设备管理** | 故障后报修，被动维修 | 远程监控，预测性维护 |

---

## 🏪 典型场景

### 场景 1：智能导购 Agent（小屏广告机）

```
顾客路过广告屏
    ↓
Agent 主动问候："您好，需要帮您推荐商品吗？"
    ↓
顾客语音提问："我想买运动鞋，有什么推荐？"
    ↓
Agent 理解需求 → 查询商品库 → 推荐 3 款
    ↓
屏幕显示商品图片 + 价格 + 库存
```

**技术需求**：
- 语音识别：Whisper-Tiny (100MB)
- 对话理解：Qwen2.5-1.5B-Q4 (1GB)
- 语音合成：Piper TTS (100MB)
- **总内存**：~1.5GB（L2 设备可行）

---

### 场景 2：会议助手 Agent（会议一体机）

```
会议开始
    ↓
Agent 自动识别参会者
    ↓
会议进行中：实时转写 + 翻译
    ↓
有人提问："刚才说的 Q3 目标是多少？"
    ↓
Agent 检索会议记录 → "Q3 目标是营收 5000 万"
    ↓
会议结束 → 自动生成纪要 → 邮件发送
```

**技术需求**：
- 语音识别：Whisper-Base (200MB)
- 对话理解：Qwen2.5-1.5B-Q4 (1GB)
- 实时翻译：NLLB/MarianMT (300MB)
- **总内存**：~1.6GB（L2 设备可行）

---

### 场景 3：展厅讲解 Agent（户外大屏）

```
观众走到展品前
    ↓
Agent 检测 → 主动讲解："这是 XX 时期的文物..."
    ↓
观众提问："它是怎么制作的？"
    ↓
Agent 检索知识库 → 详细解答
    ↓
观众："有视频展示吗？"
    ↓
Agent 播放制作流程视频
```

**技术需求**：
- 语音识别：Whisper-Tiny (100MB)
- 对话理解：Qwen2.5-1.5B-Q4 (1GB)
- 知识库检索：向量数据库 (300MB)
- **总内存**：~1.5GB（L2 设备可行）

---

## 🛠️ 技术架构

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
│  │   - 直接集成，无额外绑定                │    │
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

## 🔧 开发工具

### 安装 CLI (Linux)

```bash
# 方式 1: 下载二进制
wget https://github.com/agently-top/agently-edge/releases/download/v1.0.0/agently-linux-x64
chmod +x agently-linux-x64
sudo mv agently-linux-x64 /usr/local/bin/agently

# 方式 2: 包管理 (Phase 1 后)
apt install agently-edge  # deb
yum install agently-edge  # rpm
```

### 创建第一个 Agent

```bash
# 创建新项目
agently create my-agent
cd my-agent

# 开发模式运行
agently dev

# 运行 Agent
agently run agent.yaml

# 部署到设备
agently deploy --device <device-id>
```

### Agent 配置示例 (YAML)

```yaml
# agent.yaml
agent_id: shopping-assistant
name: 智能导购
model:
  path: /models/Qwen2.5-1.5B-Instruct-Q4_K_M.gguf
  context_length: 4096
  temperature: 0.7

tools:
  - name: query_product
    type: http
    endpoint: http://localhost:8080/api/products

prompts:
  system: |
    你是一个智能导购助手，帮助用户找到合适的商品。
    语气友好、专业。
  greeting: 欢迎光临！有什么可以帮您的吗？
```

---

## 📚 文档

- [快速开始](docs/getting-started.md)
- [API 参考](docs/api-reference.md)
- [部署指南](docs/deployment.md)
- [示例应用](examples/)

---

## 🤝 开源协议

Apache 2.0 License - 允许商业使用，需保留版权声明

---

## 📬 联系方式

- GitHub: [@agently-top](https://github.com/agently-top)
- 网站：[agently.top](https://agently.top)
- 邮箱：hello@agently.top

---

## 🗺️ 路线图

### Phase 1 (2026 Q2): MVP - Rust 核心 + Linux
**Week 1-4**
- [ ] Rust 核心运行时（Agent 调度 + 工具调用 + C FFI）
- [ ] CLI 工具（Rust，单二进制）
- [ ] 示例应用（智能导购）
- [ ] 用户文档（Sphinx）
- [ ] Linux 部署包

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

**Made with 🦐 by agently Team**
