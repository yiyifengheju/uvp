//! 配置管理模块
//!
//! 管理全局配置 (~/.uvp/uvp.toml) 和项目配置 (uvp.toml) 的读写与合并。
//! 优先级：命令行参数 > 环境变量 > 项目配置 > 全局配置 > 默认值

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

// ── 常量 ──────────────────────────────────────────

pub const PROJECT_CONFIG_FILENAME: &str = "uvp.toml";

/// AI 规则文件映射
pub fn ai_rule_files() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("claude", "CLAUDE.md");
    m.insert("cursor", ".cursorrules");
    m.insert("windsurf", ".windsurfrules");
    m.insert("cline", ".clinerules");
    m.insert("trae", ".trae/rules.md");
    m
}

/// IDE skills 部署路径映射
pub struct IdeSkillsPaths {
    pub project: Option<PathBuf>,
    pub global: Option<PathBuf>,
}

pub fn ide_skills_paths(ide: &str) -> Option<IdeSkillsPaths> {
    match ide {
        "claude" => Some(IdeSkillsPaths {
            project: Some(PathBuf::from(".claude/skills")),
            global: Some(dirs::home_dir()?.join(".claude").join("skills")),
        }),
        "trae" => Some(IdeSkillsPaths {
            project: Some(PathBuf::from(".trae/skills")),
            global: Some(dirs::home_dir()?.join(".trae").join("skills")),
        }),
        "cursor" => Some(IdeSkillsPaths {
            project: Some(PathBuf::from(".cursor/rules")),
            global: None,
        }),
        _ => None,
    }
}

// ── 配置结构体 ──────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdrConfig {
    #[serde(default = "default_adr_directory")]
    pub directory: String,
    #[serde(default = "default_adr_naming")]
    pub naming: String,
}

fn default_adr_directory() -> String { "docs/adr".into() }
fn default_adr_naming() -> String { "sequential".into() }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeatureConfig {
    #[serde(default = "default_feature_directory")]
    pub directory: String,
    #[serde(default = "default_feature_registry")]
    pub registry: String,
}

fn default_feature_directory() -> String { "docs/features".into() }
fn default_feature_registry() -> String { "docs/_meta/feature-registry.yaml".into() }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ObsidianConfig {
    #[serde(default)]
    pub vault: String,
    #[serde(default = "default_obsidian_exclude_dirs")]
    pub exclude_dirs: Vec<String>,
}

