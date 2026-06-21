# FEAT-004 Context

> 此文件为 AI 提供压缩上下文，由 Distill 步骤维护。

## 当前状态

implemented

## 核心概念

`uvp feature` 管理 Feature Ledger 中特性的全生命周期：new / list / show / status / close / archive。每个特性创建时生成 6 个标准文件（spec / plan / changelog / verification / context / deliverables）。

## 关键决策

- 状态流转：idea → planned → in_progress → implemented → verified，支持 paused / deprecated / removed
- `close` = 标记为 verified，`archive` = 标记为 deprecated
- 状态变更自动更新 verification.md / context.md
- deliverables.md 记录结构化产出（实验结果、模型指标），供周报自动读取

## 实现要点

- 源码入口：`src/ruvp/commands/feature.rs`
- 数据结构：`src/ruvp/common.rs` FeatureRegistry / FeatureEntry
- `feature_new` 创建目录 + 6 个模板文件 + 注册 registry + 更新 AI_CONTEXT.md
- `feature_status` 变更状态时自动同步 verification.md（verified）和 context.md（deprecated）
- `update_ai_context_features` 通过 marker 注释维护活跃特性列表
