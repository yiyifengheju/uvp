# FEAT-003 Plan

## 实施步骤

1. 定义 CLI 参数（title / --from-obsidian / --status / --open）
2. 实现模板加载（项目模板优先，内置默认兜底）
3. 实现编号逻辑（扫描 ADR 目录文件名）
4. 实现模板变量替换（TITLE / NUMBER / DATE / status）
5. 实现 Obsidian 笔记导入（模糊匹配 + 最短名策略）
6. 实现 registry.md 全量重新生成
7. 实现 --open 编辑器打开

## 依赖

- FEAT-005: Obsidian 同步（--from-obsidian 依赖 Vault 配置）
