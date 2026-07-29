---
doc_type: feature-changelog
title: "FEAT-019 Changelog"
date: 2026-07-27
feat_id: "FEAT-019"
updated: 2026-07-27
---

# FEAT-019 Changelog

> 记录代码变更：改了什么、为什么改、影响范围。

| 日期 | 类型 | 变更 | 说明 |
|------|------|------|------|
| 2026-07-27 | init | 创建特性 | 初始化特性 |
| 2026-07-27 | added | 扩展 UvpConfig 添加 projects 字段 | config.rs: 新增 ProjectEntry 结构体和 projects 数组，更新合并逻辑和默认模板 |
| 2026-07-27 | added | 新增 Kanban 子命令 | main.rs: 注册 Kanban 子命令 (--port 参数)；commands/mod.rs: 导出 kanban 模块 |
| 2026-07-27 | added | 添加 Web 依赖 | Cargo.toml: axum, tokio, rust-embed, tower-http, serde_json, mime_guess |
| 2026-07-27 | refactor | TODO 解析提取为公共函数 | common.rs: TodoItem/ParsedTodo/parse_todos/rebuild_todo_file 移至 common；todo_cmd.rs 改为调用公共函数 |
| 2026-07-27 | added | ADR/Roadmap 解析 | common.rs: parse_adr_file (front matter 解析), parse_roadmap ($FEAT-xxx 提取), extract_todo_adr_refs ([ADR-xxx] 提取) |
| 2026-07-27 | added | Kanban 后端 REST API | commands/kanban.rs: 项目列表/概览/TODO CRUD 五个 API 端点 + 静态文件服务 |
| 2026-07-27 | added | Kanban 前端 | web/: Svelte 5 + Vite + TailwindCSS 看板 UI，4 列布局 + SVG 连线 + 悬浮高亮 + TODO 操作 |
| 2026-07-27 | fixed | pyproject.toml 解析 | kanban.rs: 改用 serde 结构体解析 pyproject.toml，修复 toml::Value 方式无法读取 version/description 的问题 |
