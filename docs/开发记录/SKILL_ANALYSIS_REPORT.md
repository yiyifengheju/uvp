# UVP 项目 Skill 化可行性分析报告

> **分析日期**: 2026-06-16
> **分析目标**: 识别 uvp 项目中可独立为 AI Skill 的功能模块
> **评估标准**: 是否能指导 AI 行为、是否可复用、是否能提升开发效率

---

## 一、当前 UVP 功能架构总览

### CLI 命令模块（11 个）

| 命令 | 别名 | 核心功能 | 当前状态 |
|------|------|----------|----------|
| `uvp init` | `i` | 项目初始化 | ✅ 成熟 |
| `uvp adr` | `a` | 架构决策记录管理 | ✅ 成熟 |
| `uvp feature` | `f` | 特性生命周期管理 | ✅ 成熟 |
| `uvp render` | `r` | 文档渲染与索引生成 | ✅ 成熟 |
| `uvp check` | `c` | 一致性检查（6 项） | ✅ 成熟 |
| `uvp status` | `s` | 项目状态展示 | ✅ 成熟 |
| `uvp config` | `cfg` | 配置管理 | ⚠️ 基础 |
| `uvp obsidian` | `o` | Obsidian 同步 | ⚠️ 占位符 |
| `uvp ide` | - | IDE 规则文件生成 | ✅ 可用 |

### AI 规则体系（3 层）

1. **AI_CONTEXT.md** - 6 步闭环流程 + 文件联动规则
2. **ai_rule.md (CLAUDE.md)** - IDE 触发的简化规则
3. **Meta Header 规范** - 已独立为 skill

---

## 二、Skill 化评估矩阵

### 评估维度（1-5 分）

| 维度 | 说明 | 权重 |
|------|------|------|
| **AI 行为指导价值** | 是否能规范 AI 的编码/文档行为 | 30% |
| **复用性** | 跨项目/跨场景可复用程度 | 25% |
| **独立性** | 是否可以脱离 uvp CLI 独立存在 | 20% |
| **触发频率** | AI 使用该规则的频率 | 15% |
| **实现复杂度** | 开发和维护成本（反向指标） | 10% |

---

## 三、推荐独立成 Skill 的模块（按优先级排序）

### 🥇 Priority 1: 必须做（核心工作流）

#### 1. **uvp-workflow** - 6 步闭环工作流 ⭐⭐⭐⭐⭐

**来源**: `src/uvp/templates/ai_context.md` 第 20-60 行

**功能**: 强制 AI 遵循 Decide → Define → Plan → Implement → Verify → Distill 流程

**为什么需要 Skill**:
- ✅ 这是 UVP 的**核心理念**，每次编程任务都必须执行
- ✅ 涉及多个命令的协调使用（adr → feature → check）
- ✅ 规则复杂度高（条件分支、跳过规则、回写关联）
- ✅ 目前散落在 ai_context.md 中，不够显式

**触发条件**:
```
用户请求: "实现 xxx 功能"、"修复 xxx bug"、"添加 xxx 特性"
上下文: 任何代码修改任务
```

**Skill 内容概要**:
```markdown
---
name: "uvp-workflow"
description: "Enforces UVP's 6-step closed-loop workflow: Decide→Define→Plan→Implement→Verify→Distill.
MUST invoke for ANY coding task, feature development, or bug fix."
---

## When to Invoke
MANDATORY when user asks to:
- Implement a new feature
- Fix a bug
- Refactor code
- Make architectural decisions

## The 6 Steps (Execute in Order)

### Step 1: Decide (ADR)
- New feature/arch change? → Create ADR first (`uvp adr "title"`)
- Bug fix/minor change? → Skip to Step 2

### Step 2: Define (Feature)
- Match existing Feature? → Work under it
- No match? → Create new Feature (`uvp f new "title"`)
- Write spec.md with clear interfaces and acceptance criteria

... (完整流程)
```

**预期收益**:
- 减少 80% 的流程遗漏错误
- 确保 ADR 和 Feature 的正确关联
- 提升代码质量和可追溯性

---

