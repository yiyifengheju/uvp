//! uvp obsidian 命令 — 从 Obsidian 知识库导入研究素材

use std::fs;
use std::path::{Path, PathBuf};

use crate::config;
use crate::ui;
use crate::ObsidianCommands;

pub fn run(command: ObsidianCommands, vault: Option<&str>) {
    let project_dir = config::find_project_root(None, true).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });
    let cfg = config::get_effective_config(&project_dir);

    let vault_str = vault
        .map(|s| s.to_string())
        .or_else(|| Some(cfg.obsidian.vault.clone()))
        .unwrap_or_default();

    if vault_str.is_empty() {
        ui::error_msg("未配置 Obsidian Vault 路径");
        println!("请编辑 ~/.uvp/uvp.toml，设置 [obsidian] vault = \"<path>\"");
        std::process::exit(1);
    }

    let vault_path = PathBuf::from(shellexpand::tilde(&vault_str).to_string());
    if !vault_path.exists() {
        ui::error_msg(&format!("Vault 路径不存在: {}", vault_path.display()));
        std::process::exit(1);
    }

    let exclude_dirs = &cfg.obsidian.exclude_dirs;

    match command {
        ObsidianCommands::Pull { dry_run } => obsidian_pull(&project_dir, &vault_path, exclude_dirs, dry_run),
        ObsidianCommands::Sync { dry_run } => obsidian_sync(&project_dir, &vault_path, exclude_dirs, dry_run),
    }
}

fn obsidian_pull(project_dir: &Path, vault: &Path, exclude_dirs: &[String], dry_run: bool) {
    let project_name = project_dir.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "project".to_string());
    let vault_ref_src = vault.join("Projects").join(&project_name).join("reference");
    let ref_dst = project_dir.join("reference");

    if !vault_ref_src.exists() {
        ui::error_msg(&format!("Vault 中不存在路径: {}", vault_ref_src.display()));
        println!("  请确认 Obsidian Vault 中有 Projects/{project_name}/reference/ 目录");
        return;
    }

    if dry_run {
        println!("{} 模拟模式 - 不执行实际操作\n", ui::styled_bold("▶"));
        let pulled = sync_directory(&vault_ref_src, &ref_dst, true, Some(exclude_dirs));
        print_sync_result(&pulled, true, "拉取");
        return;
    }

    let pb = ui::step_start("Pulling from Obsidian");
    let pulled = sync_directory(&vault_ref_src, &ref_dst, false, Some(exclude_dirs));

    if pulled.is_empty() {
        ui::step_skip(&pb, "No files to pull");
    } else {
        ui::step_done(&pb, &format!("Pulled {} files", pulled.len()));
        for f in &pulled {
            std::thread::sleep(std::time::Duration::from_millis(ui::get_delay_ms()));
            println!("  {} {}", ui::styled_green("✓"), ui::styled_cyan(f));
        }
    }
}

fn obsidian_sync(project_dir: &Path, vault: &Path, exclude_dirs: &[String], dry_run: bool) {
    let project_name = project_dir.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "project".to_string());
    let vault_ref_dir = vault.join("Projects").join(&project_name).join("reference");
    let ref_dir = project_dir.join("reference");

    if dry_run {
        println!("{} 模拟模式 - 不执行实际操作\n", ui::styled_bold("▶"));
        let ref_pulled = if vault_ref_dir.exists() {
            sync_directory(&vault_ref_dir, &ref_dir, true, Some(exclude_dirs))
        } else {
            Vec::new()
        };
        let ref_pushed = if ref_dir.exists() {
            sync_directory(&ref_dir, &vault_ref_dir, true, Some(exclude_dirs))
        } else {
            Vec::new()
        };
        if !ref_pulled.is_empty() { print_sync_result(&ref_pulled, true, "拉取"); }
        if !ref_pushed.is_empty() { print_sync_result(&ref_pushed, true, "推送"); }
        if ref_pulled.is_empty() && ref_pushed.is_empty() {
            ui::empty_msg("没有需要同步的文件");
        }
        return;
    }

    let pb = ui::step_start("Syncing reference/");

    // Pull: Vault → 项目 reference/
    ui::step_update(&pb, "Pulling from Vault");
    let ref_pulled = if vault_ref_dir.exists() {
        sync_directory(&vault_ref_dir, &ref_dir, false, Some(exclude_dirs))
    } else {
        Vec::new()
    };

    // Push: 项目 reference/ → Vault
    ui::step_update(&pb, "Pushing to Vault");
    let ref_pushed = if ref_dir.exists() {
        let _ = fs::create_dir_all(&vault_ref_dir);
        sync_directory(&ref_dir, &vault_ref_dir, false, Some(exclude_dirs))
    } else {
        Vec::new()
    };

    let total = ref_pulled.len() + ref_pushed.len();
    if total == 0 {
        ui::step_skip(&pb, "No files to sync");
    } else {
        ui::step_done(&pb, &format!("Synced: {} pulled, {} pushed", ref_pulled.len(), ref_pushed.len()));
    }
}

fn sync_directory(src: &Path, dst: &Path, dry_run: bool, exclude_dirs: Option<&[String]>) -> Vec<String> {
    let mut synced = Vec::new();
    if !src.exists() {
        return synced;
    }

    let exclude_set: std::collections::HashSet<&str> = exclude_dirs
        .map(|v| v.iter().map(|s| s.as_str()).collect())
        .unwrap_or_default();

    if let Ok(entries) = walkdir(src) {
        for file_path in entries {
            if !file_path.is_file() {
                continue;
            }

            let rel = match file_path.strip_prefix(src) {
                Ok(r) => r.to_string_lossy().to_string(),
                Err(_) => continue,
            };

            let path = std::path::Path::new(&rel);
            if path.components().any(|c| {
                c.as_os_str().to_str().map(|s| exclude_set.contains(s)).unwrap_or(false)
            }) {
                continue;
            }

            let target = dst.join(&rel);

            if dry_run {
                synced.push(rel);
                continue;
            }

            if target.exists() {
                let src_time = fs::metadata(&file_path).and_then(|m| m.modified()).ok();
                let dst_time = fs::metadata(&target).and_then(|m| m.modified()).ok();
                if let (Some(st), Some(dt)) = (src_time, dst_time) {
                    if st <= dt {
                        continue;
                    }
                }
            }

            if let Some(parent) = target.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if fs::copy(&file_path, &target).is_ok() {
                synced.push(rel);
            }
        }
    }

    synced
}

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

fn print_sync_result(files: &[String], dry_run: bool, action: &str) {
    if files.is_empty() {
        ui::empty_msg("没有需要同步的文件");
    } else if dry_run {
        println!("{}将要{}的文件:", ui::styled_bold("▶"), action);
        for f in files {
            println!("  {}", ui::styled_cyan(f));
        }
    } else {
        println!("{}{}完成：{} 个文件", ui::styled_green("✓"), action, files.len());
        for f in files.iter().take(10) {
            println!("  {}", ui::styled_dim(f));
        }
        if files.len() > 10 {
            println!("  {}... 还有 {} 个文件", ui::styled_dim(""), files.len() - 10);
        }
    }
}
