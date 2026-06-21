# FEAT-007 Context

> 此文件为 AI 提供压缩上下文，由 Distill 步骤维护。

## 当前状态

implemented

## 核心概念

`uvp config` 显示合并后的完整配置。配置修改直接编辑文件，无 CLI 写入命令。

## 关键决策

- 去掉所有子命令，只保留默认显示
- 配置修改通过直接编辑 toml 文件，不提供 set 命令
- 优先级：项目配置 > 全局配置 > 默认值

## 实现要点

- 源码：`src/ruvp/commands/config_cmd.rs`（14 行）
- `run()` 调用 `get_effective_config` → `toml::to_string_pretty` → println
- 已删除 `get_config_value` / `set_config_value`（config.rs）
- 已删除 `ConfigCommands` 枚举（main.rs）