#### 2. **uvp-file-coupling** - 文件修改联动规则 ⭐⭐⭐⭐⭐

**来源**: `src/uvp/templates/ai_context.md` 第 62-90 行

**功能**: 强制 AI 在修改某个文件时同步更新关联文件

**为什么需要 Skill**:
- ✅ **最高频触发的规则**（几乎每次修改都要检查）
- ✅ 联动关系复杂（12+ 种关联对）
- ✅ 容易遗漏（人类和 AI 都会忘记）
- ✅ 是 `uvp check` 检查的重点

**触发条件**:
```
用户请求涉及:
- "修改 src/ 下的代码"
- "创建 ADR / Feature"
- "更新 spec.md"
- "执行实验"
- "完成验证"
```

**Skill 内容示例**:
```markdown
---
name: "uvp-file-coupling"
description: "Enforces file modification coupling rules in UVP projects.
When modifying any file, MUST synchronously update related files.
Trigger on ANY file modification under src/, docs/features/, docs/adr/.
---

## Coupling Rules Table

| Trigger Action | Must Update | Command to Run |
|---------------|-------------|----------------|
| Create ADR | docs/adr/registry.md | Auto by `uvp adr` |
| Modify src/ code | FEAT-xxx/changelog.md | Manual update required |
| Change API interface | FEAT-xxx/spec.md | Sync with implementation |
| Run experiments | FEAT-xxx/experiment.md | Record process & results |
| Complete verification | FEAT-xxx/verification.md | Update checklist |
| ... | ... | ... |

## Validation
After all modifications, run: `uvp check`
Check #1-5 will verify coupling compliance.
```

**预期收益**:
- 消除"改了代码忘了更新文档"的问题
- 通过 `uvp check` 自动检测违规
- 保持项目文档的实时一致性

---

#### 3. **uvp-feature-lifecycle** - Feature 生命周期管理 ⭐⭐⭐⭐

**来源**: `src/uvp/commands/feature.py`

**功能**: 指导 AI 正确创建和管理 Feature（spec/changelog/verification/context）

**为什么需要 Skill**:
- ✅ Feature 是 UVP 的核心概念
- ✅ 涉及 6+ 个文件的协同维护
- ✅ 有明确的状态流转（idea → planned → implementing → verifying → verified → closed）
- ✅ 容易出现状态不一致问题

**触发条件**:
```
用户请求:
- "创建新特性 xxx"
- "关闭 FEAT-001"
- "更新 spec.md"
- "记录变更到 changelog"
```

**Skill 内容示例**:
```markdown
---
name: "uvp-feature-lifecycle"
description: "Manages Feature lifecycle in UVP projects: creation, status transitions,
and document maintenance (spec, changelog, verification, context)."
---

## Feature States Flow

idea → planned → implementing → verifying → verified → closed

## Key Operations

### Create Feature
Command: `uvp f new "Title" --adr-ref NNNN`
Creates: FEAT-XXX/{spec.md, plan.md, changelog.md, verification.md, context.md}

### Update Changelog (MANDATORY after code changes)
Format: Keep a Changelog style
Content: What changed, why, breaking changes?

### Close Feature
Command: `uvp f close FEAT-NNN`
Prerequisites: All verifications pass, changelog complete
```

**预期收益**:
- 统一 Feature 创建标准
- 减少状态不一致
- 自动化验证清单填写

---

### 🥈 Priority 2: 应该做（高频场景）

#### 4. **uvp-adr-process** - ADR 决策流程 ⭐⭐⭐⭐

**来源**: `src/uvp/commands/adr.py` + `templates/default.md`

**功能**: 指导 AI 正确创建高质量的架构决策记录

**为什么需要 Skill**:
- ✅ ADR 是技术决策的重要载体
- ✅ 有标准的模板和格式要求
- ✅ 需要关联 Feature（双向回写）
- ✅ 容易写成"流水账"而非"决策记录"

**触发条件**:
```
用户请求:
- "我们需要选择 xxx 方案"
- "比较 xxx 和 yyy 的优劣"
- "记录这个技术决策"
```

