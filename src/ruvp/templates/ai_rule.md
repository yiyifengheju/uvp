---
doc_type: ai-rule
title: "Vibe Coding 工作流规则"
date: "{DATE}"
project: "{project_name}"
---

# Vibe Coding 工作流规则

请先阅读 `docs/AI_CONTEXT.md`，其中包含项目上下文和核心规则摘要。

## Skills 配置

本项目已配置以下 UVP Skills（位于 `~/.uvp/skills/`），AI 会根据场景自动调用：

| Skill | 触发场景 | 核心规则 |
|-------|---------|---------|
| **uvp-workflow** | 任何编程任务 | 6 步闭环：Decide→Define→Plan→Implement→Verify→Distill |
| **uvp-file-coupling** | 修改文件时 | 改代码必须立即更新 changelog.md |
| **uvp-feature-lifecycle** | Feature 操作 | Feature 状态流转和文档管理 |
| **uvp-meta-header** | 在 docs/ 创建文档 | 必须包含 YAML front matter（title + date） |
| **uvp-weekly-report** | 写周报/本周总结 | 周报放 `docs/周报/`，按大纲模板生成 |

---

项目：{project_name}
