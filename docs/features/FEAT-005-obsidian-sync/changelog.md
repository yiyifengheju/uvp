# FEAT-005 Changelog

| 日期 | 变更 | 说明 |
|------|------|------|
| 2026-06-16 | 创建 | 初始化特性 |
| 2026-06-19 | 重写 spec | 从"双向文档同步"改为"单向知识导入"：删除 push，sync 仅限 reference/ |
| 2026-06-19 | 删除 push | 移除 push 子命令及 obsidian_push 函数 |
| 2026-06-19 | 简化 sync | sync 改为仅 reference/ 双向同步，不再推送 docs/ |
| 2026-06-19 | 清理配置 | 删除 include_reference / direction 死字段；更新 exclude_dirs 默认值 |
| 2026-06-19 | 补全文档 | 创建 changelog / context / verification / plan |
