# CLI 工具技术设计

## 1. 架构设计

### 1.1 Workspace 结构

```
agently-edge/
├── Cargo.toml              # Workspace 根目录
├── runtime/
│   ├── Cargo.toml          # Library crate
│   └── src/
│       ├── lib.rs          # 公共 API 导出
│       └── config.rs       # 配置加载
├── cli/
│   ├── Cargo.toml          # Binary crate
│   └── src/
│       └── main.rs         # CLI 入口
└── examples/
    └── hello-world/
```

### 1.2 依赖关系

```
┌─────────────┐
│  cli (bin)  │
└──────┬──────┘
       │ uses
       ▼
┌─────────────┐
│ runtime (lib)│
└─────────────┘
```

---

## 2. 技术选型

### 2.1 依赖选择

| 依赖 | 版本 | 用途 |
|------|------|------|
| clap | 4.4 | 命令行解析 |
| anyhow | 1.0 | 错误处理 |
| agently-edge-runtime | path | 核心运行时 |

### 2.2 设计决策

#### 决策 1: Workspace vs 独立
**问题**: CLI 如何组织？  
**选项**:
- A. Workspace 多 crate
- B. 独立 crate

**决策**: **选项 A**  
**理由**: 
- 代码复用方便
- 版本同步自动
- 符合 Rust 最佳实践

#### 决策 2: clap derive vs builder
**问题**: 如何定义命令？  
**选项**:
- A. clap derive (宏)
- B. clap builder (手动)

**决策**: **选项 A**  
**理由**:
- 代码简洁
- 类型安全
- 易于维护

---

## 3. 代码结构

### 3.1 CLI 入口

```rust
// cli/src/main.rs
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        config_path: PathBuf,
        #[arg(short, long)]
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
```

### 3.2 配置加载 API

```rust
// runtime/src/lib.rs
pub mod config;
pub use config::AgentConfig;

// runtime/src/config.rs
impl AgentConfig {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        serde_yaml::from_str(&content).map_err(Into::into)
    }
}
```

---

## 4. 错误处理

### 4.1 错误流程

```
用户输入
    ↓
clap 解析 → 参数错误 → 显示帮助
    ↓
加载配置 → 文件不存在 → 显示错误
    ↓              ↓
    ↓         YAML 解析失败 → 显示详情
    ↓
运行 Agent
```

### 4.2 错误类型

```rust
#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error("Configuration file not found: {0}")]
    ConfigNotFound(PathBuf),
    
    #[error("Failed to parse configuration: {0}")]
    ConfigParseError(#[from] serde_yaml::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
```

---

## 5. 测试策略

### 5.1 单元测试

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_cli_parse_run() {
        let cli = Cli::parse_from(["agently", "run", "test.yaml"]);
        assert!(matches!(cli.command, Commands::Run { .. }));
    }
}
```

### 5.2 集成测试

```rust
// tests/cli_tests.rs
#[test]
fn test_help_command() {
    let output = Command::new("agently")
        .arg("--help")
        .output()
        .expect("Failed to run");
    assert!(output.status.success());
}
```

---

## 6. 性能考虑

### 6.1 启动时间

| 阶段 | 目标 |
|------|------|
| 二进制加载 | <50ms |
| clap 解析 | <10ms |
| 配置加载 | <10ms |
| 总启动 | <100ms |

### 6.2 二进制大小

| 模式 | 目标大小 |
|------|----------|
| debug | <50MB |
| release | <10MB |

---

**设计版本**: 1.0  
**创建日期**: 2026-03-23  
**审核状态**: ⏳ Pending
