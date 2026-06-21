# FEAT-012 Plan

## 实施步骤

1. 实现 Spinner API（step_start / step_update / step_done / step_skip / step_fail）
2. 实现图标函数（icon_ok / icon_fail）
3. 实现简单输出 API（file_created / file_exists / action_ok / action_fail / action_skip / action_info）
4. 实现面板组件（success_panel / info_panel）
5. 实现后台命令执行（spawn_command_streaming）
6. 实现延迟控制（get_delay_ms）

## 依赖

无外部特性依赖。被其他所有命令特性使用。
