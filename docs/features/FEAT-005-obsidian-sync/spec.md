# FEAT-005: Obsidian 知识导入 (uvp obsidian)

## 概述

`uvp obsidian` 是连接 Obsidian 知识库与 uvp 项目的单向导入桥梁。核心场景：用户在 Obsidian 中通过网页剪藏、论文阅读、Cherry Studio 问答等方式积累研究素材，然后通过 `uvp pull` 将提炼后的笔记导入项目 `reference/` 目录，作为 ADR 和 Feature 的输入素材。

**设计哲学：Obsidian 是输入端，项目是产出端。**

- Obsidian 负责信息获取与初步整理（网页剪藏、论文笔记、AI 问答分析）
- uvp 项目负责工程化产出（ADR、Feature、代码、文档）
- 产出文档由 mkdocs 管理展示，不回推 Obsidian

## 工作流定位

```
网页/论文 → Obsidian 剪藏 → Cherry Studio 问答分析 → 提炼笔记
                                                          ↓
                                                   uvp pull (导入)
                                                          ↓
                                              reference/ (研究素材)
                                                          ↓
                                          uvp adr --from-obsidian (构建决策)
                                                          ↓
                                              ADR → Feature → 代码实现
                                                          ↓
                                              deliverables.md + mkdocs (展示)
```

## 接口定义

### CLI 参数

```bash
uvp obsidian <SUBCOMMAND>

Subcommands:
  pull                     从 Obsidian Vault 拉取 reference/ 素材到项目
  sync                     双向同步 reference/（项目 ↔ Vault）

Options:
  --vault <VAULT_PATH>     指定 Obsidian Vault 路径（覆盖配置文件）
  --dry-run                仅显示将要同步的文件，不实际执行
  -h, --help               显示帮助信息
```

### 配置文件

```toml
[obsidian]
vault = ""                   # Obsidian Vault 路径（支持 ~ 展开）
exclude_dirs = ["node_modules", ".git", "assets", ".obsidian"]
```

## 行为规格

### `uvp obsidian pull`（主要命令）

将 Vault 中的研究素材导入项目：

1. 读取配置获取 Vault 路径
2. 源路径：`<vault>/Projects/<project-name>/reference/`
3. 目标路径：`<project>/reference/`
4. 基于文件修改时间增量同步（仅较新文件覆盖）
5. 输出同步摘要（新增/更新/跳过文件数）

典型使用场景：
- 用户在 Obsidian 中收集论文笔记、Cherry Studio 问答记录
- 整理后放入 Vault 的 `Projects/<project>/reference/` 目录
- 运行 `uvp pull` 导入项目
- 然后 `uvp adr --from-obsidian <关键词>` 引用笔记创建 ADR

### `uvp obsidian sync`

reference/ 目录的双向同步：

1. Pull 阶段：Vault → 项目 reference/
2. Push 阶段：项目 reference/ → Vault
3. 冲突策略：较新文件覆盖较旧文件

### `uvp adr --from-obsidian <keyword>`（已实现于 FEAT-003）

从 Vault 中搜索匹配笔记并嵌入 ADR：

1. 全 Vault 递归搜索文件名包含关键词的 .md 文件
2. 多结果取最短文件名（最精确匹配）
3. 将笔记内容嵌入 ADR 的"背景"部分，保留 `[[wikilink]]` 回链

### 边界条件

- Vault 路径未配置：显示错误信息
- Vault 目录不存在：显示错误信息
- 无文件需要同步：显示"已是最新"
- reference/ 目录不存在：自动创建

### 错误处理

- Vault 路径无效：显示错误信息，终止
- 文件读写失败：显示错误信息，继续处理其他文件

## 约束

### 单向原则

- 产出文档（docs/）**不**推送到 Obsidian
- 仅 reference/（研究素材）参与同步
- mkdocs 作为产出文档的展示和管理工具

### 兼容性

- 支持 Windows、macOS、Linux
- 兼容 Obsidian 的 wikilink 和 YAML front matter 格式
- Vault 路径支持 `~` 展开

### 性能

- 单次同步应在 3 秒内完成（100 个文件以内）

## 验收标准

- [ ] `uvp obsidian pull` 正确从 Vault 拉取 reference/ 文件
- [ ] `uvp obsidian sync` 正确双向同步 reference/
- [ ] `--dry-run` 正确显示将要同步的文件
- [ ] `--vault` 正确覆盖配置路径
- [ ] 排除目录正确过滤
- [ ] `uvp adr --from-obsidian` 正确搜索和嵌入笔记内容
