---
doc_type: feature-spec
title: "FEAT-016: Feature 生命周期模板 (uvp-feature-lifecycle skill)"
date: 2026-06-19
feat_id: "FEAT-016"
status: implemented
updated: 2026-06-19
related_adr: "0001"
---

# FEAT-016: Feature 生命周期模板 (uvp-feature-lifecycle skill)

## 概述

`uvp-feature-lifecycle` skill 定义 Feature 的状态流转规则和所有文档模板（spec/changelog/verification/context/deliverables/plan），约束 AI 在操作 Feature 时遵循标准化的生命周期管理。

## 接口定义

### Skill 文件

- 路径：`src/ruvp/skills/uvp-feature-lifecycle/SKILL.md`
- 部署位置：`~/.uvp/skills/uvp-feature-lifecycle/SKILL.md` 或项目 `.claude/skills/`
- 触发条件：创建/更新/关闭 Feature，或操作 `docs/features/` 目录

## 行为规格

### 状态流转

```
idea → planned → implementing → verifying → verified → closed
                                              ↑         │
                                              └─ rework ┘
```

| 状态 | 进入条件 | 退出条件 |
|------|---------|---------|
| idea | `uvp f new` 创建 | spec 完成 |
| planned | spec 已写 | 开始编码 |
| implementing | 代码变更开始 | 编码完成 |
| verifying | 准备运行验证 | 所有检查完成 |
| verified | 验证通过 | 准备提炼 |
| closed | 提炼完成 | 终态 |

### Feature 目录结构

```
docs/features/FEAT-XXX-Title/
├── spec.md              ⭐ 必须 — 定义做什么
├── plan.md              🔧 可选 — 复杂特性需要
├── changelog.md         📝 最高频 — 每次改代码都更新
├── verification.md      ✅ 关闭前必须填
├── context.md           🧠 关闭时提炼知识
└── deliverables.md      📊 产出记录
```

### 文档模板定义

Skill 为以下文档提供标准模板：

| 文档 | doc_type | 必须时机 |
|------|----------|---------|
| spec.md | `feature-spec` | 创建 Feature 后立即填写 |
| changelog.md | `feature-changelog` | 每次代码变更后更新 |
| verification.md | `feature-verification` | 关闭前填写 |
| context.md | `feature-context` | 关闭时提炼 |
| deliverables.md | `feature-deliverables` | 有产出时记录 |
| plan.md | `feature-plan` | 复杂特性可选 |

### 关闭前置条件

执行 `uvp f close FEAT-XXX` 前必须全部满足：
- verification.md 已填写
- 所有验收标准通过
- 无 critical 级别问题
- changelog 已更新
- context.md 已提炼

## 约束

### 与 FEAT-004 的关系

- FEAT-004（uvp feature 命令）负责 CLI 交互和自动化操作
- FEAT-016（本特性）负责 AI 行为规范和文档模板定义
- 两者互补：FEAT-004 是工具实现，FEAT-016 是 AI 约束

## 验收标准

- [x] SKILL.md 包含完整的状态流转定义
- [x] 6 种文档模板均有明确的格式和字段说明
- [x] 关闭前置条件清晰
- [x] 与 uvp-workflow / uvp-file-coupling 的协作关系明确
- [x] Skill 可通过 `uvp ide` 部署到项目
