//! 公共工具函数
//!
//! 提供跨命令共享的工具函数

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use chrono::Local;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml;

use crate::config::UvpConfig;

// ── Feature 状态枚举 ──────────────────────────────────

pub const ALL_STATUSES: &[&str] = &[
    "idea", "planned", "in_progress", "implemented",
    "verified", "paused", "deprecated", "removed",
];

pub fn status_emoji(status: &str) -> &'static str {
    match status {
        "idea" => "💡",
        "planned" => "📋",
        "in_progress" => "🔧",
        "implemented" => "✨",
        "verified" => "✅",
        "paused" => "⏸️",
        "deprecated" => "❌",
        "removed" => "🗑️",
        _ => "❓",
    }
}

// ── Feature Registry 数据结构 ──────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeatureRegistry {
    #[serde(default)]
    pub features: Vec<FeatureEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureEntry {
    pub id: String,
    pub title: String,
    pub status: String,
    pub directory: String,
    #[serde(default = "default_adr")]
    pub adr: String,
    pub created: String,
    pub updated: String,
}

fn default_adr() -> String { "-".into() }

// ── Feature Registry 读写 ──────────────────────────────

pub fn load_feature_registry(project_dir: &Path, config: &UvpConfig) -> FeatureRegistry {
    let registry_path = project_dir.join(&config.feature.registry);
    if !registry_path.exists() {
        return FeatureRegistry::default();
    }
    let content = match fs::read_to_string(&registry_path) {
        Ok(c) => c,
        Err(_) => return FeatureRegistry::default(),
    };
    serde_yaml::from_str(&content).unwrap_or_default()
}

pub fn save_feature_registry(project_dir: &Path, config: &UvpConfig, data: &FeatureRegistry) {
    let registry_path = project_dir.join(&config.feature.registry);
    if let Some(parent) = registry_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let header = "# Feature Registry\n# 此文件由 uvp 自动维护，记录所有特性的状态\n# AI 读取此文件判断任务属于哪个 Feature，通过 id + title 匹配\n\n";
    let yaml_content = serde_yaml::to_string(data).unwrap_or_default();
    let _ = fs::write(&registry_path, format!("{header}{yaml_content}"));
}

// ── Feature 编号 ──────────────────────────────────────

pub fn get_next_feature_number(data: &FeatureRegistry) -> i32 {
    let re = Regex::new(r"^FEAT-(\d+)").unwrap();
    let mut max_num = 0;
    for feat in &data.features {
        if let Some(caps) = re.captures(&feat.id) {
            if let Ok(n) = caps[1].parse::<i32>() {
                max_num = max_num.max(n);
            }
        }
    }
    max_num + 1
}

pub fn title_to_dirname(title: &str) -> String {
    let re = Regex::new(r"[^\w\s-]").unwrap();
    let re2 = Regex::new(r"[\s_]+").unwrap();
    let lower = title.to_lowercase();
    let slug = re.replace_all(&lower, "");
    let slug = re2.replace_all(&slug, "-").trim_matches('-').to_string();
    if slug.is_empty() { "feature".into() } else { slug }
}

pub fn find_feature<'a>(data: &'a FeatureRegistry, feat_id: &str) -> Option<&'a FeatureEntry> {
    data.features.iter().find(|f| f.id == feat_id)
}

pub fn find_feature_mut<'a>(data: &'a mut FeatureRegistry, feat_id: &str) -> Option<&'a mut FeatureEntry> {
    data.features.iter_mut().find(|f| f.id == feat_id)
}

// ── Meta Header 生成 ──────────────────────────────────

pub fn generate_meta_header(title: &str, doc_type: Option<&str>, extra: Option<&HashMap<String, String>>) -> String {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let mut lines = vec!["---".to_string()];
    lines.push(format!("title: \"{title}\""));
    lines.push(format!("date: {today}"));
    if let Some(dt) = doc_type {
        lines.push(format!("doc_type: {dt}"));
    }
    if let Some(ext) = extra {
        for (k, v) in ext {
            lines.push(format!("{k}: \"{v}\""));
        }
    }
    lines.push("---".to_string());
    lines.join("\n") + "\n\n"
}

// ── ADR 状态解析 ──────────────────────────────────────

pub fn parse_adr_status(content: &str) -> String {
    let re = Regex::new(r"(?i)(?:状态|Status)[：:]\s*(proposed|accepted|deprecated|superseded)").unwrap();
    if let Some(caps) = re.captures(content) {
        return caps[1].to_lowercase();
    }
    "proposed".into()
}

// ── ADR 编号 ──────────────────────────────────────────

pub fn get_next_adr_number(adr_dir: &Path) -> i32 {
    let re = Regex::new(r"^(\d{4})-").unwrap();
    let mut max_num = 0;
    if let Ok(entries) = fs::read_dir(adr_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name == "template.md" || name == "registry.md" { continue; }
            if let Some(caps) = re.captures(&name) {
                if let Ok(n) = caps[1].parse::<i32>() {
                    max_num = max_num.max(n);
                }
            }
        }
    }
    max_num + 1
}

pub fn title_to_filename(number: i32, title: &str) -> String {
    let re = Regex::new(r"[^\w\s-]").unwrap();
    let re2 = Regex::new(r"[\s_]+").unwrap();
    let lower = title.to_lowercase();
    let slug = re.replace_all(&lower, "");
    let slug = re2.replace_all(&slug, "-").trim_matches('-').to_string();
    let slug = if slug.is_empty() { "decision".to_string() } else { slug };
    format!("{number:04}-{slug}.md")
}

// ── 模板加载 ──────────────────────────────────────────

/// 获取可执行文件所在目录（兼容开发模式）
pub fn get_exe_dir() -> std::path::PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
}

// ── 编译时内嵌模板 ──────────────────────────────────

macro_rules! embed_templates {
    ($($name:expr => $path:expr),* $(,)?) => {
        pub fn load_builtin_template(filename: &str) -> Option<String> {
            match filename {
                $($name => Some(include_str!($path).to_string()),)*
                _ => {
                    // fallback: 从 ~/.uvp/templates/ 读取用户自定义模板
                    let home_template = crate::config::uvp_home().join("templates").join(filename);
                    if home_template.exists() {
                        return fs::read_to_string(&home_template).ok();
                    }
                    None
                }
            }
        }
    };
}

embed_templates! {
    "default.md" => "templates/default.md",
    "adr_registry.md" => "templates/adr_registry.md",
    "feature_registry.yaml" => "templates/feature_registry.yaml",
    "features_index.md" => "templates/features_index.md",
    "ai_context.md" => "templates/ai_context.md",
    "ai_rule.md" => "templates/ai_rule.md",
    "project_state.md" => "templates/project_state.md",
    "glossary.md" => "templates/glossary.md",
    "architecture_current.md" => "templates/architecture_current.md",
    "roadmap.md" => "templates/roadmap.md",
    "mkdocs.yml" => "templates/mkdocs.yml",
}

/// 加载 ADR 模板（项目 docs/adr/template.md 优先，否则使用内置默认模板）
pub fn load_adr_template(project_dir: &Path) -> Result<String, String> {
    let project_template = project_dir.join("docs/adr/template.md");
    if project_template.exists() {
        return fs::read_to_string(&project_template).map_err(|e| e.to_string());
    }
    load_builtin_template("default.md").ok_or_else(|| "内置默认模板加载失败".to_string())
}
