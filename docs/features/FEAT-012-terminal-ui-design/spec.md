# FEAT-012: 终端显示设计 (uvp UI)

## 概述

uvp 的终端 UI 参考 uv 的风格，采用 spinner 进度条 + 子项滚动 + 彩色图标的组合，提供流畅的命令行交互体验。所有 UI 组件集中在 `ui.rs` 模块中。

## UI 组件

### Spinner API（多步骤 / 耗时命令）

| 函数 | 说明 | 输出效果 |
|------|------|----------|
| `step_start(msg)` | 创建旋转中的 spinner | `⠋ Creating uvp.toml` |
| `step_update(pb, msg)` | 更新 spinner 当前消息（同一行滚动） | `⠋ Creating directories docs/prd` |
| `step_done(pb, msg)` | 完成 → 停止旋转，显示 ✅ | `✅ Project configuration written` |
| `step_skip(pb, msg)` | 跳过 → ⏭️ | `⏭️ Skipped` |
| `step_fail(pb, msg)` | 失败 → ❌ | `❌ Template not found` |

### 图标函数

| 函数 | 返回值 | 用途 |
|------|--------|------|
| `icon_ok()` | `✅` | 成功操作 |
| `icon_fail()` | `❌` | 失败操作 |

### 简单输出 API（快速命令）

| 函数 | 输出效果 | 用途 |
|------|----------|------|
| `file_created(path)` | `✅ path`（青色） | 文件创建成功 |
| `file_exists(path)` | `🔹 path`（灰色） | 文件已存在 |
| `action_ok(msg)` | `✅ msg` | 操作成功 |
| `action_skip(msg)` | `⏭️ msg`（灰色） | 操作跳过 |
| `action_fail(msg)` | `❌ msg` | 操作失败 |
| `action_info(msg)` | `💡 msg` | 信息提示 |
| `step_header(n, title, total)` | 粗体标题 | 步骤标题 |

### 面板组件

| 函数 | 说明 | 用途 |
|------|------|------|
| `success_panel(title, body)` | 绿色粗体标题 + 灰色正文 | 命令完成汇总 |
| `info_panel(title, body)` | 青色粗体标题 + 灰色正文 | 信息面板 |

### 后台命令执行

| 函数 | 说明 |
|------|------|
| `spawn_command_streaming(program, args, cwd, on_line)` | 后台线程执行命令，每行输出通过回调报告，返回 `JoinHandle<bool>` |

### 延迟控制

| 函数 | 说明 |
|------|------|
| `get_delay_ms()` | 从 `~/.uvp/uvp.toml` 读取 `[ui].delay_ms`，默认 120ms |

## 配置

```toml
[ui]
delay_ms = 120    # 每步之间的延迟（毫秒），0 = 无延迟
```

## 各命令 UI 规范

### 分层策略

| 层级 | 适用场景 | UI 模式 | 示例命令 |
|------|---------|---------|----------|
| Spinner | 多步骤 / 多文件写入 / 外部命令 | step_start → step_update → step_done | init, feature new, obsidian pull |
| 进度文本 | 多项检查 / 扫描 | 编号标题 + ✅/❌ 逐项输出 | check, render |
| 简单文本 | 瞬时操作 / 单项结果 | icon_ok/fail + println | todo, config, adr, status |

### 各命令 UI 详情

| 命令 | 当前 UI | 应采用 UI | 说明 |
|------|---------|-----------|------|
| `uvp init` | ✅ Spinner | Spinner | 9 步 spinner + 后台 uv 命令 + success_panel |
| `uvp adr` | 简单文本 | Spinner | Vault 搜索 + registry 重建可能耗时 |
| `uvp feature new` | 简单文本 | Spinner | 6 文件创建 + registry 更新 + AI_CONTEXT 更新 |
| `uvp feature list/show/status` | 简单文本 | 简单文本 | 瞬时读取，无需 spinner |
| `uvp obsidian pull/sync` | 简单文本 | Spinner | 递归遍历 Vault + 文件复制可能耗时 |
| `uvp status` | 简单文本 | 简单文本 | ADR 扫描 + Git 命令，通常 <0.5s |
| `uvp status --open` | 简单文本 | Spinner | HTML 生成 + 浏览器打开 |
| `uvp check` | 进度文本 | 进度文本 | 编号标题 + 逐项 ✅/❌，当前模式合适 |
| `uvp render` | 进度文本 | 进度文本 | 编号标题 + 逐项结果，当前模式合适 |
| `uvp config` | 纯文本 | 纯文本 | 直接打印 TOML，无需装饰 |
| `uvp todo` | 简单文本 | 简单文本 | 单文件读写，瞬时完成 |
| `uvp ide` | step_header + action_* | step_header + action_* | 当前模式合适 |