fn default_obsidian_exclude_dirs() -> Vec<String> {
    vec![
        "node_modules".into(), ".git".into(),
        "assets".into(), ".obsidian".into(),
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InitConfig {
    #[serde(default = "default_true")]
    pub auto_uv_init: bool,
    #[serde(default = "default_true")]
    pub auto_mkdocs: bool,
    #[serde(default = "default_true")]
    pub auto_ai_context: bool,
    #[serde(default = "default_true")]
    pub auto_ai_rules: bool,
    #[serde(default = "default_true")]
    pub auto_feature_ledger: bool,
    #[serde(default = "default_init_dependencies")]
    pub dependencies: Vec<String>,
}

fn default_init_dependencies() -> Vec<String> {
    vec![
        "mkdocs-material".into(),
        "mkdocs-glightbox".into(),
        "mkdocs-awesome-pages-plugin".into(),
        "neoteroi-mkdocs".into(),
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UiConfig {
    #[serde(default = "default_ui_delay_ms")]
    pub delay_ms: u64,
}

fn default_ui_delay_ms() -> u64 { 120 }

fn default_true() -> bool { true }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UvpConfig {
    #[serde(default)]
    pub adr: AdrConfig,
    #[serde(default)]
    pub feature: FeatureConfig,
    #[serde(default)]
    pub obsidian: ObsidianConfig,
    #[serde(default)]
    pub init: InitConfig,
    #[serde(default)]
    pub ui: UiConfig,
}

// ── 全局配置路径 ──────────────────────────────────────

pub fn uvp_home() -> PathBuf {
    dirs::home_dir().unwrap_or_default().join(".uvp")
}

pub fn global_config_path() -> PathBuf {
    uvp_home().join("uvp.toml")
}

// ── 配置加载与合并 ──────────────────────────────────

pub fn load_toml_config(path: &Path) -> Option<UvpConfig> {
    if !path.exists() {
        return None;
    }
    let content = fs::read_to_string(path).ok()?;
    toml::from_str(&content).ok()
}

pub fn load_global_config() -> UvpConfig {
    init_global_config();
    let path = global_config_path();
    load_toml_config(&path).unwrap_or_default()
}

pub fn load_project_config(project_dir: &Path) -> UvpConfig {
    let path = project_dir.join(PROJECT_CONFIG_FILENAME);
    load_toml_config(&path).unwrap_or_default()
}

/// 合并两个配置，other 覆盖 base 中的非空值
pub fn merge_configs(base: &UvpConfig, other: &UvpConfig) -> UvpConfig {
    let mut result = base.clone();

    // ADR
    if !other.adr.directory.is_empty() { result.adr.directory = other.adr.directory.clone(); }
    if !other.adr.naming.is_empty() { result.adr.naming = other.adr.naming.clone(); }

    // Feature
    if !other.feature.directory.is_empty() { result.feature.directory = other.feature.directory.clone(); }
    if !other.feature.registry.is_empty() { result.feature.registry = other.feature.registry.clone(); }

    // Obsidian
    if !other.obsidian.vault.is_empty() { result.obsidian.vault = other.obsidian.vault.clone(); }
    if !other.obsidian.exclude_dirs.is_empty() { result.obsidian.exclude_dirs = other.obsidian.exclude_dirs.clone(); }

    // Init
    if other.init.auto_uv_init { result.init.auto_uv_init = true; }
    if other.init.auto_mkdocs { result.init.auto_mkdocs = true; }
    if other.init.auto_ai_context { result.init.auto_ai_context = true; }
    if other.init.auto_ai_rules { result.init.auto_ai_rules = true; }
    if other.init.auto_feature_ledger { result.init.auto_feature_ledger = true; }
    if !other.init.dependencies.is_empty() { result.init.dependencies = other.init.dependencies.clone(); }

    // UI
    if other.ui.delay_ms != 0 { result.ui.delay_ms = other.ui.delay_ms; }

    result
}

/// 获取合并后的有效配置
pub fn get_effective_config(project_dir: &Path) -> UvpConfig {
    let base = UvpConfig::default();
    let global = load_global_config();
    let project = load_project_config(project_dir);

    let merged = merge_configs(&base, &global);
    merge_configs(&merged, &project)
}

// ── 项目根目录查找 ──────────────────────────────────

/// 从指定目录向上查找项目根目录
///
/// 查找标志文件：uvp.toml, pyproject.toml
/// require=true 时，找不到则报错（用于非 init 命令）
/// require=false 时，找不到则返回起始目录本身（用于 init 命令）
pub fn find_project_root(start: Option<&Path>, require: bool) -> Result<PathBuf, String> {
    let start = match start {
        Some(p) => p.to_path_buf(),
        None => std::env::current_dir().map_err(|e| format!("无法获取当前目录: {e}"))?,
    };
    let start = start.canonicalize().unwrap_or(start);

    let markers = [PROJECT_CONFIG_FILENAME, "pyproject.toml"];
    let mut current = start.clone();
    loop {
        for marker in &markers {
            if current.join(marker).exists() {
                return Ok(current);
            }
        }
        let parent = current.parent();
        match parent {
            Some(p) => current = p.to_path_buf(),
            None => break,
        }
    }

    if require {
        Err("未找到项目根目录（缺少 uvp.toml 或 pyproject.toml）。请先运行 uvp init".into())
    } else {
        Ok(start)
    }
}

// ── 全局配置初始化 ──────────────────────────────────

pub fn ensure_uvp_home() -> PathBuf {
    let home = uvp_home();
    let _ = fs::create_dir_all(&home);
    let templates_dir = home.join("templates");
    let _ = fs::create_dir_all(&templates_dir);

    // 部署内置 skills 到 ~/.uvp/skills/（不覆盖已有文件）
    deploy_builtin_skills();

    home
}

/// 将内置 skills 部署到 ~/.uvp/skills/
fn deploy_builtin_skills() {
    let target_skills_dir = uvp_home().join("skills");
    let _ = fs::create_dir_all(&target_skills_dir);

    // 内置 skills 列表（目录名 → 文件列表）
    let builtin_skills: &[(&str, &[(&str, &str)])] = &[
        ("uvp-workflow", &[
            ("SKILL.md", include_str!("skills/uvp-workflow/SKILL.md")),
        ]),
        ("uvp-feature-lifecycle", &[
            ("SKILL.md", include_str!("skills/uvp-feature-lifecycle/SKILL.md")),
        ]),
        ("uvp-file-coupling", &[
            ("SKILL.md", include_str!("skills/uvp-file-coupling/SKILL.md")),
        ]),
        ("uvp-meta-header", &[
            ("SKILL.md", include_str!("skills/uvp-meta-header/SKILL.md")),
            ("README.md", include_str!("skills/uvp-meta-header/README.md")),
        ]),
        ("uvp-weekly-report", &[
            ("SKILL.md", include_str!("skills/uvp-weekly-report/SKILL.md")),
        ]),
    ];

    for (skill_name, files) in builtin_skills {
        let skill_dir = target_skills_dir.join(skill_name);
        let _ = fs::create_dir_all(&skill_dir);

        for (filename, content) in *files {
            let file_path = skill_dir.join(filename);
            // 不覆盖已有文件
            if !file_path.exists() {
                let _ = fs::write(file_path, content);
            }
        }
    }
}

pub fn init_global_config() {
    let home = ensure_uvp_home();
    let config_path = home.join("uvp.toml");
    if config_path.exists() {
        return;
    }
    let default_config = r#"# UVP 全局配置文件
# 所有配置项的默认值，项目级 uvp.toml 可覆盖部分项

[adr]
directory = "docs/adr"
naming = "sequential"

[feature]
directory = "docs/features"
registry = "docs/_meta/feature-registry.yaml"

[obsidian]
vault = ""
exclude_dirs = ["node_modules", ".git", "assets", ".obsidian"]

[init]
auto_uv_init = true
auto_mkdocs = true
auto_ai_context = true
auto_ai_rules = true
auto_feature_ledger = true
dependencies = ["mkdocs-material", "mkdocs-glightbox", "mkdocs-awesome-pages-plugin", "neoteroi-mkdocs"]

[ui]
delay_ms = 120
"#;
    let _ = fs::write(&config_path, default_config);
}

pub fn generate_project_config_toml() -> String {
    r#"# UVP 项目配置文件
# 由 uvp init 自动生成，仅包含可按项目覆盖的配置项
# 全局默认配置见 ~/.uvp/uvp.toml，此处配置优先级高于全局

[adr]
# naming = "sequential"

[obsidian]
# vault = ""
# exclude_dirs = ["node_modules", ".git", "assets", ".obsidian"]
"#.into()
}

// ── Skills 部署 ──────────────────────────────────────

pub fn deploy_skills_to_ide(ide: &str, project_dir: Option<&Path>) -> Vec<PathBuf> {
    let mut deployed = Vec::new();
    let skills_paths = match ide_skills_paths(ide) {
        Some(p) => p,
        None => return deployed,
    };

    let source_skills_dir = uvp_home().join("skills");
    if !source_skills_dir.exists() {
        return deployed;
    }

    let mut targets: Vec<(String, PathBuf)> = Vec::new();

    if let Some(pd) = project_dir {
        if let Some(ref proj) = skills_paths.project {
            targets.push(("project".into(), pd.join(proj)));
        }
    }
    if let Some(ref g) = skills_paths.global {
        targets.push(("global".into(), g.clone()));
    }

    for (_scope, target_dir) in targets {
        let _ = fs::create_dir_all(&target_dir);

        if let Ok(entries) = fs::read_dir(&source_skills_dir) {
            for entry in entries.flatten() {
                if !entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    continue;
                }
                let skill_dir = entry.path();

                if ide == "cursor" {
                    // Cursor: SKILL.md → .mdc
                    let skill_md = skill_dir.join("SKILL.md");
                    if skill_md.exists() {
                        let mdc_name = format!("{}.mdc", skill_dir.file_name().unwrap_or_default().to_string_lossy());
                        let mdc_path = target_dir.join(&mdc_name);
                        if !mdc_path.exists() {
                            if let Ok(content) = fs::read_to_string(&skill_md) {
                                let mdc_content = convert_skill_to_mdc(
                                    &skill_dir.file_name().unwrap_or_default().to_string_lossy(),
                                    &content,
                                );
                                if fs::write(&mdc_path, &mdc_content).is_ok() {
                                    deployed.push(mdc_path);
                                }
                            }
                        }
                    }
                } else {
                    // Claude/Trae: 复制 skill 子目录
                    let dest_skill_dir = target_dir.join(skill_dir.file_name().unwrap_or_default());
                    let _ = fs::create_dir_all(&dest_skill_dir);
                    if let Ok(it) = walkdir(&skill_dir) {
                        for src_path in it {
                            if src_path.is_file() {
                                let rel = src_path.strip_prefix(&skill_dir).unwrap_or(&src_path);
                                let dst = dest_skill_dir.join(rel);
                                if !dst.exists() {
                                    let _ = fs::create_dir_all(dst.parent().unwrap_or(&dest_skill_dir));
                                    if fs::copy(&src_path, &dst).is_ok() {
                                        deployed.push(dst);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    deployed
}

fn convert_skill_to_mdc(slug: &str, content: &str) -> String {
    let mut description = slug.to_string();
    if content.starts_with("---") {
        if let Some(end) = content[3..].find("---") {
            let fm = &content[3..3 + end];
            for line in fm.lines() {
                if line.trim().starts_with("description:") {
                    description = line.splitn(2, ':').nth(1)
                        .unwrap_or(slug)
                        .trim()
                        .trim_matches('"')
                        .trim_matches('\'')
                        .to_string();
                }
            }
        }
    }

    let mut body = content.to_string();
    if body.starts_with("---") {
        if let Some(end) = body[3..].find("---") {
            body = body[3 + end + 3..].trim_start_matches('\n').to_string();
        }
    }

    format!("---\ndescription: {description}\nglobs:\nalwaysApply: true\n---\n\n{body}")
}

/// 简单递归遍历目录
fn walkdir(dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
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
