---
doc_type: feature-spec
title: "FEAT-008: 文件头 Meta 管理"
date: 2026-06-19
feat_id: "FEAT-008"
status: implemented
updated: 2026-06-19
related_adr: "0001"
---

# FEAT-008: 文件头 Meta 管理

## 概述

统一管理 `docs/` 目录下所有 markdown 文件的 YAML front matter（文件头 meta），确保文档机器可读、人可读，兼容 Material for Mkdocs。包含：生成函数、格式规范、各文档类型字段定义、一致性检查、AI skill 约束。

## 接口定义

### Rust API

```rust
// src/ruvp/common.rs
pub fn generate_meta_header(
    title: &str,
    doc_type: Option<&str>,
    extra: Option<&HashMap<String, String>>
) -> String;
```

参数：
- `title`：文档标题（必填）
- `doc_type`：文档类型标识（推荐）
- `extra`：附加字段键值对（可选）

返回格式：
```yaml
---
title: "文档标题"
date: YYYY-MM-DD
doc_type: xxx
key: "value"
---

```

### 一致性检查（uvp check）

`uvp check --adr` 的第 2 项检查包含 front matter 一致性：
- 检测 `---` 包裹的 YAML 块
- 解析 key-value 字段
- 对比 meta 中 `status` 与正文解析的状态是否一致

### AI Skill 接口

skill 名称：`uvp-meta-header`

触发条件：在 `docs/` 下创建或修改任何 `.md` 文件

## 行为规格

### 通用必填字段

所有 `docs/` 下的 markdown 文件必须包含：

| 字段 | 格式 | 说明 |
|------|------|------|
| `title` | 字符串（引号包裹） | 文档标题 |
| `date` | `YYYY-MM-DD` | 创建日期 |

### 推荐字段

| 字段 | 格式 | 说明 |
|------|------|------|
| `doc_type` | 枚举字符串 | 文档类别标识 |
| `related_feature` | `"FEAT-XXX"` | 关联 Feature |
| `tags` | YAML 列表 | 搜索关键词 |
| `status` | 枚举字符串 | 文档状态 |

### doc_type 枚举

| doc_type | 用途 |
|----------|------|
| `adr` | 架构决策记录 |
| `feature-spec` | Feature 规格 |
| `feature-changelog` | Feature 变更日志 |
| `feature-verification` | Feature 验证记录 |
| `feature-experiment` | Feature 实验记录 |
| `feature-context` | Feature AI 上下文 |
| `feature-plan` | Feature 实施计划 |
| `feature-deliverables` | Feature 产出记录 |
| `feature-index` | Feature 索引（自动生成） |
| `adr-index` | ADR 索引（自动生成） |
| `project-state` | 项目状态 |
| `prd` | 产品需求文档 |
| `research` | 研究/文献综述 |
| `experiment` | 实验过程/结果 |
| `technical` | 技术方案/API 设计 |
| `meeting` | 会议记录/决策 |
| `tutorial` | 教程/使用指南 |
| `summary` | 项目状态/周报 |

### 各文档类型专有字段

#### ADR 文档

```yaml
adr_id: "0001"
status: proposed | accepted | superseded | deprecated
supersedes: null
related_features:
  - FEAT-001
```

#### Feature 文档（spec/changelog/verification/context/plan/deliverables）

```yaml
feat_id: "FEAT-001"
status: planned | in_progress | implemented | verified | ...
updated: YYYY-MM-DD
related_adr: "0001"
```

#### 自动生成文档（index/registry）

```yaml
auto_generated: true
source: feature-registry.yaml | adr-directory-scan
```

### 生成行为

1. `uvp feature new` 创建 Feature 时，自动为每个子文件（spec/changelog/verification/context/deliverables）写入对应 meta 头
2. `uvp adr` 创建 ADR 时，模板包含 meta 头
3. `uvp render` 生成索引时，添加 `auto_generated: true` meta 头
4. `generate_meta_header()` 自动填入当天日期

### 解析行为（parse_front_matter）

位于 `src/ruvp/commands/check.rs`:

1. 检查文件是否以 `---` 开头
2. 找到第二个 `---` 作为 front matter 结束
3. 用正则逐行解析 `key: value` 格式
4. 返回 `HashMap<String, String>`

### 检查规则

`uvp check` 中 ADR 一致性检查包含：
- front matter 中 `status` 字段值必须与正文 `状态：xxx` / `Status: xxx` 解析结果一致
- 不一致时报错：`{filename}: front matter 状态 '{meta_status}' 与正文状态 '{status}' 不一致`

### AI Skill 执行步骤

1. 解析用户意图 → 选择 `doc_type`
2. 生成完整 meta 头（title + date 必填，doc_type 推荐）
3. 编写文档内容
4. 写入前验证：`---` 开头 → 包含 title → 包含 date → `---` 结尾

## 约束

### 格式约束

- 必须使用 `---` 包裹（YAML front matter 标准格式）
- 字符串值用双引号包裹
- 日期使用 ISO 8601 格式 (`YYYY-MM-DD`)
- 正文第一行应为 Markdown 标题 `# ...`，紧跟在 meta 块之后

### 兼容性

- 兼容 Material for Mkdocs 的 meta 语法
- 兼容 Obsidian 的 YAML front matter 格式
- 使用 `serde_yaml` 可解析的标准 YAML 子集

### 特殊情况

- 草稿笔记：也必须加 meta 头，设 `status: draft`
- 外部内容复制：复制后立即添加 meta 头
- 自动生成文档：`uvp` 命令自动添加合规 meta 头

## 验收标准

- [x] `generate_meta_header()` 正确生成包含 title、date 的 YAML front matter
- [x] `generate_meta_header()` 支持可选的 doc_type 和 extra 字段
- [x] `uvp feature new` 创建的所有子文件包含正确 meta 头
- [x] `uvp check` 检测 ADR front matter 与正文状态不一致
- [x] `parse_front_matter()` 正确解析 YAML front matter 为 HashMap
- [x] `uvp-meta-header` skill 定义完整，包含字段说明和模板
- [x] meta 格式兼容 Material for Mkdocs
