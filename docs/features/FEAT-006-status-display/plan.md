# FEAT-006 Plan

## 实施步骤

### Phase 1: 接口调整
1. 移除 `--verbose` 标志（原来控制是否展示工作流）
2. 默认模式只展示项目状态
3. 在 `--help` 的 after_help 中添加工作流编排提示
4. 保留 `--open` 生成 HTML 报告

### Phase 2: --onboard（在线面板）
1. 新增 `--onboard` / `--port` / `--no-open` 参数
2. 实现内嵌 HTTP 服务（tiny_http 或 axum）
3. 生成带自动刷新的 HTML 面板
4. 端口冲突自动递增
5. Ctrl+C 优雅关闭

### Phase 3: 面板增强
1. 产出时间线（聚合 deliverables.md）
2. 工作流 Mermaid 可视化
3. WebSocket 实时推送

## 依赖

- FEAT-001: Feature Ledger（registry 数据源）
- FEAT-004: Feature 生命周期（deliverables.md 数据源）
