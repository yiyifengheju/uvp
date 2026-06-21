# FEAT-003: ADR 管理 (uvp adr)

## 概述

`uvp adr` 用于快速创建架构决策记录（Architecture Decision Record），使用内置默认模板，支持从 Obsidian 导入上下文。

## 接口定义

### CLI 参数

```bash
uvp adr [OPTIONS] [TITLE]

Arguments:
  [TITLE]  ADR 标题

Options:
  --from-obsidian <KEYWORD>       从 Obsidian Vault 读取笔记作为上下文
  -s, --status <STATUS>           初始状态 [proposed, accepted, deprecated, superseded]
  -o, --open                      创建后用编辑器打开
  -h, --help                      显示帮助信息
```

### 配置文件

```toml
[adr]
directory = "docs/adr"
naming = "sequential"  # sequential | datetime
```

## 行为规格

### 正常流程

1. 解析命令行参数
2. 读取 `docs/adr/registry.md` 获取下一个编号
3. 生成文件名：`NNNN-kebab-case-title.md`
4. 根据模板生成 ADR 内容
5. 写入 `docs/adr/` 目录
6. 更新 `registry.md`
7. 如果指定 `--from-obsidian`，从 Vault 读取笔记内容填入"背景"部分
8. 如果指定 `--open`，用默认编辑器打开文件

### 边界条件

- 标题重复：提示用户确认或自动添加序号
- registry.md 不存在：自动创建
- 模板文件不存在：使用内置默认模板
- Obsidian Vault 路径未配置：显示错误信息

### 错误处理

- 文件写入失败：显示错误信息，终止执行
- Obsidian 读取失败：显示警告，继续创建空 ADR

## 约束

### 兼容性

- 支持 Windows、macOS、Linux

### 性能

- ADR 创建应在 0.5 秒内完成
- Obsidian 读取不应阻塞主流程（后台执行）

## 验收标准

- [ ] 执行 `uvp adr "选择数据库方案"` 成功创建 ADR 文件
- [ ] 文件名格式正确：`0001-选择数据库方案.md`
- [ ] registry.md 自动更新
- [ ] `--from-obsidian` 正确读取笔记内容
- [ ] `--open` 正确打开文件
