# FEAT-002 Changelog

| 日期 | 变更 | 说明 |
|------|------|------|
| 2026-06-16 | 创建 | 初始化特性 |
| 2026-06-19 | 补全文档 | 创建 changelog / context / verification / plan，完善特性目录结构 |
| 2026-06-19 | 修复幂等性 | 所有项目文件使用 write_file_safe()，已存在则跳过不覆盖 |
| 2026-06-19 | 修复错误处理 | fs::write().ok() 替换为显式错误检查，权限错误终止执行并显示错误信息 |
| 2026-06-19 | 修复 auto_feature_ledger | Feature Registry 创建受 cfg.init.auto_feature_ledger 配置控制 |
