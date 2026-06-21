//! 通用 UI 辅助模块 — uv 风格输出（单行 spinner 滚动）

use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

// ── 样式 ─────────────────────────────────────────────

fn spinner_tpl() -> ProgressStyle {
    ProgressStyle::with_template("{spinner:.cyan} {wide_msg}")
        .unwrap()
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
}

/// 每个子项后的延迟（ms），让用户看清过程
const DEFAULT_STEP_DELAY_MS: u64 = 120;

/// 获取配置的延迟时间（ms）
pub fn get_delay_ms() -> u64 {
    if let Some(home) = dirs::home_dir() {
        let config_path = home.join(".uvp").join("uvp.toml");
        if let Ok(content) = std::fs::read_to_string(&config_path) {
            if let Ok(value) = toml::from_str::<toml::Value>(&content) {
                if let Some(ui) = value.get("ui") {
                    if let Some(delay) = ui.get("delay_ms") {
                        if let Some(ms) = delay.as_integer() {
                            if ms > 0 { return ms as u64; }
                        }
                    }
                }
            }
        }
    }
    DEFAULT_STEP_DELAY_MS
}

// ════════════════════════════════════════════════════
// Spinner API（用于 init 等多步骤命令）
// ════════════════════════════════════════════════════

/// 创建旋转中的进度条（stderr 输出）
pub fn step_start(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(spinner_tpl());
    pb.set_draw_target(indicatif::ProgressDrawTarget::stderr());
    pb.set_message(msg.to_string());
    pb.enable_steady_tick(std::time::Duration::from_millis(80));
    pb
}

/// 更新当前行的消息（如 "Creating dir: docs/adr" → "Creating dir: docs/prd"）
pub fn step_update(pb: &ProgressBar, msg: &str) {
    pb.set_message(msg.to_string());
    pb.tick();
    std::thread::sleep(std::time::Duration::from_millis(get_delay_ms()));
}

/// 完成 → ✓ + 最终消息（替换 spinner，无旋转字符）
pub fn step_done(pb: &ProgressBar, msg: &str) {
    pb.set_style(ProgressStyle::with_template("{msg}").unwrap());
    pb.finish_with_message(format!("{} {}", style("✓").green(), msg));
}
pub fn step_skip(pb: &ProgressBar, msg: &str) {
    pb.set_style(ProgressStyle::with_template("{msg}").unwrap());
    pb.finish_with_message(format!("{} {}", style("·").dim(), style(msg).dim()));
}
pub fn step_fail(pb: &ProgressBar, msg: &str) {
    pb.set_style(ProgressStyle::with_template("{msg}").unwrap());
    pb.finish_with_message(format!("{} {}", style("✗").red(), msg));
}

// ════════════════════════════════════════════════════
// 兼容 API（用于非 init 命令，简单 println）
// ════════════════════════════════════════════════════

pub fn icon_ok() -> &'static str { "✓" }
pub fn icon_skip() -> &'static str { "·" }
pub fn icon_fail() -> &'static str { "✗" }

pub fn step_header(_n: usize, title: &str, _total: usize) {
    println!("\n{}", style(title).bold());
}

pub fn file_created(p: &str)  { println!("  {} {}", style("✓").green(), style(p).cyan()); }
pub fn file_exists(p: &str)   { println!("  {} {}", style("=").dim(),  style(p).dim()); }
pub fn file_skipped(p: &str)  { println!("  {} {}", style("-").dim(),  style(p).dim()); }
pub fn dir_created(p: &str)   { println!("  {} {}/", style("✓").green(), style(p).cyan()); }
pub fn dir_exists(p: &str)    { println!("  {} {}/", style("=").dim(),  style(p).dim()); }

pub fn action_ok(m: &str)   { println!("  {} {}", style("✓").green(), m); }
pub fn action_skip(m: &str)  { println!("  {} {}", style("·").dim(),  style(m).dim()); }
pub fn action_fail(m: &str)  { println!("  {} {}", style("✗").red(), m); }
pub fn action_info(m: &str)  { println!("  → {m}"); }

pub fn success_panel(title: &str, body: &str) {
    println!("\n{}", style(title).bold().green());
    for line in body.lines() {
        if !line.is_empty() { println!("  {}", style(line).dim()); } else { println!(); }
    }
}
pub fn info_panel(title: &str, body: &str) {
    println!("\n{}", style(title).bold().cyan());
    for line in body.lines() {
        if !line.is_empty() { println!("  {}", style(line).dim()); } else { println!(); }
    }
}

pub fn write_file(path: &std::path::Path, content: &str, project_dir: Option<&std::path::Path>) {
    let rel = match project_dir {
        Some(pd) => path.strip_prefix(pd).unwrap_or(path).to_string_lossy().to_string(),
        None => path.to_string_lossy().to_string(),
    };
    if path.exists() { file_exists(&rel); }
    else {
        if let Some(parent) = path.parent() { let _ = std::fs::create_dir_all(parent); }
        match std::fs::write(path, content) {
            Ok(_) => file_created(&rel),
            Err(e) => action_fail(&format!("写入失败: {rel} - {e}")),
        }
    }
}

// ════════════════════════════════════════════════════
// 后台命令执行（流式输出）
// ════════════════════════════════════════════════════

/// 在后台线程执行命令，每行输出通过 on_line 回调报告
pub fn spawn_command_streaming(
    program: &str,
    args: &[&str],
    working_dir: Option<&std::path::Path>,
    on_line: impl Fn(&str) + Send + 'static,
) -> std::thread::JoinHandle<bool> {
    let mut cmd = Command::new(program);
    cmd.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());
    if let Some(cwd) = working_dir { cmd.current_dir(cwd); }

    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("✗ Failed to start `{program}`: {e}");
            return std::thread::spawn(|| false);
        }
    };

    let stdout = child.stdout.take().expect("stdout pipe");
    let stderr = child.stderr.take().expect("stderr pipe");

    std::thread::spawn(move || {
        for line in BufReader::new(stdout).lines().flatten() { on_line(&line); }
        for line in BufReader::new(stderr).lines().flatten() { on_line(&line); }
        child.wait().map(|s| s.success()).unwrap_or(false)
    })
}
