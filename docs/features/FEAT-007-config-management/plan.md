# FEAT-007 Plan

## 实施步骤

1. 去掉 ConfigCommands 子命令 enum（show/get/set/init/path）
2. Config 命令不再持有 subcommand，直接执行
3. config_cmd::run() 简化为：加载合并配置 → toml 序列化 → 输出
4. 删除 set_config_value 等写入函数（如无其他调用方）

## 依赖

无
