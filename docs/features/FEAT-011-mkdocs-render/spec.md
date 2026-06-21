---
doc_type: feature-spec
title: "FEAT-011: Mkdocs 页面渲染 (uvp render)"
date: 2026-06-19
feat_id: "FEAT-011"
status: implemented
updated: 2026-06-19
related_adr: "0001"
---

# FEAT-011: Mkdocs 页面渲染 (uvp render)

## 概述

`uvp render` 将 feature-registry.yaml 和 ADR 目录扫描结果渲染为 Mkdocs 可展示的 markdown 索引页面，支持按类别渲染和一致性检查模式。

## 接口定义

### CLI 参数

```bash
uvp render [OPTIONS]

Options:
  --features    仅渲染 Feature Registry 页面
  --adr         仅渲染 ADR Registry 页面
  --check       仅检查一致性（不写入文件）
  -h, --help    显示帮助信息
```

别名：`uvp r`

### 筛选逻辑

- 不指定选项：渲染全部（features + adr）
- `--features`：仅渲染 `docs/features/index.md`
- `--adr`：仅渲染 `docs/adr/registry.md`
- `--check`：对比生成内容与现有文件，不一致时报错并以 exit code 1 退出

## 行为规格

### Feature Registry 渲染

输入：`docs/_meta/feature-registry.yaml`
输出：`docs/features/index.md`

生成内容：
1. YAML front matter（`doc_type: feature-index`, `auto_generated: true`, `source: feature-registry.yaml`）
2. 状态概览表（按 planned/in_progress/implemented/verified 统计）
3. 全部特性表（编号、标题、状态 emoji + 文字、关联 ADR、创建/更新日期）

### ADR Registry 渲染

输入：`docs/adr/*.md`（排除 template.md 和 registry.md）
输出：`docs/adr/registry.md`

生成内容：
1. YAML front matter（`doc_type: adr-index`, `auto_generated: true`, `source: adr-directory-scan`）
2. 状态概览表（按 proposed/accepted/superseded/deprecated 统计）
3. 全部决策表（编号链接、标题、状态 emoji + 文字、关联 Feature、日期）

### ADR 信息提取

从每个 ADR 文件解析：
- 编号：文件名前 4 位数字
- 标题：正文第一个 `# ` 标题
- 状态：`parse_adr_status()` 从正文提取
- 关联 Feature：正则匹配 `关联\s*Feature[：:]*\s*(.+)`
- 日期：文件名时间戳（`YYYYMMDD-HHMM.md`）或文件修改时间

### 一致性检查模式（--check）

- 生成期望内容但不写入文件
- 对比现有文件内容，完全相同则通过
- 全部通过：输出成功信息
- 存在不一致：输出失败信息，`exit(1)`

## 约束

### 幂等性

- 同一数据源多次渲染结果相同（日期字段取当天）
- `--check` 模式不修改任何文件

### 兼容性

- 输出兼容 Material for Mkdocs 的 YAML front matter
- 使用标准 markdown 表格语法

## 验收标准

- [x] 正确读取 feature-registry.yaml 生成 features/index.md
- [x] 正确扫描 ADR 目录生成 adr/registry.md
- [x] 生成的文件包含正确的 YAML front matter
- [x] `--check` 模式正确检测一致性
- [x] `--features` / `--adr` 筛选正常工作
- [x] ADR 状态/标题/关联 Feature 解析正确
