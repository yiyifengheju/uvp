---
doc_type: feature-plan
title: "FEAT-019 Plan"
date: 2026-07-27
feat_id: "FEAT-019"
updated: 2026-07-27
---

# FEAT-019 Plan

## 实施步骤

### Phase 1: 后端基础

1. 扩展 `UvpConfig`：在 `config.rs` 中添加 `projects: Vec<ProjectEntry>` 字段及序列化/反序列化支持
2. 新增 `commands/kanban.rs`：实现 `uvp kanban` 子命令入口（解析 `--port` 参数，启动服务器，打开浏览器）
3. 在 `main.rs` 中注册 `Kanban` 子命令
4. 添加 Cargo 依赖：`axum`、`rust-embed`、`tower-http`（cors/static）、`serde_json`

### Phase 2: 数据层

5. 实现项目数据读取模块（可在 `commands/kanban.rs` 或新建 `kanban/` 子模块）：
   - 读取 `pyproject.toml` 提取项目元信息
   - 复用 `common.rs` 的 Feature Registry 读取
   - 复用 `todo_cmd.rs` 的 TODO 解析逻辑（提取为公共函数）
   - ADR 扫描与 front matter 解析
   - Roadmap 文件读取与 `$FEAT-xxx` 提取
6. 实现关联构建：从各数据源的标记构建 TODO→ADR→Feature→Roadmap 的边列表
7. 实现 REST API 路由（GET /api/projects, GET /api/projects/:id/overview, POST/PATCH/DELETE todos）

### Phase 3: TODO 写回

8. 实现 TODO 写回逻辑：添加/完成/删除 TODO 后修改 `docs/TODO.md`（复用 `todo_cmd.rs` 的序列化格式）

### Phase 4: 前端

9. 初始化前端项目：`src/ruvp/web/`（Svelte 5 + Vite + TailwindCSS + Skeleton UI）
10. 实现看板布局：项目分区（竖向） → 子区域（横向：Roadmap / Features / ADR / TODO）
11. 集成 D3.js/elkjs 绘制关联连线
12. 实现悬浮高亮交互：鼠标悬浮节点时关联链条高亮
13. 实现 TODO 操作 UI：添加/完成/删除，调用后端 API

### Phase 5: 嵌入与集成

14. 配置 `rust-embed` 嵌入前端 `dist/` 构建产物
15. axum 静态文件路由 serve 嵌入的前端资源
16. 端到端测试：启动服务器，验证页面加载和 TODO 操作

## 风险评估

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| 连线布局算法复杂度高 | 节点多时性能下降或布局混乱 | 使用 elkjs 自动布局，设置合理的节点间距 |
| TODO 解析逻辑重复 | todo_cmd.rs 和 kanban 模块重复代码 | 将 TODO 解析提取为 common.rs 中的公共函数 |
| 前端构建产物未嵌入时开发不便 | 开发时需两个进程 | 开发模式 proxy 到 Vite dev server，生产时 rust-embed |
| Roadmap 文件路径不统一 | 各项目 roadmap 路径可能不同 | 按优先级尝试 `docs/PRD/roadmap.md` → `docs/roadmap.md` |

## 依赖

- Cargo 新增依赖：axum, tokio (full features), rust-embed, tower-http, serde_json
- Node.js 工具链（构建前端）：npm/pnpm, vite, svelte
- 现有模块复用：`common.rs`（Feature Registry）、`config.rs`（配置读取）、`todo_cmd.rs`（TODO 解析）
