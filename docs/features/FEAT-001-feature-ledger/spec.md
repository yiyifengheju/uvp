# FEAT-001: Feature Ledger 系统

## 概述

Feature Ledger 是 uvp 的核心特性管理系统，为每个特性提供独立的生命周期管理，包括规格定义、实施计划、变更记录、验证证据和 AI 上下文。

## 接口定义

### 目录结构

每个特性一个独立目录：

```
docs/features/FEAT-NNN-<slug>/
├── spec.md               # 特性的当前规格（接口、行为、约束）
├── plan.md               # 实施计划（可选）
├── changelog.md          # 特性级变更记录
├── verification.md       # 验证状态与证据
├── context.md            # 给 AI 的精简上下文
└── deliverables.md       # 产出记录（实验结果、模型指标、关键数据）
```

### 配置文件

```toml
[feature]
directory = "docs/features"
registry = "docs/_meta/feature-registry.yaml"
```

## 行为规格

### 特性状态流转

```
idea → planned → in_progress → implemented → verified
  │                              │
  └──→ paused ←─────────────────┘
  └──→ deprecated ──→ removed
```

### 文件职责

| 文件 | 职责 | AI 默认读 |
|------|------|----------|
| `spec.md` | 特性的当前规格（接口、行为、约束） | 是 |
| `plan.md` | 实施计划（可选） | 按需 |
| `changelog.md` | 该特性的变更时间线 | 否 |
| `verification.md` | 验证状态与证据 | 按需 |
| `context.md` | 给 AI 的精简上下文 | 是 |
| `deliverables.md` | 产出记录（实验结果、模型指标、关键数据） | 按需 |

### feature-registry.yaml

特性注册表，位于 `docs/_meta/feature-registry.yaml`：

```yaml
features:
  - id: FEAT-001
    title: "Feature Ledger 系统"
    status: implemented
    spec: docs/features/FEAT-001-feature-ledger/spec.md
    tests: null
    created: 2026-06-14
    updated: 2026-06-14
```

### 特性编号规则

- 格式：`FEAT-NNN-title`
- 递增编号，从 `FEAT-001-xxxx` 开始
- 编号在 `feature-registry.yaml` 中分配

## 约束

### 数据一致性

- 特性状态必须与 registry 同步
- 特性目录结构必须完整
- 所有变更必须记录到 changelog

## 验收标准

- [ ] 特性目录结构完整
- [ ] registry.yaml 正确记录所有特性
- [ ] 状态流转符合规范
- [ ] 文件职责清晰明确
