---
doc_type: feature-context
title: "FEAT-008 Context"
date: 2026-06-19
feat_id: "FEAT-008"
updated: 2026-06-19
---

# FEAT-008 Context

> 此文件为 AI 提供压缩上下文，由 Distill 步骤维护。

## 当前状态

implemented

## 目标摘要

统一管理 docs/ 下所有 markdown 文件的 YAML front matter，确保文档可被 uvp 工具链解析、兼容 Mkdocs。

## 关键决策

- 使用标准 YAML front matter（`---` 包裹），与 Material for Mkdocs 原生兼容
- `title` + `date` 为必填字段，`doc_type` 为推荐字段
- 各文档类型有专有字段定义（ADR 有 status/supersedes，Feature 有 feat_id/updated 等）
- 一致性检查集成在 `uvp check` 中，而非独立命令

## 实现要点

- `common::generate_meta_header()` 负责生成 meta 头
- `check.rs::parse_front_matter()` 负责解析 meta 头
- `feature.rs` 中各 `format_*()` 函数直接内联 meta 模板
- `uvp-meta-header` skill 约束 AI 在创建文档时必须添加 meta 头

## 关联文件

- 实现：`src/ruvp/common.rs` (generate_meta_header)
- 检查：`src/ruvp/commands/check.rs` (parse_front_matter)
- 模板：`src/ruvp/commands/feature.rs` (format_spec/changelog/verification/...)
- Skill：`src/ruvp/skills/uvp-meta-header/SKILL.md`
- 设计文档：`docs/PRD/meta_design.md`

## 最终结论

功能已完整实现。后续变更通过 FEAT-008 跟踪，同步更新 `uvp-meta-header` skill。
