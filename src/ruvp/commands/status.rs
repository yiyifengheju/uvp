//! uvp status 命令 — 展示项目状态

use std::fs;
use std::path::Path;
use std::process::Command;

use crate::common;
use crate::config;
use crate::ui;

pub fn run(open_file: bool, onboard: bool) {
    if onboard {
        println!("{} --onboard 功能即将上线（本地实时状态面板）", ui::styled_bold_cyan("coming soon"));
        println!("  当前可用: uvp status（终端）或 uvp status --open（浏览器）");
        return;
    }

    let project_dir = config::find_project_root(None, true).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });
    let cfg = config::get_effective_config(&project_dir);

    display_project_status(&project_dir, &cfg);

    if open_file {
        let pb = ui::step_start("Generating HTML report");
        let html_path = generate_status_html(&project_dir, &cfg);
        ui::step_done(&pb, "HTML report generated");
        let _ = open::that(&html_path);
    }
}

fn display_project_status(project_dir: &Path, cfg: &config::UvpConfig) {
    // 项目名称
    let mut project_name = project_dir.file_name().unwrap_or_default().to_string_lossy().to_string();
    let pyproject = project_dir.join("pyproject.toml");
    if pyproject.exists() {
        if let Ok(content) = fs::read_to_string(&pyproject) {
            if let Ok(data) = content.parse::<toml::Value>() {
                if let Some(name) = data.get("project").and_then(|p| p.get("name")).and_then(|n| n.as_str()) {
                    project_name = name.to_string();
                }
            }
        }
    }

    // ADR 状态
    let adr_dir = project_dir.join(&cfg.adr.directory);
    let adr_status = count_adr_status(&adr_dir);
    let total_adr: i32 = adr_status.values().sum();

    // Feature 状态
    let feat_status = count_feature_status(project_dir, cfg);
    let total_feat: i32 = feat_status.values().sum();

    // Git 信息
    let git_info = get_git_info(project_dir);

    // 构建状态面板
    println!("{}: {}", ui::styled_bold("项目"), project_name);
    println!("ADR: {total_adr} 个");
    if !adr_status.is_empty() {
        let detail: Vec<String> = adr_status.iter().map(|(k, v)| format!("{v} {k}")).collect();
        println!("  ({})", detail.join(", "));
    }

    println!("Features: {total_feat} 个");
    if !feat_status.is_empty() {
        let detail: Vec<String> = feat_status.iter().map(|(k, v)| format!("{v} {k}")).collect();
        println!("  ({})", detail.join(", "));
    }

    // 当前活跃 Feature
    if let Some(&count) = feat_status.get("implementing") {
        if count > 0 {
            let data = common::load_feature_registry(project_dir, cfg);
            let active: Vec<_> = data.features.iter().filter(|f| f.status == "implementing").collect();
            if !active.is_empty() {
                println!("\n{}进行中:", ui::styled_bold_yellow(""));
                for f in active {
                    println!("  {}（{}）", f.id, f.title);
                }
            }
        }
    }

    println!("\nRegistry 页面: uvp render → mkdocs serve 查看");

    if let Some(branch) = &git_info.get("branch") {
        println!("分支: {branch}");
    }
    if let Some(commit) = &git_info.get("last_commit") {
        println!("最近提交: {commit}");
    }
}

fn count_adr_status(adr_dir: &Path) -> std::collections::HashMap<String, i32> {
    let mut status_count = std::collections::HashMap::new();
    if !adr_dir.exists() {
        return status_count;
    }
    if let Ok(entries) = fs::read_dir(adr_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.ends_with(".md") || name == "template.md" || name == "registry.md" {
                continue;
            }
            if let Ok(content) = fs::read_to_string(entry.path()) {
                let status = common::parse_adr_status(&content);
                *status_count.entry(status).or_insert(0) += 1;
            }
        }
    }
    status_count
}

fn count_feature_status(project_dir: &Path, cfg: &config::UvpConfig) -> std::collections::HashMap<String, i32> {
    let data = common::load_feature_registry(project_dir, cfg);
    let mut status_count = std::collections::HashMap::new();
    for feat in &data.features {
        *status_count.entry(feat.status.clone()).or_insert(0) += 1;
    }
    status_count
}

