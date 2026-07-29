# PROJECT_STATE

> 本文件记录系统当前状态，是 AI 的首要参考文档。

---

## 项目信息

- **项目名称**：ai-uvp
- **当前阶段**：Beta
- **最后更新**：2026-07-27

## 技术栈

- 语言：Rust (edition 2024)
- CLI 框架：clap 4
- 序列化：serde + serde_yaml + toml
- Web（看板）：axum + tokio + rust-embed（后端）/ Svelte 5 + Vite + TailwindCSS（前端）
- 文档站点：MkDocs (Material 主题)
- 构建产物：跨平台二进制 (uvp)

## 活跃特性

| 编号 | 标题 | 状态 | 最后更新 |
|------|------|------|----------|
| FEAT-001 | Feature Ledger 系统 | implemented | 2026-06-19 |
| FEAT-002 | 项目初始化 (uvp init) | implemented | 2026-06-19 |
| FEAT-003 | ADR 管理 (uvp adr) | implemented | 2026-06-19 |
| FEAT-004 | 特性生命周期管理 (uvp feature) | implemented | 2026-06-19 |
| FEAT-005 | Obsidian 同步 (uvp obsidian) | implemented | 2026-06-19 |
| FEAT-006 | 状态展示 (uvp status) | implemented | 2026-06-19 |
| FEAT-007 | 配置管理 (uvp config) | implemented | 2026-06-19 |
| FEAT-008 | 文件头 Meta 管理 | implemented | 2026-06-19 |
| FEAT-009 | 文档一致性检查 (uvp check) | implemented | 2026-06-19 |
| FEAT-010 | 项目打包与分发 | implemented | 2026-06-19 |
| FEAT-011 | Mkdocs 页面渲染 (uvp render) | implemented | 2026-06-19 |
| FEAT-012 | 终端显示设计 (uvp UI) | implemented | 2026-06-19 |
| FEAT-013 | IDE 规则生成与 Skill 部署 (uvp ide) | implemented | 2026-06-19 |
| FEAT-014 | TODO 管理 (uvp todo) | implemented | 2026-06-19 |
| FEAT-015 | 6步闭环工作流 (uvp-workflow skill) | implemented | 2026-06-19 |
| FEAT-016 | Feature 生命周期模板 (uvp-feature-lifecycle skill) | implemented | 2026-06-19 |
| FEAT-017 | 文件修改联动规则 (uvp-file-coupling skill) | implemented | 2026-06-19 |
| FEAT-018 | 周报生成 (uvp-weekly-report skill) | implemented | 2026-06-19 |
| FEAT-019 | 全局看板 (uvp kanban) | implementing | 2026-07-27 |

## 系统架构

- 单二进制 CLI 工具 `uvp`，通过子命令提供功能
- 核心命令：init / adr / feature / status / check / obsidian / config / render / ide / todo / kanban
- 配置层级：全局 `~/.uvp/uvp.toml` + 项目级 `uvp.toml`
- 文档体系：Feature Ledger (spec/plan/changelog/verification/context) + ADR
- Claude Code Skills：workflow / feature-lifecycle / file-coupling / meta-header / weekly-report

## 已知约束

- Obsidian 高级功能（--watch / --with-metadata / --check-links / --generate-moc）为占位符，尚未实现
- 跨平台构建需要分别编译 (aarch64-apple-darwin / x86_64-pc-windows)
