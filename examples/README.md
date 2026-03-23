# agently Edge Examples

示例应用集合，展示如何使用 agently Edge Runtime 构建 AI Agent。

## 📚 示例列表

### 🌟 Hello World Agent

**难度**: ⭐ 入门  
**位置**: `hello-world/`  
**描述**: 简单的对话助手示例，展示基本的配置加载和对话功能。

```bash
cd hello-world
cargo run
```

**学习要点**:
- 配置文件加载
- LLM 引擎初始化
- 交互式对话
- Prompt 管理

---

## 🚀 开始使用

### 前置要求

1. Rust 1.75+ 
2. agently Edge Runtime（本地编译或安装）

### 运行示例

```bash
# 进入示例目录
cd examples/hello-world

# 运行
cargo run
```

### 编译所有示例

```bash
# 从根目录编译
cargo build --workspace
```

## 📖 文档

- [Hello World Agent](hello-world/README.md) - 入门示例
- [官方文档](https://agently-top.github.io/agently-edge/) - 完整 API 文档

## 🤝 贡献

欢迎提交新的示例！请确保：

1. 代码格式正确 (`cargo fmt`)
2. 无 clippy 警告 (`cargo clippy`)
3. 包含 README.md 说明
4. 遵循 Apache 2.0 许可证

## 📄 许可证

Apache 2.0
