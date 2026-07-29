//! uvp adr 命令 — 创建架构决策记录

use std::fs;
use std::path::Path;

use chrono::Local;
use regex::Regex;

use crate::common;
use crate::config;
use crate::ui;

pub fn run(title: &str, from_obsidian: Option<&str>, status: &str, open_file: bool) {
    let project_dir = config::find_project_root(None, true).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });
    let cfg = config::get_effective_config(&project_dir);

    let adr_dir = project_dir.join(&cfg.adr.directory);
    if let Err(e) = fs::create_dir_all(&adr_dir) {
        eprintln!("{} 创建 ADR 目录失败: {e}", ui::icon_fail());
        std::process::exit(1);
    }

    // ── 创建 ADR ──
    let pb = ui::step_start("Creating ADR");

    // 加载模板
    let template_content = match common::load_adr_template(&project_dir) {
        Ok(c) => c,
        Err(e) => {
            ui::step_fail(&pb, &format!("模板加载失败: {e}"));
            return;
        }
    };

    // 替换模板变量
    let number = common::get_next_adr_number(&adr_dir);
    let today = Local::now().format("%Y-%m-%d").to_string();
    let mut content = template_content
        .replace("{TITLE}", title)
        .replace("{NUMBER}", &number.to_string())
        .replace("{DATE}", &today);

    // 替换状态
    content = content.replace("status: proposed", &format!("status: {status}"));
    let re = Regex::new(r"(?m)^## 状态\s*\n\s*proposed").unwrap();
    content = re.replace(&content, &format!("## 状态\n\n{status}")).to_string();

    // 如果指定了 --from-obsidian
    let mut obsidian_note_stem = None;
    if let Some(keyword) = from_obsidian {
        ui::step_update(&pb, &format!("Searching Obsidian: {keyword}"));
        let vault_path_str = &cfg.obsidian.vault;
        if vault_path_str.is_empty() {
            ui::step_fail(&pb, "未配置 Obsidian Vault 路径");
            println!("  → 请编辑 ~/.uvp/uvp.toml，设置 [obsidian] vault = \"<path>\"");
            return;
        }
        let vault_path = shellexpand::tilde(vault_path_str).to_string();
        let vault = Path::new(&vault_path);
        if let Some(note_path) = find_obsidian_note(keyword, vault) {
            if let Ok(note_content) = fs::read_to_string(&note_path) {
                obsidian_note_stem = note_path.file_stem().map(|s| s.to_string_lossy().to_string());
                content = format!("> 来源：[[{}]]\n\n{content}\n\n---\n笔记内容：\n{note_content}",
                    obsidian_note_stem.as_deref().unwrap_or(keyword));
            }
        } else {
            ui::step_update(&pb, &format!("未找到匹配 '{keyword}' 的笔记，继续创建"));
        }
    }

    // 生成文件
    let filename = match cfg.adr.naming.as_str() {
        "datetime" => {
            let now = Local::now();
            format!("{}-{}.md", now.format("%Y%m%d"), now.format("%H%M"))
        }
        _ => common::title_to_filename(number, title),
    };

    let filepath = adr_dir.join(&filename);
    ui::step_update(&pb, &format!("Writing {filename}"));
    if let Err(e) = fs::write(&filepath, &content) {
        ui::step_fail(&pb, &format!("写入失败: {e}"));
        std::process::exit(1);
    }

    ui::step_done(&pb, &format!("ADR #{number:04} created: {title}"));

    // 输出详情
    println!("  编号: {}", ui::styled_cyan(&format!("{number:04}")));
    println!("  文件: {}", filepath.strip_prefix(&project_dir).unwrap_or(&filepath).display());
    println!("  状态: {}", ui::styled_cyan(status));
    if let Some(stem) = &obsidian_note_stem {
        println!("  引用: [[{stem}]]");
    }

    // ── 更新 registry ──
    let pb2 = ui::step_start("Updating ADR Registry");
    update_registry(&adr_dir, &project_dir);
    ui::step_done(&pb2, "ADR Registry updated");

    // 打开文件
    if open_file {
        let _ = open::that(&filepath);
    }
}

