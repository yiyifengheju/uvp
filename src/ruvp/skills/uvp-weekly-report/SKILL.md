---
name: "uvp-weekly-report"
description: "周报生成：从项目状态、Feature、Changelog 自动汇总生成结构化周报。用户要求写周报时触发。"
---

# UVP 周报生成 Skill

**触发条件**: 用户要求"写周报"、"生成周报"、"本周总结"、"weekly report"

---

## 核心规则

> 周报文件统一放在 `docs/周报/`，文件名格式 `YYYYMMWNN.md`（NN = 第几周，如 W23）

---

## 文件命名

```
docs/周报/202606W23.md    # 2026年6月第23周
docs/周报/202606W24.md    # 2026年6月第24周
```

---

## 周报大纲

```markdown
---
title: "周报 YYYY-MM-DD ~ YYYY-MM-DD"
date: "YYYY-MM-DD"
doc_type: weekly-report
week: "YYYYMMWNN"
---

# 周报：MM/DD ~ MM/DD

## 1. 进展概览

<!-- 一段话总结本周核心进展，3-5 句 -->

## 2. 关键成果

<!-- 量化、具体的成果列表 -->

- [成果1]: 具体描述 + 指标（如：FEAT-001 验证通过，Accuracy 96.2%）
- [成果2]: ...

## 3. 上周遗留问题闭环

| 上周遗留 | 状态 | 说明 |
|---------|------|------|
| 问题描述 | ✅ 已解决 / 🔶 部分解决 / ❌ 未解决 | 解决方式或阻塞原因 |

## 4. 工程改动摘要

<!-- 从 changelog.md 汇总，按 Feature 分组 -->

### FEAT-XXX: Feature 标题
- **Added**: 新增了什么
- **Changed**: 修改了什么
- **Fixed**: 修复了什么
- **Breaking**: 破坏性变更（如有）

### FEAT-YYY: ...
- ...

## 5. 风险与遗留

| 风险/遗留 | 影响等级 | 缓解方案 | 预计解决时间 |
|----------|---------|---------|------------|
| 描述 | 🔴高 / 🟡中 / 🟢低 | 方案 | 日期 |

## 6. 下周计划

| 任务 | 优先级 | 关联 Feature | 备注 |
|------|--------|-------------|------|
| 具体任务描述 | P0 / P1 / P2 | FEAT-XXX | 补充说明 |

## 7. 相关产物快速索引

| 产物 | 路径 | 说明 |
|------|------|------|
| ADR-0001 | `docs/adr/0001-*.md` | 决策标题 |
| FEAT-001 | `docs/features/FEAT-001-*/` | Feature 标题 |
| Deliverables | `docs/features/FEAT-001-*/deliverables.md` | 产出标题 |
| 关键文档 | `docs/xxx.md` | 文档说明 |
```

---

## 信息采集流程

生成周报时，按以下顺序采集信息：

1. **读取 `docs/_meta/feature-registry.yaml`** → 获取所有 Feature 状态
2. **读取各 Feature 的 `changelog.md`** → 汇总本周工程改动
3. **读取各 Feature 的 `verification.md`** → 提取关键成果指标
4. **读取各 Feature 的 `deliverables.md`** → 提取产出记录（实验结果、模型指标、关键数据）
5. **读取上周周报**（如存在）→ 提取遗留问题，检查闭环
6. **读取 `docs/PROJECT_STATE.md`** → 获取项目整体状态
7. **读取 `docs/adr/` 下本周新增 ADR** → 补充决策记录

---

## 执行步骤

1. 确定周报时间范围（周一至周日）
2. 采集上述信息源
3. 按大纲模板生成周报
4. 写入 `docs/周报/YYYYMMWNN.md`
5. 确保包含 meta 头（title + date + doc_type: weekly-report）

---

## 快速参考

```bash
# 查看当前 Feature 状态（辅助写周报）
uvp f list

# 检查文档一致性
uvp check

# 查看项目状态
uvp status
```

---

*工作流详见 `uvp-workflow` skill | 文件联动详见 `uvp-file-coupling` skill*
