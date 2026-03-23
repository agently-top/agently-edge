# CLI 工具功能规格

## 1. 用户故事

### 1.1 运行 Agent

**作为** 开发者  
**我想要** 通过命令行运行 Agent  
**以便于** 测试和部署我的应用

**验收场景**：
```gherkin
Given 已安装 agently CLI
And 有有效的 agent.yaml 配置文件
When 运行 agently run agent.yaml
Then Agent 启动并进入对话模式
```

### 1.2 查看帮助

**作为** 新用户  
**我想要** 查看命令帮助  
**以便于** 了解如何使用 CLI

**验收场景**：
```gherkin
Given 已安装 agently CLI
When 运行 agently --help
Then 显示可用命令列表
And 显示全局选项
```

### 1.3 错误处理

**作为** 用户  
**我想要** 清晰的错误信息  
**以便于** 快速定位问题

**验收场景**：
```gherkin
Given 配置文件不存在
When 运行 agently run nonexistent.yaml
Then 显示友好的错误信息
And 返回非零退出码
```

---

## 2. 功能需求

### 2.1 核心功能

| ID | 功能 | 优先级 | 说明 |
|----|------|--------|------|
| F1 | run 命令 | P0 | 运行 Agent |
| F2 | 帮助系统 | P0 | --help 显示帮助 |
| F3 | 配置加载 | P0 | 加载 YAML 配置 |
| F4 | 错误处理 | P0 | 友好错误信息 |
| F5 | verbose 模式 | P1 | --verbose 详细输出 |

### 2.2 非功能需求

| ID | 需求 | 说明 |
|----|------|------|
| NF1 | 启动速度 | 冷启动 <100ms |
| NF2 | 二进制大小 | <10MB (release) |
| NF3 | 跨平台 | Linux/macOS/Windows |
| NF4 | 代码质量 | 无 clippy 警告 |

---

## 3. 界面规格

### 3.1 命令行界面

**全局帮助**:
```
$ agently --help

Edge AI Agent Runtime CLI

Usage: agently <COMMAND>

Commands:
  run   Run an agent with the given configuration
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

**run 命令帮助**:
```
$ agently run --help

Run an agent with the given configuration

Usage: agently run [OPTIONS] <CONFIG_PATH>

Arguments:
  <CONFIG_PATH>  Path to agent configuration file (YAML)

Options:
  -v, --verbose  Enable verbose output
  -h, --help     Print help
```

### 3.2 错误信息

**配置文件不存在**:
```
$ agently run nonexistent.yaml
Error: Configuration file not found: nonexistent.yaml
```

**YAML 解析失败**:
```
$ agently run invalid.yaml
Error: Failed to parse configuration
  → invalid type at line 5, column 10
```

**缺少参数**:
```
$ agently run
Error: the following required arguments were not provided:
  <CONFIG_PATH>

Usage: agently run <CONFIG_PATH>

For more information, try '--help'.
```

---

## 4. 测试场景

### 4.1 单元测试

- [ ] CLI 参数解析测试
- [ ] 配置加载测试
- [ ] 错误处理测试

### 4.2 集成测试

- [ ] `agently --help` 输出正确
- [ ] `agently run --help` 输出正确
- [ ] `agently run valid.yaml` 可运行
- [ ] `agently run invalid.yaml` 返回错误

### 4.3 手动测试

- [ ] 编译无警告
- [ ] 运行无崩溃
- [ ] 错误信息清晰

---

**规格版本**: 1.0  
**创建日期**: 2026-03-23  
**审核状态**: ⏳ Pending
