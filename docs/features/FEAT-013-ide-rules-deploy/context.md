---
doc_type: feature-context
title: "FEAT-013 Context"
date: 2026-06-19
feat_id: "FEAT-013"
updated: 2026-06-19
---

# FEAT-013 Context

## 当前状态

implemented

## 目标摘要

为指定 AI 编程工具生成规则文件并部署 skills，一键配置 AI 编程环境。

## 关键决策

- 规则文件内容统一，仅路径因 IDE 而异
- Skills 从内置或 ~/.uvp/skills/ 复制到项目，不做符号链接
- 支持项目级和全局级两种部署路径

## 关联文件

- 实现：`src/ruvp/commands/ide_cmd.rs`, `src/ruvp/config.rs`
- Skills 源：`src/ruvp/skills/`
