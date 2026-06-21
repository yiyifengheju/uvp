# FEAT-006 Sub-Spec: 在线状态面板 (--onboard)

## 概述

`uvp status --onboard` 启动一个本地 Web 服务，提供实时项目状态面板。适用于团队协作、演示、持续监控等场景。面板自动刷新，展示项目的实时健康状态。

## 接口定义

### CLI

```bash
uvp status --onboard [OPTIONS]

Options:
  --port <PORT>      监听端口 [default: 8765]
  --no-open          不自动打开浏览器
```

### 启动行为

1. 启动本地 HTTP 服务（默认 `localhost:8765`）
2. 自动在浏览器中打开面板页面（除非 `--no-open`）
3. 终端显示访问 URL 和停止提示（Ctrl+C）
4. 监听文件变化，面板自动刷新

## 面板内容

### 概览区

- 项目名称 + 当前时间
- 状态概览卡片：
  - Feature 总数 / 活跃数 / 已验证数
  - ADR 总数 / 各状态分布
  - 最近更新时间

### Feature 进度区

- Feature 列表（表格/看板视图切换）
- 每个 Feature 显示：ID、标题、状态、进度条（基于验收标准完成比例）
- 点击展开查看 spec 摘要

### 产出时间线

- 从各 Feature 的 `deliverables.md` 聚合
- 按时间倒序显示最近产出
- 每条显示：日期、Feature、类型、关键结果

### 工作流引导

- 6 步闭环流程可视化（交互式，点击跳转文档）
- 当前活跃 Feature 在流程中的位置标注

### TODO 管理区

- 读取 `docs/TODO.md`，展示待办列表
- 支持在线操作：
  - 添加新 TODO（输入框 + 提交）
  - 标记完成（点击 checkbox）
  - 删除条目（删除按钮）
- 操作直接写入 `docs/TODO.md`，与 `uvp todo` CLI 数据一致
- 已完成条目折叠显示，点击展开

## 技术方案

### 方案选择

| 方案 | 优点 | 缺点 |
|------|------|------|
| 内嵌 HTTP 服务 + 静态 HTML | 零依赖、快速启动 | 功能有限 |
| 内嵌 HTTP 服务 + WebSocket | 实时刷新 | 实现复杂 |
| 生成静态 HTML + 轮询刷新 | 简单可靠 | 非实时 |

建议：先用"内嵌 HTTP 服务 + 静态 HTML + 自动刷新 meta 标签"实现 MVP，后续按需加 WebSocket。

### 数据源

| 数据 | 来源 | 刷新方式 |
|------|------|----------|
| Feature 列表 | `feature-registry.yaml` | 页面刷新时重新读取 |
| ADR 统计 | 扫描 `docs/adr/` | 页面刷新时重新读取 |
| 产出时间线 | 各 Feature 的 `deliverables.md` | 页面刷新时重新读取 |
| TODO 列表 | `docs/TODO.md` | 页面刷新时重新读取 |
| Git 信息 | `git` 命令 | 页面刷新时重新执行 |

## 约束

- 仅本地访问（localhost），不暴露到网络
- 端口冲突时自动递增尝试（最多 +10）
- Ctrl+C 优雅关闭服务
- 静态资源内嵌到二进制，无外部文件依赖

## 验收标准

- [ ] `uvp status --onboard` 启动本地服务并打开浏览器
- [ ] 面板正确展示 Feature 列表和状态
- [ ] 面板正确展示 ADR 统计
- [ ] 面板正确展示产出时间线（从 deliverables.md）
- [ ] 面板正确展示 TODO 列表（从 docs/TODO.md）
- [ ] TODO 在线添加/完成/删除操作正常
- [ ] 自动刷新工作正常
- [ ] Ctrl+C 优雅关闭
- [ ] `--port` 正确指定端口
- [ ] `--no-open` 不自动打开浏览器

## 实施阶段

### Phase 1（MVP）
- 内嵌 HTTP 服务 + 静态 HTML
- Feature 表格 + ADR 统计 + Git 信息
- TODO 列表展示
- meta 标签自动刷新（30 秒）

### Phase 2
- 产出时间线（聚合 deliverables.md）
- 工作流可视化（Mermaid 渲染）
- TODO 在线管理（添加/完成/删除）

### Phase 3
- WebSocket 实时推送
- 看板视图切换
- 进度条计算
