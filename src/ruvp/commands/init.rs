//! uvp init 命令 — 项目初始化（uv 最后执行）

use std::fs;
use std::path::{Path, PathBuf};
use std::thread;

use chrono::Local;
use console::style;
use indicatif::ProgressBar;

use crate::common;
use crate::config;
use crate::ui;

pub fn run(
    path: Option<&str>,
    name: Option<&str>,
    ide: &str,
    no_python: bool,
    no_mkdocs: bool,
    no_ai_rules: bool,
) {
    let start_dir = match path {
        Some(p) => PathBuf::from(p),
        None => std::env::current_dir().unwrap_or_default(),
    };
    let start_dir = start_dir.canonicalize().unwrap_or(start_dir);

    let project_dir = config::find_project_root(Some(&start_dir), false)
        .unwrap_or_else(|e| { eprintln!("{e}"); std::process::exit(1); });

    let project_name = name
        .map(|s| s.to_string())
        .unwrap_or_else(|| project_dir.file_name().unwrap_or_default().to_string_lossy().to_string());

    // ── 欢迎 ──
    eprintln!("\n{}", style_bold_cyan(&format!("uvp init  {project_name}")));
    eprintln!("  IDE: {}  |  {}", style_cyan(ide), style_dim(&format!("目标: {}", project_dir.display())));

    let cfg = config::load_global_config();

    // ══════════════════════════════════════════
    // 阶段 1: 项目配置 + 目录 + 模板（带 spinner）
    // ══════════════════════════════════════════

    // ── uvp.toml ──
    let pb1 = ui::step_start("Creating uvp.toml");
    std::thread::sleep(std::time::Duration::from_millis(ui::get_delay_ms()));
    let project_config_content = config::generate_project_config_toml();
    match write_file_safe(&project_dir.join("uvp.toml"), &project_config_content) {
        Ok(true) => ui::step_done(&pb1, "Project configuration written"),
        Ok(false) => ui::step_skip(&pb1, "uvp.toml already exists"),
        Err(e) => { ui::step_fail(&pb1, &format!("Failed to write uvp.toml: {e}")); std::process::exit(1); }
    }

    // ── 目录结构 ──
    let pb2 = ui::step_start("Creating directories");
    for dir_name in &["docs/adr", "docs/prd", "docs/features", "docs/architecture", "docs/_meta", "src", "reference"] {
        ui::step_update(&pb2, dir_name);
        let d = project_dir.join(dir_name);
        if !d.exists() {
            if let Err(e) = fs::create_dir_all(&d) {
                ui::step_fail(&pb2, &format!("Failed to create {dir_name}: {e}"));
                std::process::exit(1);
            }
        }
    }
    ui::step_done(&pb2, "Directory structure created");

    // ── MkDocs ──
    let pb3 = ui::step_start("Configuring MkDocs");
    if !no_mkdocs && cfg.init.auto_mkdocs {
        if let Some(tmpl) = common::load_builtin_template("mkdocs.yml") {
            ui::step_update(&pb3, "Writing mkdocs.yml");
            match write_file_safe(&project_dir.join("mkdocs.yml"), &tmpl.replace("{project_name}", &project_name)) {
                Ok(true) => ui::step_done(&pb3, "MkDocs configured"),
                Ok(false) => ui::step_skip(&pb3, "mkdocs.yml already exists"),
                Err(e) => { ui::step_fail(&pb3, &format!("Failed to write mkdocs.yml: {e}")); std::process::exit(1); }
            }
        } else {
            ui::step_fail(&pb3, "Template not found");
        }
    } else {
        ui::step_skip(&pb3, "Skipped");
    }

    // ── ADR ──
    let pb4 = ui::step_start("Setting up ADR templates");
    if let Some(t) = common::load_builtin_template("default.md") {
        ui::step_update(&pb4, "docs/adr/template.md");
        if let Err(e) = write_file_safe(&project_dir.join("docs/adr/template.md"), &t) {
            ui::step_fail(&pb4, &format!("Failed: {e}")); std::process::exit(1);
        }
    }
    if let Some(t) = common::load_builtin_template("adr_registry.md") {
        ui::step_update(&pb4, "docs/adr/registry.md");
        if let Err(e) = write_file_safe(&project_dir.join("docs/adr/registry.md"), &t.replace("{DATE}", &today())) {
            ui::step_fail(&pb4, &format!("Failed: {e}")); std::process::exit(1);
        }
    }
    ui::step_done(&pb4, "ADR templates ready");

    // ── Feature Registry ──
    let pb5 = ui::step_start("Creating Feature Registry");
    if cfg.init.auto_feature_ledger {
        if let Some(t) = common::load_builtin_template("feature_registry.yaml") {
            ui::step_update(&pb5, "docs/_meta/feature-registry.yaml");
            if let Err(e) = write_file_safe(&project_dir.join("docs/_meta/feature-registry.yaml"), &t) {
                ui::step_fail(&pb5, &format!("Failed: {e}")); std::process::exit(1);
            }
        }
        if let Some(t) = common::load_builtin_template("features_index.md") {
            ui::step_update(&pb5, "docs/features/index.md");
            if let Err(e) = write_file_safe(&project_dir.join("docs/features/index.md"), &t.replace("{DATE}", &today())) {
                ui::step_fail(&pb5, &format!("Failed: {e}")); std::process::exit(1);
            }
        }
        ui::step_done(&pb5, "Feature Registry created");
    } else {
        ui::step_skip(&pb5, "Skipped (auto_feature_ledger = false)");
    }

    // ── AI 上下文 ──
    let pb6 = ui::step_start("Writing AI context");
    if cfg.init.auto_ai_context {
        if let Some(t) = common::load_builtin_template("ai_context.md") {
            ui::step_update(&pb6, "docs/AI_CONTEXT.md");
            if let Err(e) = write_file_safe(
                &project_dir.join("docs/AI_CONTEXT.md"),
                &t.replace("{project_name}", &project_name).replace("{DATE}", &today()),
            ) {
                ui::step_fail(&pb6, &format!("Failed: {e}")); std::process::exit(1);
            }
        }
    }
    if let Some(t) = common::load_builtin_template("project_state.md") {
        ui::step_update(&pb6, "docs/PROJECT_STATE.md");
        if let Err(e) = write_file_safe(
            &project_dir.join("docs/PROJECT_STATE.md"),
            &t.replace("{project_name}", &project_name).replace("{DATE}", &today()),
        ) {
            ui::step_fail(&pb6, &format!("Failed: {e}")); std::process::exit(1);
        }
    }
    ui::step_done(&pb6, "AI context written");

    // ── AI 规则 ──
    let pb7 = ui::step_start(&format!("Generating AI rules ({ide})"));
    if !no_ai_rules && cfg.init.auto_ai_rules {
        let rule_files = config::ai_rule_files();
        if let Some(&rule_path) = rule_files.get(ide) {
            let target = project_dir.join(rule_path);
            if target.exists() {
                ui::step_skip(&pb7, &format!("{rule_path} already exists"));
            } else {
                let source = find_existing_rule_file(&project_dir);
                let content = match source {
                    Some(src) => fs::read_to_string(&src).ok(),
                    None => common::load_builtin_template("ai_rule.md")
                        .map(|t| t.replace("{project_name}", &project_name)),
                };
                if let Some(text) = content {
                    if let Some(parent) = target.parent() {
                        if let Err(e) = fs::create_dir_all(parent) {
                            ui::step_fail(&pb7, &format!("Failed: {e}")); std::process::exit(1);
                        }
                    }
                    ui::step_update(&pb7, rule_path);
                    if let Err(e) = fs::write(&target, text) {
                        ui::step_fail(&pb7, &format!("Failed to write {rule_path}: {e}")); std::process::exit(1);
                    }
                }
                ui::step_done(&pb7, &format!("AI rules generated for {ide}"));
            }
            let _deployed = config::deploy_skills_to_ide(ide, Some(&project_dir));
        }
    } else {
        ui::step_skip(&pb7, "Skipped");
    }

    // ── 辅助文件 ──
    let pb8 = ui::step_start("Writing auxiliary files");
    if let Some(t) = common::load_builtin_template("glossary.md") {
        ui::step_update(&pb8, "docs/GLOSSARY.md");
        if let Err(e) = write_file_safe(&project_dir.join("docs/GLOSSARY.md"), &t.replace("{DATE}", &today())) {
            ui::step_fail(&pb8, &format!("Failed: {e}")); std::process::exit(1);
        }
    }
    if let Some(t) = common::load_builtin_template("architecture_current.md") {
        ui::step_update(&pb8, "docs/architecture/current.md");
        if let Err(e) = write_file_safe(&project_dir.join("docs/architecture/current.md"), &t.replace("{project_name}", &project_name).replace("{DATE}", &today())) {
            ui::step_fail(&pb8, &format!("Failed: {e}")); std::process::exit(1);
        }
    }
    if let Some(t) = common::load_builtin_template("roadmap.md") {
        ui::step_update(&pb8, "docs/prd/roadmap.md");
        if let Err(e) = write_file_safe(&project_dir.join("docs/prd/roadmap.md"), &t.replace("{project_name}", &project_name).replace("{DATE}", &today())) {
            ui::step_fail(&pb8, &format!("Failed: {e}")); std::process::exit(1);
        }
    }
    {
        let todo_content = "# TODO\n\n> 项目想法、待验证方向、灵感收集。成熟后转为 ADR 或 Feature。\n\n## 待办\n\n## 已完成\n";
        ui::step_update(&pb8, "docs/TODO.md");
        if let Err(e) = write_file_safe(&project_dir.join("docs/TODO.md"), todo_content) {
            ui::step_fail(&pb8, &format!("Failed: {e}")); std::process::exit(1);
        }
    }
    ui::step_done(&pb8, "Auxiliary files written");
    let pb9 = ui::step_start("Setting up global config");
    let _home = config::ensure_uvp_home();
    config::init_global_config();
    ui::step_done(&pb9, "Global config ready");

    // ══════════════════════════════════════════
    // 阶段 2: uv init + uv add（最后执行）
    // ══════════════════════════════════════════

    let mut uv_init_handle: Option<thread::JoinHandle<bool>> = None;
    let mut pb_uv: Option<ProgressBar> = None;

    if !no_python && cfg.init.auto_uv_init {
        if !project_dir.join("pyproject.toml").exists() {
            let pb = ui::step_start("Running uv init ...");
            pb_uv = Some(pb);
            uv_init_handle = Some(ui::spawn_command_streaming(
                "uv",
                &["init", &project_dir.to_string_lossy()],
                Some(&project_dir),
                move |_line| {},
            ));
        } else {
            eprintln!("{}", style("· Skipped — pyproject.toml exists").dim());
        }
    }

    // 等待 uv init 完成（spinner 保持旋转）
    if let Some(h) = uv_init_handle {
        let ok = h.join().unwrap_or(false);
        // 停止 spinner，显示结果
        if let Some(ref pb) = pb_uv {
            pb.set_style(indicatif::ProgressStyle::with_template("{msg}").unwrap());
        }
        if ok {
            if let Some(ref pb) = pb_uv {
                pb.finish_with_message(format!("{} {}", style("✓").green(), "Project initialized with uv"));
            }
            let main_py = project_dir.join("main.py");
            if main_py.exists() { let _ = fs::remove_file(main_py); }
        } else {
            eprintln!("{}", style("✗ uv init failed").red());
        }
    }

    // ── uv add ──
    let global_config = config::get_effective_config(&project_dir);
    let deps = &global_config.init.dependencies;

    if !deps.is_empty() && !no_python && project_dir.join("pyproject.toml").exists() {
        let args: Vec<String> = std::iter::once("add".to_string()).chain(deps.iter().cloned()).collect();
        let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

        let pb_add = ui::step_start(&format!("Installing {}...", deps.join(", ")));
        let h = ui::spawn_command_streaming("uv", &arg_refs, Some(&project_dir), move |_line| {});

        // 等 join 完成，再停 spinner（让用户看到旋转）
        let ok = h.join().unwrap_or(false);
        pb_add.set_style(indicatif::ProgressStyle::with_template("{msg}").unwrap());
        if ok {
            pb_add.finish_with_message(format!("{} {}", style("✓").green(), "Dependencies installed"));
        } else {
            pb_add.finish_with_message(format!("{} {}", style("✗").red(), "Dependency installation failed"));
        }
    }

    // ══════════════════════════════════════════
    // 完成信息
    // ══════════════════════════════════════════
    eprintln!(
        "\n{} {}",
        style("🎉🎉 Initialized project").bold(),
        style(format!("`{project_name}` at `{}`", project_dir.display())).cyan()
    );

    ui::success_panel(
        "Initialized successfully",
        "Next: Write PRDs in docs/prd/, then let AI execute via AI_CONTEXT.md",
    );
}

/// 写入文件，已存在则跳过（返回 Ok(false)），权限错误等则返回 Err。
fn write_file_safe(path: &Path, content: &str) -> Result<bool, std::io::Error> {
    if path.exists() {
        return Ok(false);
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(true)
}

fn find_existing_rule_file(project_dir: &Path) -> Option<PathBuf> {
    for (_, rule_path) in &config::ai_rule_files() {
        let candidate = project_dir.join(rule_path);
        if candidate.exists() { return Some(candidate); }
    }
    None
}

fn today() -> String { Local::now().format("%Y-%m-%d").to_string() }

fn style_cyan(s: &str) -> String { console::style(s).cyan().to_string() }
fn style_dim(s: &str) -> String { console::style(s).dim().to_string() }
fn style_bold_cyan(s: &str) -> String { console::style(s).bold().cyan().to_string() }
