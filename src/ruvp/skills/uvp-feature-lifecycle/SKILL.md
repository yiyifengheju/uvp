---
name: "uvp-feature-lifecycle"
description: "Feature 生命周期管理：创建、状态流转、文档模板、关闭。操作 Feature 时触发。"
---

# UVP Feature 生命周期管理

**触发条件**: 创建/更新/关闭 Feature，或操作 `docs/features/` 目录

---

## 状态流转

```
idea → planned → implementing → verifying → verified → closed
                                              ↑         │
                                              └─ rework ┘
```

| 状态 | 含义 | 进入条件 | 退出条件 |
|------|------|---------|---------|
| **idea** | 初始概念 | `uvp f new` 创建 | spec 完成 |
| **planned** | 准备实施 | spec 已写 | 开始编码 |
| **implementing** | 开发中 | 代码变更开始 | 编码完成，测试就绪 |
| **verifying** | 验证中 | 准备运行验证 | 所有检查完成 |
| **verified** | 已验证 | 验证通过 | 准备提炼 |
| **closed** | 已归档 | 提炼完成 | 终态 |

---

## Feature 目录结构

```
docs/features/FEAT-XXX-Title/
├── spec.md              ⭐ 必须有 — 定义做什么
├── plan.md              🔧 可选 — 复杂特性需要
├── changelog.md         📝 最高频 — 每次改代码都更新！
├── verification.md      ✅ 关闭前必须填
├── context.md           🧠 关闭时提炼知识
└── deliverables.md      📊 产出记录（实验结果、模型指标、关键数据）
```

---

## 操作指南

### 创建 Feature

```bash
uvp f new "标题" [--adr-ref NNNN]
```

自动创建目录和模板文件。创建后**立即填写 spec.md**。

### 关闭 Feature

```bash
uvp f close FEAT-XXX
```

**前提条件**（全部满足才能关闭）:
- [ ] verification.md 已填写
- [ ] 所有验收标准通过
- [ ] 无 critical 级别问题
- [ ] changelog 已更新
- [ ] context.md 已提炼

---

## 文档模板

### spec.md（定义做什么）

```markdown
---
doc_type: feature-spec
title: "FEAT-XXX: 标题"
date: "YYYY-MM-DD"
feat_id: "FEAT-XXX"
status: planned
related_adr: "NNNN"  # 或 null
---

# FEAT-XXX: 标题

## 概述
<!-- 一句话：这个 Feature 做什么 -->

## 验收标准
<!-- 必须可量化、可测试 -->
- [ ] 指标1 ≥ 值（如：Accuracy ≥ 95%）
- [ ] 指标2 < 值（如：Latency P99 < 200ms）

## 接口定义
### Public API
```python
def function(param1: Type1) -> ReturnType:
    """描述"""
```
### CLI（如适用）
```bash
$ command --option value
```

## 行为规格
### 正常流程
1. ...
### 边界情况
- 空输入：应...
- 并发访问：应...
### 错误处理
- 错误A：处理方式...

## 约束
### 性能
- 内存 < X MB, 响应 < X ms
### 安全
- 输入验证、认证授权
### 兼容性
- Python ≥ X.X, 依赖列表
```

### changelog.md（记录变更）

```markdown
---
doc_type: feature-changelog
title: "FEAT-XXX Changelog"
date: "YYYY-MM-DD"
feat_id: "FEAT-XXX"
---

# FEAT-XXX Changelog

## [YYYY-MM-DD]
### Added / Changed / Fixed / Breaking Changes
- 具体改了什么 + 为什么
```

> 详细规范见 `uvp-file-coupling` skill 的 Changelog 章节

### verification.md（验证报告）

```markdown
---
doc_type: feature-verification
title: "FEAT-XXX Verification Report"
date: "YYYY-MM-DD"
feat_id: "FEAT-XXX"
status: passed  # passed | failed | partial
---

# FEAT-XXX Verification Report

## 测试结果
- 单元测试: XX passed / XX failed, 覆盖率 XX%
- 集成测试: XX/XX 通过

## 性能测试
| 指标 | 要求 | 实际 | 状态 |
|------|------|------|------|
| Accuracy | ≥ 95% | 96.2% | ✅ |

## 验收标准检查
- [x] 标准1: 通过（证据: ...）
- [ ] 标准2: 未通过（差距: ...，缓解: ...）

## 发现的问题
### Critical（必须修复才能关闭）
- 无 / 列出

### Minor（可延后）
1. 问题描述 → 建议

## 签收
- 验证人: [名称]
- 日期: YYYY-MM-DD
- 建议: ✅ 批准关闭 / 🔶 有条件批准 / ❌ 未就绪
```

### context.md（知识提炼）

```markdown
---
doc_type: feature-context
title: "FEAT-XXX: Key Learnings"
date: "YYYY-MM-DD"
feat_id: "FEAT-XXX"
status: closed
---

# FEAT-XXX: 提炼上下文

## 摘要（2-3句）

## 关键决策及原因
1. **决策**: 选择方案A
   **原因**: ...
   **权衡**: ...

## 经验教训
### 成功经验 ✅
- ...
### 失败教训 ❌
- ...
### 意外发现 🤔
- ...

## 架构影响
- 修改: `src/path/file.py`
- 新增: `src/path/new_file.py`
- 依赖: `package >= X.Y`

## 已知限制
1. 限制描述 → 影响范围 → 缓解方案

## 快速参考
```python
# 最小可用示例
from xxx import MyClass
obj = MyClass(config)
result = obj.process(data)
```

## 相关文档
- ADR-0001: [标题]
```

### deliverables.md（产出记录）

```markdown
---
doc_type: feature-deliverables
title: "FEAT-XXX Deliverables"
date: "YYYY-MM-DD"
feat_id: "FEAT-XXX"
updated: "YYYY-MM-DD"
---

# FEAT-XXX Deliverables

> 记录结构化产出：实验结果、模型指标、关键数据、产出物路径。
> 周报生成时自动读取此文件。

## 产出记录

### YYYY-MM-DD | 模型评估 | baseline

- **方法**：使用 XX 模型 + YY 数据集
- **结果**：准确率 92.3%，F1 0.91
- **结论**：达到基线要求，可进入下一阶段
- **产出物**：`experiments/baseline/results.json`

### YYYY-MM-DD | 对比实验 | 方案A vs 方案B

| 指标 | Baseline | 方案A | 方案B | 最优 |
|------|----------|-------|-------|------|
| Accuracy | 85% | 93% | 88% | **A** |

- **结论**：✅ ADOPT 方案A
- **下一步**：集成到主代码
```

---

## 常用命令

```bash
uvp f new "标题"              # 创建 Feature
uvp f list                    # 列出所有 Feature
uvp f show FEAT-XXX           # 查看 Feature 详情
uvp f close FEAT-XXX          # 关闭 Feature
uvp check --features          # 检查 Feature 完整性
```

---

*工作流步骤详见 `uvp-workflow` skill*
*文件联动规则详见 `uvp-file-coupling` skill*
