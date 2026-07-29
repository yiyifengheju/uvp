//! uvp todo 命令 — 管理项目想法和待办

use std::fs;
use std::path::Path;

use chrono::Local;

use crate::common::{self, TodoItem};
use crate::config;
use crate::TodoCommands;
use crate::ui;

pub fn run(command: Option<TodoCommands>, all: bool) {
    let project_dir = config::find_project_root(None, true).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    match command {
        None => todo_list(&project_dir, all),
        Some(TodoCommands::Add { content }) => todo_add(&project_dir, &content),
        Some(TodoCommands::Done { id }) => todo_done(&project_dir, id),
        Some(TodoCommands::Remove { id }) => todo_remove(&project_dir, id),
    }
}

fn todo_file_path(project_dir: &Path) -> std::path::PathBuf {
    project_dir.join("docs/TODO.md")
}

fn load_or_create(project_dir: &Path) -> String {
    let path = todo_file_path(project_dir);
    if path.exists() {
        return fs::read_to_string(&path).unwrap_or_default();
    }
    let template = default_template();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(&path, &template);
    template
}

fn default_template() -> String {
    "# TODO\n\n> 项目想法、待验证方向、灵感收集。成熟后转为 ADR 或 Feature。\n\n## 待办\n\n## 已完成\n".to_string()
}

fn next_id(items: &[TodoItem]) -> u32 {
    items.iter().map(|i| i.id).max().unwrap_or(0) + 1
}

fn save_todos(project_dir: &Path, items: &[TodoItem], extra_lines: &[String]) {
    let path = todo_file_path(project_dir);
    let content = common::rebuild_todo_file(items, extra_lines);
    if let Err(e) = fs::write(&path, content) {
        eprintln!("{} 写入 TODO.md 失败: {e}", ui::icon_fail());
        std::process::exit(1);
    }
}

fn todo_list(project_dir: &Path, all: bool) {
    let content = load_or_create(project_dir);
    let parsed = common::parse_todos(&content);
    let items = &parsed.items;

    let pending: Vec<&TodoItem> = items.iter().filter(|i| !i.done).collect();
    let done: Vec<&TodoItem> = items.iter().filter(|i| i.done).collect();

    if pending.is_empty() && (!all || done.is_empty()) {
        ui::empty_msg("暂无 TODO");
        return;
    }

    println!("{}", ui::styled_bold(&format!("TODO ({} 项待办)", pending.len())));
    println!("{}", "═".repeat(30));
    println!();

    for item in &pending {
        println!("  {}  {:<40} {}", ui::styled_cyan(&format!("#{}", item.id)), item.content, ui::styled_dim(&item.created));
    }

    if all && !done.is_empty() {
        println!("\n{}", ui::styled_dim("已完成"));
        for item in &done {
            println!("  {}  {}", ui::styled_dim(&format!("#{}", item.id)), ui::styled_dim(&item.content));
        }
    }
}

fn todo_add(project_dir: &Path, content: &str) {
    let file_content = load_or_create(project_dir);
    let parsed = common::parse_todos(&file_content);
    let mut items = parsed.items;
    let id = next_id(&items);
    let today = Local::now().format("%Y-%m-%d").to_string();

    items.push(TodoItem {
        id,
        content: content.to_string(),
        created: today.clone(),
        completed: None,
        done: false,
    });

    save_todos(project_dir, &items, &parsed.extra_lines);
    println!("{} 已添加 TODO #{}: {}", ui::icon_ok(), id, content);
}

fn todo_done(project_dir: &Path, id: u32) {
    let file_content = load_or_create(project_dir);
    let parsed = common::parse_todos(&file_content);
    let mut items = parsed.items;

    let item = items.iter_mut().find(|i| i.id == id);
    match item {
        None => {
            println!("{} TODO #{id} 不存在", ui::icon_fail());
        }
        Some(item) if item.done => {
            println!("{} TODO #{id} 已完成", ui::styled_dim("·"));
        }
        Some(item) => {
            let today = Local::now().format("%Y-%m-%d").to_string();
            item.done = true;
            item.completed = Some(today);
            save_todos(project_dir, &items, &parsed.extra_lines);
            println!("{} TODO #{id} 已标记完成", ui::icon_ok());
        }
    }
}

fn todo_remove(project_dir: &Path, id: u32) {
    let file_content = load_or_create(project_dir);
    let parsed = common::parse_todos(&file_content);
    let mut items = parsed.items;

    let len_before = items.len();
    items.retain(|i| i.id != id);

    if items.len() == len_before {
        println!("{} TODO #{id} 不存在", ui::icon_fail());
    } else {
        save_todos(project_dir, &items, &parsed.extra_lines);
        println!("{} TODO #{id} 已删除", ui::icon_ok());
    }
}
