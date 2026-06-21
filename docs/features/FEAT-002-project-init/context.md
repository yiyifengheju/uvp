# FEAT-002 Context

> 此文件为 AI 提供压缩上下文，由 Distill 步骤维护。

## 当前状态

implemented

## 核心概念

`uvp init` 一键初始化 vibe coding 项目，创建标准目录结构、配置文件、AI 上下文和文档模板。支持 5 种 IDE（claude / cursor / windsurf / cline / trae）。

## 关键决策

- 分两阶段执行：Phase 1 同步创建目录和模板文件，Phase 2 后台运行 `uv init` + `uv add`
- 支持 `--no-python` / `--no-mkdocs` / `--no-ai-rules` 跳过可选步骤
- AI 规则文件支持跨 IDE 复用：切换 IDE 时自动复制已有规则内容
- 全局配置 `~/.uvp/` 和内置 skills 仅首次创建，不覆盖

## 实现要点

- 源码入口：`src/ruvp/commands/init.rs`（262 行）
- CLI 定义：`src/ruvp/main.rs` Init 子命令
- 配置：`src/ruvp/config.rs` InitConfig 结构体
- 模板文件通过 `embed_templates!` 宏编译内嵌
- `uv init` / `uv add` 通过 `spawn_command_streaming` 后台流式执行

## 已知差异（spec vs 实现）

所有已知差异已修复（2026-06-19）。
