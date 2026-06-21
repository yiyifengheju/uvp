# FEAT-010 Context

> 此文件为 AI 提供压缩上下文，由 Distill 步骤维护。

## 当前状态

implemented

## 核心概念

uvp 是 Rust CLI 工具，通过 `include_str!` 内嵌 11 个模板和 5 个 skills，编译为零依赖单文件二进制。分发两种方式：独立二进制和 Python 包装器（uv-plus）。

## 关键决策

- Rust 编译（非 Python/PyInstaller），零运行时依赖
- 资源全部内嵌，不依赖外部模板文件
- Python 包装器是薄 shim，仅定位并执行 Rust 二进制
- 版本号格式 YYYY.M.P，Cargo.toml 和 pyproject.toml 需同步

## 实现要点

- Cargo.toml：`src/ruvp/Cargo.toml`
- 模板内嵌：`src/ruvp/common.rs` embed_templates! 宏
- Skills 内嵌：`src/ruvp/config.rs` deploy_builtin_skills
- Python shim：`python/uv_plus/__init__.py`
- 构建脚本：`scripts/build.sh` / `scripts/build.bat`