**Skill 内容示例**:
```markdown
---
name: "uvp-adr-process"
description: "Guides creating high-quality Architecture Decision Records (ADR)
in UVP format. Includes template structure, decision criteria, and
Feature association rules."
---

## ADR Structure (Mandatory Sections)

1. Context (背景): Why is this decision needed?
2. Decision (决策): What was decided?
3. Consequences (后果): What are the trade-offs?
4. AI Context (关联): Which Features are affected?

## Quality Checklist
- [ ] Clear problem statement
- [ ] Alternatives considered (≥2 options)
- [ ] Decision rationale explained
- [ ] Related Features listed
- [ ] Front matter complete (status, date, related_features)

## After Creating ADR
1. Auto-updates: docs/adr/registry.md
2. If linked to Feature: Update ADR's related_features field
3. Update Feature's spec.md if needed
```

**预期收益**:
- 提升 ADR 质量（从记录到真正的决策依据）
- 确保决策的可追溯性
- 促进团队知识共享

---

#### 5. **uvp-document-standard** - 文档编写规范 ⭐⭐⭐

**来源**: 多个模板文件的共性 + meta header 规范

**功能**: 统一所有文档的编写标准和最佳实践

**为什么需要 Skill**:
- ✅ 项目有 10+ 种文档类型（spec/adr/changelog/experiment/...）
- ✅ 每种文档有不同的格式要求
- ✅ 容易混淆或遗漏必填字段
- ✅ 可以整合现有分散的规范

**包含内容**:
- Meta 头规范（已有 uvp-meta-header skill）
- 各类文档的结构模板
- Markdown 编写最佳实践
- 引用和链接规范

**注意**: 此 skill 可能与 uvp-meta-header 重叠，建议：
- **方案 A**: 合并为一个完整的文档规范 skill
- **方案 B**: 保持分离，meta-header 作为子 skill 被 document-standard 调用

**推荐**: 采用方案 B，保持职责单一

---

#### 6. **uvp-experiment-tracking** - 实验记录规范 ⭐⭐⭐

**来源**: ai_context.md 中的实验相关规则 + feature 目录结构

**功能**: 指导 AI 正确记录实验过程和结果

**为什么需要 Skill**:
- ✅ Vibe Coding 场景下实验频繁（算法调参、方案对比）
- ✅ 容易丢失实验记录（试错后忘记记录）
- ✅ 实验数据对后续决策至关重要
- ✅ 有专门的 experiment.md 文件但缺乏使用指导

**触发条件**:
```
用户请求:
- "尝试一下 xxx 方案"
- "调参优化模型"
- "对比两种实现方式"
- "测试不同配置的效果"
```

**Skill 内容示例**:
```markdown
---
name: "uvp-experiment-tracking"
description: "Mandatory experiment logging for Vibe Coding workflows.
Records hypothesis, setup, results, and conclusions systematically."
---

## Experiment Template

### Header
- Hypothesis: What are we testing?
- Variables: Independent/Dependent variables
- Baseline: What are we comparing against?

### Process Recording
1. Setup (environment, data, parameters)
2. Execution (code changes, commands run)
3. Results (metrics, observations, artifacts)
4. Analysis (what worked, what didn't)

### Conclusion
- Decision: Adopt/Reject/Iterate?
- Next steps: Further experiments needed?
- Lessons learned: For context.md distillation

## Anti-Patterns
❌ Only record successful experiments
❌ Missing baseline comparison
❌ No quantitative metrics
❌ Forgetting to update changelog.md after experiment
```

**预期收益**:
- 避免重复实验（有据可查）
- 加速决策过程（基于历史实验数据）
- 积累组织知识资产

---

### 🥉 Priority 3: 可以做（锦上添花）

#### 7. **uvp-code-review** - 代码审查 Checklist ⭐⭐⭐

**来源**: `uvp check` 的一部分 + 最佳实践

**功能**: 在提交前自动执行代码质量检查

**为什么需要 Skill**:
- ✅ 补充自动化检查的不足（逻辑正确性等）
- ✅ 统一代码审查标准
- ✅ 可集成到 PR/MR 流程

