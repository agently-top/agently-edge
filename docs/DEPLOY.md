# agently Edge 部署指南

**版本**: 0.1.0  
**更新日期**: 2026-03-24  
**平台**: Linux (x86_64, aarch64)

---

## 📦 快速开始

### 1. 编译

```bash
# 安装 Rust 工具链
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# 克隆仓库
git clone https://github.com/your-org/agently-edge-runtime.git
cd agently-edge-runtime

# 编译 release 版本
cargo build --release
```

编译产物：
- `target/release/agently` - CLI 工具
- `target/release/libagently_edge.so` - Rust 库（可选）

### 2. 运行示例

```bash
# Mock 模式（无需模型文件）
./target/release/agently run examples/hello-world/agent.yaml --mock

# 真实模型模式
# 1. 下载 GGUF 模型
mkdir -p examples/hello-world/models
wget https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/qwen2.5-0.5b-instruct-q4_k_m.gguf \
  -O examples/hello-world/models/qwen2.5-0.5b-instruct-q4_k_m.gguf

# 2. 运行
./target/release/agently run examples/hello-world/agent.yaml --verbose
```

### 3. CLI 参数

```bash
./target/release/agently run <config.yaml> [OPTIONS]

Options:
  --verbose    启用详细输出（显示性能统计）
  --mock       使用 mock 模式（无需模型文件）
  -h, --help   打印帮助信息
```

---

## 📋 配置文件说明

### agent.yaml 结构

```yaml
agent:
  id: hello-world          # Agent 唯一标识
  name: Hello World Agent  # 显示名称
  version: 0.1.0           # 版本号
  description: "描述"      # 描述信息

runtime:
  model_path: ./models/qwen2.5-0.5b-instruct-q4_k_m.gguf  # GGUF 模型路径
  context_length: 4096     # 上下文长度
  temperature: 0.7         # 生成温度 (0.0-2.0)
  max_tokens: 512          # 最大生成长度
  n_threads: 4             # CPU 线程数

prompts:
  system: |                # 系统提示词
    You are a friendly assistant.
  welcome: |               # 欢迎信息
    👋 Hello! How can I help?

logging:
  level: info              # 日志级别：debug, info, warn, error
  format: pretty           # 日志格式：pretty, json
```

---

## 🚀 设备部署

### 嵌入式设备 (RK3566, 树莓派等)

#### 1. 交叉编译

```bash
# 安装交叉编译工具链
# aarch64 (树莓派 4, RK3566)
apt-get install gcc-aarch64-linux-gnu

# 配置 .cargo/config.toml
cat >> .cargo/config.toml << EOF
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
EOF

# 编译
cargo build --release --target aarch64-unknown-linux-gnu
```

#### 2. 部署到设备

```bash
# 复制二进制文件
scp target/aarch64-unknown-linux-gnu/release/agently user@device:/usr/local/bin/

# 复制示例配置
scp -r examples/hello-world user@device:~/

# 在设备上运行
ssh user@device
./agently run hello-world/agent.yaml --mock
```

### 资源建议

| 模型大小 | 内存需求 | CPU 要求 | 推荐设备 |
|----------|----------|----------|----------|
| 0.5B Q4  | 512MB    | 4 核心   | RK3566, Pi Zero 2 |
| 1.5B Q4  | 1GB      | 4 核心   | Pi 4, RK3588 |
| 3B Q4    | 2GB      | 8 核心   | Pi 5, RK3588 |
| 7B Q4    | 4GB      | 8 核心   | x86 服务器 |

---

## 🔧 性能优化

### 1. 模型量化

使用 GGUF 量化模型减少内存占用：

```bash
# 推荐量化等级
- q4_k_m  (平衡质量和大小)
- q3_k_m  (最小内存占用)
- q5_k_m  (更高质量)
```

### 2. 线程数调整

```yaml
runtime:
  n_threads: 4  # 设置为 CPU 物理核心数
```

### 3. GPU 加速（如有）

```yaml
runtime:
  n_gpu_layers: 35  # 卸载到 GPU 的层数
```

### 4. 上下文长度

```yaml
runtime:
  context_length: 2048  # 减少内存占用
```

---

## 📊 性能基准

### RK3566 (4 核心, 2GB RAM)

| 模型 | 量化 | 速度 (tokens/s) | 内存占用 |
|------|------|-----------------|----------|
| 0.5B | Q4_K_M | ~8-12 t/s | ~400MB |
| 1.5B | Q4_K_M | ~3-5 t/s  | ~1GB |

### x86_64 (8 核心, 16GB RAM)

| 模型 | 量化 | 速度 (tokens/s) | 内存占用 |
|------|------|-----------------|----------|
| 0.5B | Q4_K_M | ~30-50 t/s | ~400MB |
| 7B   | Q4_K_M | ~8-12 t/s  | ~4GB |

---

## ⚠️ 故障排查

### 模型加载失败

```
Error: Model file not found: ./models/xxx.gguf
```

**解决**: 检查模型路径是否正确，文件是否存在。

### 内存不足

```
Error: Out of memory
```

**解决**:
1. 使用更小的模型（0.5B 或 1.5B）
2. 使用更低精度的量化（Q3_K_M）
3. 减少上下文长度

### 速度过慢

**解决**:
1. 减少 `n_threads` 到 CPU 物理核心数
2. 使用更小的模型
3. 启用 GPU 加速（如有）

---

## 📞 支持

- GitHub Issues: https://github.com/your-org/agently-edge-runtime/issues
- 文档：https://docs.agently.ai
- 示例：`examples/` 目录

---

**最后更新**: 2026-03-24  
**维护者**: agently Team
