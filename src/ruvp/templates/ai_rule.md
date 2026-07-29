---
doc_type: ai-rule
title: "Vibe Coding 工作流规则"
date: "{DATE}"
project: "{project_name}"
---

# Vibe Coding 工作流规则

请先阅读 `docs/AI_CONTEXT.md`，其中包含项目上下文、核心规则和关键模板。

## Skills 配置

本项目已配置以下 UVP Skills（位于 `.claude/skills/`），AI 会根据场景自动调用：

| Skill | 触发场景 | 核心规则 |
|-------|---------|---------|
| **uvp-workflow** | 任何编程任务 | 6 步闭环：Decide→Define→Plan→Implement→Verify→Distill |
| **uvp-file-coupling** | 修改文件时 | 改代码必须立即更新 changelog.md |
| **uvp-feature-lifecycle** | Feature 操作 | Feature 状态流转和文档管理 |
| **uvp-meta-header** | 在 docs/ 创建文档 | 必须包含 YAML front matter（title + date） |
| **uvp-weekly-report** | 写周报/本周总结 | 周报放 `docs/周报/`，按大纲模板生成 |

### 关于 Skill 调用

- **编程任务**：通过 Skill 工具调用 `uvp-workflow`，获取详细步骤指导
- **闭环检查（verification.md / context.md）**：模板已内联在 `docs/AI_CONTEXT.md` 中，**不需要**再读 skill 文件
- **只有需要详细模板细节时**才调用 skill（如验证证据清单、周报格式）
- **实验产出**：用户指定保存路径时，按 `uvp-feature-lifecycle` 中的实验产出模板填写，文件名以日期开头（如 `2026-07-29-xxx.md`）

---

项目：{project_name}
