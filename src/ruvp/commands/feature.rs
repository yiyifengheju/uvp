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
    create_feature_files(&feat_dir, &feat_id, title, &project_dir, adr_ref);

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
    println!("  状态: {}", ui::styled_cyan("planned"));
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
        ui::empty_msg("-- 没有特性");
        return;
    }

    // 表头
    println!("{:<10} {:<30} {:<15} {:<10} {:<12} {:<12}", "编号", "标题", "状态", "关联ADR", "创建日期", "更新日期");
    println!("{}", "─".repeat(89));

    for feat in &features {
        let emoji = common::status_emoji(&feat.status);
        let title_display = if feat.title.chars().count() > 28 {
            let truncated: String = feat.title.chars().take(26).collect();
            format!("{truncated}…")
        } else {
            feat.title.clone()
        };
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
    println!("{}: {}", ui::styled_bold(&feat.id), &feat.title);
    println!("  状态: {emoji} {}", feat.status);
    println!("  关联 ADR: {}", feat.adr);
    println!("  创建日期: {}", feat.created);
    println!("  更新日期: {}", feat.updated);
    println!("  目录: {}", feat.directory);

    // 显示 spec.md 摘要
    let spec_path = project_dir.join(&feat.directory).join("spec.md");
    if spec_path.exists() {
        if let Ok(content) = fs::read_to_string(&spec_path) {
            println!("\n{} spec.md 摘要:", ui::styled_dim(">>"));
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
    if !common::ALL_STATUSES.contains(&new_status) {
        println!("{} 非法状态 '{new_status}'", ui::icon_fail());
        println!("  合法状态: {}", common::ALL_STATUSES.join(", "));
        return;
    }

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
            println!("{} 状态已更新: {feat_id} → {}", ui::icon_ok(), ui::styled_cyan(&new_status_owned));

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
                            .replace("## 当前状态\n\nimplementing", "## 当前状态\n\ndeprecated")
                            .replace("## 当前状态\n\nverifying", "## 当前状态\n\ndeprecated");
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
    feature_status(feat_id, "closed");
}

fn feature_archive(feat_id: &str) {
    feature_status(feat_id, "deprecated");
}

fn create_feature_files(feat_dir: &std::path::Path, feat_id: &str, title: &str, project_dir: &std::path::Path, adr_ref: Option<&str>) {
    let _ = fs::create_dir_all(feat_dir);
    let today = Local::now().format("%Y-%m-%d").to_string();

    let files = [
        ("spec.md", format_spec(feat_id, title, &today, adr_ref)),
        ("changelog.md", format_changelog(feat_id, title, &today)),
        ("verification.md", format_verification(feat_id, title, &today)),
        ("context.md", format_context(feat_id, title, &today)),
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

fn format_spec(feat_id: &str, title: &str, date: &str, adr_ref: Option<&str>) -> String {
    let related_adr = match adr_ref {
        Some(adr) => format!("\"{adr}\""),
        None => "null".to_string(),
    };
    format!(r#"---
doc_type: feature-spec
title: "{feat_id}: {title}"
date: {date}
feat_id: "{feat_id}"
status: planned
updated: {date}
related_adr: {related_adr}
---

# {feat_id}: {title}

## 概述

<!-- 一句话描述此特性的目标 -->

## 决策记录

<!-- 人工与 AI 的问答决策结论。记录"决定做什么、为什么这样选、否决了什么" -->

## 验收标准

<!-- 量化标准，如：精度 ≥ 95%、延迟 < 20ms、支持 1000 QPS -->

## 接口定义

<!-- 描述 API、CLI 参数、配置项等 -->

## 行为规格

<!-- 描述正常流程、边界条件、错误处理 -->

## 约束

<!-- 性能、安全、兼容性等约束 -->
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
metrics: {{}}
figures: []
repro_cmd: ""
---

# {feat_id} Verification

## 验证状态

未验证

## 验收标准

<!-- 从 spec.md 中提取的验收标准 -->

## 验证证据（按适用勾选，算法类至少含 ★）

- [ ] ★ 量化指标表（Se/PPV/准确率/混淆矩阵… 对标基准）
- [ ] ★ 可复现命令（一行能重跑出下列结果）
- [ ] 可视化（结果图，存 images/，嵌入本文件）— 算法/信号/分类类默认要
- [ ] 失败样例/边界样例（不只报成功）
- [ ] 与基准/既有方案对比

## 量化结果

<!-- 填写指标表，同步更新 front matter metrics 字段 -->

## 可复现命令

```bash
# 同步更新 front matter repro_cmd 字段
```

## 可视化

<!-- 图像放 images/ 目录，同步更新 front matter figures 字段 -->

## 验证结论

<!-- ✅ 通过 / ❌ 未通过 / 🔶 有条件通过 -->
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

> 交给 AI 的既定事实上下文。Distill 步骤维护，关闭时填写。
> AI 读取此文件即可获得最短上手路径，无需翻阅历史对话。

## 当前状态

planned

## 既定事实

<!-- 当前生效的约束、选型结论、架构要点——只记"是什么"，不记"为什么"（为什么在 spec 决策记录） -->

## 要点摘要

<!-- 实现中的关键技术点、易踩坑的地方 -->

## 上手指南

<!-- 未来维护者需要知道的最少信息 -->
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

> 可选文件。简单特性可直接指向 ADR Actions，无需填满此文件。

## 实施步骤

1.

## 风险与依赖

<!-- 潜在风险和外部依赖（如无可删除本节） -->
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
    let new_content = if let Some(marker_pos) = content.find(marker) {
        let before = &content[..marker_pos];
        let after_marker_start = marker_pos + marker.len();
        let after_marker = &content[after_marker_start..];
        // 跳过 marker 之后连续的非空行（即旧的特性列表），保留后续内容
        let rest = after_marker
            .find("\n\n")
            .map(|pos| &after_marker[pos..])
            .unwrap_or("");
        format!("{before}{marker}\n{features_text}{rest}")
    } else {
        format!("{content}\n\n### 活跃特性列表\n{marker}\n{features_text}\n")
    };

    let _ = fs::write(&ai_context_path, new_content);
}
