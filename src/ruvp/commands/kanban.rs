//! uvp kanban 命令 — 全局看板 Web 服务器
//!
//! 启动本地 HTTP 服务器，聚合展示所有注册项目的 TODO/Feature/ADR/Roadmap。

use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use axum::extract::{Path as AxumPath, State};
use axum::http::{header, StatusCode, Uri};
use axum::response::{IntoResponse, Json, Response};
use axum::routing::{delete, get, patch, post};
use axum::Router;
use chrono::Local;
use regex::Regex;
use rust_embed::Embed;
use serde::{Deserialize, Serialize};

use crate::common;
use crate::config;
use crate::ui;

// ── 嵌入前端静态文件 ──────────────────────────────────

#[derive(Embed)]
#[folder = "web/dist/"]
struct WebAssets;

// ── API 数据模型 ──────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
struct ProjectInfo {
    id: usize,
    name: String,
    version: String,
    description: String,
    path: String,
    available: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ProjectOverview {
    project: ProjectInfo,
    todos: Vec<TodoView>,
    features: Vec<FeatureView>,
    adrs: Vec<AdrView>,
    roadmap: Vec<RoadmapView>,
    edges: Vec<Edge>,
}

#[derive(Debug, Clone, Serialize)]
struct TodoView {
    id: u32,
    content: String,
    done: bool,
    created: String,
    completed: Option<String>,
    adr_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct FeatureView {
    id: String,
    title: String,
    status: String,
}

#[derive(Debug, Clone, Serialize)]
struct AdrView {
    id: String,
    title: String,
    status: String,
    related_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct RoadmapView {
    section: String,
    text: String,
    linked_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct Edge {
    from_type: String,
    from_id: String,
    to_type: String,
    to_id: String,
}

#[derive(Debug, Deserialize)]
struct AddTodoRequest {
    content: String,
}

#[derive(Debug, Deserialize)]
struct PatchTodoRequest {
    done: Option<bool>,
}

// ── 应用状态 ──────────────────────────────────────────

struct AppState {
    projects: Vec<ProjectInfo>,
}

// ── 入口 ──────────────────────────────────────────────

pub fn run(port: u16) {
    let global_cfg = config::load_global_config();

    if global_cfg.projects.is_empty() {
        println!("{} 未注册任何项目。请在 ~/.uvp/uvp.toml 中添加：", ui::icon_fail());
        println!();
        println!("  [[projects]]");
        println!("  path = \"/path/to/your/project\"");
        return;
    }

    let projects: Vec<ProjectInfo> = global_cfg
        .projects
        .iter()
        .enumerate()
        .map(|(idx, p)| build_project_info(idx, &p.path))
        .collect();

    let pb = ui::step_start("Starting kanban server");

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let state = Arc::new(AppState {
            projects: projects.clone(),
        });

        let api = Router::new()
            .route("/api/projects", get(api_list_projects))
            .route("/api/projects/{id}/overview", get(api_project_overview))
            .route("/api/projects/{id}/todos", post(api_add_todo))
            .route("/api/projects/{id}/todos/{todo_id}", patch(api_patch_todo))
            .route("/api/projects/{id}/todos/{todo_id}", delete(api_delete_todo))
            .with_state(state);

        let app = api.fallback(static_handler);

        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        ui::step_done(&pb, &format!("Kanban server running at http://localhost:{port}"));

        let url = format!("http://localhost:{port}");
        let _ = open::that(&url);

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap_or_else(|e| {
            eprintln!("{} 端口 {port} 被占用: {e}，请使用 --port 指定其他端口", ui::icon_fail());
            std::process::exit(1);
        });
        axum::serve(listener, app).await.unwrap();
    });
}

// ── 静态文件服务 ──────────────────────────────────────

async fn static_handler(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };

    match WebAssets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref())], content.data.to_vec()).into_response()
        }
        None => {
            // SPA fallback
            match WebAssets::get("index.html") {
                Some(content) => {
                    let body = content.data.to_vec();
                    ([(header::CONTENT_TYPE, "text/html")], body).into_response()
                }
                None => (StatusCode::NOT_FOUND, "Kanban frontend not found").into_response(),
            }
        }
    }
}

// ── API 处理器 ────────────────────────────────────────

async fn api_list_projects(State(state): State<Arc<AppState>>) -> Json<Vec<ProjectInfo>> {
    Json(state.projects.clone())
}

async fn api_project_overview(
    State(state): State<Arc<AppState>>,
    AxumPath(id): AxumPath<usize>,
) -> Result<Json<ProjectOverview>, StatusCode> {
    let project = state.projects.get(id).ok_or(StatusCode::NOT_FOUND)?;
    if !project.available {
        return Err(StatusCode::NOT_FOUND);
    }
    let overview = build_project_overview(project);
    Ok(Json(overview))
}

