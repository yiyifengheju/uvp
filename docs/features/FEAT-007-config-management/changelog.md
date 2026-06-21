# FEAT-007 Changelog

| 日期 | 变更 | 说明 |
|------|------|------|
| 2026-06-16 | 创建 | 初始化特性 |
| 2026-06-19 | 重写 spec | 去掉子命令（show/get/set/init/path），简化为 `uvp config` 直接输出合并配置 |
| 2026-06-19 | 补全文档 | 创建 changelog / context / verification / plan |
| 2026-06-19 | 实现 spec | 去掉 ConfigCommands 子命令；config_cmd::run() 直接输出合并配置；删除 get_config_value / set_config_value |
