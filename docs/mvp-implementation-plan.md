# agently Edge MVP 实施计划

**版本**: 1.0  
**日期**: 2026-03-21  
**范围**: 方案 B2 - 平台 + 基础语音（3 个月）  
**目标设备**: RK3566 (2GB 内存，Android 10+)

---

## 计划概览

**目标**：在 3 个月内完成 agently Edge MVP，实现可演示的智能导购 Agent 场景

**核心交付物**：
1. Agent 运行时核心（Python）
2. 语音交互模块（Whisper + Piper）
3. 智能导购 Agent 示例
4. Python SDK + CLI 工具
5. 设备管理后台（基础版）

**执行方式**：Subagent-Driven Development（claw-superpowers Section 5）
- 每个任务由独立子代理实现
- 每项任务经过 Spec Review + Code Quality Review
- TDD 强制（先写测试，后写实现）

---

## Task 列表

| # | 任务 | 估时 | 优先级 | 状态 |
|---|------|------|--------|------|
| 1 | 项目初始化 | 0.5 天 | P0 | ⬜ |
| 2 | Agent 基类实现 | 2 天 | P0 | ⬜ |
| 3 | llama.cpp 推理引擎集成 | 3 天 | P0 | ⬜ |
| 4 | Whisper 语音识别集成 | 2 天 | P0 | ⬜ |
| 5 | Piper TTS 语音合成集成 | 2 天 | P0 | ⬜ |
| 6 | 商品查询工具实现 | 2 天 | P0 | ⬜ |
| 7 | 智能导购 Agent 实现 | 3 天 | P0 | ⬜ |
| 8 | 音频播放模块 | 1 天 | P1 | ⬜ |
| 9 | UI 界面（Webview） | 3 天 | P1 | ⬜ |
| 10 | 设备状态上报 | 2 天 | P1 | ⬜ |
| 11 | Python SDK 封装 | 2 天 | P1 | ⬜ |
| 12 | CLI 工具实现 | 2 天 | P1 | ⬜ |
| 13 | 云端后台（MVP） | 3 天 | P2 | ⬜ |
| 14 | 性能优化 | 3 天 | P0 | ⬜ |
| 15 | 文档和测试 | 3 天 | P1 | ⬜ |

**总计**: 33 天 ≈ 7 周（考虑并行和缓冲，3 个月完成）

---

## 详细任务说明

### Task 1: 项目初始化（0.5 天）

**Files:**
- 完善：`pyproject.toml`
- 创建：`core/__init__.py`
- 创建：`core/agent/__init__.py`
- 创建：`core/inference/__init__.py`
- 创建：`core/tools/__init__.py`
- 创建：`sdk/python/agently/__init__.py`
- 创建：`cli/agently_cli/__init__.py`

**Steps:**
1. 完善 pyproject.toml（添加语音依赖）
2. 创建模块 __init__.py 文件
3. 验证项目结构
4. Commit

**Commands:**
```bash
mkdir -p core/{agent,inference,tools}
mkdir -p sdk/python/agently
mkdir -p cli/agently_cli
touch core/__init__.py core/agent/__init__.py core/inference/__init__.py core/tools/__init__.py
touch sdk/python/agently/__init__.py cli/agently_cli/__init__.py
git add -A && git commit -m "feat: 初始化项目结构"
```

---

### Task 2: Agent 基类实现（2 天）

**Files:**
- 创建：`core/agent/base.py`
- 创建：`tests/unit/test_agent_base.py`

**TDD Steps:**
1. 编写测试（test_agent_base.py）
2. 运行测试，验证失败（Red）
3. 实现 Agent 基类（base.py）
4. 运行测试，验证通过（Green）
5. 重构（Refactor）
6. Commit

**Key Classes:**
- `AgentContext`: 对话上下文管理
- `Tool`: 工具基类
- `Agent`: Agent 基类

**Verification:**
```bash
python -m pytest tests/unit/test_agent_base.py -v
# 期望：3 passed
```

---

### Task 3: llama.cpp 推理引擎集成（3 天）

**Files:**
- 创建：`core/inference/llama_engine.py`
- 创建：`tests/unit/test_llama_engine.py`

**Dependencies:**
```bash
pip install llama-cpp-python
```

**Key Methods:**
- `LlamaEngine.generate()`: 文本生成
- `LlamaEngine.chat()`: 对话模式
- `LlamaEngine._build_chat_prompt()`: 构建对话提示

**Model:**
- Qwen2.5-1.5B-Instruct-GGUF (Q4_K_M)
- 下载：https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF

**Verification:**
```bash
python -m pytest tests/unit/test_llama_engine.py -v
```

---

### Task 4: Whisper 语音识别集成（2 天）

**Files:**
- 创建：`core/inference/whisper_asr.py`
- 创建：`tests/unit/test_whisper_asr.py`

**Dependencies:**
```bash
pip install openai-whisper
```

**Key Methods:**
- `WhisperASR.transcribe()`: 文件转写
- `WhisperASR.transcribe_live()`: 实时转写

**Model:**
- openai/whisper-tiny (~100MB)
- 首次运行自动下载

**Performance:**
- 延迟：<200ms
- 内存：<100MB

---

### Task 5: Piper TTS 语音合成集成（2 天）

**Files:**
- 创建：`core/inference/piper_tts.py`
- 创建：`tests/unit/test_piper_tts.py`

**Dependencies:**
```bash
pip install piper-tts
```

**Model:**
- zh_CN-Xiaoxiao-medium (~80MB)
- 下载：https://huggingface.co/rhasspy/piper-voices

