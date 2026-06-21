# FEAT-010: 项目打包与分发

## 概述

uvp 使用 Rust（Cargo）编译为独立二进制文件，通过两种方式分发：独立可执行文件和 Python 包装器（`uv-plus`）。所有模板和 skills 通过 `include_str!` 内嵌到二进制中，运行时零外部依赖。

## 构建

### 版本管理

- 版本号格式：`YYYY.M.P`（如 `2026.6.0`）
- Rust 版本来源：`src/ruvp/Cargo.toml` 的 `version` 字段
- Python 版本来源：`pyproject.toml` 的 `version` 字段（需保持一致）

### 编译命令

```bash
# 当前平台
cd src/ruvp && cargo build --release

# 跨平台（通过 scripts/build.sh）
scripts/build.sh current    # 当前平台
scripts/build.sh all        # 所有目标平台
```

### 目标平台

| 平台 | Target Triple | 输出文件名 |
|------|---------------|-----------|
| Windows x64 | `x86_64-pc-windows-msvc` | `uvp.exe` |
| macOS x64 | `x86_64-apple-darwin` | `uvp-macos-x64` |
| macOS ARM | `aarch64-apple-darwin` | `uvp-macos-arm64` |
| Linux x64 | `x86_64-unknown-linux-gnu` | `uvp-linux-x64` |
| Linux ARM | `aarch64-unknown-linux-gnu` | `uvp-linux-arm64` |

### 输出目录

```
build/rs-v{version}/uvp.exe     # 预编译二进制
```

## 分发

### 方式一：独立二进制

直接使用 `build/` 下的编译产物，无需安装。

### 方式二：Python 包装器（uv-plus）

```bash
uv add uv-plus     # 推荐
pip install uv-plus
```

- 包名：`uv-plus`（PyPI）
- 构建系统：Hatchling
- 入口点：`uvp = "uv_plus:main"`
- 原理：Python shim 定位并执行内嵌的 Rust 二进制（`python/uv_plus/bin/uvp.exe`）
- 安装到项目的虚拟环境 `.venv/`，`uvp` 命令自动注册到 PATH
- 与独立二进制版本功能完全一致

### 资源内嵌

| 资源类型 | 数量 | 嵌入方式 |
|----------|------|----------|
| 文档模板 | 11 个 | `embed_templates!` 宏（`include_str!`） |
| Skills | 5 个（6 文件） | `deploy_builtin_skills`（`include_str!`） |

#### 内嵌模板列表

default.md / adr_registry.md / feature_registry.yaml / features_index.md / ai_context.md / ai_rule.md / project_state.md / glossary.md / architecture_current.md / roadmap.md / mkdocs.yml

#### 内嵌 Skills 列表

uvp-workflow / uvp-feature-lifecycle / uvp-file-coupling / uvp-meta-header / uvp-weekly-report

## 依赖

### Rust 依赖

| 分类 | Crate |
|------|-------|
| CLI | clap 4 (derive) |
| 配置 | toml 0.8, serde 1 (derive), serde_yaml 0.9 |
| 文件 | dirs 6, shellexpand 3, open 5 |
| 时间 | chrono 0.4 |
| 文本 | regex 1, console 0.16, indicatif 0.18 |
| 平台 | winapi 0.3 (Windows), libc 0.2 (Unix) |

### 构建脚本

| 文件 | 说明 |
|------|------|
| `scripts/build.sh` | 跨平台编译（支持 all / current） |
| `scripts/build.bat` | Windows 单平台编译 |
| `scripts/test_feat013.py` | pip 安装端到端测试 |
| `scripts/test_pip_install.py` | pip 安装冒烟测试 |

## 约束

### 运行时

- 无需 Python、Node.js 等运行时
- 无需网络连接
- 所有资源内嵌，单文件分发

### 跨平台一致性

- **路径处理**：使用 `std::path::Path` 统一处理路径分隔符
- **配置目录**：使用 `dirs` crate，全局配置统一为 `~/.uvp/uvp.toml`
- **终端输出**：使用 `console` + `indicatif` 处理跨平台终端兼容
- 路径中包含空格、中文、符号链接均正确处理

### 平台兼容性

- Windows 10+ / Windows Server 2016+
- macOS 10.15+ (Catalina)
- Linux: glibc 2.17+ (CentOS 7+, Ubuntu 18.04+)
- Python ≥ 3.10（仅 pip 安装方式需要）
- Rust 2024 edition

### 性能

- 跨平台性能差异不超过 10%
- 启动时间 < 50ms

## 验收标准

- [x] `cargo build --release` 生成独立二进制
- [x] `uvp --version` 正确显示版本号
- [x] 模板和 skills 正确内嵌
- [x] `uv add uv-plus` 安装后 `uvp` 命令可用
- [x] 跨平台编译脚本可用
- [x] Windows / macOS / Linux 三平台编译通过
- [x] 跨平台路径处理正确
- [x] 跨平台配置目录正确
