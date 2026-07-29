# AI_CONTEXT

> 本文件是 AI 编程工具的强制规则文件。AI 在执行任务时必须遵守以下规则。

---

## 一、6 步闭环流程（强制）

每次 AI 编程任务必须按以下顺序执行：

### Step 1: Decide
- 如果任务涉及新功能、架构变更、技术选型 → 必须先创建 ADR
  - 命令：`vup adr "标题"`
  - 填写"我要做什么"、"为什么这样做"、"做成什么样"
- 如果是 Bug 修复或小改动 → 可跳过 ADR，直接进入 Step 2

### Step 2: Define
- 创建或更新 FEAT-xxx/spec.md
  - 新特性：`vup feature new "标题" [--adr NNNN]`
  - 已有特性：确认对应的 FEAT 编号
- spec.md 必须写清楚：接口、行为、约束
- 在 ADR 中添加关联的 FEAT 编号

### Step 3: Plan
- 更新 FEAT-xxx/plan.md，列出实施步骤
- 简单特性可跳过此步

### Step 4: Implement
- 修改 src/ 和 tests/
- **每完成一个逻辑单元，立即更新 changelog.md**

### Step 5: Verify
- 运行测试，确认实现符合 spec
- 更新 FEAT-xxx/verification.md
- 验证通过后执行：`vup feature close FEAT-NNN`

### Step 6: Distill
- 更新 FEAT-xxx/context.md（提炼压缩上下文）
- 更新 docs/PROJECT_STATE.md（系统最新状态）
- 更新 docs/AI_CONTEXT.md（如果项目事实有变化）

---

## 二、文件修改联动规则（强制）

AI 在执行任务时，修改某些文件必须同步修改关联文件：

| 触发动作 | 必须同步修改 | 说明 |
|----------|-------------|------|
| 创建 ADR | docs/_meta/adr-registry.yaml（源） → docs/adr/index.md（渲染） | 注册新 ADR |
| 创建 Feature | feature-registry.yaml, docs/features/index.md | 注册新特性 |
| 修改 src/ 代码 | FEAT-xxx/changelog.md | 记录变更 |
| 修改 src/ 代码 | FEAT-xxx/spec.md（如接口变化） | 保持 spec 与代码一致 |
| 完成验证 | FEAT-xxx/verification.md | 记录验证结果 |
| Feature 状态变更 | feature-registry.yaml, docs/features/index.md | 同步状态 |
| 任何文档变更 | docs/AI_CONTEXT.md（如事实变化） | 保持上下文最新 |

**禁止事项：**
- ❌ 修改代码后不更新 changelog.md
- ❌ 修改接口后不更新 spec.md
- ❌ 创建 ADR/Feature 后不更新 registry
- ❌ 把 superseded/deprecated 的 ADR 当成当前事实

---

## 三、项目事实

### 你应该优先阅读
1. docs/PROJECT_STATE.md
2. docs/architecture/current.md
3. docs/features/*/spec.md
4. docs/_meta/feature-registry.yaml

### 你不应该默认阅读
- docs/adr/ 下的全部历史 ADR
- reference/ 下的论文和外部资料
- 已标记为 Superseded / Deprecated 的文档

### 当前项目状态
- 项目名称：ai-uvp
- 技术栈：Rust (edition 2024) + clap 4 + serde + MkDocs Material
- 当前阶段：Beta
- 优先目标：稳定性打磨、Bug 修复、文档完善
- 禁止事项：
  - 不新增未经确认的第三方服务
  - 不修改 feature spec 而直接改代码


### 活跃特性列表
<!-- 此列表由 uvp feature new/close 自动维护，不要手动编辑 -->
- FEAT-001: Feature Ledger 系统 (implemented)
- FEAT-002: 项目初始化 (uvp init) (implemented)
- FEAT-003: ADR 管理 (uvp adr) (implemented)
- FEAT-004: 特性生命周期管理 (uvp feature) (implemented)
- FEAT-005: Obsidian 同步 (uvp obsidian) (implemented)
- FEAT-006: 状态展示 (uvp status) (implemented)
- FEAT-007: 配置管理 (uvp config) (implemented)
- FEAT-008: 文件头 Meta 管理 (implemented)
- FEAT-009: 文档一致性检查 (uvp check) (implemented)
- FEAT-010: 项目打包与分发 (implemented)
- FEAT-011: Mkdocs 页面渲染 (uvp render) (implemented)
- FEAT-012: 终端显示设计 (uvp UI) (implemented)
- FEAT-013: IDE 规则生成与 Skill 部署 (uvp ide) (implemented)
- FEAT-014: TODO 管理 (uvp todo) (implemented)
- FEAT-015: 6步闭环工作流 (uvp-workflow skill) (implemented)
- FEAT-016: Feature 生命周期模板 (uvp-feature-lifecycle skill) (implemented)
- FEAT-017: 文件修改联动规则 (uvp-file-coupling skill) (implemented)
- FEAT-018: 周报生成 (uvp-weekly-report skill) (implemented)
- FEAT-019: 全局看板 (uvp kanban) (implemented)
