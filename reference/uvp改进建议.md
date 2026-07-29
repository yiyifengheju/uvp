---
doc_type: reference
title: "uvp 工具改进建议"
date: "2026-07-28"
---

# uvp 工具改进建议

> 来源：ADR-0009 Phase 1~3（FEAT-013/014/015）实践中踩到的问题。仅为改进备忘，供后续修改 uvp 工具与 skill 参考，不代表已实施。

## 一、Bug（应优先修）

### 1. `uvp f new` 会 panic（阻断性）
- **现象**：执行 `uvp f new "标题" --adr 0009` 时，CLI 在写入 `feature-registry.yaml` **之后**、创建目录/更新 AI_CONTEXT **之前** panic。
- **报错**：`commands/feature.rs:424` — 对 AI_CONTEXT「活跃列表」HTML 注释用了含 look-around 的正则（`(?s)<!-- ... -->.*?(?=...)`），Rust `regex` crate **不支持 look-around**。
- **后果**：registry 写了一半，目录和 AI_CONTEXT 未更新，需手动补齐目录模板文件、手动往 AI_CONTEXT 加行。状态不一致。
- **建议**：改用非 look-around 写法（如捕获组或两段切分），或用 `fancy-regex`。修复后确保 new 的多步操作**原子**：要么全成功，要么回滚 registry。

### 2. `--adr-ref` 参数名不一致
- `docs/AI_CONTEXT.md` 的快速参考写的是 `uvp f new "标题" --adr-ref NNNN`，实际 CLI 只接受 `--adr`。
- **建议**：统一为其一（或同时接受两个别名），并让文档与 `--help` 一致。

### 3. `related_adr` 未写入
- `uvp f new --adr 0009` 创建的 spec.md，front matter `related_adr` 仍是 `null`（可能因 panic 半途中止，也可能本就没接线）。
- **建议**：确认 `--adr` 会正确回填 spec 的 `related_adr` 与 ADR 的 `related_features`（双向），并纳入 new 的原子操作。

### 4. `uvp f close` 语义 = verified，需两步才到 closed
- `close` 只把状态推到 `verified`；要 `closed` 得再 `uvp f status FEAT-XXX closed`。
- `uvp f close --help` 描述即「标记为 verified」，与直觉（close→closed）不符。
- **建议**：要么 `close` 直接到 `closed`，要么增加 `--to verified|closed` 显式控制；至少让命名与行为对齐。

## 二、文档模板边界（skill 层面）

实践中发现 `spec.md` / `plan.md` / `context.md` 边界模糊，同一批内容（背景、决策、步骤）在三处重复。建议在 `uvp-feature-lifecycle` skill 里明确各文件的**时间点 + 回答的问题 + 是否冻结**：

| 文件 | 何时写 | 回答 | 关闭后 |
|------|--------|------|--------|
| **spec** | 动手前 | 做什么 + 怎样算完成（验收标准、接口契约、约束） | 冻结，作验收基准 |
| **plan** | 动手前/中 | 怎么做（步骤、风险、依赖）——过程脚手架 | 用完即弃，可省略/极短 |
| **context** | 关闭时 | 学到了什么（为何这样选、意外发现、最短上手） | 长期存活，给未来读者 |

一句话：**spec 是合同，plan 是施工路线，context 是竣工经验笔记。**

具体建议：
1. **plan.md 明确「可选」并鼓励留空**。ADR 已列 Action 的特性，plan 基本冗余；模板可只留一句「简单特性可指向 ADR Action，不必填满」，避免关闭时形式主义硬补。
2. **spec 收窄为契约**：去掉大段「背景与约定」，背景只留一两句约束。进行中的决策写在 ADR 执行日志（决策发生地），关闭时蒸馏进 context。
3. **决策类内容单一归属**：避免 spec「背景」与 context「关键决策」讲同一批取舍。以 context 为决策的最终落点。

## 三、流程/一致性小改进

1. **状态同步一处漏更**：关闭特性时，`feature-registry.yaml` 与 `AI_CONTEXT.md` 由 CLI 维护，但 `docs/PROJECT_STATE.md` 的特性表与「源码结构」块需手动改，易漏、易过期（本项目 PROJECT_STATE 的源码树一度还停留在旧 `src/ecg/` 结构）。建议 `uvp render` 一并校验/更新 PROJECT_STATE，或至少 `uvp check` 报告不一致。
2. **`uvp check --features` 校验闭环完整性**：关闭前检查 verification.md 是否已填、spec 验收框是否全勾、context 是否非模板，缺失则告警，防止「状态 closed 但文档仍是空模板」。
3. **meta 头自动补全**：`docs/` 下新建 md 若缺 `title`/`date` front matter，`uvp` 可自动补或告警（呼应 uvp-meta-header）。
