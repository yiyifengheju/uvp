---
name: "uvp-file-coupling"
description: "文件修改联动规则：修改文件时必须同步更新关联文件。几乎每次文件修改都触发。"
---

# UVP 文件修改联动规则

**触发条件**: 修改 `src/`、`docs/features/`、`docs/adr/` 下的任何文件

---

## 联动矩阵

### 🔴 必须遵守（违反 = 数据不一致）

| # | 当你... | 必须同步更新... | 自动/手动 |
|---|---------|----------------|----------|
| 1 | 创建 ADR | `docs/_meta/adr-registry.yaml`（源） → `docs/adr/index.md`（渲染） | ✅ `uvp adr` 自动 |
| 2 | 关联 Feature 到 ADR | ADR `related_features` + 正文"🎯 AI 上下文" | ❌ 手动（2处） |
| 3 | 创建 Feature | `feature-registry.yaml` + `docs/features/index.md` | ✅ `uvp f new` 自动 |
| 4 | **修改 src/ 代码** | **`FEAT-xxx/changelog.md`** | ❌ **手动（最重要！）** |
| 5 | 修改 API/接口 | `FEAT-xxx/spec.md` 接口定义 | ❌ 手动 |
| 6 | 完成验证 | `FEAT-xxx/verification.md` | ❌ 手动 |

### 🟡 应该遵守

| # | 当你... | 应该同步更新... |
|---|---------|----------------|
| 7 | Feature 状态变更 | `feature-registry.yaml`（`uvp f close` 自动） |
| 8 | 文档变更影响项目事实 | `docs/AI_CONTEXT.md` 第三节 |
| 9 | 更新 spec 验收标准 | 重新运行验证 |
| 10 | 关闭 Feature | `FEAT-xxx/context.md` |
| 11 | 关闭 Feature | `docs/PROJECT_STATE.md` |
| 12 | 关闭 Feature | `docs/roadmap.md`（AI 语义匹配，追加 `[[FEAT-xxx]]` 标签） |

---

## Changelog 编写规范（Rule #4 详解）

**格式**: Keep a Changelog 标准

```markdown
## [YYYY-MM-DD]
### Added
- 新增 `process_data()` 批处理函数
- 支持 CSV 和 JSON 输入格式

### Changed
- 重构 `Engine` 类，改用依赖注入
- 默认超时从 30s 改为 60s

### Fixed
- 修复 `DataLoader.__del__()` 内存泄漏 (issue #123)
- 修复并发处理竞态条件 (PR #456)

### Breaking Changes
- `old_api()` → `new_api()`，迁移指南见 docs/migration.md
```

### 反模式

| ❌ 错误 | ✅ 正确 |
|--------|--------|
| "Fixed bugs" | "Fixed null pointer in Parser.parse_line() when input empty (issue #42)" |
| "Updated code" | "Refactored AuthMiddleware to extract TokenValidator class for testability" |
| 一天结束后批量写 | 每个逻辑单元完成后立即写 |

---

## ADR ↔ Feature 双向关联（Rule #2 详解）

创建 ADR 后又创建 Feature 时，**必须回写 ADR**：

```yaml
# ADR front matter:
related_features:
  - "FEAT-001 (数据预处理模块)"
```

```markdown
# ADR 正文 "🎯 AI 上下文" 部分:
关联 Feature:
- FEAT-001（数据预处理模块）
```

---

## 自检清单

修改文件后问自己：
- [ ] 改了 src/ → changelog 更新了吗？
- [ ] 改了接口 → spec 同步了吗？
- [ ] 做了实验 → verification 引用了结果路径吗？
- [ ] 关闭 Feature → context + PROJECT_STATE 更新了吗？

验证：`uvp check`

---

*工作流步骤详见 `uvp-workflow` skill*
*Feature 模板详见 `uvp-feature-lifecycle` skill*