async fn api_add_todo(
    State(state): State<Arc<AppState>>,
    AxumPath(id): AxumPath<usize>,
    Json(body): Json<AddTodoRequest>,
) -> Result<Json<TodoView>, StatusCode> {
    let project = state.projects.get(id).ok_or(StatusCode::NOT_FOUND)?;
    if !project.available {
        return Err(StatusCode::NOT_FOUND);
    }

    let project_dir = PathBuf::from(&project.path);
    let todo_path = project_dir.join("docs/TODO.md");
    let content = fs::read_to_string(&todo_path).unwrap_or_default();
    let parsed = common::parse_todos(&content);
    let mut items = parsed.items;

    let new_id = items.iter().map(|i| i.id).max().unwrap_or(0) + 1;
    let today = Local::now().format("%Y-%m-%d").to_string();

    // Normalize [adr-xxx] → [ADR-xxx] (case-insensitive, zero-padded to 3 digits)
    let normalized_content = normalize_adr_tags(&body.content);

    let new_item = common::TodoItem {
        id: new_id,
        content: normalized_content.clone(),
        created: today.clone(),
        completed: None,
        done: false,
    };
    items.push(new_item);

    let rebuilt = common::rebuild_todo_file(&items, &parsed.extra_lines);
    fs::write(&todo_path, rebuilt).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let adr_refs = common::extract_todo_adr_refs(&normalized_content);
    Ok(Json(TodoView {
        id: new_id,
        content: normalized_content,
        done: false,
        created: today,
        completed: None,
        adr_refs,
    }))
}

async fn api_patch_todo(
    State(state): State<Arc<AppState>>,
    AxumPath((id, todo_id)): AxumPath<(usize, u32)>,
    Json(body): Json<PatchTodoRequest>,
) -> Result<Json<TodoView>, StatusCode> {
    let project = state.projects.get(id).ok_or(StatusCode::NOT_FOUND)?;
    if !project.available {
        return Err(StatusCode::NOT_FOUND);
    }

    let project_dir = PathBuf::from(&project.path);
    let todo_path = project_dir.join("docs/TODO.md");
    let content = fs::read_to_string(&todo_path).unwrap_or_default();
    let parsed = common::parse_todos(&content);
    let mut items = parsed.items;

    let item = items.iter_mut().find(|i| i.id == todo_id).ok_or(StatusCode::NOT_FOUND)?;

    if let Some(done) = body.done {
        item.done = done;
        if done {
            item.completed = Some(Local::now().format("%Y-%m-%d").to_string());
        } else {
            item.completed = None;
        }
    }

    let result = TodoView {
        id: item.id,
        content: item.content.clone(),
        done: item.done,
        created: item.created.clone(),
        completed: item.completed.clone(),
        adr_refs: common::extract_todo_adr_refs(&item.content),
    };

    let rebuilt = common::rebuild_todo_file(&items, &parsed.extra_lines);
    fs::write(&todo_path, rebuilt).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(result))
}

