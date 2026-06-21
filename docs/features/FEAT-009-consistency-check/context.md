# FEAT-009 Context

> 此文件为 AI 提供压缩上下文，由 Distill 步骤维护。

## 当前状态

implemented（部分差异待对齐）

## 核心概念

`uvp check` 检查项目文档一致性，包括 Feature 闭环、ADR 一致性、AI 上下文完整性、渲染一致性、AI 规则文件。

## 实现要点

- 源码：`src/ruvp/commands/check.rs`（270 行）
- CLI 参数：`--features` / `--adr` / `--fix`（fix 未实现）
- 5 项检查：Feature 闭环 / ADR 一致性 / AI 上下文 / 渲染一致性 / AI 规则文件
- 不指定 --features 或 --adr 时执行全部 5 项检查

## Spec vs 实现差异

Spec 已更新对齐代码实现。唯一未实现项：`--fix` 自动修复逻辑。
