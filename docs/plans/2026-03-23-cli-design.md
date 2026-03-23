# agently CLI Design

**日期**: 2026-03-23  
**阶段**: Phase 1 (Rust 核心 + Linux MVP)  
**方案**: A. Rust 单二进制 + workspace

---

## 1. 架构概述

### 1.1 Workspace 结构

```
agently-edge/
├── Cargo.toml              # Workspace 根目录
├── runtime/
│   ├── Cargo.toml          # 核心库 crate
│   └── src/
│       ├── lib.rs
│       ├── agent/
│       ├── services/
│       └── llama/
├── cli/
│   ├── Cargo.toml          # CLI 二进制 crate
│   └── src/
│       └── main.rs         # clap 命令定义
└── examples/
    └── hello-world/
```

### 1.2 依赖关系

```
cli (binary)
  ↓ depends on
runtime (library)
```

---

## 2. 命令设计

### 2.1 命令结构

```bash
agently <command> [arguments] [options]

Commands:
  run       Run an agent with the given configuration
  help      Print help information
```

### 2.2 `run` 命令

```bash
agently run <config-path>

Arguments:
  <config-path>    Path to agent configuration file (YAML)

Options:
  -v, --verbose    Enable verbose output
  -h, --help       Print help
```

### 2.3 使用示例

```bash
# 运行 Hello World Agent
agently run examples/hello-world/agent.yaml

# 运行并显示详细信息
agently run examples/hello-world/agent.yaml --verbose

# 查看帮助
agently run --help
agently --help
```

---

## 3. 实现细节

### 3.1 Workspace 配置

**文件**: `Cargo.toml` (workspace 根目录)

```toml
[workspace]
resolver = "2"
members = [
    "runtime",
    "cli",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "Apache-2.0"
```

### 3.2 CLI crate 配置

**文件**: `cli/Cargo.toml`

```toml
[package]
name = "agently"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "CLI tool for agently Edge Runtime"

[[bin]]
name = "agently"
path = "src/main.rs"

[dependencies]
agently-edge-runtime = { path = "../runtime" }
clap = { version = "4.4", features = ["derive"] }
anyhow = "1.0"
```

### 3.3 CLI 入口

**文件**: `cli/src/main.rs`

```rust
use clap::{Parser, Subcommand};
use agently_edge::AgentConfig;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "agently")]
#[command(about = "Edge AI Agent Runtime CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run an agent with the given configuration
    Run {
        /// Path to agent configuration file (YAML)
        config_path: PathBuf,
        
        /// Enable verbose output
        #[arg(short, long, default_value_t = false)]
        verbose: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Run { config_path, verbose } => {
            run_agent(config_path, verbose)?;
        }
    }
    
    Ok(())
}

fn run_agent(config_path: PathBuf, verbose: bool) -> anyhow::Result<()> {
    // 1. 加载配置
    let config = AgentConfig::load(&config_path)?;
    
    // 2. 打印欢迎信息
    if verbose {
        println!("Loading agent: {}", config.agent.name);
        println!("Config: {:?}", config_path);
    }
    
    // 3. 运行 Agent（复用 hello-world 的逻辑）
    // TODO: 未来这里调用 runtime 的 Agent 运行接口
    println!("Agent running... (MVP: mock mode)");
    
    Ok(())
}
```

### 3.4 配置加载 API

**文件**: `runtime/src/lib.rs`

需要导出的公共 API：

```rust
pub use config::AgentConfig;

// 在 runtime/src/config.rs 中
impl AgentConfig {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        // 复用 hello-world 的 load_config 函数
    }
}
```

---

## 4. 错误处理

### 4.1 错误类型

| 错误场景 | 错误信息 |
|----------|----------|
| 配置文件不存在 | `Error: Configuration file not found: <path>` |
| YAML 解析失败 | `Error: Failed to parse configuration: <details>` |
| 未知命令 | `agently: error: unrecognized subcommand '<cmd>'` |
| 缺少参数 | `agently: error: the following required arguments were not provided: <config-path>` |

### 4.2 退出码

| 退出码 | 含义 |
|--------|------|
| 0 | 成功 |
| 1 | 一般错误 |
| 2 | 命令行参数错误 |

---

## 5. 测试策略

### 5.1 单元测试

- CLI 参数解析测试
- 配置加载测试（复用 runtime 测试）

### 5.2 集成测试

```bash
# 测试帮助命令
agently --help
agently run --help

# 测试错误处理
agently run nonexistent.yaml  # 应返回错误
agently run                   # 应提示缺少参数
```

### 5.3 手动测试

```bash
# 编译
cargo build --release

# 运行测试
./target/release/agently run examples/hello-world/agent.yaml
```

---

## 6. 目录结构变更

### 6.1 当前结构

```
agently-edge/
├── runtime/Cargo.toml
├── examples/hello-world/
└── ...
```

### 6.2 目标结构

```
agently-edge/
├── Cargo.toml              # 新增：workspace 根目录
├── runtime/
│   ├── Cargo.toml          # 修改：移除 [package]，用 workspace
│   └── src/
├── cli/
│   ├── Cargo.toml          # 新增
│   └── src/
│       └── main.rs         # 新增
├── examples/
│   └── hello-world/
└── ...
```

---

## 7. 里程碑

| 里程碑 | 交付物 | 验收标准 |
|--------|--------|----------|
| M1 | Workspace 搭建 | `cargo build` 通过 |
| M2 | CLI 命令实现 | `agently run --help` 可用 |
| M3 | 配置加载集成 | `agently run agent.yaml` 可运行 |
| M4 | 错误处理 | 错误信息友好清晰 |
| M5 | 测试验证 | 所有测试通过 |

---

## 8. 下一步

1. ✅ 设计文档批准（当前）
2. ⏳ 创建 OpenSpec 正式提案
3. ⏳ 拆解任务为详细计划
4. ⏳ Subagent-Driven 执行

---

**设计作者**: 虾 (Xia)  
**批准状态**: ✅ 用户已批准方案 A  
**批准日期**: 2026-03-23
