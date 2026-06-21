# FEAT-012 Context

> 此文件为 AI 提供压缩上下文，由 Distill 步骤维护。

## 当前状态

implemented

## 核心概念

所有终端 UI 集中在 `src/ruvp/ui.rs`，分三层 API：
1. **Spinner API**（init 等多步骤命令）：step_start / step_update / step_done / step_skip / step_fail
2. **简单输出 API**（非 init 命令）：file_created / file_exists / action_ok / action_fail / action_skip / action_info
3. **面板 + 后台命令**：success_panel / info_panel / spawn_command_streaming

## 关键决策

- Spinner 输出到 stderr，简单输出到 stdout
- 延迟可配置（`[ui].delay_ms`，默认 120ms）
- 后台命令通过 `spawn_command_streaming` 在独立线程执行

## 实现要点

- 源码：`src/ruvp/ui.rs`（157 行）
- 依赖：`console` 0.16（彩色文本）、`indicatif` 0.18（spinner）
- Spinner 帧率 80ms，braille 字符集
