//! uvp config 命令 — 显示合并后的完整配置

use crate::config;

pub fn run() {
    let project_dir = match config::find_project_root(None, false) {
        Ok(p) => p,
        Err(_) => std::env::current_dir().unwrap_or_default(),
    };
    let cfg = config::get_effective_config(&project_dir);
    let config_str = toml::to_string_pretty(&cfg).unwrap_or_default();
    println!("{config_str}");
}
