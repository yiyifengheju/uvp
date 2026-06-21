mod config;
mod ui;
mod common;
mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "uvp", version = "2026.6.0", about = "UVP - Vibe Coding 初始化工具")]
struct Cli {
    /// 静默模式
    #[arg(short, long)]
    quiet: bool,

    /// 详细输出
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 初始化项目目录结构和配置
    Init {
        /// 项目路径
        path: Option<String>,
        /// 项目名称
        #[arg(long)]
        name: Option<String>,
        /// 目标 IDE
        #[arg(long, default_value = "claude")]
        ide: String,
        /// 跳过 uv init
        #[arg(long)]
        no_python: bool,
        /// 跳过 mkdocs.yml 创建
        #[arg(long)]
        no_mkdocs: bool,
        /// 跳过 AI 规则文件生成
        #[arg(long)]
        no_ai_rules: bool,
    },
    /// 创建架构决策记录（ADR）
    #[command(alias = "a")]
    Adr {
        /// ADR 标题
        title: String,
        /// 从 Obsidian Vault 读取匹配关键词的笔记
        #[arg(long)]
        from_obsidian: Option<String>,
        /// 初始状态
        #[arg(short, long, default_value = "proposed")]
        status: String,
        /// 创建后用默认编辑器打开
        #[arg(short, long)]
        open: bool,
    },
    /// 管理特性生命周期（Feature Ledger）
    #[command(alias = "f")]
    Feature {
        #[command(subcommand)]
        command: FeatureCommands,
    },
    /// 生成指定 AI 工具的规则文件并部署 skills
    Ide {
        /// IDE 工具名称
        tool: String,
    },
    /// Obsidian 知识导入工具
    #[command(alias = "o")]
    Obsidian {
        #[command(subcommand)]
        command: ObsidianCommands,
        /// Obsidian Vault 路径
        #[arg(long)]
        vault: Option<String>,
    },
    /// 展示项目状态
    #[command(alias = "s", after_help = "6 步闭环工作流:\n  1. Decide    → uvp adr \"标题\"\n  2. Define    → uvp feature new \"功能名\"\n  3. Plan      → 编辑 FEAT-xxx/plan.md\n  4. Implement → 修改 src/ + 更新 changelog.md\n  5. Verify    → uvp feature close FEAT-NNN\n  6. Distill   → 更新 context.md + PROJECT_STATE.md")]
    Status {
        /// 打开 HTML 报告
        #[arg(long)]
        open: bool,
        /// 启动在线实时状态面板
        #[arg(long)]
        onboard: bool,
    },
    /// 检查文档一致性与特性闭环
    #[command(alias = "c")]
    Check {
        /// 仅检查特性闭环
        #[arg(long)]
        features: bool,
        /// 仅检查 ADR 一致性
        #[arg(long)]
        adr: bool,
        /// 自动修复
        #[arg(long)]
        fix: bool,
    },
    /// 渲染 Registry 为 mkdocs 页面
    #[command(alias = "r")]
    Render {
        /// 仅渲染 Feature Registry 页面
        #[arg(long)]
        features: bool,
        /// 仅渲染 ADR Registry 页面
        #[arg(long)]
        adr: bool,
        /// 仅检查一致性
        #[arg(long)]
        check: bool,
    },
    /// 显示当前合并配置
    #[command(alias = "cfg")]
    Config,
    /// 管理项目想法和待办
    #[command(alias = "td")]
    Todo {
        #[command(subcommand)]
        command: Option<TodoCommands>,
        /// 显示所有 TODO（含已完成）
        #[arg(long)]
        all: bool,
    },
}

#[derive(Subcommand)]
enum FeatureCommands {
    /// 创建新特性
    New {
        /// 特性标题
        title: String,
        /// 关联的 ADR 编号
        #[arg(long)]
        adr: Option<String>,
    },
    /// 列出所有特性
    List {
        /// 按状态筛选
        #[arg(long)]
        status: Option<String>,
    },
    /// 显示特性详情
    Show {
        /// 特性 ID
        feat_id: String,
    },
    /// 更新特性状态
    Status {
        /// 特性 ID
        feat_id: String,
        /// 新状态
        new_status: String,
    },
    /// 关闭特性（标记为 verified）
    Close {
        /// 特性 ID
        feat_id: String,
    },
    /// 将特性标记为 deprecated
    Archive {
        /// 特性 ID
        feat_id: String,
    },
}

#[derive(Subcommand)]
enum ObsidianCommands {
    /// 从 Obsidian 拉取 reference/ 素材到项目
    Pull {
        /// 仅显示将要拉取的文件
        #[arg(long)]
        dry_run: bool,
    },
    /// 双向同步 reference/（项目 ↔ Vault）
    Sync {
        /// 仅显示将要同步的文件
        #[arg(long)]
        dry_run: bool,
    },
}

#[derive(Subcommand)]
enum TodoCommands {
    /// 添加一条 TODO
    Add {
        /// TODO 内容
        content: String,
    },
    /// 标记完成
    Done {
        /// TODO 编号
        id: u32,
    },
    /// 删除一条 TODO
    Remove {
        /// TODO 编号
        id: u32,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path, name, ide, no_python, no_mkdocs, no_ai_rules } => {
            commands::init::run(path.as_deref(), name.as_deref(), &ide, no_python, no_mkdocs, no_ai_rules);
        }
        Commands::Adr { title, from_obsidian, status, open } => {
            commands::adr::run(&title, from_obsidian.as_deref(), &status, open);
        }
        Commands::Feature { command } => {
            commands::feature::run(command);
        }
        Commands::Ide { tool } => {
            commands::ide_cmd::run(&tool);
        }
        Commands::Obsidian { command, vault } => {
            commands::obsidian::run(command, vault.as_deref());
        }
        Commands::Status { open, onboard } => {
            commands::status::run(open, onboard);
        }
        Commands::Check { features, adr, fix } => {
            commands::check::run(features, adr, fix);
        }
        Commands::Render { features, adr, check } => {
            commands::render::run(features, adr, check);
        }
        Commands::Config => {
            commands::config_cmd::run();
        }
        Commands::Todo { command, all } => {
            commands::todo_cmd::run(command, all);
        }
    }
}
