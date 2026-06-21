---
doc_type: feature-spec
title: "FEAT-017: 文件修改联动规则 (uvp-file-coupling skill)"
date: 2026-06-19
feat_id: "FEAT-017"
status: implemented
updated: 2026-06-19
related_adr: "0001"
---

# FEAT-017: 文件修改联动规则 (uvp-file-coupling skill)

## 概述

`uvp-file-coupling` skill 定义文件修改时的联动更新规则，确保修改代码后同步更新 changelog、spec、deliverables 等关联文件，防止文档与代码状态不一致。

## 接口定义

### Skill 文件

- 路径：`src/ruvp/skills/uvp-file-coupling/SKILL.md`
- 部署位置：`~/.uvp/skills/uvp-file-coupling/SKILL.md` 或项目 `.claude/skills/`
- 触发条件：修改 `src/`、`docs/features/`、`docs/adr/` 下的任何文件

## 行为规格

### 必须遵守（违反 = 数据不一致）

| # | 触发动作 | 必须同步更新 | 自动/手动 |
|---|---------|-------------|----------|
| 1 | 创建 ADR | `docs/adr/registry.md` | `uvp adr` 自动 |
| 2 | 关联 Feature 到 ADR | ADR `related_features` + 正文 "AI 上下文" | 手动（2处） |
| 3 | 创建 Feature | `feature-registry.yaml` + `docs/features/index.md` | `uvp f new` 自动 |
| 4 | **修改 src/ 代码** | **`FEAT-xxx/changelog.md`** | **手动（最重要）** |
| 5 | 修改 API/接口 | `FEAT-xxx/spec.md` 接口定义 | 手动 |
| 6 | 运行实验/试错 | `FEAT-xxx/deliverables.md` | 手动 |
| 7 | 完成验证 | `FEAT-xxx/verification.md` | 手动 |

### 应该遵守

| # | 触发动作 | 应该同步更新 |
|---|---------|-------------|
| 8 | Feature 状态变更 | `feature-registry.yaml`（`uvp f close` 自动） |
| 9 | 文档变更影响项目事实 | `docs/AI_CONTEXT.md` |
| 10 | 更新 spec 验收标准 | 重新运行验证 |
| 11 | 关闭 Feature | `FEAT-xxx/context.md` |
| 12 | 关闭 Feature | `docs/PROJECT_STATE.md` |

### Changelog 编写规范

格式：Keep a Changelog 标准

```markdown
## [YYYY-MM-DD]
### Added / Changed / Fixed / Breaking Changes
- 具体改了什么 + 为什么
```

反模式：
- "Fixed bugs" → 应写 "Fixed null pointer in Parser.parse_line() when input empty"
- "Updated code" → 应写 "Refactored AuthMiddleware to extract TokenValidator class"
- 一天结束后批量写 → 应每个逻辑单元完成后立即写

### ADR ↔ Feature 双向关联

创建 ADR 后又创建 Feature 时，必须回写 ADR：
- front matter 的 `related_features` 列表
- 正文 "AI 上下文" 部分

### Experiment 记录规范

必须记录：算法对比、参数调优、架构方案比较、A/B 测试
可跳过：已有模式的简单应用、小配置变更

### 自检清单

修改文件后的自检：
- 改了 src/ → changelog 更新了吗？
- 改了接口 → spec 同步了吗？
- 做了实验 → deliverables 记录了吗？
- 关闭 Feature → context + PROJECT_STATE 更新了吗？

验证命令：`uvp check`

## 约束

### AI 行为约束

- AI 修改 `src/` 代码后**必须**更新对应 Feature 的 changelog.md
- AI 不可在 changelog 中使用模糊描述
- AI 修改 API 接口后必须同步更新 spec.md

## 验收标准

- [x] SKILL.md 包含完整的联动矩阵（必须 + 应该）
- [x] Changelog 编写规范有正反示例
- [x] ADR ↔ Feature 双向关联规则清晰
- [x] Experiment 记录有模板
- [x] 自检清单完整
- [x] Skill 可通过 `uvp ide` 部署到项目
