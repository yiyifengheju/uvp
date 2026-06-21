# FEAT-003 Context

> 此文件为 AI 提供压缩上下文，由 Distill 步骤维护。

## 当前状态

implemented

## 核心概念

`uvp adr` 一键创建架构决策记录，使用内置默认模板（`docs/adr/template.md` 或编译内嵌），支持 `--from-obsidian` 从 Vault 导入笔记作为上下文。

## 关键决策

- 只支持一个默认模板，不支持多模板选择
- 模板加载优先级：项目 `docs/adr/template.md` > 内置 `default.md`
- 编号通过扫描 ADR 目录文件名获取，非读取 registry
- registry.md 每次全量重新生成（非追加）
- 支持 `sequential`（NNNN-slug.md）和 `datetime`（YYYYMMDD-HHMM.md）两种命名

## 实现要点

- 源码入口：`src/ruvp/commands/adr.rs`
- 模板加载：`src/ruvp/common.rs` load_adr_template()
- CLI 参数：title（必填）/ --from-obsidian / --status / --open
- Obsidian 匹配策略：模糊匹配关键词，多结果取最短文件名
