---
doc_type: feature-verification
title: "FEAT-019 Verification"
date: 2026-07-27
feat_id: "FEAT-019"
updated: 2026-07-29
---

# FEAT-019 Verification

## 验证状态

已验证

## 验收标准逐项验证

| # | 验收标准 | 结果 |
|---|----------|------|
| 1 | `uvp kanban` 启动本地 HTTP 服务器并自动打开浏览器 | PASS — axum 监听 localhost:3000，open::that 自动打开 |
| 2 | 看板展示 `~/.uvp/uvp.toml` 中注册的所有项目 | PASS — GET /api/projects 返回注册项目列表 |
| 3 | 每个项目展示 Roadmap、Features、ADR、TODO 四个子区域 | PASS — 四列 grid 布局 |
| 4 | 子区域之间通过连线展示关联关系 | PASS — EdgeCanvas SVG 贝塞尔曲线连线 |
| 5 | 鼠标悬浮节点时，关联链条高亮 | PASS — BFS 分层高亮（直接实线 + 间接虚线） |
| 6 | 可在看板上直接添加/删除/完成 TODO | PASS — POST/PATCH/DELETE API + 前端交互 |
| 7 | 关联标记缺失时不影响看板其余功能 | PASS — 缺失边不画，节点正常展示 |
| 8 | 前端通过 rust-embed 嵌入二进制 | PASS — `#[derive(Embed)] #[folder = "web/dist/"]` |

## 可复现命令

```bash
cd src/ruvp && cargo check    # 编译通过
cd src/ruvp/web && npm run build  # 前端构建通过
```

## 附加验证

- Features 列智能折叠（P1）：默认只显示活跃态 ✓
- Roadmap 进度指示（P4）：根据关联 Feature 状态计算完成度 ✓
- 高亮分层（P8）：直接/间接关联视觉区分 ✓
- 快捷键（P3）：Esc 取消锁定、/ 聚焦输入框 ✓
- 多项目折叠（P5）：支持折叠/展开 ✓
