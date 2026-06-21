---
doc_type: feature-context
title: "FEAT-018 Context"
date: 2026-06-19
feat_id: "FEAT-018"
updated: 2026-06-19
---

# FEAT-018 Context

## 当前状态

implemented

## 目标摘要

定义从项目状态、Feature、Changelog 自动汇总生成结构化周报的流程和模板。

## 关键决策

- 周报放在 `docs/周报/` 目录，文件名用 ISO 周数
- 信息采集有明确的 7 步顺序，覆盖 registry/changelog/verification/deliverables/上周周报/PROJECT_STATE/ADR
- 工程改动从 changelog 汇总，不重新编写，保持单一事实来源

## 关联文件

- Skill：`src/ruvp/skills/uvp-weekly-report/SKILL.md`
