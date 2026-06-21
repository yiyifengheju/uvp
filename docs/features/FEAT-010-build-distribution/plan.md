# FEAT-010 Plan

## 实施步骤

1. Cargo.toml 配置（包名、版本、依赖、二进制目标）
2. embed_templates! 宏内嵌模板文件
3. deploy_builtin_skills 内嵌 skills
4. Python 包装器 shim（uv_plus/__init__.py）
5. pyproject.toml 配置（Hatchling 构建、入口点）
6. 编写构建脚本（build.sh / build.bat）
7. 编写安装测试脚本

## 依赖

- FEAT-013: 跨平台支持（pip 安装路径）
