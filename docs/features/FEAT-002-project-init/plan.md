# FEAT-002 Plan

## 实施步骤

1. 定义 CLI 参数（path / name / ide / no-python / no-mkdocs / no-ai-rules）
2. 实现目录创建逻辑（7 个标准目录）
3. 实现模板文件写入（uvp.toml / mkdocs.yml / ADR / Feature Registry / AI 文档）
4. 实现 AI 规则文件生成与跨 IDE 复用
5. 实现全局配置初始化（~/.uvp/）
6. 实现后台 uv init + uv add 流式执行
7. 实现 UI 输出（spinner + 状态面板）

## 待修复项

全部已修复（2026-06-19）：
- ~~项目级文件应检查已存在则跳过，而非覆盖~~ → write_file_safe()
- ~~权限错误应检测并终止，而非 `.ok()` 静默吞掉~~ → 显式错误检查 + exit(1)
- ~~`auto_feature_ledger` 配置应被 init 逻辑使用~~ → Feature Registry 创建受配置控制

## 依赖

- FEAT-001: Feature Ledger 系统（目录结构规范）
- FEAT-012: 终端显示设计（UI 输出）
