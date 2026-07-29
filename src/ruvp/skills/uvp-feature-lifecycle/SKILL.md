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
| **idea** | 初始概念 | 手动设置 | spec 完成 |
| **planned** | 准备实施 | `uvp f new` 创建 / spec 已写 | 开始编码 |
| **implementing** | 开发中 | 代码变更开始 | 编码完成，测试就绪 |
| **verifying** | 验证中 | 准备运行验证 | 所有检查完成 |
| **verified** | 已验证 | 验证通过 | 准备提炼 |
| **closed** | 已归档 | `uvp f close` / 提炼完成 | 终态 |

---

## Feature 目录结构

```
docs/features/FEAT-XXX-Title/
├── spec.md              ⭐ 必须有 — 合同：做什么 + 验收标准
├── plan.md              🔧 可选 — 施工路线：怎么做（简单特性可省略）
├── changelog.md         📝 最高频 — 每次改代码都更新！
├── verification.md      ✅ 关闭前必须填（复杂任务引用实验结果路径即可）
└── context.md           🧠 AI 压缩上下文 + 竣工经验笔记（Distill 步骤维护）
```

### 三文件边界原则

| 文件 | 何时写 | 回答什么 | 关闭后 |
|------|--------|----------|--------|
| **spec.md** | 动手前 | 做什么 + 验收标准 + 接口契约 + 约束 + 人工决策结论 | 冻结（验收基准） |
| **plan.md** | 动手前/中 | 怎么做（AI 规划的步骤、风险、依赖，决策间接体现） | 用完即弃，可省略 |
| **context.md** | 关闭时（Distill 步骤维护） | 交给 AI 的既定事实上下文（当前状态、要点、上手指南） | 长期存活 |

> **spec 是合同（含决策），plan 是施工路线，context 是既定事实的压缩上下文。**

- spec 收窄为契约：不写大段"背景与约定"，背景只留约束性一两句
- plan 标注为「可选」：简单特性可指向 ADR Actions，不必填满
- 决策类内容单一归属：最终落点在 context.md，避免与 spec 重复

---

## 操作指南

### 创建 Feature

```bash
uvp f new "标题" [--adr NNNN]
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
metrics: {se: 0.975, ppv: 0.942}   # 结构化指标，供 uvp check 机器校验
figures: [images/result.png]         # 证据图像路径
repro_cmd: "uv run python -m eval"   # 一行可复现命令
---

# FEAT-XXX Verification Report

## 验证证据（按适用勾选，算法类至少含 ★）

- [ ] ★ 量化指标表（Se/PPV/准确率/混淆矩阵… 对标基准）
- [ ] ★ 可复现命令（一行能重跑出下列结果）
- [ ] 可视化（结果图，存 images/）— 算法/信号/分类类默认要
- [ ] 失败样例/边界样例（不只报成功）
- [ ] 与基准/既有方案对比

## 量化结果
| 指标 | 要求 | 实际 | 状态 |
|------|------|------|------|
| Accuracy | ≥ 95% | 96.2% | ✅ |

## 可复现命令
```bash
uv run python -m evaluation.eval_xxx
```

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

### 验证证据规则

**图表触发规则**：
- 涉及信号、波形、分类、分布的 Feature → verification **默认要求 ≥1 张图**
- 纯逻辑/IO/配置类 Feature → 可豁免图表，量化指标表即可

**证据路径约定**：
- 图像放 `<feature>/images/` 目录
- 生成脚本一并留存（如 `images/make_plots.py`），确保证据可复现
- 不接受一次性截图——必须有脚本能重新生成

### context.md（既定事实上下文）

```markdown
---
doc_type: feature-context
title: "FEAT-XXX Context"
date: "YYYY-MM-DD"
feat_id: "FEAT-XXX"
status: closed
---

# FEAT-XXX Context

> 交给 AI 的既定事实上下文。AI 读取此文件即可获得最短上手路径。

## 当前状态

closed

## 既定事实

- 选型结论：使用方案A（决策过程见 spec.md 决策记录）
- 架构要点：...
- 生效约束：...

## 要点摘要

- 关键技术点 1
- 易踩坑的地方

## 上手指南
```python
# 最小可用示例
from xxx import MyClass
obj = MyClass(config)
result = obj.process(data)
```

## 相关文档
- ADR-0001: [标题]
```

---

## 实验产出模板（按需使用）

当用户指定将实验结果保存到某路径时（如"将结果记录在 docs/模型构建"），新建文件按此格式填写：

```markdown
---
doc_type: deliverables
title: "标题"
date: "YYYY-MM-DD"
updated: "YYYY-MM-DD"
---

# 标题

> 记录结构化产出：实验结果、模型指标、关键数据、产出物路径。

## 产出记录

### YYYY-MM-DD | 类型 | 标签

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

**触发条件**：用户明确指定保存实验结果的路径（不自动创建）。
**命名约定**：文件名以日期开头，如 `2026-07-29-baseline-evaluation.md`。
**闭环**：在 verification.md 中引用该文件路径即可。

---

## Roadmap 联动指南（Feature 闭环时）

### 🎯 关闭 Feature 后的 Distill 阶段

当 `uvp feature close FEAT-XXX` 执行完成后，AI 需要执行以下步骤：

1. **理解 Feature 语义**：阅读被关闭 Feature 的 `spec.md` 和 `context.md`
2. **匹配 Roadmap 条目**：打开 `docs/roadmap.md`，根据 Feature 的功能语义，判断应该关联到哪个里程碑条目
3. **更新 Roadmap**：将 `[[FEAT-XXX]]` 标签追加到对应的 roadmap 条目末尾
4. **格式示例**：
   ```markdown
   ## 近期目标
   
   - 完成项目初始化功能 [[FEAT-001]] [[FEAT-002]]
   - 补充测试用例 [[FEAT-003]]
   ```

**关键原则**：
- Roadmap 是纯列表形式（无 checkbox），呈现的是里程碑目标
- 一个 Feature 对应的 `[[FEAT-XXX]]` 标签可能关联到多个里程碑条目（如果该 Feature 贡献到多个目标）
- 如果没有匹配的 roadmap 条目，可以跳过或新建一个条目（可选）

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
