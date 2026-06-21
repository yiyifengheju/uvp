---
doc_type: feature-context
title: "FEAT-016 Context"
date: 2026-06-19
feat_id: "FEAT-016"
updated: 2026-06-19
---

# FEAT-016 Context

## 当前状态

implemented

## 目标摘要

定义 Feature 状态流转规则和所有文档模板，约束 AI 操作 Feature 时遵循标准化生命周期管理。

## 关键决策

- 与 FEAT-004 (uvp feature 命令) 互补：FEAT-004 是工具实现，FEAT-016 是 AI 约束
- 提供 6 种文档模板：spec/changelog/verification/context/deliverables/plan
- 关闭 Feature 有明确前置条件检查清单

## 关联文件

- Skill：`src/ruvp/skills/uvp-feature-lifecycle/SKILL.md`
- 工具实现：`src/ruvp/commands/feature.rs` (FEAT-004)
