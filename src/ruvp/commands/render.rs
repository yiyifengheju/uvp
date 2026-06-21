//! uvp render 命令 — 将 Registry 渲染为 mkdocs 页面

use std::fs;
use std::path::Path;

use chrono::Local;
use regex::Regex;

use crate::common;
use crate::config;
use crate::ui;

pub fn run(render_features: bool, render_adr: bool, check_only: bool) {
    let project_dir = config::find_project_root(None, true).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });
    let cfg = config::get_effective_config(&project_dir);

    let render_all = !render_features && !render_adr;

    if check_only {
        ui::step_header(0, "检查渲染一致性", 0);
    } else {
        ui::step_header(0, "渲染 Registry 页面", 0);
    }

    let mut results = Vec::new();

    if render_all || render_features {
        results.push(render_features_index(&project_dir, &cfg, check_only));
    }

    if render_all || render_adr {
        results.push(render_adr_registry(&project_dir, &cfg, check_only));
    }

    if check_only {
        if results.iter().all(|r| *r) {
            println!();
            ui::action_ok("所有渲染页面与数据源一致");
        } else {
            println!();
            ui::action_fail("部分渲染页面与数据源不一致，请运行 uvp render 更新");
            std::process::exit(1);
        }
    } else {
        println!();
        ui::action_ok("渲染完成");
        ui::action_info("运行 mkdocs serve 在浏览器中查看");
    }
}

fn render_features_index(project_dir: &Path, cfg: &config::UvpConfig, check_only: bool) -> bool {
    let data = common::load_feature_registry(project_dir, cfg);
    let features = &data.features;

    let mut status_count: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
    for feat in features {
        *status_count.entry(feat.status.clone()).or_insert(0) += 1;
    }

    let today = Local::now().format("%Y-%m-%d").to_string();

    let mut lines = vec![
        "---".to_string(),
        "doc_type: feature-index".to_string(),
        "title: \"Feature Registry\"".to_string(),
        format!("date: {today}"),
        "auto_generated: true".to_string(),
        "source: feature-registry.yaml".to_string(),
        "---".to_string(),
        String::new(),
        "<!-- 此文件由 uvp render 自动生成，不应手动编辑 -->\n".to_string(),
        "# Feature Registry\n".to_string(),
        "## 状态概览\n".to_string(),
        "| 状态 | 数量 |".to_string(),
        "|------|------|".to_string(),
    ];

    for s in ["planned", "in_progress", "implemented", "verified"] {
        if let Some(count) = status_count.get(s) {
            lines.push(format!("| {s} | {count} |"));
        }
    }
    if status_count.is_empty() {
        lines.push("| (无) | 0 |".to_string());
    }

    lines.push("\n## 全部特性\n".to_string());
    lines.push("| 编号 | 标题 | 状态 | 关联 ADR | 创建日期 | 更新日期 |".to_string());
    lines.push("|------|------|------|----------|----------|----------|".to_string());

    if features.is_empty() {
        lines.push("| （暂无） | | | | | |".to_string());
    } else {
        for feat in features {
            let emoji = common::status_emoji(&feat.status);
            lines.push(format!(
                "| {} | {} | {} {} | {} | {} | {} |",
                feat.id, feat.title, emoji, feat.status, feat.adr, feat.created, feat.updated
            ));
        }
    }

    let expected_content = lines.join("\n") + "\n";

    let index_path = project_dir.join("docs/features/index.md");
    if check_only {
        if index_path.exists() {
            match fs::read_to_string(&index_path) {
                Ok(current) if current == expected_content => {
                    ui::action_ok("features/index.md 一致");
                    return true;
                }
                _ => {
                    ui::action_fail("features/index.md 不一致");
                    return false;
                }
            }
        } else {
            ui::action_fail("features/index.md 不存在");
            return false;
        }
    } else {
        if let Some(parent) = index_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        match fs::write(&index_path, &expected_content) {
            Ok(_) => {
                ui::action_ok("已渲染: docs/features/index.md");
                return true;
            }
            Err(e) => {
                ui::action_fail(&format!("写入失败: {e}"));
                return false;
            }
        }
    }
}

