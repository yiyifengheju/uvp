---
name: "uvp-meta-header"
description: "docs/ 目录下 markdown 文件必须包含 YAML front matter（title + date）。创建文档时触发。"
---

# UVP Meta Header 规范

**触发条件**: 在 `docs/` 下创建或修改任何 `.md` 文件

---

## 核心规则

> `docs/` 下所有 markdown **必须**以 YAML front matter 开头，至少包含 `title` 和 `date`。

验证命令：`uvp check`（第 6 项检查）

---

## 字段说明

### 必填

| 字段 | 示例 | 说明 |
|------|------|------|
| `title` | `"BERT 模型特征改进研究"` | 简洁描述文档内容 |
| `date` | `"2026-06-16"` | ISO 8601 格式 |

### 推荐

| 字段 | 示例 | 说明 |
|------|------|------|
| `doc_type` | `research` / `experiment` / `technical` | 标识文档类别 |
| `related_feature` | `"FEAT-002"` | 关联的 Feature |
| `tags` | `[nlp, transformer]` | 搜索关键词 |
| `status` | `draft` / `published` | 文档状态 |

### doc_type 选择

| 用户意图 | doc_type |
|---------|----------|
| 整理研究/文献综述 | `research` |
| 记录实验过程/结果 | `experiment` |
| 技术方案/API设计 | `technical` |
| 会议记录/决策 | `meeting` |
| 教程/使用指南 | `tutorial` |
| 项目状态/周报 | `summary` |
| 特性规格 | `feature-spec` |
| 变更日志 | `feature-changelog` |
| 验证记录 | `feature-verification` |
| 架构决策 | `adr` |

---

## 模板

### 最小合法

```yaml
---
title: "文档标题"
date: "YYYY-MM-DD"
---
```

### 完整推荐

```yaml
---
title: "文档标题"
date: "YYYY-MM-DD"
doc_type: research
related_feature: "FEAT-XXX"
tags: [keyword1, keyword2]
status: draft
---
```

---

## 执行步骤

1. 解析用户意图 → 选择 `doc_type`
2. 生成完整 meta 头（title + date 必填，doc_type 推荐）
3. 编写文档内容
4. 写入前验证：`---` 开头 → 包含 title → 包含 date → `---` 结尾

---

## 示例

### ✅ 正确

**用户**: "将 BERT 特征改进研究整理到 `docs/模型改进/`"

```markdown
---
title: "BERT 模型特征改进研究"
date: "2026-06-16"
doc_type: research
tags: [bert, nlp, feature-engineering]
---

# BERT 模型特征改进研究

## 研究背景
...
```

### ❌ 错误

```markdown
# BERT 模型特征改进研究
（缺少 YAML front matter）
```

---

## 特殊情况

- **草稿笔记**: 也必须加 meta 头，设 `status: draft`
- **外部内容复制**: 复制后立即添加 meta 头
- **自动生成文档**: `uvp` 命令会自动添加合规 meta 头

---

*验证：`uvp check`*
