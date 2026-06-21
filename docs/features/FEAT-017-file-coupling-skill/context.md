---
doc_type: feature-context
title: "FEAT-017 Context"
date: 2026-06-19
feat_id: "FEAT-017"
updated: 2026-06-19
---

# FEAT-017 Context

## 当前状态

implemented

## 目标摘要

定义文件修改时的联动更新规则，防止文档与代码状态不一致。

## 关键决策

- 联动规则分"必须遵守"和"应该遵守"两级
- Rule #4（改 src/ 必须更新 changelog）是最重要的规则
- 提供自检清单作为每次修改后的快速验证
- 与 uvp check 集成：检查可自动发现部分违规

## 关联文件

- Skill：`src/ruvp/skills/uvp-file-coupling/SKILL.md`
- 检查：`src/ruvp/commands/check.rs` (FEAT-009)
