# FEAT-006 Context

> 此文件为 AI 提供压缩上下文，由 Distill 步骤维护。

## 当前状态

implemented（基础功能），--onboard 待实现

## 核心概念

`uvp status` 是项目状态的快速查看入口。默认在终端输出项目状态（ADR/Feature 统计 + Git 信息），不展示工作流。工作流信息通过 `--help` 获取。`--open` 在浏览器查看 HTML 报告。`--onboard` 启动在线实时面板（新功能，待实现）。

## 关键决策

- 默认只展示项目状态，不展示工作流（减少噪音）
- 工作流通过 --help 查看（天然入口，无需记忆额外命令）
- 去掉 --workflow / --project 分类标志（简化接口）
- --onboard 作为独立子特性，本地 HTTP 服务实现

## 实现要点

- 源码入口：`src/ruvp/commands/status.rs`
- 当前 `--verbose` 展示工作流 → 需改为默认不展示，移到 --help
- `--open` 生成 HTML 到 `.uvp/status.html` 并用 `open::that` 打开
- --onboard 需要新增 HTTP 服务（Phase 2 实现）
