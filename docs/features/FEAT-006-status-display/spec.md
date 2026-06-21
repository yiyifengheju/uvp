# FEAT-006: 状态展示 (uvp status)

## 概述

`uvp status` 在终端展示当前项目状态（Feature 进度、ADR 统计、Git 信息），是最常用的状态查看命令。通过 `--open` 在浏览器查看 HTML 报告，通过 `--onboard` 启动在线实时状态面板。

**设计哲学：默认只展示项目状态，工作流信息通过 --help 获取。**

## 接口定义

### CLI 参数

```bash
uvp status [OPTIONS]

Options:
  --open             在浏览器中打开 HTML 项目状态报告
  --onboard          启动在线实时状态面板（详见 sub-spec-onboard.md）
  -h, --help         显示帮助信息 + 工作流编排提示
```

别名：`uvp s`

## 行为规格

### 默认模式（终端项目状态）

终端输出当前项目状态，不展示工作流：

```
ai-uvp
══════

  ADR    3 (proposed: 1, accepted: 2)
  Feature  12 (implemented: 9, planned: 3)

  活跃特性
    🔧 FEAT-011  某某特性

  Git    main · abc1234 最近提交信息
```

信息来源：

| 信息项 | 来源 |
|--------|------|
| 项目名称 | `pyproject.toml` `[project].name` 或目录名 |
| ADR 统计 | 扫描 `docs/adr/*.md`（排除 template.md / registry.md） |
| Feature 统计 | 读取 `feature-registry.yaml` |
| 活跃特性 | registry 中 status = `in_progress` 的条目 |
| Git 信息 | `git rev-parse --abbrev-ref HEAD` + `git log -1 --oneline` |

### `--help` 模式

除标准帮助信息外，附带工作流编排提示（终端输出 + 提示查看 SVG）：

```
6 步闭环工作流:
  1. Decide    → uvp adr "标题"
  2. Define    → uvp feature new "功能名"
  3. Plan      → 编辑 FEAT-xxx/plan.md
  4. Implement → 修改 src/ + 更新 changelog.md
  5. Verify    → uvp feature close FEAT-NNN
  6. Distill   → 更新 context.md + PROJECT_STATE.md

查看完整工作流图: uvp status --open-workflow
```

同时在浏览器中打开 `~/.uvp/workflow.svg`。

### `--open` 模式（浏览器 HTML 报告）

生成 HTML 状态报告并在浏览器中打开：

- 项目名称 + 生成时间
- 概览卡片（ADR 总数、Feature 总数、进行中、已验证）
- Git 信息（分支 + 最近提交）
- Feature 完整表格（ID、标题、状态、创建日期、更新日期）
- 文件路径：`<project>/.uvp/status.html`

### `--onboard` 模式

启动在线实时状态面板，详见 [sub-spec-onboard.md](sub-spec-onboard.md)。

### 边界条件

- 项目未初始化：显示提示 "请先运行 uvp init"
- Git 未安装：跳过 Git 信息，不报错
- feature-registry.yaml 不存在：显示 Feature 数为 0
- SVG 文件不存在：`--help` 时跳过浏览器打开，仅终端输出

### 错误处理

- HTML 生成失败：显示错误信息
- 浏览器打开失败：显示文件路径，提示手动打开

## 约束

### 性能

- 终端输出应在 0.5 秒内完成
- HTML 生成应在 1 秒内完成

### 兼容性

- 支持 Windows、macOS、Linux
- HTML 报告兼容主流浏览器

## 验收标准

- [ ] 默认输出项目状态（无工作流）
- [ ] `--help` 输出工作流编排 + 打开 SVG
- [ ] `--open` 生成并打开 HTML 报告
- [ ] 正确统计 ADR 和 Feature 数量
- [ ] Git 信息正确显示
- [ ] `--onboard` 启动在线面板（见 sub-spec）
