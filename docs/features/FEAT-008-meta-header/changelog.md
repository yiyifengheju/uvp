---
doc_type: feature-changelog
title: "FEAT-008 Changelog"
date: 2026-06-19
feat_id: "FEAT-008"
updated: 2026-06-19
---

# FEAT-008 Changelog

> 记录代码变更：改了什么、为什么改、影响范围。

| 日期 | 类型 | 变更 | 说明 |
|------|------|------|------|
| 2026-06-19 | init | 创建特性 | 从已有实现中提炼为独立特性 |
| 2026-06-19 | doc | 整理 spec.md | 根据 common.rs、check.rs、uvp-meta-header skill 整理规格 |

## [2026-06-19]

### Added
- 创建 FEAT-008 特性目录
- 编写 spec.md：整合 `generate_meta_header()`、`parse_front_matter()`、各文档类型字段定义、AI skill 规范
- 将 `docs/PRD/meta_design.md` 中的设计方案正式归档为特性规格

### Notes
- 代码实现已在此前完成（`common.rs`、`check.rs`、`feature.rs` 中的模板函数）
- Skill 定义已存在于 `src/ruvp/skills/uvp-meta-header/`
- 后续此特性的更新将同步维护 `uvp-meta-header` skill
