# FEAT-009: 文档一致性检查 (uvp check)

## 概述

`uvp check` 检查项目文档的一致性和特性闭环，确保所有关联文件正确同步。支持按类别筛选检查项，不指定筛选时执行全部 5 项检查。

## 接口定义

### CLI 参数

```bash
uvp check [OPTIONS]

Options:
  --features          仅检查特性闭环
  --adr               仅检查 ADR 一致性
  --fix               自动修复可修复的问题（待实现）
  -h, --help          显示帮助信息
```

别名：`uvp c`

### 筛选逻辑

- 不指定 `--features` / `--adr`：执行全部 5 项检查
- 指定 `--features`：仅执行"1. 特性闭环检查"
- 指定 `--adr`：仅执行"2. ADR 一致性检查"

## 行为规格

### 1. 特性闭环检查

| 检查项 | 说明 |
|--------|------|
| 目录存在 | registry 中每个特性的 directory 必须存在 |
| spec.md 存在 | 每个特性目录必须包含 spec.md |
| verified 标记一致 | status = `verified` 的特性，verification.md 中必须包含"已验证" |
| 状态合法性 | 特性状态必须在 `ALL_STATUSES` 列表中 |

### 2. ADR 一致性检查

| 检查项 | 说明 |
|--------|------|
| ADR 目录存在 | `docs/adr/` 目录必须存在 |
| registry.md 存在 | `docs/adr/registry.md` 必须存在 |
| 状态合法性 | ADR 状态必须为 proposed / accepted / superseded / deprecated |
| front matter 一致 | YAML front matter 中的 status 与正文解析的状态一致 |
| superseded 引用 | status = `superseded` 的 ADR 必须引用替代 ADR |

### 3. AI 上下文检查

| 检查项 | 说明 |
|--------|------|
| AI_CONTEXT.md 存在 | `docs/AI_CONTEXT.md` 必须存在 |
| PROJECT_STATE.md 存在 | `docs/PROJECT_STATE.md` 必须存在 |


### 输出格式

```
文档一致性检查

1. 特性闭环检查
  ✓ 所有特性闭环正常

2. ADR 一致性检查
  ✓ ADR 一致性正常

3. AI 上下文检查
  ✓ AI 上下文正常

════════════════════════════════════════
发现 1 个问题
提示：运行 uvp render 更新渲染页面
```

### `--fix` 行为（待实现）

自动修复可修复的问题：
- 重新生成 registry.md

### 边界条件

- 项目未初始化：显示错误信息
- ADR 目录不存在：标记为问题，继续检查其他项
- Feature 目录不存在：标记为问题，继续检查其他特性

### 错误处理

- 文件读取失败：跳过该文件，继续检查
- registry 解析失败：标记为问题

## 约束

### 兼容性

- 支持 Windows、macOS、Linux

## 验收标准

- [x] 正确检测 Feature 闭环问题（目录/文件缺失、状态不合法、验证标记不一致）
- [x] 正确检测 ADR 一致性问题（状态合法性、front matter 一致性、superseded 引用）
- [x] 正确检测 AI 上下文和 PROJECT_STATE 完整性
- [x] `--features` / `--adr` 筛选正确
- [ ] `--fix` 自动修复可修复问题（参数已定义，逻辑待实现）
- [x] 输出清晰的检查结果
