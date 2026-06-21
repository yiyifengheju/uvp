# FEAT-014: TODO 管理 (uvp todo)

## 概述

`uvp todo` 提供轻量级的项目想法/待办管理。与 ADR（架构决策）不同，TODO 用于记录尚未形成决策的想法、灵感、待验证的方向——比 ADR 更轻量、比 Feature 更随意。

**定位：想法收集箱，ADR 的前置阶段。**

```
想法/灵感 → docs/TODO.md → 评估成熟后 → uvp adr（决策）→ uvp feature（实施）
```

## 接口定义

### CLI 参数

```bash
uvp todo [SUBCOMMAND]

Subcommands:
  (无)                     默认列出所有 TODO 项
  add <CONTENT>            添加一条 TODO
  done <ID>                标记完成
  remove <ID>              删除一条 TODO

Options:
  --all                    显示所有 TODO（含已完成）
  -h, --help              显示帮助信息
```

别名：`uvp td`

### 数据文件

```
docs/TODO.md
```

### 文件格式

```markdown
# TODO

> 项目想法、待验证方向、灵感收集。成熟后转为 ADR 或 Feature。

## 待办

- [ ] #1 尝试 RLHF 方案替代当前 SFT <!-- 2026-06-19 -->
- [ ] #2 调研 vLLM 部署方案 <!-- 2026-06-20 -->
- [ ] #3 考虑增加 A/B 测试框架 <!-- 2026-06-20 -->

## 已完成

- [x] #4 确认数据集许可证问题 <!-- 2026-06-18 → 2026-06-19 -->
```

### 配置文件

无独立配置，沿用项目根目录。

## 行为规格

### `uvp todo`（默认：列出待办）

终端输出未完成的 TODO 列表：

```
TODO (3 项待办)
═══════════════

  #1  尝试 RLHF 方案替代当前 SFT          2026-06-19
  #2  调研 vLLM 部署方案                   2026-06-20
  #3  考虑增加 A/B 测试框架                2026-06-20
```

### `uvp todo add <CONTENT>`

1. 读取 `docs/TODO.md`
2. 分配下一个 ID（当前最大 ID + 1）
3. 在"## 待办"下追加新条目：`- [ ] #N <content> <!-- date -->`
4. 输出确认信息

### `uvp todo done <ID>`

1. 在"## 待办"中找到对应 ID 的条目
2. 将 `- [ ]` 改为 `- [x]`，追加完成日期
3. 将条目从"## 待办"移动到"## 已完成"
4. 输出确认信息

### `uvp todo remove <ID>`

1. 从文件中删除对应 ID 的条目
2. 输出确认信息

### `uvp todo --all`

显示所有条目（含已完成），已完成条目用删除线或灰色展示。

### init 集成

`uvp init` 在 Phase 1 创建 `docs/TODO.md`（使用标准模板，包含空的"待办"和"已完成"段落）。遵循已有的 `write_file_safe` 逻辑（已存在不覆盖）。

### 边界条件

- `docs/TODO.md` 不存在：`uvp todo` 时自动创建
- ID 不存在：显示错误信息
- ID 已完成：`done` 时提示"已完成"
- 空列表：显示"暂无 TODO"

### 错误处理

- 文件读写失败：显示错误信息
- 格式解析异常：跳过异常行，不影响其他条目

## 约束

### 格式兼容性

- Markdown checkbox 格式，兼容 Obsidian / GitHub / mkdocs 渲染
- HTML 注释存储日期元数据，不影响渲染

### 性能

- 列表展示应在 0.3 秒内完成
- 文件操作应在 0.5 秒内完成

### 兼容性

- 支持 Windows、macOS、Linux
- 文件使用 UTF-8 编码

## 验收标准

- [ ] `uvp init` 创建 `docs/TODO.md`
- [ ] `uvp todo` 正确列出待办项
- [ ] `uvp todo add "内容"` 正确添加条目
- [ ] `uvp todo done 1` 正确标记完成并移动
- [ ] `uvp todo remove 1` 正确删除条目
- [ ] `uvp todo --all` 显示含已完成条目
- [ ] ID 自增正确
- [ ] 文件不存在时自动创建
