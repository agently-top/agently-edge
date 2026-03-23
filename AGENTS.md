# agently-edge 开发约束

**强制工作流** — 所有任务必须遵循以下 6 步流程，无例外。

---

## 开发流程（6 步）

1. **superpowers 脑暴提案** — Section 2 (Brainstorming)
   - ❌ 禁止直接写代码
   - ✅ 先问澄清问题，提 2-3 个方案，获得设计批准

2. **讨论确认提案** — 获得用户明确批准

3. **用 openspec 创建正式提案**
   - Proposal → Specs → Design → Tasks

4. **形成子 Plan** — 拆解为可执行步骤

5. **执行 Plan** — Section 5 (Subagent-Driven Development)
   - 每个任务独立子代理
   - Spec Review → Code Quality Review
   - TDD 强制（先红后绿）

6. **更新提案状态** — 更新 openspec/changes/*/README.md

---

## 触发检查清单

**每次开始新任务前，必须确认**：

- [ ] 已读取 `memory/agently-edge-workflow.md`
- [ ] 已读取本项目 `AGENTS.md`
- [ ] 当前处于 Brainstorm 阶段（未写代码）
- [ ] 已提出 2-3 个方案待用户选择

**如果跳过任何步骤 → 停止，退回上一步**

---

## 当前状态

**Phase**: 1 (Rust 核心 + Linux MVP)  
**Priority**: B. 架构优先  
**Next Task**: Task 1.4 (Hello World Agent) — 等待 Brainstorm 批准

---

## 违反流程的后果

| 跳过步骤 | 后果 |
|---------|------|
| Brainstorm | 做错方向，返工 |
| OpenSpec | 缺少文档，维护困难 |
| TDD | 回归 bug，测试不足 |
| Review | 技术债务累积 |

**所有步骤都是强制的，没有"简单任务可以跳过"的例外。**
