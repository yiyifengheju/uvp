# FEAT-007: 配置管理 (uvp config)

## 概述

`uvp config` 显示当前合并后的完整配置。配置来源于全局配置（`~/.uvp/uvp.toml`）和项目配置（`uvp.toml`），按优先级合并后以 TOML 格式输出。

**设计哲学：配置查看即可，修改直接编辑文件。**

## 接口定义

### CLI 参数

```bash
uvp config

Options:
  -h, --help    显示帮助信息
```

别名：`uvp cfg`

### 配置文件

| 文件 | 位置 | 优先级 |
|------|------|--------|
| 全局配置 | `~/.uvp/uvp.toml` | 低 |
| 项目配置 | `<project>/uvp.toml` | 高 |

## 行为规格

### 配置优先级

```
项目配置 (uvp.toml) > 全局配置 (~/.uvp/uvp.toml) > 默认值
```

### 默认行为

执行 `uvp config` 时，终端输出合并后的完整配置（TOML 格式）：

```toml
[adr]
directory = "docs/adr"
naming = "sequential"

[feature]
directory = "docs/features"
registry = "docs/_meta/feature-registry.yaml"

[obsidian]
vault = ""
exclude_dirs = ["node_modules", ".git", "assets", ".obsidian"]

[init]
auto_uv_init = true
auto_mkdocs = true
auto_ai_context = true
auto_ai_rules = true
auto_feature_ledger = true
dependencies = ["mkdocs-material", "mkdocs-glightbox", "mkdocs-awesome-pages-plugin", "neoteroi-mkdocs"]

[ui]
delay_ms = 120
```

### 边界条件

- 项目未初始化：仅显示全局配置 + 默认值
- 全局配置不存在：自动创建后显示
- 配置解析失败：显示错误信息

### 错误处理

- 配置文件格式错误：显示错误信息

## 约束

### 修改方式

- 直接编辑 `~/.uvp/uvp.toml`（全局）或 `<project>/uvp.toml`（项目）
- 无 CLI 写入命令，避免复杂性

### 兼容性

- 支持 Windows、macOS、Linux
- TOML 格式

## 验收标准

- [ ] `uvp config` 显示合并后的完整配置
- [ ] 配置优先级合并正确
- [ ] 项目未初始化时仅显示全局 + 默认
