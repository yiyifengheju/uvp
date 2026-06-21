---
doc_type: feature-spec
title: "FEAT-018: 周报生成 (uvp-weekly-report skill)"
date: 2026-06-19
feat_id: "FEAT-018"
status: implemented
updated: 2026-06-19
related_adr: "0001"
---

# FEAT-018: 周报生成 (uvp-weekly-report skill)

## 概述

`uvp-weekly-report` skill 定义从项目状态、Feature、Changelog 自动汇总生成结构化周报的流程和模板，约束 AI 在生成周报时遵循标准化格式。

## 接口定义

### Skill 文件

- 路径：`src/ruvp/skills/uvp-weekly-report/SKILL.md`
- 部署位置：`~/.uvp/skills/uvp-weekly-report/SKILL.md` 或项目 `.claude/skills/`
- 触发条件：用户要求 "写周报"、"生成周报"、"本周总结"、"weekly report"

### 输出文件

- 目录：`docs/周报/`
- 文件名格式：`YYYYMMWNN.md`（NN = ISO 周数，如 W23）

## 行为规格

### 周报模板结构

```markdown
---
title: "周报 YYYY-MM-DD ~ YYYY-MM-DD"
date: "YYYY-MM-DD"
doc_type: weekly-report
week: "YYYYMMWNN"
---

# 周报：MM/DD ~ MM/DD

## 1. 进展概览（3-5句总结）
## 2. 关键成果（量化指标）
## 3. 上周遗留问题闭环
## 4. 工程改动摘要（按 Feature 分组）
## 5. 风险与遗留
## 6. 下周计划
## 7. 相关产物快速索引
```

### 信息采集流程

按以下顺序采集信息：

1. 读取 `docs/_meta/feature-registry.yaml` → Feature 状态
2. 读取各 Feature 的 `changelog.md` → 本周工程改动
3. 读取各 Feature 的 `verification.md` → 关键成果指标
4. 读取各 Feature 的 `deliverables.md` → 产出记录
5. 读取上周周报（如存在）→ 遗留问题闭环检查
6. 读取 `docs/PROJECT_STATE.md` → 项目整体状态
7. 读取 `docs/adr/` 下本周新增 ADR → 决策记录

### 工程改动摘要规则

- 按 Feature 分组展示
- 使用 Added/Changed/Fixed/Breaking 分类
- 从 changelog.md 汇总，不重新描述

### 遗留问题闭环

- 对比上周周报的风险/遗留条目
- 标注状态：已解决 / 部分解决 / 未解决

## 约束

### 格式约束

- 周报必须包含 YAML front matter（title + date + doc_type: weekly-report）
- 时间范围为周一至周日
- 文件名使用 ISO 周数

### 内容约束

- 关键成果必须量化
- 工程改动从 changelog 汇总而非重新编写
- 下周计划必须关联到 Feature

## 验收标准

- [x] SKILL.md 包含完整的周报模板
- [x] 信息采集流程有明确的数据源和顺序
- [x] 文件命名格式规范（YYYYMMWNN.md）
- [x] 周报包含 YAML front matter
- [x] 遗留问题闭环检查逻辑清晰
- [x] Skill 可通过 `uvp ide` 部署到项目