**Verification:**
```bash
python -c "from core.inference.piper_tts import PiperTTS; tts = PiperTTS('model.onnx'); tts.synthesize('你好', 'out.wav')"
```

---

### Task 6: 商品查询工具实现（2 天）

**Files:**
- 创建：`core/tools/product_query.py`
- 创建：`tests/unit/test_product_query.py`
- 创建：`scripts/seed_products.py`
- 创建：`data/products.db`

**Tools:**
- `ProductQueryTool`: 商品检索
- `StockCheckTool`: 库存查询

**Database Schema:**
```sql
CREATE TABLE products (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    category TEXT,
    price REAL,
    brand TEXT,
    rating REAL,
    image_url TEXT,
    description TEXT
);

CREATE TABLE inventory (
    product_id INTEGER,
    size TEXT,
    stock INTEGER,
    location TEXT,
    FOREIGN KEY (product_id) REFERENCES products(id)
);
```

**Seed Data:**
- 50+ 示例商品
- 多尺码库存

---

### Task 7: 智能导购 Agent 实现（3 天）

**Files:**
- 创建：`examples/customer_service/agent.py`
- 创建：`examples/customer_service/config.yaml`
- 创建：`examples/customer_service/run.py`

**Agent Flow:**
```
用户语音 → Whisper → 文字 → LLM → 回复文字 → Piper → 语音播放
                    ↓
              工具调用（商品查询/库存检查）
```

**System Prompt:**
```
你是一个门店导购助手，热情友好地帮助顾客。
职责：推荐商品、查询库存、解答问题、引导到店员
回答简洁，每次不超过 3 句话。
```

**Verification:**
```bash
cd examples/customer_service
python run.py
# 测试多轮对话
```

---

### Task 8: 音频播放模块（1 天）

**Files:**
- 创建：`core/inference/audio_player.py`

**Dependencies:**
```bash
pip install sounddevice soundfile
```

**Key Methods:**
- `AudioPlayer.play()`: 播放文件
- `AudioPlayer.play_memory()`: 播放内存音频
- `AudioPlayer.record()`: 录音

---

### Task 9: UI 界面（Webview）（3 天）

**Files:**
- 创建：`ui/webview/index.html`
- 创建：`ui/webview/app.js`
- 创建：`ui/webview/style.css`

**Screens:**
1. 待机界面（数字人/Logo + 问候语）
2. 对话界面（语音波形 + 转写文字）
3. 商品展示界面（商品卡片）

**Tech:**
- HTML/CSS/JavaScript
- PyWebView 集成

---

### Task 10: 设备状态上报（2 天）

**Files:**
- 创建：`core/device/status.py`
- 创建：`core/device/heartbeat.py`

**Features:**
- 心跳上报（5 分钟间隔）
- CPU/内存/温度监控
- 离线检测

---

### Task 11: Python SDK 封装（2 天）

**Files:**
- 创建：`sdk/python/agently/client.py`
- 创建：`sdk/python/agently/agent.py`

**API:**
```python
from agently import Agent

agent = Agent(name="my-agent")
agent.register_tool(my_tool)
response = agent.chat("你好")
```

---

### Task 12: CLI 工具实现（2 天）

**Files:**
- 创建：`cli/agently_cli/main.py`
- 创建：`cli/agently_cli/commands.py`

**Commands:**
```bash
agently create <name>     # 创建新 Agent
agently deploy <name>     # 部署到设备
agently status            # 查看状态
agently logs              # 查看日志
```

---

### Task 13: 云端后台（MVP）（3 天）

**Files:**
- 创建：`backend/api/main.py`
- 创建：`backend/dashboard/pages/index.py`

**Tech:**
- FastAPI（后端 API）
- Streamlit/Next.js（管理看板）
- SQLite/PostgreSQL（数据库）

**Features:**
- 设备列表
- 状态监控
- 配置下发

---

### Task 14: 性能优化（3 天）

**Focus:**
1. 内存优化（动态加载、缓存清理）
2. 推理速度优化（量化、线程）
3. 启动速度优化（预加载）

**Targets:**
- 响应延迟：<800ms
- 内存占用：<1.8GB
- 冷启动：<5s

---

### Task 15: 文档和测试（3 天）

**Deliverables:**
- README.md（完善）
- docs/getting-started.md
- docs/api-reference.md
- docs/deployment.md
- 单元测试（覆盖率>80%）
- 端到端测试

---

## 里程碑

| 里程碑 | 任务 | 时间 | 交付物 |
|--------|------|------|--------|
| **M1** | Task 1-4 | Week 4 | 核心运行时 + 模型推理 |
| **M2** | Task 5-7 | Week 7 | 智能导购场景可演示 |
| **M3** | Task 8-15 | Week 12 | MVP v0.1.0 发布 |

---

## 风险与应对

| 风险 | 概率 | 影响 | 应对 |
|------|------|------|------|
| RK3566 性能不足 | 中 | 高 | 准备 RK3568 备选 |
| 内存超限 | 高 | 高 | 模型量化、动态加载 |
| 语音识别率低 | 中 | 中 | 降噪、热词优化 |
| 开发延期 | 中 | 中 | 砍非核心功能 |

---

## 下一步

1. **确认计划**：审查本计划，调整范围和优先级
2. **OpenSpec 提案**：基于确认的计划创建 OpenSpec Proposal
3. **开始实现**：从 Task 1 开始，按顺序执行

---

**Made with 🦐 by agently Team**
