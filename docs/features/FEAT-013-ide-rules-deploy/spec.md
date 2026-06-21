---
doc_type: feature-spec
title: "FEAT-013: IDE 规则生成与 Skill 部署 (uvp ide)"
date: 2026-06-19
feat_id: "FEAT-013"
status: implemented
updated: 2026-06-19
related_adr: "0001"
---

# FEAT-013: IDE 规则生成与 Skill 部署 (uvp ide)

## 概述

`uvp ide <tool>` 为指定的 AI 编程工具生成规则文件，并将 skills 部署到项目目录或全局目录，实现一键配置 AI 编程环境。

## 接口定义

### CLI 参数

```bash
uvp ide <TOOL>
```

`TOOL`：目标 IDE/AI 工具名称（claude / cursor / windsurf / cline / trae）

### 支持的 IDE 及规则文件

| IDE | 规则文件路径 |
|-----|------------|
| Claude Code | `CLAUDE.md` |
| Cursor | `.cursorrules` |
| Windsurf | `.windsurfrules` |
| Cline | `.clinerules` |
| Trae | `.trae/rules.md` |

## 行为规格

### 规则文件生成

1. 在项目中查找已有的 AI 规则文件（按优先级遍历所有已知规则文件路径）
2. 找到第一个存在的规则文件作为源内容
3. 将源内容写入目标 IDE 对应的规则文件路径
4. 如果目标文件已存在，覆盖并提示

### Skill 部署

1. 调用 `config::deploy_skills_to_ide(tool, project_dir)` 部署 skills
2. 支持项目级部署（如 `.claude/skills/`）和全局部署
3. 部署完成后显示已部署文件数量和路径

### 部署的 Skills

从 `~/.uvp/skills/` 或内置 skills 复制到项目目录：
- `uvp-workflow`
- `uvp-feature-lifecycle`
- `uvp-file-coupling`
- `uvp-meta-header`
- `uvp-weekly-report`

### 错误处理

- 未找到任何 AI 规则文件：提示运行 `uvp init`
- 不支持的 IDE 名称：提示错误
- 写入失败：显示错误信息

## 约束

### 幂等性

- Skills 已是最新时跳过部署
- 规则文件内容相同时仍覆盖（无 diff 检查）

## 验收标准

- [x] 正确查找源规则文件并生成目标规则文件
- [x] 支持 claude/cursor/windsurf/cline/trae 五种 IDE
- [x] Skills 正确部署到项目目录
- [x] 已存在的目标文件被覆盖并提示
- [x] 未找到规则文件时给出有意义的错误提示
