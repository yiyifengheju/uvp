# FEAT-006 Verification

## 验证状态

⚠️ 部分通过（--onboard 待实现）

## 验收标准

- [x] 默认输出项目状态（无工作流）
- [x] `--help` 输出工作流编排
- [x] `--open` 生成并打开 HTML 报告
- [x] 正确统计 ADR 和 Feature 数量
- [x] Git 信息正确显示
- [ ] `--onboard` 启动在线面板（当前为 coming soon 占位）

## 验证结果

| 检查项 | 结果 | 说明 |
|--------|------|------|
| 默认输出 | ✅ | 终端输出项目状态，不含工作流 |
| --help | ✅ | after_help 包含 6 步闭环工作流文本 |
| --open | ✅ | 生成 .uvp/status.html 并浏览器打开 |
| --onboard | 🔶 | 输出 coming soon 提示，待后续实现 |
| ADR/Feature 统计 | ✅ | 正确扫描和统计 |
| Git 信息 | ✅ | 分支 + 最近提交 |
