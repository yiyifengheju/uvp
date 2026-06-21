# FEAT-002: 项目初始化 (uvp init)

## 概述

`uvp init` 是 uvp 的核心命令，用于快速初始化 vibe coding 项目。一键创建标准目录结构、配置文件、AI 上下文文件和文档模板。

## 接口定义

### CLI 参数

```bash
uvp init [OPTIONS] [PATH]

Arguments:
  [PATH]  项目路径，默认为当前目录

Options:
  --name <NAME>           项目名称，默认为目录名
  --ide <IDE>             目标 IDE [claude, cursor, windsurf, cline, trae] [default: claude]
  --no-python             跳过 uv init（不创建 Python 项目）
  --no-mkdocs             跳过 mkdocs.yml 创建
  --no-ai-rules           跳过 AI 规则文件生成
  -h, --help              显示帮助信息
```

### 配置文件

```toml
[init]
auto_uv_init = true
auto_mkdocs = true
auto_ai_context = true
auto_ai_rules = true
dependencies = ["mkdocs-material", "mkdocs-glightbox", "mkdocs-awesome-pages-plugin", "neoteroi-mkdocs"]
```

## 行为规格

### 正常流程

1. 调用 `uv init` 初始化 Python 项目（除非 `--no-python`）
2. 创建标准目录结构：
   ```
   <project>/
   ├── docs/
   │   ├── adr/
   │   ├── prd/
   │   ├── features/
   │   ├── architecture/
   │   ├── _meta/
   ├── src/
   ├── reference/
   ```
3. 创建 `uvp.toml` 项目配置文件
4. 创建 `mkdocs.yml`（除非 `--no-mkdocs`）
5. 创建 ADR 模板和 registry
6. 创建 Feature Registry
7. 创建 `AI_CONTEXT.md` 和 `PROJECT_STATE.md`
8. 生成 AI 规则文件（CLAUDE.md / .cursorrules 等）
9. 创建辅助文档（GLOSSARY.md、roadmap.md 等）
10. 设置全局配置 `~/.uvp/`
11. 执行 `uv add` 安装依赖

### 边界条件

- 目录已存在：跳过创建，显示已存在状态
- 文件已存在：不覆盖，显示已存在状态
- pyproject.toml 已存在：跳过 `uv init`
- `~/.uvp/` 已存在：跳过全局配置初始化

### 错误处理

- `uv init` 失败：显示错误信息，继续后续步骤
- `uv add` 失败：显示错误信息，不影响其他步骤
- 权限不足：显示错误信息，终止执行

## 约束

### 性能

- 目录创建应在 1 秒内完成
- `uv init` 和 `uv add` 在后台执行，不阻塞主流程
- UI 输出流畅，每步之间有适当延迟（可配置）

### 兼容性

- 支持 Windows、macOS、Linux
- 兼容 Python 3.13+
- 支持 PyInstaller 打包

### 幂等性

- 重复执行 `uvp init` 不会覆盖已有文件
- 所有操作都是幂等的

## 验收标准

- [ ] 执行 `uvp init my-project` 成功创建完整目录结构
- [ ] 所有配置文件正确生成
- [ ] AI 规则文件内容正确指向 AI_CONTEXT.md
- [ ] UI 输出清晰，显示每步状态
- [ ] 重复执行不覆盖已有文件
- [ ] `--no-python` 跳过 uv init
- [ ] `--no-mkdocs` 跳过 mkdocs.yml 创建
