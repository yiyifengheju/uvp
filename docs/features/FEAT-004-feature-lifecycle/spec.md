# FEAT-004: 特性生命周期管理 (uvp feature)

## 概述

`uvp feature` 用于管理 Feature Ledger 中的特性全生命周期，从创建、规划、实现到验证闭环。

## 接口定义

### CLI 参数

```bash
uvp feature <SUBCOMMAND>

Subcommands:
  new <TITLE>              创建新特性
  list                     列出所有特性
  show <ID>                显示特性详情
  status <ID> <STATUS>     更新特性状态
  close <ID>               将特性标记为 verified（闭环）
  archive <ID>             将特性标记为 deprecated
```

### 特性状态流转

```
idea → planned → in_progress → implemented → verified
  │                              │
  └──→ paused ←─────────────────┘
  └──→ deprecated ──→ removed
```

### 配置文件

```toml
[feature]
directory = "docs/features"
registry = "docs/_meta/feature-registry.yaml"
```

## 行为规格

### `uvp feature new` 流程

1. 读取 `feature-registry.yaml` 获取下一个编号
2. 创建目录 `docs/features/FEAT-NNN-<slug>/`
3. 生成文件：
   - `spec.md`：特性规格（模板）
   - `plan.md`：实施计划（模板）
   - `changelog.md`：变更记录（模板）
   - `verification.md`：验证记录（模板）
   - `context.md`：AI 上下文（模板）
   - `deliverables.md`：产出记录（模板）— 实验结果、模型指标、关键数据
4. 更新 `feature-registry.yaml`
5. 更新 `docs/AI_CONTEXT.md` 中的活跃特性列表
6. 输出特性编号和目录路径

### `uvp feature list` 输出

```
Features
════════

  FEAT-001  Feature Ledger      implemented    2026-06-14
  FEAT-002  项目初始化           in_progress    2026-06-14
  FEAT-003  ADR 管理            planned        2026-06-14
```

### `uvp feature show` 输出

显示特性的完整信息：
- 基本信息（ID、标题、状态、创建日期）
- spec.md 内容摘要
- 最近的 changelog 条目
- 验证状态

### `uvp feature close` 流程

1. 将特性状态更新为 `verified`
2. 在 `verification.md` 中标记 `Status: verified`
3. 更新 `feature-registry.yaml`
4. 更新 `docs/AI_CONTEXT.md`
5. 输出闭环确认信息

### 边界条件

- 特性 ID 不存在：显示错误信息
- 状态转换不合法：显示错误信息（如从 verified 回到 in_progress）
- registry.yaml 不存在：自动创建
- 特性目录已存在：显示错误信息

### 错误处理

- 文件写入失败：显示错误信息
- registry 解析失败：显示错误信息
- 状态转换非法：显示错误信息和合法转换路径

## 约束

### 兼容性

- 支持 Windows、macOS、Linux
- 兼容 YAML 和 JSON 格式

### 数据一致性

- 特性状态必须与 registry 同步
- 特性目录结构必须完整
- 所有变更必须记录到 changelog

## 验收标准

- [ ] `uvp feature new "用户登录"` 成功创建完整特性目录
- [ ] `uvp feature list` 正确显示所有特性及状态
- [ ] `uvp feature show FEAT-001` 显示完整特性信息
- [ ] `uvp feature status FEAT-001 in_progress` 成功更新状态
- [ ] `uvp feature close FEAT-001` 成功闭环特性
- [ ] registry.yaml 自动更新
- [ ] AI_CONTEXT.md 自动更新
- [ ] 状态转换验证正确
