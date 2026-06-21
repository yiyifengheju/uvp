# FEAT-014 Context

> 此文件为 AI 提供压缩上下文，由 Distill 步骤维护。

## 当前状态

implemented

## 核心概念

`uvp todo` 是轻量想法收集箱，定位在 ADR 之前——记录尚未成熟到需要决策的想法和灵感。数据存储在 `docs/TODO.md`，使用 Markdown checkbox 格式，兼容 Obsidian / GitHub / mkdocs。

工作流定位：想法 → TODO → 评估成熟 → ADR → Feature → 代码

## 关键决策

- 单文件存储（docs/TODO.md），不引入数据库
- Markdown checkbox 格式，HTML 注释存日期元数据
- ID 自增，不复用已删除 ID
- init 时自动创建，todo 命令时若不存在也自动创建
- 在线管理通过 FEAT-006 --onboard 面板实现（见 sub-spec-onboard.md）

## 实现要点

- 源码：`src/ruvp/commands/todo_cmd.rs`
- CLI 定义：`src/ruvp/main.rs` Todo + TodoCommands
- init 集成：`src/ruvp/commands/init.rs` 辅助文件步骤
- 解析正则：`^- \[([ x])\] #(\d+) (.+?) <!-- (.+?) -->`
- rebuild_file 全量重建 TODO.md（待办区 + 已完成区）
