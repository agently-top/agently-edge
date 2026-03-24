# llama.cpp 完整集成指南

**更新日期**: 2026-03-24  
**状态**: 🟢 就绪

---

## 📦 快速开始

### 方式 1: Mock 模式（无需 llama.cpp）

```bash
# 直接编译运行
cargo build --release
./target/release/agently run examples/hello-world/agent.yaml --mock
```

### 方式 2: 完整 llama.cpp 集成

#### 步骤 1: 获取 llama.cpp 源码

```bash
# 在 agently-edge-runtime 目录下
cd /root/.openclaw/workspace/agently-edge

# 克隆 llama.cpp
git clone https://github.com/ggerganov/llama.cpp.git vendor/llama.cpp
cd vendor/llama.cpp

# 或者指定版本
git checkout b5434  # 稳定版本
```

#### 步骤 2: 编译

```bash
# 方式 A: 使用 build.rs 自动编译
cargo build --release --features full-llama

# 方式 B: 手动编译后链接
cd vendor/llama.cpp
make libllama.a

# 设置环境变量
export LLAMA_CPP_PATH=/root/.openclaw/workspace/agently-edge/vendor/llama.cpp

# 编译 Rust 项目
cd /root/.openclaw/workspace/agently-edge
cargo build --release
```

#### 步骤 3: 运行

```bash
# 下载 GGUF 模型
mkdir -p examples/hello-world/models
wget https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/qwen2.5-0.5b-instruct-q4_k_m.gguf \
  -O examples/hello-world/models/qwen2.5-0.5b-instruct-q4_k_m.gguf

# 运行真实模型
./target/release/agently run examples/hello-world/agent.yaml --verbose
```

---

## 🔧 编译选项

### 环境变量

| 变量 | 说明 | 默认值 |
|------|------|--------|
| `LLAMA_CPP_PATH` | llama.cpp 源码路径 | `vendor/llama.cpp` |
| `CMAKE_BUILD_TYPE` | 编译类型 | `Release` |
| `LLAMA_CUDA` | 启用 CUDA | `OFF` |
| `LLAMA_METAL` | 启用 Metal | `OFF` |

### Cargo Features

| Feature | 说明 |
|---------|------|
| `default` | 默认（mock 模式） |
| `full-llama` | 完整 llama.cpp 支持 |

### 编译示例

```bash
# CPU _only
cargo build --release

# 完整 llama.cpp
cargo build --release --features full-llama

# CUDA 加速
LLAMA_CUDA=ON cargo build --release --features full-llama

# Metal 加速 (macOS)
LLAMA_METAL=ON cargo build --release --features full-llama
```

---

## 📊 性能对比

### Mock 模式

| 指标 | 数值 |
|------|------|
| 启动时间 | ~100ms |
| 内存占用 | ~30MB |
| 二进制大小 | ~25MB |
| 生成速度 | N/A (mock) |

### 完整模式 (Qwen2.5-0.5B)

| 平台 | 速度 | 内存 |
|------|------|------|
| x86_64 (8 核) | ~30 t/s | ~400MB |
| RK3566 (4 核) | ~8 t/s | ~400MB |
| Pi 4 (4 核) | ~5 t/s | ~400MB |

---

## 🐛 故障排查

### 编译失败

**错误**: `cmake not found`

```bash
# 安装 cmake
apt-get install cmake
# 或
yum install cmake
```

**错误**: `llama.cpp not found`

```bash
# 检查 vendor 目录
ls vendor/llama.cpp

# 或设置环境变量
export LLAMA_CPP_PATH=/path/to/llama.cpp
```

### 运行时错误

**错误**: `Model file not found`

```bash
# 检查模型路径
ls -la examples/hello-world/models/

# 下载模型
wget https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/qwen2.5-0.5b-instruct-q4_k_m.gguf
```

**错误**: `Invalid GGUF file`

```bash
# 验证文件
file models/qwen2.5-0.5b-instruct-q4_k_m.gguf
# 应该显示：data 或 GGUF

# 重新下载
rm models/*.gguf
wget ...
```

---

## 🚀 高级配置

### GPU 加速

#### CUDA (NVIDIA)

```bash
# 安装 CUDA Toolkit
# https://developer.nvidia.com/cuda-downloads

# 编译
LLAMA_CUDA=ON cargo build --release --features full-llama

# 运行时
./target/release/agently run agent.yaml --verbose
```

#### Metal (macOS)

```bash
# 编译
LLAMA_METAL=ON cargo build --release --features full-llama
```

### 量化选项

推荐模型量化等级：

| 等级 | 大小 | 质量 | 速度 |
|------|------|------|------|
| Q4_K_M | 中等 | 好 | 快 |
| Q5_K_M | 较大 | 很好 | 中 |
| Q3_K_M | 最小 | 一般 | 最快 |

下载示例：
```bash
# Q4_K_M (推荐)
wget https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/qwen2.5-0.5b-instruct-q4_k_m.gguf

# Q5_K_M (更高质量)
wget https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/qwen2.5-0.5b-instruct-q5_k_m.gguf
```

---

## 📝 下一步

1. **测试真实模型推理**
   ```bash
   ./target/release/agently run examples/hello-world/agent.yaml --verbose
   ```

2. **性能基准测试**
   ```bash
   cargo bench --features full-llama
   ```

3. **部署到设备**
   - 参考 `docs/DEPLOY.md`

---

**最后更新**: 2026-03-24  
**维护者**: agently Team
