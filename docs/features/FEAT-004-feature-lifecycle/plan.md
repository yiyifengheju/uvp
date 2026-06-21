# FEAT-004 Plan

## 实施步骤

1. 定义 CLI 子命令（new / list / show / status / close / archive）
2. 实现 FeatureRegistry 数据结构和 YAML 读写
3. 实现 feature_new：创建目录 + 6 个模板文件 + 注册 registry
4. 实现 feature_list：读取 registry 显示表格
5. 实现 feature_show：显示详情 + spec 摘要
6. 实现 feature_status：状态变更 + 自动同步关联文件
7. 实现 close / archive 快捷命令
8. 实现 update_ai_context_features 自动维护活跃列表

## 依赖

- FEAT-001: Feature Ledger 系统（目录结构和 registry 规范）
