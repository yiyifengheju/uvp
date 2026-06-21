---
doc_type: feature-context
title: "FEAT-011 Context"
date: 2026-06-19
feat_id: "FEAT-011"
updated: 2026-06-19
---

# FEAT-011 Context

## 当前状态

implemented

## 目标摘要

将 feature-registry.yaml 和 ADR 目录渲染为 Mkdocs 可展示的 markdown 索引页面。

## 关键决策

- 渲染为纯 markdown 而非 HTML，保持 Mkdocs 兼容
- `--check` 模式用字符串完全相等判断一致性，简单可靠
- ADR 信息从文件内容解析，不依赖额外的注册表

## 关联文件

- 实现：`src/ruvp/commands/render.rs`
- 输出：`docs/features/index.md`, `docs/adr/registry.md`
