# FEAT-005 Context

> 此文件为 AI 提供压缩上下文，由 Distill 步骤维护。

## 当前状态

implemented

## 核心概念

`uvp obsidian` 是 Obsidian → 项目的单向知识导入桥梁。Obsidian 是输入端（网页剪藏、论文笔记、AI 问答），项目是产出端（ADR、Feature、代码）。产出由 mkdocs 管理展示，不回推 Obsidian。

工作流：Obsidian 笔记 → `uvp pull` → reference/ → `uvp adr --from-obsidian` → ADR → Feature → 代码

## 关键决策

- 只有 pull 和 sync 两个子命令，没有 push（产出不回推 Obsidian）
- sync 仅限 reference/ 双向同步
- Vault 中的项目路径：`<vault>/Projects/<project-name>/reference/`
- 增量同步基于文件修改时间，较新文件覆盖较旧文件
- exclude_dirs 同时应用于 pull 和 sync

## 实现要点

- 源码入口：`src/ruvp/commands/obsidian.rs`
- 配置：`ObsidianConfig { vault, exclude_dirs }`
- `adr --from-obsidian` 在 `src/ruvp/commands/adr.rs` 中实现（属于 FEAT-003）
