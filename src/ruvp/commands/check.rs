//! uvp check 命令 — 文档一致性检查

use std::fs;
use std::path::Path;

use regex::Regex;

use crate::common;
use crate::config;
use crate::ui;

pub fn run(check_features: bool, check_adr: bool, auto_fix: bool) {
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
        ui::section_header(1, "特性闭环检查");
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
        println!();
        ui::section_header(2, "ADR 一致性检查");
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
        println!();
        ui::section_header(3, "AI 上下文检查");
        let issues = check_ai_context(&project_dir);
        all_issues.extend(issues.iter().cloned());
        if issues.is_empty() {
            ui::action_ok("AI 上下文正常");
        } else {
            for issue in &issues {
                ui::action_fail(issue);
            }
        }

        println!();
        ui::section_header(4, "PROJECT_STATE 特性表一致性");
        let issues = check_project_state_features(&project_dir, &cfg);
        all_issues.extend(issues.iter().cloned());
        if issues.is_empty() {
            ui::action_ok("PROJECT_STATE 特性表一致");
        } else {
            for issue in &issues {
                ui::action_fail(issue);
            }
        }

        println!();
        ui::section_header(5, "文档 Meta Header 检查");
        let issues = check_docs_meta_header(&project_dir);
        all_issues.extend(issues.iter().cloned());
        if issues.is_empty() {
            ui::action_ok("所有文档均含 front matter");
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
        if auto_fix {
            println!("\n{} 自动修复中...", ui::styled_cyan("→"));
            super::render::run(true, true, false);
            println!("{} 已重新渲染 registry 页面", ui::icon_ok());
        } else {
            ui::action_info("运行 uvp check --fix 自动修复，或 uvp render 手动更新");
        }
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

        if feat.status == "verified" || feat.status == "closed" {
            let verif_path = feat_dir.join("verification.md");
            if !verif_path.exists() {
                issues.push(format!("{}: 状态为 {} 但缺少 verification.md", feat.id, feat.status));
            } else if let Ok(content) = fs::read_to_string(&verif_path) {
                if content.contains("未验证") && !content.contains("已验证") {
                    issues.push(format!("{}: 状态为 {} 但 verification.md 未标记为已验证", feat.id, feat.status));
                }

                // P5-12: 算法类 Feature 检查量化证据
                let spec_path = feat_dir.join("spec.md");
                let is_algo_feature = spec_path.exists()
                    && fs::read_to_string(&spec_path).map(|s| {
                        let lower = s.to_lowercase();
                        lower.contains("精度") || lower.contains("准确率")
                            || lower.contains("accuracy") || lower.contains("recall")
                            || lower.contains("sensitivity") || lower.contains("ppv")
                            || lower.contains("f1") || lower.contains("se ")
                            || lower.contains("信号") || lower.contains("波形")
                            || lower.contains("分类") || lower.contains("检测")
                    }).unwrap_or(false);

                if is_algo_feature {
                    let has_metrics = content.contains("metrics:") && !content.contains("metrics: {}");
                    if !has_metrics {
                        issues.push(format!("{}: 算法类特性但 verification.md 缺少量化指标（metrics 为空）", feat.id));
                    }
                }
            }

            let context_path = feat_dir.join("context.md");
            if !context_path.exists() {
                issues.push(format!("{}: 状态为 {} 但缺少 context.md", feat.id, feat.status));
            } else if let Ok(content) = fs::read_to_string(&context_path) {
                let is_template = content.contains("<!-- 为什么这样选")
                    && !content.lines().any(|l| {
                        let trimmed = l.trim();
                        !trimmed.is_empty()
                            && !trimmed.starts_with('#')
                            && !trimmed.starts_with('>')
                            && !trimmed.starts_with("<!--")
                            && !trimmed.starts_with("---")
                            && !trimmed.starts_with("planned")
                            && !trimmed.starts_with("doc_type")
                            && !trimmed.starts_with("title:")
                            && !trimmed.starts_with("date:")
                            && !trimmed.starts_with("feat_id:")
                            && !trimmed.starts_with("updated:")
                            && !trimmed.contains("Distill")
                            && !trimmed.contains("压缩上下文")
                            && !trimmed.contains("竣工")
                    });
                if is_template {
                    issues.push(format!("{}: 状态为 {} 但 context.md 仍为空模板", feat.id, feat.status));
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

    if !adr_dir.join("index.md").exists() {
        issues.push("docs/adr/index.md 不存在".into());
    }

    if let Ok(entries) = fs::read_dir(&adr_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.ends_with(".md") || name == "template.md" || name == "index.md" {
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

fn check_project_state_features(project_dir: &Path, cfg: &config::UvpConfig) -> Vec<String> {
    let mut issues = Vec::new();
    let ps_path = project_dir.join("docs/PROJECT_STATE.md");

    if !ps_path.exists() {
        issues.push("docs/PROJECT_STATE.md 不存在".into());
        return issues;
    }

    let content = match fs::read_to_string(&ps_path) {
        Ok(c) => c,
        Err(_) => {
            issues.push("无法读取 docs/PROJECT_STATE.md".into());
            return issues;
        }
    };

    let data = common::load_feature_registry(project_dir, cfg);

    for feat in &data.features {
        if feat.status == "deprecated" || feat.status == "removed" {
            continue;
        }
        if !content.contains(&feat.id) {
            issues.push(format!("PROJECT_STATE.md 缺少特性 {} ({})", feat.id, feat.title));
        }
    }

    issues
}

fn check_docs_meta_header(project_dir: &Path) -> Vec<String> {
    let mut issues = Vec::new();
    let docs_dir = project_dir.join("docs");

    if !docs_dir.exists() {
        return issues;
    }

    fn walk_md(dir: &Path, project_dir: &Path, issues: &mut Vec<String>) {
        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk_md(&path, project_dir, issues);
            } else if path.extension().is_some_and(|e| e == "md") {
                let content = match fs::read_to_string(&path) {
                    Ok(c) => c,
                    Err(_) => continue,
                };
                if !content.starts_with("---") {
                    let rel = path.strip_prefix(project_dir).unwrap_or(&path);
                    issues.push(format!("{}: 缺少 YAML front matter", rel.to_string_lossy().replace('\\', "/")));
                } else {
                    let rest = &content[3..];
                    if let Some(end) = rest.find("---") {
                        let fm = &rest[..end];
                        let has_title = fm.lines().any(|l| l.starts_with("title:") || l.starts_with("title :"));
                        let has_date = fm.lines().any(|l| l.starts_with("date:") || l.starts_with("date :"));
                        if !has_title || !has_date {
                            let rel = path.strip_prefix(project_dir).unwrap_or(&path);
                            let missing: Vec<&str> = [
                                if !has_title { Some("title") } else { None },
                                if !has_date { Some("date") } else { None },
                            ].iter().filter_map(|x| *x).collect();
                            issues.push(format!("{}: front matter 缺少 {}", rel.to_string_lossy().replace('\\', "/"), missing.join(", ")));
                        }
                    }
                }
            }
        }
    }

    walk_md(&docs_dir, project_dir, &mut issues);
    issues
}
