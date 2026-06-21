# FEAT-005 Plan

## 实施步骤

1. 定义 CLI 子命令（pull / sync + --dry-run / --vault）
2. 实现 Vault 路径解析（配置读取 + ~ 展开 + 路径验证）
3. 实现 sync_directory（增量同步 + mtime 比较 + exclude_dirs 过滤）
4. 实现 obsidian_pull（Vault reference/ → 项目 reference/）
5. 实现 obsidian_sync（reference/ 双向同步）

## 依赖

- FEAT-003: ADR 管理（`adr --from-obsidian` 使用 Vault 配置）
- FEAT-007: 配置管理（`uvp config set obsidian.vault`）
