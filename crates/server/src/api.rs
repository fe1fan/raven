// HTTP API 接口示例
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

// 数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub id: String,
    pub name: String,
    pub ip: String,
    pub status: String,
    pub cpu: f32,
    pub memory: f32,
    pub disk: f32,
    pub env: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStats {
    pub online: u32,
    pub warning: u32,
    pub offline: u32,
    pub total: u32,
}

#[derive(Debug, Deserialize)]
pub struct ServerQuery {
    pub env: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(code: u16, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }
}

// 应用状态（模拟数据库）
#[derive(Clone)]
pub struct AppState {
    pub servers: Arc<RwLock<Vec<Server>>>,
}

impl Default for AppState {
    fn default() -> Self {
        // 初始化一些模拟数据
        let servers = vec![
            Server {
                id: "1".to_string(),
                name: "web-prod-01".to_string(),
                ip: "192.168.1.101".to_string(),
                status: "online".to_string(),
                cpu: 45.2,
                memory: 68.5,
                disk: 55.0,
                env: "生产".to_string(),
            },
            Server {
                id: "2".to_string(),
                name: "web-prod-02".to_string(),
                ip: "192.168.1.102".to_string(),
                status: "online".to_string(),
                cpu: 32.1,
                memory: 54.3,
                disk: 48.0,
                env: "生产".to_string(),
            },
            Server {
                id: "3".to_string(),
                name: "db-master-01".to_string(),
                ip: "192.168.1.201".to_string(),
                status: "warning".to_string(),
                cpu: 78.5,
                memory: 89.2,
                disk: 92.0,
                env: "生产".to_string(),
            },
        ];

        Self {
            servers: Arc::new(RwLock::new(servers)),
        }
    }
}

// API 路由
pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/servers", get(get_servers))
        .route("/servers/:id", get(get_server_by_id))
        .route("/servers", post(create_server))
        .route("/servers/stats", get(get_server_stats))
}

// GET /api/servers - 获取服务器列表
async fn get_servers(
    State(state): State<AppState>,
    Query(query): Query<ServerQuery>,
) -> impl IntoResponse {
    let servers = state.servers.read().await;

    // 根据查询参数过滤
    let filtered: Vec<Server> = servers
        .iter()
        .filter(|s| {
            let env_match = query.env.as_ref().map_or(true, |env| &s.env == env);
            let status_match = query.status.as_ref().map_or(true, |status| &s.status == status);
            env_match && status_match
        })
        .cloned()
        .collect();

    Json(ApiResponse::success(filtered))
}

// GET /api/servers/:id - 获取单个服务器
async fn get_server_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let servers = state.servers.read().await;

    if let Some(server) = servers.iter().find(|s| s.id == id) {
        Json(ApiResponse::success(server.clone()))
    } else {
        Json(ApiResponse::error(404, "Server not found".to_string()))
    }
}

// POST /api/servers - 创建服务器
async fn create_server(
    State(state): State<AppState>,
    Json(new_server): Json<Server>,
) -> impl IntoResponse {
    let mut servers = state.servers.write().await;

    // 检查 ID 是否已存在
    if servers.iter().any(|s| s.id == new_server.id) {
        return (
            StatusCode::CONFLICT,
            Json(ApiResponse::<Server>::error(409, "Server ID already exists".to_string()))
        );
    }

    servers.push(new_server.clone());

    (
        StatusCode::CREATED,
        Json(ApiResponse::success(new_server))
    )
}

// GET /api/servers/stats - 获取服务器统计
async fn get_server_stats(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let servers = state.servers.read().await;

    let stats = ServerStats {
        online: servers.iter().filter(|s| s.status == "online").count() as u32,
        warning: servers.iter().filter(|s| s.status == "warning").count() as u32,
        offline: servers.iter().filter(|s| s.status == "offline").count() as u32,
        total: servers.len() as u32,
    };

    Json(ApiResponse::success(stats))
}
