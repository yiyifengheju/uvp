use self_update::backends::github::Update;
use self_update::cargo_crate_version;

use crate::ui;

pub fn update() {
    println!("{} 检查更新...", ui::styled_bold_cyan("[UPDATE]"));

    let current = cargo_crate_version!();
    println!("  当前版本: v{}", current);

    let result = Update::configure()
        .repo_owner("yiyifengheju")
        .repo_name("uvp")
        .bin_name("uvp")
        .show_download_progress(true)
        .current_version(current)
        .build();

    let updater = match result {
        Ok(u) => u,
        Err(e) => {
            eprintln!("{} 配置更新失败: {e}", ui::styled_red("[FAIL]"));
            std::process::exit(1);
        }
    };

    match updater.update() {
        Ok(status) => {
            let latest = status.version();
            if latest == current {
                println!("  {} 已是最新版本 v{}", ui::styled_green("[OK]"), current);
            } else {
                println!(
                    "  {} 已更新: v{} → v{}",
                    ui::styled_green("[OK]"),
                    current,
                    latest
                );
            }
        }
        Err(e) => {
            eprintln!("{} 更新失败: {e}", ui::styled_red("[FAIL]"));
            std::process::exit(1);
        }
    }
}
