use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use axum::{
    Router,
    routing::get,
};
use tower_http::cors::CorsLayer;

mod app;
mod pages;
mod components;
mod api;
mod websocket;
mod server_functions;
mod example_page;

use app::App;
use api::AppState;
use websocket::WsState;

#[tokio::main]
async fn main() {
    // 初始化日志
    env_logger::init();

    // 设置环境变量以便 Leptos 正确加载配置
    std::env::set_var("LEPTOS_OUTPUT_NAME", "server");
    std::env::set_var("LEPTOS_SITE_ROOT", "target/site");
    std::env::set_var("LEPTOS_SITE_PKG_DIR", "pkg");
    std::env::set_var("LEPTOS_SITE_ADDR", "127.0.0.1:3000");
    std::env::set_var("LEPTOS_RELOAD_PORT", "3001");

    // Leptos 配置
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // 创建应用状态
    let app_state = AppState::default();
    let ws_state = WsState::new();

    // 构建路由
    let app = Router::new()
        // 健康检查接口
        .route("/health", get(|| async { "OK" }))

        // WebSocket 路由 (需要在 /ws 前缀下)
        .nest("/ws", websocket::ws_routes())
        .with_state(ws_state)

        // HTTP API 路由 (需要在 /api 前缀下)
        .nest("/api", api::api_routes())
        .with_state(app_state)

        // Leptos 路由 (包括 Server Functions 和页面路由)
        .leptos_routes(&leptos_options, routes, App)
        .with_state(leptos_options)

        // 添加 CORS 支持 (如果需要跨域访问)
        .layer(CorsLayer::permissive());

    println!("╔══════════════════════════════════════════╗");
    println!("║  Server started successfully!            ║");
    println!("╠══════════════════════════════════════════╣");
    println!("║  🌐 Web UI:    http://{}     ║", addr);
    println!("║  📡 HTTP API:  http://{}/api ║", addr);
    println!("║  🔌 WebSocket: ws://{}/ws    ║", addr);
    println!("╠══════════════════════════════════════════╣");
    println!("║  API Endpoints:                          ║");
    println!("║    GET  /api/servers                     ║");
    println!("║    GET  /api/servers/:id                 ║");
    println!("║    POST /api/servers                     ║");
    println!("║    GET  /api/servers/stats               ║");
    println!("║                                          ║");
    println!("║  WebSocket Endpoints:                    ║");
    println!("║    /ws/terminal/:server_id               ║");
    println!("║    /ws/monitoring                        ║");
    println!("║                                          ║");
    println!("║  Example Page:                           ║");
    println!("║    http://{}/example         ║", addr);
    println!("╚══════════════════════════════════════════╝");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
