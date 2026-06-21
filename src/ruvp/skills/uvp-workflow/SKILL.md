---
name: "uvp-workflow"
description: "6步闭环工作流：Decide→Define→Plan→Implement→Verify→Distill。任何编程任务必须遵循。"
---

# UVP 6-Step Workflow

**触发条件**: 任何编程任务（新功能、Bug修复、重构）

**不适用**: 纯文档创建（见 uvp-meta-header）、纯查询操作

---

## 流程总览

```
Step 1: DECIDE ──→ Step 2: DEFINE ──→ Step 3: PLAN ──→ Step 4: IMPLEMENT ──→ Step 5: VERIFY ──→ Step 6: DISTILL
(创建ADR)          (创建Feature)       (可选)              (编码+日志)         (测试验证)          (提炼知识)
```

---

## Step 1: DECIDE — 是否需要架构决策？

**需要 ADR 的情况**: 新功能、架构变更、技术选型、重大重构
**可跳过 ADR**: 简单 Bug 修复、小改动、纯文档

```bash
uvp a "决策标题"          # 如：选择随机森林算法进行分类
```

ADR 标题要求：描述**做什么**和**为什么**，避免模糊标题。

跳过时在 changelog 记录：`Skip ADR: trivial fix`

---

## Step 2: DEFINE — 创建/匹配 Feature

```bash
uvp f list               # 检查已有 Feature
uvp f new "标题" --adr-ref NNNN   # 创建新 Feature
```

| 情况 | 操作 |
|------|------|
| 匹配到已有 Feature | 在该 Feature 下工作 |
| 无匹配 | 创建新 Feature |
| 跨多个 Feature | 创建父 Feature 或关联多个 |

**创建 Feature 后必须**:
1. 填写 `spec.md`（详见 `uvp-feature-lifecycle` skill）
2. 回写 ADR 关联：更新 ADR 的 `related_features` 字段和正文"🎯 AI 上下文"

---

## Step 3: PLAN — 实施计划（可选）

**需要 plan.md**: 复杂特性（>3天）、多人协作、高风险变更
**可跳过**: 简单特性（<1天）、单人任务、低风险

---

## Step 4: IMPLEMENT — 编码 + 即时记录

### 核心规则：每完成一个逻辑单元，立即更新 changelog.md

```markdown
## [YYYY-MM-DD]
### Added / Changed / Fixed / Breaking Changes
- 具体改了什么 + 为什么（不是代码diff）
```

### 同步更新规则

| 如果你... | 还要更新... |
|-----------|------------|
| 修改了 API 接口 | `spec.md` 接口定义 |
| 运行了实验/试错 | `deliverables.md`（详见 uvp-file-coupling） |
| 发现新的约束 | `spec.md` 约束部分 |

---

## Step 5: VERIFY — 测试验证

1. 运行测试 `pytest tests/`
2. 逐项检查 `spec.md` 验收标准
3. 填写 `verification.md`（详见 `uvp-feature-lifecycle` skill）
4. **任何标准未通过** → 回到 Step 4 修复，不要关闭 Feature

```bash
uvp f close FEAT-XXX     # 全部通过后才执行
```

---

## Step 6: DISTILL — 知识提炼

1. 更新 `context.md`（关键决策、经验教训、架构影响）
2. 更新 `docs/PROJECT_STATE.md`（系统最新状态）
3. 仅当项目事实变化时更新 `docs/AI_CONTEXT.md`

模板详见 `uvp-feature-lifecycle` skill。

---

## 反模式

| 错误做法 | 正确做法 |
|---------|---------|
| "简单，直接写代码" | 至少执行 Step 2 (Define) |
| "文档稍后补" | changelog 立即更新 |
| "测试等会再说" | 验证通过才能关闭 |
| "Make it fast" | "Response time < 100ms (P99)" |
| "Fixed bugs" | "Fixed null pointer in Parser.parse_line() when input empty (issue #42)" |

---

## 与其他 Skill 的关系

| 步骤 | 调用的 Skill | 原因 |
|------|-------------|------|
| Step 2 | uvp-feature-lifecycle | Feature 创建和 spec 模板 |
| Step 4 | uvp-file-coupling | 文件联动规则 |
| Step 4 | uvp-meta-header | 创建文档时添加 meta 头 |
| Step 5-6 | uvp-feature-lifecycle | verification/context 模板 |

---

*详细模板（spec/changelog/verification/context/deliverables）见 `uvp-feature-lifecycle` skill*
*文件联动规则详见 `uvp-file-coupling` skill*