fn find_obsidian_note<'a>(keyword: &str, vault: &Path) -> Option<std::path::PathBuf> {
    if !vault.exists() {
        return None;
    }
    let keyword_lower = keyword.to_lowercase();
    let mut matches: Vec<std::path::PathBuf> = Vec::new();

    if let Ok(entries) = walkdir(vault) {
        for path in entries {
            if path.extension().map(|e| e == "md").unwrap_or(false) {
                if let Some(stem) = path.file_stem() {
                    if stem.to_string_lossy().to_lowercase().contains(&keyword_lower) {
                        matches.push(path);
                    }
                }
            }
        }
    }

    if matches.is_empty() {
        return None;
    }
    if matches.len() == 1 {
        return Some(matches.into_iter().next().unwrap());
    }
    // 返回最短名称的匹配
    matches.sort_by_key(|p| p.file_stem().map(|s| s.to_string_lossy().len()).unwrap_or(usize::MAX));
    Some(matches.into_iter().next().unwrap())
}

fn walkdir(dir: &Path) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    let mut result = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                result.extend(walkdir(&path)?);
            } else {
                result.push(path);
            }
        }
    }
    Ok(result)
}

fn update_registry(adr_dir: &Path, project_dir: &Path) {
    if !adr_dir.exists() {
        return;
    }

    let mut entries: Vec<(String, String, String, String, String, String)> = Vec::new();

    if let Ok(files) = fs::read_dir(adr_dir) {
        let mut file_list: Vec<_> = files.flatten().collect();
        file_list.sort_by_key(|f| f.file_name());

        for f in &file_list {
            let name = f.file_name().to_string_lossy().to_string();
            if !name.ends_with(".md") || name == "template.md" || name == "index.md" {
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

            let re_date = Regex::new(r"日期[：:]\s*(.+)").unwrap();
            let date_str = re_date.captures(&content)
                .map(|c| c[1].trim().to_string())
                .unwrap_or_else(|| {
                    f.metadata().ok()
                        .and_then(|m| m.modified().ok())
                        .map(|t| {
                            let dt: chrono::DateTime<Local> = t.into();
                            dt.format("%Y-%m-%d").to_string()
                        })
                        .unwrap_or_default()
                });

            entries.push((number, name, title, status, feat_ref, date_str));
        }
    }

    // Step 1: Write adr-registry.yaml (single source of truth)
    let yaml_path = project_dir.join("docs/_meta/adr-registry.yaml");
    let mut yaml_lines = vec![
        "# ADR Registry".to_string(),
        "# 此文件由 uvp 自动维护，记录所有架构决策记录的状态".to_string(),
        "# AI 读取此文件判断 ADR 状态和关联关系".to_string(),
        String::new(),
        "adrs:".to_string(),
    ];

    for (number, filename, title, status, feat_ref, date_str) in &entries {
        yaml_lines.push(format!("- id: ADR-{number}"));
        yaml_lines.push(format!("  title: {title}"));
        yaml_lines.push(format!("  status: {status}"));
        yaml_lines.push(format!("  file: {filename}"));
        let features: Vec<&str> = if feat_ref == "-" {
            vec![]
        } else {
            feat_ref.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
        };
        yaml_lines.push(format!("  related_features: [{}]", features.join(", ")));
        yaml_lines.push(format!("  date: {date_str}"));
    }

    if let Err(e) = fs::write(&yaml_path, yaml_lines.join("\n") + "\n") {
        eprintln!("{} 写入 adr-registry.yaml 失败: {e}", ui::icon_fail());
        return;
    }

    // Step 2: Render index.md (output derived from yaml)
    let mut status_count: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
    for (_, _, _, status, _, _) in &entries {
        *status_count.entry(status.clone()).or_insert(0) += 1;
    }

    let mut lines = vec!["# ADR Registry\n".to_string()];
    lines.push("## 状态概览\n".to_string());
    lines.push("| 状态 | 数量 |".to_string());
    lines.push("|------|------|".to_string());
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
        match s {
            "proposed" => "📝",
            "accepted" => "✅",
            "superseded" => "⏳",
            "deprecated" => "❌",
            _ => "❓",
        }
    };

    if entries.is_empty() {
        lines.push("| （暂无） | | | | |".to_string());
    } else {
        for (number, _, title, status, feat_ref, date_str) in &entries {
            let emoji = status_emoji(status);
            lines.push(format!("| {number} | {title} | {emoji} {status} | {feat_ref} | {date_str} |"));
        }
    }

    let index_path = adr_dir.join("index.md");
    if let Err(e) = fs::write(&index_path, lines.join("\n") + "\n") {
        eprintln!("{} 写入 index.md 失败: {e}", ui::icon_fail());
    }
}
