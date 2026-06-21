//! uvp feature 命令 — 管理特性生命周期

use std::fs;

use chrono::Local;

use crate::common::{self, FeatureEntry, FeatureRegistry};
use crate::config;
use crate::ui;

pub fn run(command: crate::FeatureCommands) {
    match command {
        crate::FeatureCommands::New { title, adr } => feature_new(&title, adr.as_deref()),
        crate::FeatureCommands::List { status } => feature_list(status.as_deref()),
        crate::FeatureCommands::Show { feat_id } => feature_show(&feat_id),
        crate::FeatureCommands::Status { feat_id, new_status } => feature_status(&feat_id, &new_status),
        crate::FeatureCommands::Close { feat_id } => feature_close(&feat_id),
        crate::FeatureCommands::Archive { feat_id } => feature_archive(&feat_id),
    }
}

fn get_project_dir() -> std::path::PathBuf {
    config::find_project_root(None, true).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    })
}

fn feature_new(title: &str, adr_ref: Option<&str>) {
    let project_dir = get_project_dir();
    let cfg = config::get_effective_config(&project_dir);

    let mut data = common::load_feature_registry(&project_dir, &cfg);
    let number = common::get_next_feature_number(&data);
    let feat_id = format!("FEAT-{number:03}");
    let slug = common::title_to_dirname(title);
    let feat_dirname = format!("{feat_id}-{slug}");

    let feature_base = project_dir.join(&cfg.feature.directory);
    let feat_dir = feature_base.join(&feat_dirname);

    let pb = ui::step_start(&format!("Creating {feat_id}: {title}"));

    // 创建 Feature 目录和文件
    create_feature_files(&feat_dir, &feat_id, title, &project_dir);

    // 更新 registry
    ui::step_update(&pb, "Updating feature registry");
    let now = Local::now().format("%Y-%m-%d").to_string();
    let new_feature = FeatureEntry {
        id: feat_id.clone(),
        title: title.to_string(),
        status: "planned".into(),
        directory: format!("docs/features/{feat_dirname}"),
        adr: adr_ref.unwrap_or("-").to_string(),
        created: now.clone(),
        updated: now,
    };
    data.features.push(new_feature);
    common::save_feature_registry(&project_dir, &cfg, &data);

    // 更新 AI_CONTEXT.md
    ui::step_update(&pb, "Updating AI_CONTEXT.md");
    update_ai_context_features(&project_dir, &cfg, &data);

    ui::step_done(&pb, &format!("{feat_id} created: {title}"));
    println!("  目录: docs/features/{feat_dirname}");
    println!("  状态: {}", console::style("planned").cyan());
    if let Some(adr) = adr_ref {
        println!("  关联 ADR: {adr}");
    }
}

fn feature_list(filter_status: Option<&str>) {
    let project_dir = get_project_dir();
    let cfg = config::get_effective_config(&project_dir);

    let data = common::load_feature_registry(&project_dir, &cfg);
    let mut features: Vec<&FeatureEntry> = data.features.iter().collect();

    if let Some(status) = filter_status {
        features.retain(|f| f.status == status);
    }

    if features.is_empty() {
        println!("{}", console::style("-- 没有特性").dim());
        return;
    }

    // 表头
    println!("{:<10} {:<30} {:<15} {:<10} {:<12} {:<12}", "编号", "标题", "状态", "关联ADR", "创建日期", "更新日期");
    println!("{}", "─".repeat(89));

    for feat in &features {
        let emoji = common::status_emoji(&feat.status);
        let title_display = if feat.title.len() > 28 { format!("{}…", &feat.title[..26]) } else { feat.title.clone() };
        println!("{:<10} {:<30} {} {:<12} {:<12} {:<12}",
            feat.id, title_display, emoji, feat.adr, feat.created, feat.updated);
    }
}

