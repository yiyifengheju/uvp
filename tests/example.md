---
title: "UVP 核心功能测试流程"
date: "2026-06-19"
doc_type: tutorial
---

# UVP 核心功能测试流程

以"鸢尾花数据集分类"为例，验证 uvp v2026.6.0 全部核心功能。

**前置条件**: 已安装 uvp 并添加到 PATH

---

## 测试流程

### Phase 1: 项目初始化

| # | 命令 | 验证功能 | 预期结果 |
|---|------|---------|---------|
| 1 | `uvp init C:\tmp\iris --name iris-classifier --no-python --no-mkdocs` | 项目初始化 (FEAT-002) | 生成目录结构 + uvp.toml + AI_CONTEXT.md + TODO.md 等；已有文件不覆盖；`~/.uvp/` 自动创建 |
| 2 | `cd C:\tmp\iris` | 进入项目 | — |
| 3 | `uvp init C:\tmp\iris --name iris-classifier --no-python --no-mkdocs` | 幂等性验证 | 所有文件显示 ⏭️ skip，不覆盖 |

### Phase 2: 配置与状态

| # | 命令 | 验证功能 | 预期结果 |
|---|------|---------|---------|
| 4 | `uvp config` | 配置管理 (FEAT-007) | 输出合并后的完整 TOML 配置 |
| 5 | `uvp status` | 状态展示 (FEAT-006) | 终端输出项目名、ADR/Feature 统计、Git 信息；无工作流输出 |
| 6 | `uvp status --help` | 工作流展示 | 帮助末尾显示 6 步闭环工作流 |
| 7 | `uvp status --onboard` | onboard 占位 | 输出 "coming soon" 提示 |

### Phase 3: ADR 管理

| # | 命令 | 验证功能 | 预期结果 |
|---|------|---------|---------|
| 8 | `uvp adr "选择随机森林算法进行鸢尾花分类"` | ADR 创建 (FEAT-003) | spinner 转动 → ✅ ADR #0001 created；生成 `docs/adr/0001-*.md`；registry.md 自动更新 |
| 9 | `uvp adr "对比 SVM 与 RandomForest" -s accepted` | ADR 状态 | 生成 #0002，状态为 accepted |

### Phase 4: Feature 生命周期

| # | 命令 | 验证功能 | 预期结果 |
|---|------|---------|---------|
| 10 | `uvp feature new "鸢尾花分类模块" --adr 0001` | Feature 创建 (FEAT-004) | spinner → ✅；生成 6 个文件（spec/plan/changelog/verification/context/deliverables） |
| 11 | `uvp f list` | Feature 列表 | 显示 FEAT-001 状态为 planned |
| 12 | `uvp f show FEAT-001` | Feature 详情 | 显示完整信息 + spec 摘要 |
| 13 | `uvp f status FEAT-001 in_progress` | 状态更新 | 状态变为 in_progress |
| 14 | `uvp f status FEAT-001 implemented` | 状态更新 | 状态变为 implemented |
| 15 | `uvp f close FEAT-001` | Feature 关闭 | 状态变为 verified；verification.md 标记"已验证" |

### Phase 5: TODO 管理

| # | 命令 | 验证功能 | 预期结果 |
|---|------|---------|---------|
| 16 | `uvp todo` | 空列表 (FEAT-014) | 显示"暂无 TODO"；自动创建 docs/TODO.md |
| 17 | `uvp todo add "尝试 XGBoost 替代 RF"` | 添加 | ✅ 已添加 TODO #1 |
| 18 | `uvp todo add "调研特征选择方法"` | 添加 | ✅ 已添加 TODO #2 |
| 19 | `uvp td` | 别名 + 列表 | 显示 2 项待办 |
| 20 | `uvp todo done 1` | 标记完成 | ✅ TODO #1 已标记完成 |
| 21 | `uvp todo --all` | 全部列表 | 显示 1 项待办 + 1 项已完成 |
| 22 | `uvp todo remove 2` | 删除 | ✅ TODO #2 已删除 |

### Phase 6: 文档一致性检查

| # | 命令 | 验证功能 | 预期结果 |
|---|------|---------|---------|
| 23 | `uvp check` | 全量检查 (FEAT-009) | 3 项检查全部 ✅ |
| 24 | `uvp check --features` | 筛选检查 | 仅执行特性闭环检查 |
| 25 | `uvp check --adr` | 筛选检查 | 仅执行 ADR 一致性检查 |

### Phase 7: 渲染与浏览

| # | 命令 | 验证功能 | 预期结果 |
|---|------|---------|---------|
| 26 | `uvp render` | Registry 渲染 | ✅ 已渲染 features/index.md + adr/registry.md |
| 27 | `uvp render --check` | 渲染一致性 | 所有页面与数据源一致 |
| 28 | `uvp status --open` | HTML 报告 (FEAT-006) | spinner → ✅；浏览器打开 .uvp/status.html |

### Phase 8: Obsidian（可选，需配置 Vault）

| # | 命令 | 验证功能 | 预期结果 |
|---|------|---------|---------|
| 29 | `uvp obsidian pull --dry-run` | 模拟拉取 (FEAT-005) | 列出将要拉取的文件或"没有需要同步的文件" |
| 30 | `uvp obsidian sync --dry-run` | 模拟同步 | 列出将要同步的文件 |

### Phase 9: IDE 工具

| # | 命令 | 验证功能 | 预期结果 |
|---|------|---------|---------|
| 31 | `uvp ide claude` | IDE 规则生成 | 生成 CLAUDE.md + 部署 skills 到 .claude/skills/ |

---

## 快速冒烟测试（最小集）

如果时间有限，只运行以下命令验证核心链路：

```bash
uvp init C:\tmp\iris --name iris --no-python --no-mkdocs
cd C:\tmp\iris
uvp adr "测试决策"
uvp f new "测试特性" --adr 0001
uvp todo add "测试想法"
uvp todo
uvp status
uvp check
uvp config
uvp f close FEAT-001
uvp check
```

---

## 清理

```powershell
Remove-Item -Recurse -Force C:\tmp\iris
```