fn get_git_info(project_dir: &Path) -> std::collections::HashMap<String, String> {
    let mut info = std::collections::HashMap::new();

    if let Ok(output) = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(project_dir)
        .output()
    {
        if output.status.success() {
            info.insert("branch".into(), String::from_utf8_lossy(&output.stdout).trim().to_string());
        }
    }

    if let Ok(output) = Command::new("git")
        .args(["log", "-1", "--oneline"])
        .current_dir(project_dir)
        .output()
    {
        if output.status.success() {
            info.insert("last_commit".into(), String::from_utf8_lossy(&output.stdout).trim().to_string());
        }
    }

    info
}

fn generate_status_html(project_dir: &Path, cfg: &config::UvpConfig) -> std::path::PathBuf {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let mut project_name = project_dir.file_name().unwrap_or_default().to_string_lossy().to_string();
    let pyproject = project_dir.join("pyproject.toml");
    if pyproject.exists() {
        if let Ok(content) = fs::read_to_string(&pyproject) {
            if let Ok(data) = content.parse::<toml::Value>() {
                if let Some(name) = data.get("project").and_then(|p| p.get("name")).and_then(|n| n.as_str()) {
                    project_name = name.to_string();
                }
            }
        }
    }

    let adr_status = count_adr_status(&project_dir.join(&cfg.adr.directory));
    let feat_status = count_feature_status(project_dir, cfg);
    let data = common::load_feature_registry(project_dir, cfg);
    let git_info = get_git_info(project_dir);

    let total_adr: i32 = adr_status.values().sum();
    let total_feat: i32 = feat_status.values().sum();

    let mut features_rows = String::new();
    for feat in &data.features {
        features_rows.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
            feat.id, feat.title, feat.status, feat.created, feat.updated
        ));
    }

    let html = format!(r#"<!DOCTYPE html>
<html lang="zh-CN">
<head><meta charset="UTF-8"><title>{project_name} - 项目状态报告</title>
<style>
body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; max-width: 1200px; margin: 0 auto; padding: 20px; background: #f5f5f5; }}
.header {{ background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 30px; border-radius: 10px; margin-bottom: 20px; }}
.card {{ background: white; border-radius: 8px; padding: 20px; margin-bottom: 20px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
table {{ width: 100%; border-collapse: collapse; margin-top: 15px; }}
th, td {{ padding: 12px; text-align: left; border-bottom: 1px solid #ddd; }}
th {{ background-color: #667eea; color: white; }}
</style>
</head>
<body>
<div class="header"><h1>{project_name}</h1><div style="opacity:0.8;margin-top:10px">生成时间: {now}</div></div>
<div class="card"><h2 style="color:#667eea">概览</h2>
<div style="display:grid;grid-template-columns:repeat(4,1fr);gap:15px;text-align:center">
<div style="background:#f8f9fa;padding:15px;border-radius:6px"><div style="font-size:2em;font-weight:bold;color:#667eea">{total_adr}</div><div style="color:#666">ADR 总数</div></div>
<div style="background:#f8f9fa;padding:15px;border-radius:6px"><div style="font-size:2em;font-weight:bold;color:#667eea">{total_feat}</div><div style="color:#666">Feature 总数</div></div>
<div style="background:#f8f9fa;padding:15px;border-radius:6px"><div style="font-size:2em;font-weight:bold;color:#667eea">{}</div><div style="color:#666">进行中</div></div>
<div style="background:#f8f9fa;padding:15px;border-radius:6px"><div style="font-size:2em;font-weight:bold;color:#667eea">{}</div><div style="color:#666">已验证</div></div>
</div></div>
<div class="card"><h2 style="color:#667eea">Git 信息</h2><p><strong>分支:</strong> {}</p><p><strong>最近提交:</strong> {}</p></div>
<div class="card"><h2 style="color:#667eea">Feature 列表</h2><table><thead><tr><th>ID</th><th>标题</th><th>状态</th><th>创建日期</th><th>更新日期</th></tr></thead><tbody>{features_rows}</tbody></table></div>
</body></html>"#,
        feat_status.get("implementing").unwrap_or(&0),
        feat_status.get("verified").unwrap_or(&0),
        git_info.get("branch").unwrap_or(&"N/A".into()),
        git_info.get("last_commit").unwrap_or(&"N/A".into()),
    );

    let output_path = project_dir.join(".uvp").join("status.html");
    let _ = fs::create_dir_all(output_path.parent().unwrap());
    let _ = fs::write(&output_path, &html);
    output_path
}
