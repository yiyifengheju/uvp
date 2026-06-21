# FEAT-001 Context

> 此文件为 AI 提供压缩上下文，由 Distill 步骤维护。

## 当前状态

implemented

## 核心概念

Feature Ledger 为每个特性提供独立目录，包含 spec / plan / changelog / verification / context 五个标准文件，统一管理生命周期。

## 关键决策

- 每个特性一个独立目录：`docs/features/FEAT-NNN-<slug>/`
- 特性注册表集中在 `docs/_meta/feature-registry.yaml`
- 编号格式 `FEAT-NNN-title`，递增分配
- 状态流转：idea → planned → in_progress → implemented → verified，支持 paused / deprecated / removed 分支

## 实现要点

- 源码入口：`src/ruvp/commands/feature.rs`（命令实现）、`src/ruvp/common.rs`（数据结构与 registry I/O）
- CLI 子命令：new / list / show / status / close / archive
- `feature_new` 自动创建目录、模板文件、注册 registry
- `feature_status` 变更状态并同步 verification.md / context.md
- `render_features_index()` 从 registry 渲染 `docs/features/index.md`
