# FEAT-014 Plan

## 实施步骤

### Phase 1: 基础命令
1. 定义 CLI（Todo 子命令：默认 list / add / done / remove + --all）
2. 实现 TODO.md 解析器（提取 ID、内容、日期、完成状态）
3. 实现 `uvp todo`（列出待办）
4. 实现 `uvp todo add`（追加条目 + ID 分配）
5. 实现 `uvp todo done`（标记完成 + 移动到已完成区）
6. 实现 `uvp todo remove`（删除条目）

### Phase 2: init 集成
7. 在 `init.rs` Phase 1 辅助文件步骤中添加 TODO.md 模板创建
8. 模板内容：标题 + 空的"待办"/"已完成"段落

### Phase 3: onboard 集成
9. 在 FEAT-006 --onboard 面板中展示 TODO 列表
10. 支持在线添加/完成/删除操作

## 依赖

- FEAT-002: 项目初始化（init 集成）
- FEAT-006: 状态展示（onboard 面板集成）
