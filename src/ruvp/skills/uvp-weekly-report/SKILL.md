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
docs/周报/202607W30.md    # 2026年第30周
docs/周报/202607W31.md    # 2026年第31周
```

---

## 时间范围确定

用户可在命令中指定起止日期（如"写7/24到7/30的周报"），AI 按指定范围采集信息：

1. 用户指定起止日期时，直接使用
2. 用户未指定时，默认为**上次周报结束日的次日 ~ 今天**
3. 如无上次周报，默认最近 7 天

> 提示：文件名仍按自然周编号，但内容覆盖的时间范围以用户指定为准。

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

<!-- 一段话总结核心进展，3-5 句 -->

## 2. 关键成果

<!-- 量化、具体的成果列表。指标必须完整摘录（不省略、不四舍五入） -->

- [成果1]: 具体描述 + 完整指标
  - 示例：FEAT-016 验证通过，QRS Se=97.5%, PPV=94.2%, Beat Acc=81.4%
- [成果2]: ...

> ⚠️ 摘录指标时必须从 verification.md 的 metrics 字段或量化结果表**完整复制**，不要只摘部分。

## 3. 上期遗留问题闭环

| 上期遗留 | 状态 | 说明 |
|---------|------|------|
| 问题描述 | ✅ 已解决 / 🔶 部分解决 / ❌ 未解决 | 解决方式或阻塞原因 |

## 4. 工程改动摘要

<!-- 从 changelog.md 汇总，仅包含时间范围内的条目，按 Feature 分组 -->

### FEAT-XXX: Feature 标题
- **Added**: 新增了什么
- **Changed**: 修改了什么
- **Fixed**: 修复了什么

### FEAT-YYY: ...
- ...

## 5. 风险与遗留

| 风险/遗留 | 影响等级 | 缓解方案 | 预计解决时间 |
|----------|---------|---------|------------|
| 描述 | 🔴高 / 🟡中 / 🟢低 | 方案 | 日期 |

## 6. 下期计划

| 任务 | 优先级 | 关联 Feature | 备注 |
|------|--------|-------------|------|
| 具体任务描述 | P0 / P1 / P2 | FEAT-XXX | 补充说明 |

## 7. 相关产物快速索引

| 产物 | 路径 | 说明 |
|------|------|------|
| ADR-NNNN | `docs/adr/NNNN-*.md` | 决策标题 |
| FEAT-XXX | `docs/features/FEAT-XXX-*/` | Feature 标题 |
| 实验结果 | `docs/xxx/YYYY-MM-DD-*.md` | 产出说明 |
```

---

## 信息采集流程

生成周报时，按以下顺序采集信息：

1. **确定时间范围**：用户指定 > 上次周报推算 > 默认最近 7 天
2. **读取 `docs/_meta/feature-registry.yaml`** → 获取所有 Feature 状态
3. **读取各 Feature 的 `changelog.md`** → 筛选时间范围内的条目汇总
4. **读取各 Feature 的 `verification.md`** → 完整提取 metrics 字段和量化结果表
5. **读取上期周报**（如存在）→ 提取遗留问题，检查闭环
6. **读取 `docs/PROJECT_STATE.md`** → 获取项目整体状态
7. **读取 `docs/adr/` 下时间范围内新增 ADR** → 补充决策记录

### 指标摘录规则

- 从 verification.md front matter 的 `metrics` 字段完整复制
- 如 metrics 为空，从正文「量化结果」表格完整复制
- **禁止**：只摘部分指标、四舍五入、用"等"省略

---

## 执行步骤

1. 确定周报时间范围（见上述规则）
2. 采集上述信息源（按时间范围过滤）
3. 按大纲模板生成周报
4. 写入 `docs/周报/YYYYMMWNN.md`
5. 确保包含 meta 头（title + date + doc_type + week）

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
