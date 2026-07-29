---
doc_type: feature-context
title: "FEAT-019 Context"
date: 2026-07-27
feat_id: "FEAT-019"
updated: 2026-07-29
---

# FEAT-019 Context

## 当前状态

implemented

## 目标摘要

为 uvp 新增本地 Web 看板，跨项目聚合展示 TODO/ADR/Feature/Roadmap 及其关联关系。

## 关键决策

- 后端 axum + rust-embed，前端 Svelte 5 + TailwindCSS 4，编译产物嵌入二进制保持单文件分发
- 数据关联标记采用 Obsidian 双括号格式：`[[ADR-NNN]]`、`[[FEAT-NNN]]`
- 高亮采用 BFS 分层：距离 ≤1 强高亮+实线，距离 ≥2 次高亮+虚线
- 无数据库，纯文件系统读写；不调用 LLM，只读取离线预处理标记

## 实现要点

- 全局配置 `~/.uvp/uvp.toml` 通过 `[[projects]]` 注册项目路径
- REST API：项目列表、项目概览、TODO CRUD（5 端点）
- 前端四列布局（Roadmap / Features / ADR / TODO），SVG 贝塞尔曲线连线
- 体验优化：Features 智能折叠、Roadmap 进度指示、多项目折叠、快捷键、定时刷新
