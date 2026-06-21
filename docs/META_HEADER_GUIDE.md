# Meta 头全局约定 - 使用示例

本文档展示当用户要求 AI 创建文档时，AI 应该如何自动遵守 meta 头规范。

---

## 场景示例

### 用户指令

> "将关于 BERT 模型特征改进的相关研究整理到 `docs/模型改进/`"

### AI 的正确响应

AI 应该自动执行以下操作：

#### 1. 创建目录结构
```bash
mkdir -p docs/模型改进/
```

#### 2. 创建文档并包含标准 meta 头

**文件路径**: `docs/模型改进/bert-feature-improvement.md`

```markdown
---
title: "BERT 模型特征改进研究"
date: "2026-06-16"
doc_type: research
related_feature: "FEAT-002"
tags: [bert, nlp, feature-engineering, transformer]
status: draft
author: AI-Assistant
---

# BERT 模型特征改进研究

## 研究背景

本文档整理关于 BERT 模型特征改进的最新研究成果...

## 主要发现

### 1. 特征融合方法
...

## 参考文献列表

- [Paper 1] Title, Authors, Year
- [Paper 2] Title, Authors, Year

## 下一步计划

- [ ] 实验方案设计
- [ ] 基线模型准备
```

#### 3. 如果需要创建多个相关文档

**文件 1**: `docs/模型改进/experiment-design.md`
```markdown
---
title: "特征改进实验设计方案"
date: "2026-06-16"
doc_type: experiment
related_research: "bert-feature-improvement"
related_feature: "FEAT-002"
tags: [experiment, design, bert]
status: planned
---

# 特征改进实验设计方案

## 实验目标
...
```

**文件 2**: `docs/模型改进/baseline-results.md`
```markdown
---
title: "基线模型实验结果"
date: "2026-06-16"
doc_type: experiment
parent_experiment: "experiment-design"
tags: [baseline, results, evaluation]
---

# 基线模型实验结果

## 数据集
...
```

---

## 错误示范（AI 不应该这样做）

### ❌ 错误：缺少 meta 头

```markdown
# BERT 模型特征改进研究

## 研究背景

本文档整理关于 BERT 模型特征改进的最新研究成果...
```

**问题**：
- 缺少 YAML front matter
- 无法被 `uvp check` 工具识别和验证
- 缺少元数据（日期、类型、关联等）
- 不符合项目文档规范

### ❌ 错误：meta 头不完整

```markdown
---
title: "BERT 研究"
---

# BERT 模型特征改进研究
...
```

**问题**：
- 缺少必填字段 `date`
- 标题过于简略，不够描述性
- 缺少 `doc_type` 字段（推荐）

---

## AI 执行清单

当收到"在 docs/ 下创建文档"类指令时，AI 必须：

### ✅ 必须执行的步骤

1. **解析用户意图**
   - 确定文档主题和内容范围
   - 选择合适的 `doc_type`（参考常用类型表）

2. **生成完整 meta 头**
   ```yaml
   ---
   title: "<简洁且描述性的标题>"
   date: "<YYYY-MM-DD>"
   doc_type: "<合适的文档类型>"
   ---
   ```

3. **根据上下文补充可选字段**
   - `related_feature`: 如果与某个 Feature 相关
   - `tags`: 关键词标签
   - `status`: 文档状态（draft/planned/published）
   - `author`: 作者信息

4. **组织文档内容**
   - 使用标准的 markdown 格式
   - 保持结构清晰（标题层级、列表、表格等）

5. **验证输出**
   - 确认 meta 头格式正确
   - 确认必填字段齐全

### 📋 doc_type 选择指南

| 用户意图 | 推荐的 doc_type | 示例 |
|----------|----------------|------|
| 整理研究文献 | `research` | 论文总结、文献综述 |
| 记录实验过程 | `experiment` | 实验设计、结果记录 |
| 设计技术方案 | `technical` | API 设计、架构图 |
| 记录会议内容 | `meeting` | 会议纪要、决策记录 |
| 编写教程文档 | `tutorial` | 使用指南、操作手册 |
| 总结项目状态 | `summary` | 周报、里程碑总结 |

---

## 自动化检查

创建完文档后，可以运行检查命令验证是否符合规范：

```bash
$ uvp check

# 输出示例：
# ==================================================
# 文档一致性检查
#
# 6. Meta 头一致性检查（全局约定）
#   已检查 25 个 markdown 文件
#   ✓ 所有文件都包含有效的 meta 头
#
# 所有检查通过！
```

如果某个文件不符合规范：
```bash
# 输出示例：
# 6. Meta 头一致性检查（全局约定）
#   已检查 26 个 markdown 文件
#   ✗ docs/模型改进/bert-research.md: 缺少必填字段: date
#
# 发现 1 个问题
# 提示：运行 uvp render 更新渲染页面
```

---

## 最佳实践建议

### 1. 标题命名规范
- ✅ **好**: `"BERT 模型特征改进研究"` - 清晰描述内容
- ✅ **好**: `"2026-Q2 产品路线图"` - 包含时间范围
- ❌ **差**: `"文档1"` - 过于模糊
- ❌ **差**: `"关于那个东西的研究"` - 不正式

### 2. 日期格式
- 统一使用 ISO 8601 格式：`YYYY-MM-DD`
- 示例：`"2026-06-16"`
- 对于持续更新的文档，使用最后更新日期

### 3. 使用有意义的 tags
- ✅ **好**: `[nlp, transformer, optimization]` - 具体技术标签
- ✅ **好**: `[feature-feat-002, research]` - 包含关联 ID
- ❌ **差**: `[文档, 重要]` - 过于宽泛

### 4. 关联关系维护
- 如果文档属于某个 Feature，填写 `related_feature`
- 如果是系列文档，使用 `parent_doc` 或 `related_docs` 字段
- 这有助于后续文档检索和知识图谱构建

---

## 总结

**核心原则**：
> 在 `docs/` 目录下创建任何 markdown 文档时，**必须**以 YAML front matter 开头，
> 至少包含 `title` 和 `date` 两个字段。这是项目的强制规范，不是可选操作。

遵循此规范的好处：
1. **可追溯性**：知道文档何时创建、由谁创建
2. **可搜索性**：通过元数据快速定位相关文档
3. **一致性**：所有文档格式统一，便于维护
4. **自动化支持**：工具可以自动解析和处理元数据
5. **质量保证**：`uvp check` 可以自动检测不规范文档

---

*本文档作为 AI 行为规范的补充说明，应与 `CLAUDE.md` / `AI_CONTEXT.md` 配合使用*
