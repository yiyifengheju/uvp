---
doc_type: feature-spec
title: "FEAT-015: 6步闭环工作流 (uvp-workflow skill)"
date: 2026-06-19
feat_id: "FEAT-015"
status: implemented
updated: 2026-06-19
related_adr: "0001"
---

# FEAT-015: 6步闭环工作流 (uvp-workflow skill)

## 概述

`uvp-workflow` skill 定义 AI 辅助编程的 6 步闭环流程（Decide → Define → Plan → Implement → Verify → Distill），约束 AI 在执行编程任务时必须遵循结构化工作流，而非直接跳入编码。

## 接口定义

### Skill 文件

- 路径：`src/ruvp/skills/uvp-workflow/SKILL.md`
- 部署位置：`~/.uvp/skills/uvp-workflow/SKILL.md` 或项目 `.claude/skills/`
- 触发条件：任何编程任务（新功能、Bug 修复、重构）
- 不适用：纯文档创建、纯查询操作

## 行为规格

### 6 步流程

| 步骤 | 名称 | 动作 | 关联命令 |
|------|------|------|---------|
| 1 | Decide | 是否需要架构决策？需要则创建 ADR | `uvp adr "标题"` |
| 2 | Define | 创建/匹配 Feature，填写 spec.md | `uvp f new "标题"` |
| 3 | Plan | 复杂特性编写 plan.md（可选） | 手动编辑 |
| 4 | Implement | 编码 + 即时更新 changelog.md | 手动编辑 |
| 5 | Verify | 运行测试、检查验收标准、填写 verification.md | `uvp f close` |
| 6 | Distill | 提炼 context.md、更新 PROJECT_STATE.md | 手动编辑 |

### 跳过规则

- Step 1 (ADR)：简单 Bug 修复、小改动可跳过，需在 changelog 记录 `Skip ADR: trivial fix`
- Step 3 (Plan)：简单特性（<1天）、单人、低风险可跳过
- Step 2 (Define)：**不可跳过**，至少要匹配到一个 Feature

### 反模式约束

| 禁止 | 要求 |
|------|------|
| 直接写代码不走流程 | 至少执行 Step 2 (Define) |
| 文档稍后补 | changelog 立即更新 |
| 测试等会再说 | 验证通过才能关闭 |
| 模糊描述（如 "Make it fast"） | 量化指标（如 "Response time < 100ms P99"） |
| 模糊变更（如 "Fixed bugs"） | 精确描述（如 "Fixed null pointer in Parser.parse_line() when input empty"） |

### 与其他 Skill 的协作

| 步骤 | 调用的 Skill |
|------|-------------|
| Step 2 | `uvp-feature-lifecycle`：Feature 创建和 spec 模板 |
| Step 4 | `uvp-file-coupling`：文件联动规则 |
| Step 4 | `uvp-meta-header`：创建文档时添加 meta 头 |
| Step 5-6 | `uvp-feature-lifecycle`：verification/context 模板 |

## 约束

### 适用范围

- 适用于所有编程任务，包括 AI 辅助编程和人工编程
- 不适用于纯文档编写、查询操作

### AI 行为约束

- AI 必须在开始编码前确认当前任务属于哪个 Feature
- AI 不可在未创建/匹配 Feature 的情况下直接修改 `src/` 代码
- AI 修改代码后必须同步更新 changelog.md

## 验收标准

- [x] SKILL.md 定义完整，包含 6 步流程说明
- [x] 每步有明确的进入条件和退出条件
- [x] 跳过规则有明确指引
- [x] 反模式列表完整
- [x] 与其他 skill 的协作关系清晰
- [x] Skill 可通过 `uvp ide` 部署到项目