async fn api_delete_todo(
    State(state): State<Arc<AppState>>,
    AxumPath((id, todo_id)): AxumPath<(usize, u32)>,
) -> Result<StatusCode, StatusCode> {
    let project = state.projects.get(id).ok_or(StatusCode::NOT_FOUND)?;
    if !project.available {
        return Err(StatusCode::NOT_FOUND);
    }

    let project_dir = PathBuf::from(&project.path);
    let todo_path = project_dir.join("docs/TODO.md");
    let content = fs::read_to_string(&todo_path).unwrap_or_default();
    let parsed = common::parse_todos(&content);
    let mut items = parsed.items;

    let len_before = items.len();
    items.retain(|i| i.id != todo_id);
    if items.len() == len_before {
        return Err(StatusCode::NOT_FOUND);
    }

    let rebuilt = common::rebuild_todo_file(&items, &parsed.extra_lines);
    fs::write(&todo_path, rebuilt).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

// ── 数据构建 ──────────────────────────────────────────

fn build_project_info(id: usize, path: &str) -> ProjectInfo {
    let project_dir = PathBuf::from(path);
    if !project_dir.exists() {
        return ProjectInfo {
            id,
            name: path.to_string(),
            version: String::new(),
            description: String::new(),
            path: path.to_string(),
            available: false,
        };
    }

    let (name, version, description) = read_pyproject_info(&project_dir);

    ProjectInfo {
        id,
        name: name.unwrap_or_else(|| project_dir.file_name().unwrap_or_default().to_string_lossy().to_string()),
        version: version.unwrap_or_default(),
        description: description.unwrap_or_default(),
        path: path.to_string(),
        available: true,
    }
}

fn read_pyproject_info(project_dir: &Path) -> (Option<String>, Option<String>, Option<String>) {
    let pyproject_path = project_dir.join("pyproject.toml");
    if !pyproject_path.exists() {
        return (None, None, None);
    }
    let content = match fs::read_to_string(&pyproject_path) {
        Ok(c) => c,
        Err(_) => return (None, None, None),
    };

    #[derive(Deserialize)]
    struct PyProject {
        project: Option<PyProjectMeta>,
    }
    #[derive(Deserialize)]
    struct PyProjectMeta {
        name: Option<String>,
        version: Option<String>,
        description: Option<String>,
    }

    match toml::from_str::<PyProject>(&content) {
        Ok(pp) => {
            let meta = pp.project.unwrap_or(PyProjectMeta { name: None, version: None, description: None });
            (meta.name, meta.version, meta.description)
        }
        Err(_) => (None, None, None),
    }
}

fn build_project_overview(project: &ProjectInfo) -> ProjectOverview {
    let project_dir = PathBuf::from(&project.path);
    let cfg = config::get_effective_config(&project_dir);

    let todos = read_todos(&project_dir);
    let features = read_features(&project_dir, &cfg);
    let adrs = read_adrs(&project_dir, &cfg);
    let roadmap = read_roadmap(&project_dir);

    let edges = build_edges(&todos, &adrs, &features, &roadmap);

    ProjectOverview {
        project: project.clone(),
        todos,
        features,
        adrs,
        roadmap,
        edges,
    }
}

fn read_todos(project_dir: &Path) -> Vec<TodoView> {
    let todo_path = project_dir.join("docs/TODO.md");
    if !todo_path.exists() {
        return Vec::new();
    }
    let content = fs::read_to_string(&todo_path).unwrap_or_default();
    let parsed = common::parse_todos(&content);
    parsed
        .items
        .iter()
        .map(|item| TodoView {
            id: item.id,
            content: item.content.clone(),
            done: item.done,
            created: item.created.clone(),
            completed: item.completed.clone(),
            adr_refs: common::extract_todo_adr_refs(&item.content),
        })
        .collect()
}

fn read_features(project_dir: &Path, cfg: &config::UvpConfig) -> Vec<FeatureView> {
    let data = common::load_feature_registry(project_dir, cfg);
    data.features
        .iter()
        .map(|f| FeatureView {
            id: f.id.clone(),
            title: f.title.clone(),
            status: f.status.clone(),
        })
        .collect()
}

fn read_adrs(project_dir: &Path, cfg: &config::UvpConfig) -> Vec<AdrView> {
    let adr_dir = project_dir.join(&cfg.adr.directory);
    if !adr_dir.exists() {
        return Vec::new();
    }
    let mut adrs = Vec::new();
    if let Ok(entries) = fs::read_dir(&adr_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.ends_with(".md") || name == "template.md" || name == "index.md" {
                continue;
            }
            if let Ok(content) = fs::read_to_string(entry.path()) {
                if let Some(adr) = common::parse_adr_file(&content) {
                    adrs.push(AdrView {
                        id: adr.id,
                        title: adr.title,
                        status: adr.status,
                        related_features: adr.related_features,
                    });
                }
            }
        }
    }
    adrs.sort_by(|a, b| a.id.cmp(&b.id));
    adrs
}

fn read_roadmap(project_dir: &Path) -> Vec<RoadmapView> {
    let candidates = [
        project_dir.join("docs/roadmap.md"),
        project_dir.join("docs/PRD/roadmap.md"),
    ];
    for path in &candidates {
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                return common::parse_roadmap(&content)
                    .into_iter()
                    .map(|r| RoadmapView {
                        section: r.section,
                        text: r.text,
                        linked_features: r.linked_features,
                    })
                    .collect();
            }
        }
    }
    Vec::new()
}

/// Normalize [[adr-1]], [[Adr-01]], [[ADR-001]], [adr-1], [ADR-001] etc. → [[ADR-001]]
fn normalize_adr_tags(content: &str) -> String {
    let re = Regex::new(r"(?i)\[?\[adr-(\d+)\]\]?").unwrap();
    re.replace_all(content, |caps: &regex::Captures| {
        let num: u32 = caps[1].parse().unwrap_or(0);
        format!("[[ADR-{:03}]]", num)
    }).to_string()
}

fn build_edges(
    todos: &[TodoView],
    adrs: &[AdrView],
    _features: &[FeatureView],
    roadmap: &[RoadmapView],
) -> Vec<Edge> {
    let mut edges = Vec::new();

    // TODO → ADR
    for todo in todos {
        for adr_ref in &todo.adr_refs {
            edges.push(Edge {
                from_type: "todo".into(),
                from_id: format!("#{}", todo.id),
                to_type: "adr".into(),
                to_id: adr_ref.clone(),
            });
        }
    }

    // ADR → Feature
    for adr in adrs {
        for feat in &adr.related_features {
            edges.push(Edge {
                from_type: "adr".into(),
                from_id: adr.id.clone(),
                to_type: "feature".into(),
                to_id: feat.clone(),
            });
        }
    }

    // Feature → Roadmap
    for (idx, item) in roadmap.iter().enumerate() {
        for feat in &item.linked_features {
            edges.push(Edge {
                from_type: "feature".into(),
                from_id: feat.clone(),
                to_type: "roadmap".into(),
                to_id: format!("roadmap-{idx}"),
            });
        }
    }

    edges
}