fn feature_show(feat_id: &str) {
    let project_dir = get_project_dir();
    let cfg = config::get_effective_config(&project_dir);

    let data = common::load_feature_registry(&project_dir, &cfg);
    let feat = match common::find_feature(&data, feat_id) {
        Some(f) => f,
        None => {
            println!("{} 特性 '{feat_id}' 不存在", ui::icon_fail());
            return;
        }
    };

    let emoji = common::status_emoji(&feat.status);
    println!("{}: {}", console::style(&feat.id).bold(), &feat.title);
    println!("  状态: {emoji} {}", feat.status);
    println!("  关联 ADR: {}", feat.adr);
    println!("  创建日期: {}", feat.created);
    println!("  更新日期: {}", feat.updated);
    println!("  目录: {}", feat.directory);

    // 显示 spec.md 摘要
    let spec_path = project_dir.join(&feat.directory).join("spec.md");
    if spec_path.exists() {
        if let Ok(content) = fs::read_to_string(&spec_path) {
            println!("\n{} spec.md 摘要:", console::style(">>").dim());
            let preview: Vec<&str> = content.lines().take(20).collect();
            for line in preview {
                println!("  {line}");
            }
            if content.lines().count() > 20 {
                println!("  ...");
            }
        }
    }
}

fn feature_status(feat_id: &str, new_status: &str) {
    let project_dir = get_project_dir();
    let cfg = config::get_effective_config(&project_dir);

    let mut data = common::load_feature_registry(&project_dir, &cfg);
    let now = Local::now().format("%Y-%m-%d").to_string();

    match common::find_feature_mut(&mut data, feat_id) {
        Some(feat) => {
            let new_status_owned = new_status.to_string();
            let directory = feat.directory.clone();
            feat.status = new_status_owned.clone();
            feat.updated = now;
            common::save_feature_registry(&project_dir, &cfg, &data);
            println!("{} 状态已更新: {feat_id} → {}", ui::icon_ok(), console::style(&new_status_owned).cyan());

            // 如果标记为 verified，更新 verification.md
            if new_status_owned == "verified" {
                let feat_dir = project_dir.join(&directory);
                let verif_path = feat_dir.join("verification.md");
                if verif_path.exists() {
                    if let Ok(content) = fs::read_to_string(&verif_path) {
                        let content = content.replace("未验证", "✅ 已验证");
                        let _ = fs::write(&verif_path, content);
                    }
                }
            }

            // 如果标记为 deprecated，更新 context.md
            if new_status_owned == "deprecated" {
                let feat_dir = project_dir.join(&directory);
                let context_path = feat_dir.join("context.md");
                if context_path.exists() {
                    if let Ok(content) = fs::read_to_string(&context_path) {
                        let content = content
                            .replace("## 当前状态\n\nplanned", "## 当前状态\n\ndeprecated")
                            .replace("## 当前状态\n\nin_progress", "## 当前状态\n\ndeprecated");
                        let _ = fs::write(&context_path, content);
                    }
                }
            }

            println!("提示: 运行 uvp render 更新 features/index.md");
        }
        None => {
            println!("{} 特性 '{feat_id}' 不存在", ui::icon_fail());
        }
    }
}

fn feature_close(feat_id: &str) {
    feature_status(feat_id, "verified");
}

fn feature_archive(feat_id: &str) {
    feature_status(feat_id, "deprecated");
}

fn create_feature_files(feat_dir: &std::path::Path, feat_id: &str, title: &str, project_dir: &std::path::Path) {
    let _ = fs::create_dir_all(feat_dir);
    let today = Local::now().format("%Y-%m-%d").to_string();

    let files = [
        ("spec.md", format_spec(feat_id, title, &today)),
        ("changelog.md", format_changelog(feat_id, title, &today)),
        ("verification.md", format_verification(feat_id, title, &today)),
        ("context.md", format_context(feat_id, title, &today)),
        ("deliverables.md", format_deliverables(feat_id, title, &today)),
        ("plan.md", format_plan(feat_id, title, &today)),
    ];

    for (filename, content) in &files {
        let path = feat_dir.join(filename);
        let rel = path.strip_prefix(project_dir).unwrap_or(&path).to_string_lossy();
        if path.exists() {
            ui::file_exists(&rel);
        } else {
            match fs::write(&path, content) {
                Ok(_) => ui::file_created(&rel),
                Err(e) => ui::action_fail(&format!("写入 {rel} 失败: {e}")),
            }
        }
    }
}