fn render_adr_registry(project_dir: &Path, cfg: &config::UvpConfig, check_only: bool) -> bool {
    let adr_dir = project_dir.join(&cfg.adr.directory);
    if !adr_dir.exists() {
        ui::action_skip("ADR 渲染（目录不存在）");
        return true;
    }

    let mut entries: Vec<(String, String, String, String, String, String)> = Vec::new();

    if let Ok(files) = fs::read_dir(&adr_dir) {
        let mut file_list: Vec<_> = files.flatten().collect();
        file_list.sort_by_key(|f| f.file_name());

        for f in &file_list {
            let name = f.file_name().to_string_lossy().to_string();
            if !name.ends_with(".md") || name == "template.md" || name == "registry.md" {
                continue;
            }
            let content = match fs::read_to_string(f.path()) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let re_num = Regex::new(r"^(\d{4})-").unwrap();
            let number = re_num.captures(&name).map(|c| c[1].to_string()).unwrap_or_else(|| name.trim_end_matches(".md").to_string());

            let re_title = Regex::new(r"(?m)^#\s+.+").unwrap();
            let title = re_title.captures(&content)
                .map(|c| c[0].trim_start_matches('#').trim().to_string())
                .unwrap_or_else(|| name.trim_end_matches(".md").to_string());

            let status = common::parse_adr_status(&content);

            let re_feat = Regex::new(r"(?i)关联\s*Feature[：:]*\s*(.+)").unwrap();
            let feat_ref = re_feat.captures(&content)
                .map(|c| c[1].trim().to_string())
                .unwrap_or_else(|| "-".into());

            let date_str = {
                let ts_re = Regex::new(r"^(\d{4})(\d{2})(\d{2})-(\d{2})(\d{2})\.md$").unwrap();
                if let Some(caps) = ts_re.captures(&name) {
                    format!("{}-{}-{}", &caps[1], &caps[2], &caps[3])
                } else {
                    f.metadata().ok()
                        .and_then(|m| m.modified().ok())
                        .map(|t| {
                            let dt: chrono::DateTime<Local> = t.into();
                            dt.format("%Y-%m-%d").to_string()
                        })
                        .unwrap_or_default()
                }
            };

            entries.push((number, name, title, status, feat_ref, date_str));
        }
    }

    let mut status_count: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
    for (_, _, _, status, _, _) in &entries {
        *status_count.entry(status.clone()).or_insert(0) += 1;
    }

    let today = Local::now().format("%Y-%m-%d").to_string();

    let mut lines = vec![
        "---".to_string(),
        "doc_type: adr-index".to_string(),
        "title: \"ADR Registry\"".to_string(),
        format!("date: {today}"),
        "auto_generated: true".to_string(),
        "source: adr-directory-scan".to_string(),
        "---".to_string(),
        String::new(),
        "# ADR Registry\n".to_string(),
        "## 状态概览\n".to_string(),
        "| 状态 | 数量 |".to_string(),
        "|------|------|".to_string(),
    ];

    for s in ["proposed", "accepted", "superseded", "deprecated"] {
        if let Some(count) = status_count.get(s) {
            lines.push(format!("| {s} | {count} |"));
        }
    }
    if status_count.is_empty() {
        lines.push("| (无) | 0 |".to_string());
    }

    lines.push("\n## 全部决策\n".to_string());
    lines.push("| 编号 | 标题 | 状态 | 关联 Feature | 日期 |".to_string());
    lines.push("|------|------|------|-------------|------|".to_string());

    let status_emoji = |s: &str| -> &str {
        match s { "proposed" => "📝", "accepted" => "✅", "superseded" => "⏳", "deprecated" => "❌", _ => "❓" }
    };

    if entries.is_empty() {
        lines.push("| （暂无） | | | | |".to_string());
    } else {
        for (number, filename, title, status, feat_ref, date_str) in &entries {
            let emoji = status_emoji(status);
            lines.push(format!("| [{number}]({filename}) | {title} | {emoji} {status} | {feat_ref} | {date_str} |"));
        }
    }

    let expected_content = lines.join("\n") + "\n";

    let registry_path = adr_dir.join("registry.md");
    if check_only {
        if registry_path.exists() {
            match fs::read_to_string(&registry_path) {
                Ok(current) if current == expected_content => {
                    ui::action_ok("docs/adr/registry.md 一致");
                    return true;
                }
                _ => {
                    ui::action_fail("docs/adr/registry.md 不一致");
                    return false;
                }
            }
        } else {
            ui::action_fail("docs/adr/registry.md 不存在");
            return false;
        }
    } else {
        match fs::write(&registry_path, &expected_content) {
            Ok(_) => {
                ui::action_ok("已渲染: docs/adr/registry.md");
                return true;
            }
            Err(e) => {
                ui::action_fail(&format!("写入失败: {e}"));
                return false;
            }
        }
    }
}