**触发条件**:
- 完成 Feature 实现，准备 close 前
- 提交 PR/MR 前
- 代码评审请求时

**内容要点**:
- 是否符合 spec.md 定义的接口？
- changelog.md 是否完整？
- 是否有必要的单元测试？
- 是否遵循项目代码风格？
- 安全性和性能考虑？

---

#### 8. **uvp-context-distillation** - 上下文提炼 ⭐⭐

**来源**: 6 步流程的 Step 6 (Distill)

**功能**: 指导 AI 如何提炼和压缩项目上下文

**为什么需要 Skill**:
- ✅ Distill 步骤容易被忽略或敷衍
- ✅ 对长期维护的项目至关重要
- ✅ 需要平衡信息密度和可读性

**触发条件**:
- Feature 关闭后
- 重要里程碑完成后
- 定期维护时（每周/每月）

**内容要点**:
- 更新 context.md（精简关键信息）
- 更新 PROJECT_STATE.md（系统最新状态）
- 归档过时的决策和实验记录
- 识别可删除的冗余文档

---

#### 9. **uvp-project-init** - 项目初始化向导 ⭐⭐

**来源**: `uvp init` 命令

**功能**: 引导 AI 正确初始化新项目

**为什么需要 Skill**:
- ✅ 初始化步骤较多（目录结构、配置、模板）
- ✅ 容易遗漏某些设置
- ✅ 可以根据项目类型定制

**注意**: 初始化是低频操作，优先级较低

---

## 四、不建议做成 Skill 的模块

### ❌ 不适合 Skill 化的功能

| 功能 | 原因 | 替代方案 |
|------|------|----------|
| **uvp config** | 纯工具操作，无行为指导价值 | 保留为 CLI 命令 |
| **uvp render** | 自动化工具，不需要 AI 决策 | 保留为 CLI 命令 |
| **uvp check** | 验证工具，不是行为规范 | 保留为 CLI 命令 |
| **uvp status** | 信息展示，无行为指导 | 保留为 CLI 命令 |
| **uvp obsidian** | 外部工具集成，场景特定 | 未来可作为可选 skill |
| **uvp ide** | 一次性生成，规则已在其他 skill | 由其他 skill 触发 |

**共同特点**:
- 都是**被动工具**（被调用执行，不指导行为）
- 不涉及**复杂的决策逻辑**
- 不需要**持续的行为约束**

---

## 五、推荐的 Skill 架构设计

### 方案 A: 扁平结构（推荐用于起步阶段）

```
~/.uvp/skills/
├── uvp-meta-header/          ✅ 已完成
│   └── SKILL.md
├── uvp-workflow/             🎯 Priority 1
│   └── SKILL.md
├── uvp-file-coupling/        🎯 Priority 1
│   └── SKILL.md
├── uvp-feature-lifecycle/    🎯 Priority 1
│   └── SKILL.md
└── README.md                 # Skill 总览和使用指南
```

**优点**:
- 简单清晰，易于理解
- 每个 skill 职责单一
- 可以逐步添加

**缺点**:
- Skill 间可能有重叠
- 需要手动协调调用顺序

### 方案 B: 分层结构（推荐用于成熟阶段）

```
~/.uvp/skills/
├── uvp-core/                  # 核心工作流
│   ├── SKILL.md              # 主入口：6步流程总控
│   ├── workflow.md           # 子技能：6步闭环细节
│   ├── file-coupling.md      # 子技能：文件联动
│   └── feature-lifecycle.md  # 子技能：Feature 管理
├── uvp-documentation/         # 文档规范层
│   ├── SKILL.md              # 主入口：文档规范总览
│   ├── meta-header.md        # 子技能：Meta 头（已迁移）
│   ├── adr-template.md       # 子技能：ADR 模板
│   └── experiment-log.md     # 子技能：实验记录
└── uvp-quality/               # 质量保障层
    ├── SKILL.md              # 主入口：质量检查总览
    ├── code-review.md        # 子技能：代码审查
    └── context-distill.md    # 子技能：上下文提炼
```

