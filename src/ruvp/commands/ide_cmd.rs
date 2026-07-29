//! uvp ide 命令 — 生成 AI 工具规则文件并部署 skills

use std::fs;

use crate::config;
use crate::ui;

pub fn run(tool: &str) {
    let project_dir = config::find_project_root(None, true).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    let rule_files = config::ai_rule_files();

    // 1. 查找源规则文件
    let mut source_path = None;
    let mut source_content = None;
    for (_, rule_file) in &rule_files {
        let candidate = project_dir.join(rule_file);
        if candidate.exists() {
            if let Ok(content) = fs::read_to_string(&candidate) {
                source_path = Some(candidate);
                source_content = Some(content);
                break;
            }
        }
    }

    let rule_content = match source_content {
        Some(c) => c,
        None => {
            ui::action_fail("未找到任何 AI 规则文件，请先运行 uvp init");
            return;
        }
    };

    // 2. 生成目标规则文件
    ui::step_header(1, &format!("生成规则文件 → {tool}"), 2);
    let rule_path = match rule_files.get(tool) {
        Some(p) => p,
        None => {
            ui::action_fail(&format!("不支持的 IDE: {tool}"));
            return;
        }
    };

    let target = project_dir.join(rule_path);
    if target.exists() {
        ui::action_info(&format!("{rule_path} 已存在，将被覆盖"));
    }

    if let Some(parent) = target.parent() {
        let _ = fs::create_dir_all(parent);
    }
    match fs::write(&target, &rule_content) {
        Ok(_) => {
            let source_name = source_path
                .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
                .unwrap_or_default();
            ui::action_ok(&format!("{rule_path} ← {source_name}"));
        }
        Err(e) => ui::action_fail(&format!("写入 {rule_path} 失败: {e}")),
    }

    // 3. 部署 skills
    ui::step_header(2, "部署 Skills", 2);
    let deployed = config::deploy_skills_to_ide(tool, Some(&project_dir));
    let skills_paths = config::ide_skills_paths(tool);

    if !deployed.is_empty() {
        if let Some(ref sp) = skills_paths {
            if sp.project.is_some() {
                ui::action_ok(&format!("Skills → .claude/skills/ ({} 文件)", deployed.len()));
            }
            if sp.global.is_some() {
                ui::action_info(&format!("全局 Skills → {}/", sp.global.as_ref().unwrap().display()));
            }
        }
    } else if skills_paths.as_ref().map(|sp| sp.project.is_none()).unwrap_or(true) {
        ui::action_skip(&format!("{tool} 暂不支持自动部署 skills，请手动配置"));
    } else {
        ui::action_skip("Skills 已是最新（无需更新）");
    }

    println!();
    ui::success_panel(
        &format!("IDE 配置完成: {tool}"),
        &format!("规则文件: {}", ui::styled_cyan(rule_path)),
    );
}
