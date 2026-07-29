---
doc_type: reference
title: "uvp 改进建议 · 验证证据"
date: "2026-07-28"
---

# uvp 改进建议 · 验证证据

> 来源：FEAT-016（L1 波形描绘）验证阶段的实践。核心问题——验证证据是否充足，取决于用户是否临场要求（本次是用户要图才补图）；应让「充足证据」成为 skill 的默认档位，而非靠临场提醒。与 `reference/uvp改进建议.md`（Bug/模板边界/流程）互补，此篇专注**验证证据**。
> 仅为改进备忘，不代表已实施。

## 一、skill 层：把证据从自由发挥变成默认清单

### 1. verification.md 模板加「证据类型」清单
现模板只说「记录验证结果」，太空。改为按 Feature 类型给默认要求：

```markdown
## 验证证据（按适用勾选，算法类至少含 ★）
- [ ] ★ 量化指标表（Se/PPV/准确率/混淆矩阵… 对标基准）
- [ ] ★ 可复现命令（一行能重跑出下列结果）
- [ ] 可视化（结果图，存 images/，嵌入本文件）— 算法/信号/分类类默认要
- [ ] 失败样例/边界样例（不只报成功）
- [ ] 与基准/既有方案对比
```

### 2. 明确「图表 vs 数字」的触发规则
涉及信号、波形、分类、分布的 Feature，verification 默认要 ≥1 张图；纯逻辑/IO 类可豁免。写进 lifecycle 或 file-coupling skill，不靠 AI 自觉。

### 3. 约定图像放 `<feature>/images/` + 保留生成脚本
证据要可复现，不是一次性截图。skill 点明约定：图像路径 `<feature>/images/`，生成脚本一并留存（如 `images/make_plots.py`）。

### 4. 措辞从「记录结果」改为带清单的「逐项提供证据」
AI 对清单的遵从度远高于对开放式指令。把 verify 阶段设成默认产出「量化表 + 图 + 可复现命令」，用户不必每次提醒。

## 二、工具层：让证据可被机器校验

### 5. `uvp f close` 前置校验证据完整性
（呼应 `reference/uvp改进建议.md` 第三节的 `uvp check --features`）
- verification.md 仍是模板 / 空「验证结果」→ 拒绝关闭。
- 算法类 Feature（由 spec 验收标准含「精度/准确率/Se」等关键词判定）若 verification 无量化表 → 告警。

### 6. verification front matter 加结构化字段
让证据可被工具检查，而非靠读散文：

```yaml
metrics: {qrs_se: 0.975, qrs_ppv: 0.942, beat_acc: 0.814}
figures: [images/delineation_208.png, images/beat_confusion.png]
repro_cmd: "uv run python -m evaluation.eval_delineation"
```

`uvp check` 据此机器校验「有无证据」。可选提供 `uvp f evidence FEAT-XXX` 汇总展示。

## 根本改进

现状：证据充分与否取决于用户是否临场要求。目标：skill 把「充足证据」设为**默认档位**，AI 在 verify 阶段自动产出量化表 + 图 + 可复现命令。skill 用词的一字之差（「记录验证结果」→ 带清单的「逐项提供下列证据」）带来显著的行为差别。
