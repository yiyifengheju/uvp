---
doc_type: ai-context
title: "AI_CONTEXT"
date: "{DATE}"
project: "{project_name}"
---

# AI_CONTEXT

> 本文件是 AI 编程工具的项目上下文。AI 在执行任务时必须遵守以下规则。

---

## 一、核心规则（强制）

### 1. 工作流：6 步闭环
每次编程任务必须遵循 **Decide → Define → Plan → Implement → Verify → Distill** 流程。
详见 skill：`~/.uvp/skills/uvp-workflow/SKILL.md`

### 2. 文件联动：改代码必须同步文档
修改 `src/` 后**立即**更新 `changelog.md`；接口变更同步 `spec.md`；产出记录到 `deliverables.md`。
详见 skill：`~/.uvp/skills/uvp-file-coupling/SKILL.md`

### 3. Feature 生命周期
Feature 有 6 种状态（idea→planned→implementing→verifying→verified→closed），每个状态有对应的文档要求。
详见 skill：`~/.uvp/skills/uvp-feature-lifecycle/SKILL.md`

### 4. 文档 Meta 头
`docs/` 下所有 markdown 必须包含 YAML front matter（至少 `title` + `date`）。
详见 skill：`~/.uvp/skills/uvp-meta-header/SKILL.md`

### 5. 周报生成
周报统一放在 `docs/周报/`，文件名 `YYYYMMWNN.md`，按大纲模板生成。
详见 skill：`~/.uvp/skills/uvp-weekly-report/SKILL.md`

---

## 二、快速参考

| 你要做什么 | 命令 | 必须同步更新 |
|-----------|------|-------------|
| 技术决策 | `uvp a "决策标题"` | ADR registry 自动更新 |
| 创建 Feature | `uvp f new "标题" --adr-ref NNNN` | 回写 ADR 的 `related_features` |
| 修改代码 | — | `changelog.md`（必须！）、`spec.md`（如接口变） |
| 运行实验 | — | `deliverables.md` |
| 完成验证 | — | `verification.md` |
| 关闭 Feature | `uvp f close FEAT-NNN` | `context.md`、`PROJECT_STATE.md` |
| 创建文档 | — | 必须包含 meta 头 |
| 写周报 | — | `docs/周报/YYYYMMWNN.md`，按大纲模板 |

---

## 三、项目事实

### 你应该优先阅读
1. docs/PROJECT_STATE.md
2. docs/architecture/current.md
3. docs/features/*/spec.md
4. docs/features/*/context.md
5. docs/_meta/feature-registry.yaml

### 你不应该默认阅读
- docs/adr/ 下的全部历史 ADR
- reference/ 下的论文和外部资料
- 已标记为 Superseded / Deprecated 的文档

### 当前项目状态
- 项目名称：{project_name}
- 技术栈：（待填写）
- 当前阶段：初始化
- 优先目标：（待填写）
- 禁止事项：
  - 不新增未经确认的第三方服务
  - 不修改数据库 schema 而不更新 migration
  - 不修改 feature spec 而直接改代码

### 活跃特性列表
<!-- 此列表由 uvp feature new/close 自动维护，不要手动编辑 -->
{active_features}
