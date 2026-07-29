---
doc_type: feature-spec
title: "FEAT-019: 全局看板 (uvp kanban)"
date: 2026-07-27
feat_id: "FEAT-019"
status: planned
updated: 2026-07-27
related_adr: "0007"
---

# FEAT-019: 全局看板 (uvp kanban)

## 概述

为 uvp 新增 `kanban` 子命令，启动本地 Web 服务器，聚合展示所有注册项目的 TODO、Feature、ADR、Roadmap，并通过关联连线构建端到端可追溯视图。

## 验收标准

1. `uvp kanban` 启动本地 HTTP 服务器并自动打开浏览器
2. 看板展示 `~/.uvp/uvp.toml` 中注册的所有项目
3. 每个项目展示 Roadmap、Features、ADR、TODO 四个子区域
4. 子区域之间通过连线展示关联关系（TODO→ADR→Feature→Roadmap）
5. 鼠标悬浮节点时，关联链条高亮
6. 可在看板上直接添加/删除/完成 TODO，变更写回项目 `docs/TODO.md`
7. 关联标记缺失时（如 LLM 未运行），不影响看板其余功能
8. 前端构建产物通过 rust-embed 嵌入二进制，保持单文件分发

## 接口定义

### CLI

```
uvp kanban              # 启动看板服务器，默认 localhost:3000，自动打开浏览器
uvp kanban --port 8080  # 指定端口（可选）
```

### 全局配置扩展 (~/.uvp/uvp.toml)

```toml
[[projects]]
path = "G:/600_PycharmProjects/ai-uvp"

[[projects]]
path = "G:/600_PycharmProjects/another-project"
```

### REST API（后端 → 前端）

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/api/projects` | 获取所有注册项目及元信息 |
| GET | `/api/projects/:id/overview` | 获取单个项目的完整数据（TODO + Feature + ADR + Roadmap + 关联关系） |
| POST | `/api/projects/:id/todos` | 添加 TODO |
| PATCH | `/api/projects/:id/todos/:todo_id` | 修改 TODO 状态（完成/取消完成） |
| DELETE | `/api/projects/:id/todos/:todo_id` | 删除 TODO |

### 数据关联标记

| 关联方向 | 标记格式 | 提取方式 |
|----------|----------|----------|
| TODO → ADR | `[ADR-xxx]` 在 TODO 内容中 | 正则 `\[ADR-(\d+)\]` |
| ADR → Feature | front matter `related_features: [FEAT-xxx]` | YAML 解析 |
| Feature → Roadmap | Roadmap 条目中 `$FEAT-xxx` | 正则 `\$FEAT-(\d+)` |

## 行为规格

### 启动流程
1. 解析 `~/.uvp/uvp.toml` 中的 `[[projects]]` 列表
2. 验证各项目路径存在且包含 `pyproject.toml` 或 `uvp.toml`
3. 构建前端静态文件的 axum 路由（rust-embed 提供）
4. 注册 REST API 路由
5. 启动 HTTP 服务器
6. 自动打开浏览器

### 数据读取
- 项目元信息：从 `pyproject.toml` 的 `[project]` 表读取 `name`、`version`、`description`
- TODO：解析 `docs/TODO.md`，格式 `- [x] #id content [ADR-xxx] <!-- date -->`
- Feature：读取 `docs/_meta/feature-registry.yaml`
- ADR：扫描 `docs/ADR/*.md`（或配置的 adr directory），解析 front matter 的 `status` 和 `related_features`
- Roadmap：读取 `docs/PRD/roadmap.md` 或 `docs/roadmap.md`，提取 `$FEAT-xxx` 标签

### 关联构建
1. 从 TODO 内容提取 `[ADR-xxx]` → 建立 TODO→ADR 边
2. 从 ADR front matter 提取 `related_features` → 建立 ADR→Feature 边
3. 从 Roadmap 条目提取 `$FEAT-xxx` → 建立 Feature→Roadmap 边
4. 任一环节标记缺失时，该边不创建，不影响已有边

### TODO 写回
- 添加/删除/完成 TODO 时，直接修改对应项目的 `docs/TODO.md`
- 写回格式与 `uvp todo` 命令保持一致

### 错误处理
- 项目路径不存在：跳过该项目，前端标记为"不可用"
- 文件解析失败：该子区域显示为空，不影响其他子区域
- 端口被占用：提示用户指定其他端口

## 约束

- 仅单人本地使用，不考虑并发写入
- 无数据库，纯文件系统读写
- 不调用 LLM API，仅读取 LLM 离线预处理的标记
- 前端嵌入二进制，保持单文件分发
- UI 风格统一（Skeleton UI/shadcn-svelte + TailwindCSS 统一主题）
