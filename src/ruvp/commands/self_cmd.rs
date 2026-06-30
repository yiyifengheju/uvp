use console::style;
use self_update::backends::github::Update;
use self_update::cargo_crate_version;

pub fn update() {
    println!("{} 检查更新...", style("[UPDATE]").cyan().bold());

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
            eprintln!("{} 配置更新失败: {}", style("[FAIL]").red().bold(), e);
            std::process::exit(1);
        }
    };

    match updater.update() {
        Ok(status) => {
            let latest = status.version();
            if latest == current {
                println!("  {} 已是最新版本 v{}", style("[OK]").green().bold(), current);
            } else {
                println!(
                    "  {} 已更新: v{} → v{}",
                    style("[OK]").green().bold(),
                    current,
                    latest
                );
            }
        }
        Err(e) => {
            eprintln!("{} 更新失败: {}", style("[FAIL]").red().bold(), e);
            std::process::exit(1);
        }
    }
}