### init 输出示例

```
uvp init  my-project
  IDE: claude  |  目标: /path/to/my-project

⠋ Creating uvp.toml
✅ Project configuration written
⠋ Creating directories docs/adr
✅ Directory structure created
⠋ Configuring MkDocs
✅ MkDocs configured
...
⠋ Running uv init ...
✅ Project initialized with uv
⠋ Installing mkdocs-material, ...
✅ Dependencies installed

🎉🎉 Initialized project `my-project`

Initialized successfully
  Next: Write PRDs in docs/prd/, then let AI execute via AI_CONTEXT.md
```

### check 输出示例

```
文档一致性检查

1. 特性闭环检查
  ✅ 所有特性闭环正常

2. ADR 一致性检查
  ❌ 0001-xxx.md: front matter 状态与正文不一致

3. AI 上下文检查
  ✅ AI 上下文正常

════════════════════════════════════════
发现 1 个问题
```

### todo 输出示例

```
TODO (3 项待办)
══════════════════════════════

  #1  尝试 RLHF 方案替代当前 SFT          2026-06-19
  #2  调研 vLLM 部署方案                   2026-06-20
```

### feature list 输出示例

```
编号       标题                          状态           关联ADR    创建日期     更新日期
─────────────────────────────────────────────────────────────────────────────────────────
FEAT-001   Feature Ledger 系统            ✨ implemented  0001       2026-06-14   2026-06-19
FEAT-002   项目初始化 (uvp init)           ✨ implemented  0001       2026-06-16   2026-06-19
```

## 行为规格

### Spinner 生命周期

```
⠋ 初始消息          ← step_start: 创建旋转动画
⠋ 更新消息          ← step_update: 同一行内切换子项
✅ 最终消息          ← step_done: spinner 消失，显示最终状态
```

### 状态图标映射

| 状态 | 图标 | 颜色 |
|------|------|------|
| 成功 | ✅ | — |
| 失败 | ❌ | — |
| 跳过 | ⏭️ | — |
| 已存在 | 🔹 | — |
| 信息 | 💡 | — |
| 旋转中 | ⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ | 青色 |

### 输出目标

- Spinner（`step_*`）：输出到 **stderr**（不干扰管道）
- 简单输出（`file_*` / `action_*` / `success_panel`）：输出到 **stdout**

### 设计原则

1. **Spinner 完成后消失** — 不残留旋转字符
2. **子项在同一行滚动** — 避免输出过长
3. **后台命令实时执行** — spinner 保持旋转直到命令完成
4. **适当延迟** — 让用户看清过程，但不拖沓（可配置）
5. **统一使用 ui.rs** — 所有命令应通过 ui 模块输出，避免裸 `console::style` 分散在各命令中

## 技术实现

| 库 | 用途 |
|-----|------|
| `console` 0.16 | 彩色文本输出（`style().green()` / `.red()` / `.cyan()` / `.dim()`） |
| `indicatif` 0.18 | Spinner 进度条（`ProgressBar` + `ProgressStyle`） |

### Spinner 参数

- 旋转字符：`⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏`（braille 风格）
- 帧率：80ms/tick
- 样式模板：`{spinner:.cyan} {wide_msg}`
- 输出目标：stderr

## 约束

### 性能

- Spinner 动画帧率：80ms/帧
- 子项切换延迟：可配置（默认 120ms）
- 不阻塞主线程（`enable_steady_tick`）

### 兼容性

- 支持 Windows Terminal、PowerShell、CMD
- 支持 macOS Terminal、iTerm2
- 支持 Linux 终端

## 待优化项

以下命令当前使用裸 `console::style` 输出，应迁移到 `ui.rs` API：

| 命令 | 当前问题 | 建议改动 |
|------|---------|---------|
| `adr` | Vault 搜索 + registry 重建用裸 println | 添加 spinner 包裹 |
| `feature new` | 6 文件创建用 file_created 但无 spinner | 用 spinner 包裹整个创建流程 |
| `obsidian` | 完全不使用 ui 模块 | Vault 遍历 + 文件同步添加 spinner |
| `status --open` | HTML 生成无进度 | 添加 spinner |
| `check` | 裸 console::style | 迁移到 ui 图标函数 |
| `render` | 裸 console::style | 迁移到 ui 图标函数 |

## 验收标准

- [x] Spinner 动画流畅
- [x] 完成后 spinner 消失，显示 ✅ / ❌ / ⏭️
- [x] 子项在同一行滚动
- [x] 延迟时间可配置
- [x] 后台命令执行支持流式输出
- [x] 面板组件正确显示
- [ ] 所有命令统一使用 ui.rs API（待优化）
