//! uvp todo 命令 — 管理项目想法和待办

use std::fs;
use std::path::Path;

use chrono::Local;
use regex::Regex;

use crate::config;
use crate::TodoCommands;
use crate::ui;

struct TodoItem {
    id: u32,
    content: String,
    created: String,
    completed: Option<String>,
    done: bool,
}

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

fn parse_todos(content: &str) -> Vec<TodoItem> {
    let re = Regex::new(r"^- \[([ x])\] #(\d+) (.+?) <!-- (.+?) -->").unwrap();
    let mut items = Vec::new();
    for line in content.lines() {
        if let Some(caps) = re.captures(line) {
            let done = &caps[1] == "x";
            let id: u32 = caps[2].parse().unwrap_or(0);
            let text = caps[3].to_string();
            let date_str = caps[4].to_string();
            let (created, completed) = if done {
                let parts: Vec<&str> = date_str.splitn(2, " → ").collect();
                if parts.len() == 2 {
                    (parts[0].trim().to_string(), Some(parts[1].trim().to_string()))
                } else {
                    (date_str.clone(), Some(date_str))
                }
            } else {
                (date_str, None)
            };
            items.push(TodoItem { id, content: text, created, completed, done });
        }
    }
    items
}

fn next_id(items: &[TodoItem]) -> u32 {
    items.iter().map(|i| i.id).max().unwrap_or(0) + 1
}

fn rebuild_file(items: &[TodoItem]) -> String {
    let mut lines = Vec::new();
    lines.push("# TODO".to_string());
    lines.push(String::new());
    lines.push("> 项目想法、待验证方向、灵感收集。成熟后转为 ADR 或 Feature。".to_string());
    lines.push(String::new());
    lines.push("## 待办".to_string());
    lines.push(String::new());

    for item in items.iter().filter(|i| !i.done) {
        lines.push(format!("- [ ] #{} {} <!-- {} -->", item.id, item.content, item.created));
    }

    lines.push(String::new());
    lines.push("## 已完成".to_string());
    lines.push(String::new());

    for item in items.iter().filter(|i| i.done) {
        let date = match &item.completed {
            Some(c) => format!("{} → {}", item.created, c),
            None => item.created.clone(),
        };
        lines.push(format!("- [x] #{} {} <!-- {} -->", item.id, item.content, date));
    }

    lines.push(String::new());
    lines.join("\n")
}

fn save_todos(project_dir: &Path, items: &[TodoItem]) {
    let path = todo_file_path(project_dir);
    let content = rebuild_file(items);
    if let Err(e) = fs::write(&path, content) {
        eprintln!("{} 写入 TODO.md 失败: {e}", ui::icon_fail());
        std::process::exit(1);
    }
}

fn todo_list(project_dir: &Path, all: bool) {
    let content = load_or_create(project_dir);
    let items = parse_todos(&content);

    let pending: Vec<&TodoItem> = items.iter().filter(|i| !i.done).collect();
    let done: Vec<&TodoItem> = items.iter().filter(|i| i.done).collect();

    if pending.is_empty() && (!all || done.is_empty()) {
        println!("{}", console::style("暂无 TODO").dim());
        return;
    }

    println!("{}", console::style(format!("TODO ({} 项待办)", pending.len())).bold());
    println!("{}", "═".repeat(30));
    println!();

    for item in &pending {
        println!("  {}  {:<40} {}", console::style(format!("#{}", item.id)).cyan(), item.content, console::style(&item.created).dim());
    }

    if all && !done.is_empty() {
        println!("\n{}", console::style("已完成").dim());
        for item in &done {
            println!("  {}  {}", console::style(format!("#{}", item.id)).dim(), console::style(&item.content).dim());
        }
    }
}

fn todo_add(project_dir: &Path, content: &str) {
    let file_content = load_or_create(project_dir);
    let mut items = parse_todos(&file_content);
    let id = next_id(&items);
    let today = Local::now().format("%Y-%m-%d").to_string();

    items.push(TodoItem {
        id,
        content: content.to_string(),
        created: today.clone(),
        completed: None,
        done: false,
    });

    save_todos(project_dir, &items);
    println!("{} 已添加 TODO #{}: {}", ui::icon_ok(), id, content);
}

fn todo_done(project_dir: &Path, id: u32) {
    let file_content = load_or_create(project_dir);
    let mut items = parse_todos(&file_content);

    let item = items.iter_mut().find(|i| i.id == id);
    match item {
        None => {
            println!("{} TODO #{id} 不存在", ui::icon_fail());
        }
        Some(item) if item.done => {
            println!("{} TODO #{id} 已完成", console::style("·").dim());
        }
        Some(item) => {
            let today = Local::now().format("%Y-%m-%d").to_string();
            item.done = true;
            item.completed = Some(today);
            save_todos(project_dir, &items);
            println!("{} TODO #{id} 已标记完成", ui::icon_ok());
        }
    }
}

fn todo_remove(project_dir: &Path, id: u32) {
    let file_content = load_or_create(project_dir);
    let mut items = parse_todos(&file_content);

    let len_before = items.len();
    items.retain(|i| i.id != id);

    if items.len() == len_before {
        println!("{} TODO #{id} 不存在", ui::icon_fail());
    } else {
        save_todos(project_dir, &items);
        println!("{} TODO #{id} 已删除", ui::icon_ok());
    }
}
