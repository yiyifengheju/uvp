# FEAT-009 Plan

## 实施步骤（待对齐差异时执行）

1. 实现 `--fix`：创建缺失的 changelog.md / verification.md
2. 决定是否保留 `--features` / `--adr` 筛选参数（代码有，spec 无）
3. 决定是否添加 `--verbose` 详细输出
4. 决定是否实现配置检查（uvp.toml 格式验证）
5. 更新 spec 或代码以消除差异

## 依赖

- FEAT-001: Feature Ledger（目录结构和 registry 规范）
- FEAT-003: ADR 管理（ADR 状态规范）