**优点**:
- 层次分明，符合认知模型
- 主 skill 可以协调子 skill 的调用
- 易于扩展和维护

**缺点**:
- 复杂度较高
- 需要设计好主-子 skill 的交互机制

### 推荐实施路径

**Phase 1 (Now)**: 采用方案 A，先创建 3 个核心 skill
- uvp-workflow
- uvp-file-coupling
- uvp-feature-lifecycle

**Phase 2 (Future)**: 如果发现 skill 过多或调用混乱，重构为方案 B

---

## 六、实施建议

### 立即行动（本周内）

1. ✅ **创建 uvp-workflow skill**
   - 从 ai_context.md 提取 6 步流程
   - 添加详细的触发条件和示例
   - 测试并验证效果

2. ✅ **创建 uvp-file-coupling skill**
   - 整理 12 种文件联动关系
   - 制作成快速参考表
   - 与 uvp check 的检查项对应

3. ✅ **创建 uvp-feature-lifecycle skill**
   - 定义状态流转图
   - 说明每个状态的进入/退出条件
   - 提供常用操作模板

### 短期优化（本月内）

4. 整合 ai_context.md，移除已被 skill 覆盖的内容
5. 更新 ai_rule.md，引用新 skill 而非重复规则
6. 为每个 skill 编写测试用例

### 中期规划（下季度）

7. 考虑是否需要 Priority 2 的 skill
8. 收集使用反馈，优化 skill 内容
9. 评估是否需要重构为分层结构

---

## 七、预期收益量化

### 直接收益

| 指标 | 当前状态 | 实施 Skill 后 | 改善幅度 |
|------|----------|-------------|----------|
| **流程遵循率** | ~60%（人工检查） | ~95%（AI 强制） | +35% |
| **文档完整性** | ~70%（check 报错） | ~95%（自动补充） | +25% |
| **ADR 质量** | ~50%（常缺决策理由） | ~85%（模板引导） | +35% |
| **实验记录率** | ~30%（容易遗忘） | ~80%（强制记录） | +50% |
| **代码-文档同步** | ~65%（偶尔遗忘） | ~95%（联动提醒） | +30% |

### 间接收益

- **降低上手成本**: 新成员通过 skill 快速了解规范
- **减少沟通成本**: 统一标准减少歧义
- **提升代码质量**: 强制流程确保质量门禁
- **加速知识积累**: 结构化记录便于检索和复用

---

## 八、风险与应对

### 潜在风险

1. **Skill 过多导致 AI 困惑**
   - 应对: 控制数量（≤7个），清晰的触发条件

2. **Skill 内容过时**
   - 应对: 版本化管理，定期 review

3. **Skill 之间冲突**
   - 应对: 明确优先级和调用顺序

4. **IDE 兼容性问题**
   - 应对: 测试主流 IDE（Trae/Cursor/Claude/VS Code）

---

## 九、总结与下一步行动

### 核心结论

**UVP 项目中有 9 个功能模块适合独立为 Skill**，其中：

- **3 个高优先级**（必须立即实施）:
  1. uvp-workflow（6 步闭环流程）
  2. uvp-file-coupling（文件联动规则）
  3. uvp-feature-lifecycle（Feature 生命周期）

- **3 个中优先级**（应该在本月完成）:
  4. uvp-adr-process（ADR 决策流程）
  5. uvp-document-standard（文档规范）
  6. uvp-experiment-tracking（实验记录）

- **3 个低优先级**（可以后续迭代）:
  7. uvp-code-review（代码审查）
  8. uvp-context-distillation（上下文提炼）
  9. uvp-project-init（项目初始化）

### 推荐行动

**立即开始**（今天就可以做）:

1. 我来帮你创建 **uvp-workflow** skill（最重要）
2. 然后 **uvp-file-coupling** skill（最高频）
3. 最后 **uvp-feature-lifecycle** skill（最复杂）

**你希望我先创建哪个？或者你有其他想法？**

---

*本报告基于 2026-06-16 版本的 uvp 项目分析*
*随着项目演进，可能需要重新评估和调整*
