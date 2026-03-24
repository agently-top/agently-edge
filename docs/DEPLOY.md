# agently Edge 部署指南

**版本**: 0.2.0  
**更新日期**: 2026-03-24  
**部署方式**: 单二进制文件

---

## 🎯 核心优势

✅ **单文件部署** - 只需一个二进制文件 (~2MB)  
✅ **零依赖** - llama.cpp 已静态链接  
✅ **跨平台** - Linux x86_64/aarch64  
✅ **开箱即用** - Mock 模式无需模型文件  

---

## 📦 快速部署

### 方式 1: 使用预编译二进制

```bash
# 1. 下载
wget https://github.com/agently-top/agently-edge/releases/latest/download/agently

# 2. 授权
chmod +x agently

# 3. 运行（Mock 模式）
./agently run agent.yaml --mock
```

### 方式 2: 源码编译

```bash
# 1. 克隆仓库
git clone https://github.com/agently-top/agently-edge.git
cd agently-edge

# 2. 一键编译
./scripts/build-deploy.sh

# 3. 运行
./target/release/agently run examples/hello-world/agent.yaml --mock
```

### 方式 3: Cargo 安装

```bash
cargo install --git https://github.com/agently-top/agently-edge agently
agently run agent.yaml --mock
```

---

## 🚀 设备部署

### 嵌入式设备 (RK3566, 树莓派等)

#### 1. 交叉编译

```bash
# 安装工具链
apt-get install gcc-aarch64-linux-gnu

# 配置 .cargo/config.toml
cat >> .cargo/config.toml << EOF
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
rustflags = ["-C", "target-feature=+crt-static"]
EOF

# 编译
./scripts/build-deploy.sh --target aarch64-unknown-linux-gnu
```

#### 2. 部署到设备

```bash
# 复制二进制
scp target/aarch64-unknown-linux-gnu/release/agently user@device:/usr/local/bin/

# 复制示例
scp -r examples user@device:~/

# 在设备上运行
ssh user@device
agently run examples/hello-world/agent.yaml --mock
```

### 服务器部署 (x86_64)

```bash
# 直接编译
./scripts/build-deploy.sh

# 部署
sudo cp target/release/agently /usr/local/bin/
sudo chmod +x /usr/local/bin/agently

# 验证
agently --help
```

---

## 📋 配置文件

### agent.yaml

```yaml
agent:
  id: hello-world
  name: Hello World Agent
  version: 0.1.0
  description: "A friendly assistant"

runtime:
  model_path: ./models/qwen2.5-0.5b-instruct-q4_k_m.gguf
  context_length: 2048
  temperature: 0.7
  max_tokens: 512
  n_threads: 4

prompts:
  system: |
    You are a helpful assistant.
  welcome: |
    👋 Hello! How can I help?

logging:
  level: info
  format: pretty
```

---

## 🔧 运行模式

### Mock 模式（测试用）

无需模型文件，快速测试：

```bash
agently run agent.yaml --mock
```

### 真实模型模式

```bash
# 1. 下载模型
mkdir -p models
wget https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/qwen2.5-0.5b-instruct-q4_k_m.gguf \
  -O models/qwen2.5-0.5b-instruct-q4_k_m.gguf

# 2. 运行
agently run agent.yaml --verbose
```

---

## 📊 资源需求

### 最小配置

| 组件 | 需求 |
|------|------|
| CPU | 单核 |
| 内存 | 64MB (binary) |
| 存储 | 10MB (binary + config) |

### 推荐配置 (带模型)

| 模型 | CPU | 内存 | 存储 |
|------|-----|------|------|
| 0.5B Q4 | 4 核 | 512MB | 400MB |
| 1.5B Q4 | 4 核 | 1GB | 1GB |
| 3B Q4 | 8 核 | 2GB | 2GB |
| 7B Q4 | 8 核 | 4GB | 4GB |

---

## ⚡ 性能基准

### RK3566 (4 核, 2GB RAM)

| 模型 | 速度 | 内存 |
|------|------|------|
| 0.5B Q4 | ~8 t/s | 400MB |
| Mock | N/A | 30MB |

### x86_64 (8 核, 16GB RAM)

| 模型 | 速度 | 内存 |
|------|------|------|
| 0.5B Q4 | ~30 t/s | 400MB |
| 7B Q4 | ~10 t/s | 4GB |
| Mock | N/A | 30MB |

---

## 🐛 故障排查

### 二进制无法运行

```bash
# 检查依赖
ldd agently

# 如果是动态链接，重新编译静态版本
./scripts/build-deploy.sh
```

### 模型加载失败

```bash
# 检查文件
ls -la models/*.gguf
file models/*.gguf  # 应该显示 data

# 验证 GGUF 格式
head -c 4 models/*.gguf  # 应该显示 GGUF
```

### 内存不足

```bash
# 使用更小的模型
wget .../qwen2.5-0.5b-instruct-q3_k_m.gguf

# 减少上下文
# 编辑 agent.yaml: context_length: 1024
```

---

## 📞 支持

- **GitHub**: https://github.com/agently-top/agently-edge
- **Issues**: https://github.com/agently-top/agently-edge/issues
- **文档**: https://docs.agently.ai

---

**最后更新**: 2026-03-24  
**维护者**: agently Team
