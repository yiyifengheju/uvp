//! uvp check 命令 — 文档一致性检查

use std::fs;
use std::path::Path;

use regex::Regex;

use crate::common;
use crate::config;
use crate::ui;

pub fn run(check_features: bool, check_adr: bool, _auto_fix: bool) {
    let project_dir = config::find_project_root(None, true).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });
    let cfg = config::get_effective_config(&project_dir);

    let check_all = !check_features && !check_adr;

    ui::step_header(0, "文档一致性检查", 0);
    println!();

    let mut all_issues: Vec<String> = Vec::new();

    if check_all || check_features {
        println!("{}1. 特性闭环检查", console::style("  ").bold());
        let issues = check_feature_closure(&project_dir, &cfg);
        all_issues.extend(issues.iter().cloned());
        if issues.is_empty() {
            ui::action_ok("所有特性闭环正常");
        } else {
            for issue in &issues {
                ui::action_fail(issue);
            }
        }
    }

    if check_all || check_adr {
        println!("\n{}2. ADR 一致性检查", console::style("  ").bold());
        let issues = check_adr_consistency(&project_dir, &cfg);
        all_issues.extend(issues.iter().cloned());
        if issues.is_empty() {
            ui::action_ok("ADR 一致性正常");
        } else {
            for issue in &issues {
                ui::action_fail(issue);
            }
        }
    }

    if check_all {
        println!("\n{}3. AI 上下文检查", console::style("  ").bold());
        let issues = check_ai_context(&project_dir);
        all_issues.extend(issues.iter().cloned());
        if issues.is_empty() {
            ui::action_ok("AI 上下文正常");
        } else {
            for issue in &issues {
                ui::action_fail(issue);
            }
        }
    }

    println!("\n{}", "═".repeat(40));
    if all_issues.is_empty() {
        println!("{} 所有检查通过！", ui::icon_ok());
    } else {
        println!("{} 发现 {} 个问题", ui::icon_fail(), all_issues.len());
        ui::action_info("运行 uvp render 更新渲染页面");
    }
}

fn check_feature_closure(project_dir: &Path, cfg: &config::UvpConfig) -> Vec<String> {
    let mut issues = Vec::new();
    let data = common::load_feature_registry(project_dir, cfg);

    for feat in &data.features {
        let feat_dir = project_dir.join(&feat.directory);

        if !feat_dir.exists() {
            issues.push(format!("{}: 目录不存在 ({})", feat.id, feat.directory));
            continue;
        }

        if !feat_dir.join("spec.md").exists() {
            issues.push(format!("{}: 缺少 spec.md", feat.id));
        }

        if feat.status == "verified" {
            let verif_path = feat_dir.join("verification.md");
            if verif_path.exists() {
                if let Ok(content) = fs::read_to_string(&verif_path) {
                    if !content.contains("已验证") {
                        issues.push(format!("{}: 状态为 verified 但 verification.md 未标记为已验证", feat.id));
                    }
                }
            }
        }

        if !common::ALL_STATUSES.contains(&feat.status.as_str()) {
            issues.push(format!("{}: 非法状态 '{}'", feat.id, feat.status));
        }
    }

    issues
}

fn check_adr_consistency(project_dir: &Path, cfg: &config::UvpConfig) -> Vec<String> {
    let mut issues = Vec::new();
    let adr_dir = project_dir.join(&cfg.adr.directory);

    if !adr_dir.exists() {
        issues.push("ADR 目录不存在".into());
        return issues;
    }

    if !adr_dir.join("registry.md").exists() {
        issues.push("docs/adr/registry.md 不存在".into());
    }

    if let Ok(entries) = fs::read_dir(&adr_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.ends_with(".md") || name == "template.md" || name == "registry.md" {
                continue;
            }
            let content = match fs::read_to_string(entry.path()) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let status = common::parse_adr_status(&content);
            if !["proposed", "accepted", "superseded", "deprecated"].contains(&status.as_str()) {
                issues.push(format!("{name}: 非法状态 '{status}'"));
            }

            let meta = parse_front_matter(&content);
            if let Some(meta_status) = meta.get("status") {
                if meta_status != &status {
                    issues.push(format!("{name}: front matter 状态 '{meta_status}' 与正文状态 '{status}' 不一致"));
                }
            }

            if status == "superseded" {
                if !content.contains("替代") && !content.to_lowercase().contains("superseded by") && !content.contains("被替代") {
                    issues.push(format!("{name}: 状态为 superseded 但未引用替代 ADR"));
                }
            }
        }
    }

    issues
}

fn check_ai_context(project_dir: &Path) -> Vec<String> {
    let mut issues = Vec::new();

    if !project_dir.join("docs/AI_CONTEXT.md").exists() {
        issues.push("docs/AI_CONTEXT.md 不存在".into());
    }

    if !project_dir.join("docs/PROJECT_STATE.md").exists() {
        issues.push("docs/PROJECT_STATE.md 不存在".into());
    }

    issues
}

fn parse_front_matter(content: &str) -> std::collections::HashMap<String, String> {
    let mut result = std::collections::HashMap::new();
    if !content.starts_with("---") {
        return result;
    }
    let rest = &content[3..];
    let end = match rest.find("---") {
        Some(i) => i,
        None => return result,
    };
    let fm = &rest[..end];

    let re = Regex::new(r"^(\w+):\s*(.+)$").unwrap();
    for line in fm.lines() {
        if let Some(caps) = re.captures(line) {
            let key = caps[1].to_string();
            let value = caps[2].trim().trim_matches('"').trim_matches('\'').to_string();
            result.insert(key, value);
        }
    }

    result
}
