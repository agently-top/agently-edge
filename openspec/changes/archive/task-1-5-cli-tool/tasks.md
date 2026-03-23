# Task 1.5: CLI 工具实现任务

**状态**: 📋 Todo  
**创建日期**: 2026-03-23

---

## ✅ 任务清单

### Phase 1: Workspace 搭建

- [ ] **Task 1.5.1**: 创建 workspace Cargo.toml
  - 根目录 `Cargo.toml`
  - 配置 members: runtime, cli
  - 预计：10 分钟

- [ ] **Task 1.5.2**: 调整 runtime Cargo.toml
  - 移除 [package]，使用 workspace
  - 保留 [lib] 配置
  - 预计：10 分钟

- [ ] **Task 1.5.3**: 创建 cli crate
  - `cli/Cargo.toml`
  - `cli/src/main.rs`
  - 预计：20 分钟

### Phase 2: CLI 实现（TDD）

- [ ] **Task 1.5.4**: 编写 CLI 参数解析测试
  - 测试 `run` 命令解析
  - 测试 `--help` 输出
  - 预计：20 分钟

- [ ] **Task 1.5.5**: 实现 CLI 命令（RED→GREEN）
  - clap 定义
  - run 命令处理
  - 预计：40 分钟

- [ ] **Task 1.5.6**: 集成配置加载
  - 调用 runtime API
  - 错误处理
  - 预计：30 分钟

### Phase 3: 文档 + 测试

- [ ] **Task 1.5.7**: 编写 cli/README.md
  - 使用说明
  - 命令示例
  - 预计：20 分钟

- [ ] **Task 1.5.8**: 代码格式化 + clippy
  - `cargo fmt`
  - `cargo clippy`
  - 预计：15 分钟

- [ ] **Task 1.5.9**: 功能测试
  - `agently run --help`
  - `agently run agent.yaml`
  - 预计：20 分钟

### Phase 4: 提交归档

- [ ] **Task 1.5.10**: Git 提交
  - 编写提交信息
  - 提交代码
  - 预计：10 分钟

- [ ] **Task 1.5.11**: 更新任务状态
  - 更新 README.md
  - 移动到 archive
  - 预计：10 分钟

---

## 📊 进度追踪

| Phase | 总任务 | 已完成 | 进度 |
|-------|--------|--------|------|
| Phase 1: Workspace 搭建 | 3 | 0 | 0% |
| Phase 2: CLI 实现 | 3 | 0 | 0% |
| Phase 3: 文档 + 测试 | 3 | 0 | 0% |
| Phase 4: 提交归档 | 2 | 0 | 0% |
| **总计** | **11** | **0** | **0%** |

---

**创建日期**: 2026-03-23  
**最后更新**: 2026-03-23
