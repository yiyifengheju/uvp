# FEAT-003 Changelog

| 日期 | 变更 | 说明 |
|------|------|------|
| 2026-06-16 | 创建 | 初始化特性 |
| 2026-06-19 | 移除多模板支持 | 删除 --template CLI 参数、nygard/alexandrian 模板文件、adr.template 配置字段；简化 load_adr_template() 只支持默认模板 |
| 2026-06-19 | 修复错误处理 | fs::write 错误不再被静默吞掉，写入失败时显示错误信息并终止 |
| 2026-06-19 | 更新 spec | 移除多模板相关描述，与实现对齐 |
| 2026-06-19 | 补全文档 | 创建 changelog / context / verification / plan |
