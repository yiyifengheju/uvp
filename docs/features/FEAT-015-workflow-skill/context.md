---
doc_type: feature-context
title: "FEAT-015 Context"
date: 2026-06-19
feat_id: "FEAT-015"
updated: 2026-06-19
---

# FEAT-015 Context

## 当前状态

implemented

## 目标摘要

定义 AI 辅助编程的 6 步闭环流程，约束 AI 在执行编程任务时遵循结构化工作流。

## 关键决策

- 6 步中仅 Step 2 (Define) 不可跳过，保证最低限度的特性追踪
- 纯 AI 行为约束，无 CLI 命令实现（流程中调用其他命令如 `uvp adr`/`uvp f`）
- 与 uvp-feature-lifecycle/uvp-file-coupling 分工明确：workflow 管流程，lifecycle 管模板，coupling 管联动

## 关联文件

- Skill：`src/ruvp/skills/uvp-workflow/SKILL.md`