fn format_spec(feat_id: &str, title: &str, date: &str) -> String {
    format!(r#"---
doc_type: feature-spec
title: "{feat_id}: {title}"
date: {date}
feat_id: "{feat_id}"
status: planned
updated: {date}
related_adr: null
---

# {feat_id}: {title}

## 概述

<!-- 一句话描述此特性的目标 -->

## 验收标准

<!-- 量化标准，如：精度 ≥ 95%、延迟 < 20ms、支持 1000 QPS -->

## 接口定义

<!-- 描述 API、CLI 参数、配置项等 -->

## 行为规格

<!-- 描述正常流程、边界条件、错误处理 -->

## 约束

<!-- 描述性能、安全、兼容性等约束 -->
"#)
}

fn format_changelog(feat_id: &str, _title: &str, date: &str) -> String {
    format!(r#"---
doc_type: feature-changelog
title: "{feat_id} Changelog"
date: {date}
feat_id: "{feat_id}"
updated: {date}
---

# {feat_id} Changelog

> 记录代码变更：改了什么、为什么改、影响范围。

| 日期 | 类型 | 变更 | 说明 |
|------|------|------|------|
| {date} | init | 创建特性 | 初始化特性 |
"#)
}

fn format_verification(feat_id: &str, _title: &str, date: &str) -> String {
    format!(r#"---
doc_type: feature-verification
title: "{feat_id} Verification"
date: {date}
feat_id: "{feat_id}"
updated: {date}
---

# {feat_id} Verification

## 验证状态

未验证

## 验收标准

<!-- 从 spec.md 中提取的验收标准 -->

## 测试用例

<!-- 列出验证此特性的测试用例 -->

## 验证结果

<!-- 记录验证结果 -->
"#)
}

fn format_context(feat_id: &str, _title: &str, date: &str) -> String {
    format!(r#"---
doc_type: feature-context
title: "{feat_id} Context"
date: {date}
feat_id: "{feat_id}"
updated: {date}
---

# {feat_id} Context

> 此文件为 AI 提供压缩上下文，由 Distill 步骤维护。

## 当前状态

planned

## 目标摘要

<!-- 一句话描述此特性要达成什么 -->

## 关键决策

（待填写）

## 实现要点

（待填写）

## 最终结论

<!-- 特性关闭后填写 -->
"#)
}

fn format_deliverables(feat_id: &str, _title: &str, date: &str) -> String {
    format!(r#"---
doc_type: feature-deliverables
title: "{feat_id} Deliverables"
date: {date}
feat_id: "{feat_id}"
updated: {date}
---

# {feat_id} Deliverables

> 记录结构化产出：实验结果、模型指标、关键数据、产出物路径。
> 周报生成时自动读取此文件。

## 产出记录

<!-- 每条产出用三级标题，包含日期、类型、结果。示例：

### {date} | 模型评估 | baseline

- **方法**：使用 XX 模型 + YY 数据集
- **结果**：准确率 92.3%，F1 0.91
- **结论**：达到基线要求，可进入下一阶段
- **产出物**：`experiments/baseline/results.json`

-->
"#)
}

fn format_plan(feat_id: &str, _title: &str, date: &str) -> String {
    format!(r#"---
doc_type: feature-plan
title: "{feat_id} Plan"
date: {date}
feat_id: "{feat_id}"
updated: {date}
---

# {feat_id} Plan

## 实施步骤

<!-- 列出实施步骤 -->

1.

## 风险评估

<!-- 列出潜在风险 -->

## 依赖

<!-- 列出依赖的其他特性或外部条件 -->
"#)
}

fn update_ai_context_features(project_dir: &std::path::Path, _cfg: &config::UvpConfig, data: &FeatureRegistry) {
    let ai_context_path = project_dir.join("docs/AI_CONTEXT.md");
    if !ai_context_path.exists() {
        return;
    }

    let content = match fs::read_to_string(&ai_context_path) {
        Ok(c) => c,
        Err(_) => return,
    };

    let active_features: Vec<&FeatureEntry> = data.features.iter()
        .filter(|f| f.status != "deprecated" && f.status != "removed")
        .collect();

    let features_text = if active_features.is_empty() {
        "- （无活跃特性）".to_string()
    } else {
        active_features.iter()
            .map(|f| format!("- {}: {} ({})", f.id, f.title, f.status))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let marker = "<!-- 此列表由 uvp feature new/close 自动维护，不要手动编辑 -->";
    let new_content = if content.contains(marker) {
        let re = regex::Regex::new(&format!("(?s){}.*?(?=\n\n|\n$|$)", regex::escape(marker))).unwrap();
        re.replace(&content, &format!("{marker}\n{features_text}")).to_string()
    } else {
        format!("{content}\n\n### 活跃特性列表\n{marker}\n{features_text}\n")
    };

    let _ = fs::write(&ai_context_path, new_content);
}
