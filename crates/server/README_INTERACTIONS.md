# 前后端交互方式说明

本项目演示了三种前后端交互方式，每种方式都有其适用场景。

## 目录

1. [Server Functions (推荐)](#1-server-functions-推荐)
2. [HTTP REST API](#2-http-rest-api)
3. [WebSocket](#3-websocket)
4. [使用示例](#使用示例)
5. [API 文档](#api-文档)

---

## 1. Server Functions (推荐)

### 概述
Server Functions 是 Leptos 提供的类型安全的 RPC 机制。使用 `#[server]` 宏标记的函数可以在客户端调用，但在服务器端执行。

### 优点
- ✅ 类型安全 - 编译时检查参数和返回值类型
- ✅ 自动序列化/反序列化
- ✅ 代码共享 - 前后端使用相同的数据结构
- ✅ 简单易用 - 像调用本地函数一样调用远程函数
- ✅ 自动生成 API 端点

### 适用场景
- Leptos 应用的主要数据交互方式
- 需要类型安全的场景
- 快速开发原型

### 代码示例

**定义 Server Function (server_functions.rs):**

```rust
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub id: String,
    pub name: String,
    pub ip: String,
    pub status: String,
}

// 使用 #[server] 宏定义服务器函数
#[server(GetServers, "/api")]
pub async fn get_servers_sf(
    env_filter: Option<String>,
    status_filter: Option<String>,
) -> Result<Vec<ServerInfo>, ServerFnError> {
    // 这段代码只在服务器端运行
    let servers = query_database(env_filter, status_filter).await?;
    Ok(servers)
}

#[server(ExecuteCommand, "/api")]
pub async fn execute_command_sf(
    server_id: String,
    command: String,
) -> Result<String, ServerFnError> {
    // 执行命令的服务器端逻辑
    let output = run_command(&server_id, &command).await?;
    Ok(output)
}
```

**在组件中使用:**

```rust
use leptos::*;

#[component]
fn ServerList() -> impl IntoView {
    // 创建 Resource 自动获取数据
    let servers = create_resource(
        || (),
        |_| async move {
            get_servers_sf(None, None).await
        }
    );

    view! {
        <Suspense fallback=|| view! { <p>"加载中..."</p> }>
            {move || {
                servers.get().map(|result| {
                    match result {
                        Ok(servers) => view! {
                            <ul>
                                {servers.iter().map(|s| view! {
                                    <li>{&s.name} " - " {&s.ip}</li>
                                }).collect::<Vec<_>>()}
                            </ul>
                        },
                        Err(e) => view! { <p>"错误: " {e.to_string()}</p> }
                    }
                })
            }}
        </Suspense>
    }
}
```

**使用 Action 执行命令:**

```rust
#[component]
fn CommandExecutor() -> impl IntoView {
    let execute = create_action(|(server_id, cmd): &(String, String)| {
        let server_id = server_id.clone();
        let cmd = cmd.clone();
        async move {
            execute_command_sf(server_id, cmd).await
        }
    });

    view! {
        <button on:click=move |_| {
            execute.dispatch(("web-prod-01".to_string(), "ls -la".to_string()))
        }>
            "执行命令"
        </button>

        {move || execute.value().get().map(|result| {
            match result {
                Ok(output) => view! { <pre>{output}</pre> },
                Err(e) => view! { <p>"错误: " {e.to_string()}</p> }
            }
        })}
    }
}
```

---

## 2. HTTP REST API

### 概述
传统的 HTTP REST API，使用 Axum 路由处理。

### 优点
- ✅ 标准化 - 遵循 REST 规范
- ✅ 跨平台 - 可被任何 HTTP 客户端调用
- ✅ 精确控制 - 完全控制 HTTP 头、状态码等
- ✅ 缓存友好 - 支持 HTTP 缓存机制
- ✅ 文档化 - 易于生成 API 文档

### 适用场景
- 需要被外部系统调用的公开 API
- 需要精确控制 HTTP 行为
- 移动应用或第三方集成
- 需要 RESTful 规范的场景

### 代码示例

**定义 API 路由 (api.rs):**

```rust
use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Server {
    pub id: String,
    pub name: String,
    pub ip: String,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

// 创建 API 路由
pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/servers", get(get_servers))
        .route("/servers/:id", get(get_server_by_id))
        .route("/servers", post(create_server))
        .route("/servers/stats", get(get_server_stats))
}

// GET /api/servers
async fn get_servers(
    State(state): State<AppState>,
    Query(query): Query<ServerQuery>,
) -> Json<ApiResponse<Vec<Server>>> {
    let servers = state.servers.read().await;
    // 应用过滤逻辑...
    Json(ApiResponse::success(filtered))
}

// GET /api/servers/:id
async fn get_server_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<ApiResponse<Server>> {
    // 查找服务器...
    Json(ApiResponse::success(server))
}

// POST /api/servers
async fn create_server(
    State(state): State<AppState>,
    Json(new_server): Json<Server>,
) -> Json<ApiResponse<Server>> {
    // 创建服务器...
    Json(ApiResponse::success(new_server))
}
```

**在客户端调用 (JavaScript/Fetch):**

```javascript
// 获取服务器列表
fetch('/api/servers?env=生产&status=online')
  .then(res => res.json())
  .then(data => {
    console.log(data.data); // 服务器列表
  });

// 创建服务器
fetch('/api/servers', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    id: '123',
    name: 'new-server',
    ip: '192.168.1.100'
  })
})
  .then(res => res.json())
  .then(data => console.log(data));
```

**在 Leptos 中调用 (使用 reqwasm):**

```rust
use reqwasm::http::Request;

#[component]
fn ApiExample() -> impl IntoView {
    let (response, set_response) = create_signal(String::new());

    let fetch_data = move |_| {
        spawn_local(async move {
            let resp = Request::get("/api/servers/stats")
                .send()
                .await
                .unwrap();
            let text = resp.text().await.unwrap();
            set_response.set(text);
        });
    };

    view! {
        <button on:click=fetch_data>"获取统计"</button>
        <pre>{move || response.get()}</pre>
    }
}
```

---

## 3. WebSocket

### 概述
WebSocket 提供全双工通信通道，适合实时双向数据传输。

### 优点
- ✅ 实时性 - 低延迟的双向通信
- ✅ 服务器推送 - 服务器可主动向客户端推送数据
- ✅ 长连接 - 减少建立连接的开销
- ✅ 高效 - 相比轮询更节省带宽

### 适用场景
- Web 终端 (SSH/Shell)
- 实时监控数据推送
- 聊天应用
- 实时通知
- 协作编辑

### 代码示例

**定义 WebSocket 处理器 (websocket.rs):**

```rust
use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade, Path, State},
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};

// WebSocket 路由
pub fn ws_routes() -> Router<WsState> {
    Router::new()
        .route("/terminal/:server_id", get(terminal_handler))
        .route("/monitoring", get(monitoring_handler))
}

// 终端 WebSocket 处理器
async fn terminal_handler(
    ws: WebSocketUpgrade,
    Path(server_id): Path<String>,
    State(state): State<WsState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_terminal(socket, server_id, state))
}

async fn handle_terminal(socket: WebSocket, server_id: String, state: WsState) {
    let (mut sender, mut receiver) = socket.split();

    // 接收客户端消息
    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            // 处理命令
            let output = execute_command(&server_id, &text).await;
            // 发送结果
            let _ = sender.send(Message::Text(output)).await;
        }
    }
}

// 监控数据推送
async fn handle_monitoring(socket: WebSocket, state: WsState) {
    let (mut sender, _) = socket.split();
    let mut rx = state.monitoring_tx.subscribe();

    // 持续推送监控数据
    while let Ok(data) = rx.recv().await {
        let json = serde_json::to_string(&data).unwrap();
        if sender.send(Message::Text(json)).await.is_err() {
            break;
        }
    }
}
```

**客户端 JavaScript:**

```javascript
// 连接终端 WebSocket
const terminalWs = new WebSocket('ws://localhost:3000/ws/terminal/web-prod-01');

terminalWs.onopen = () => {
  console.log('Terminal connected');
  terminalWs.send('ls -la'); // 发送命令
};

terminalWs.onmessage = (event) => {
  console.log('Output:', event.data);
  document.getElementById('terminal').textContent += event.data;
};

terminalWs.onclose = () => {
  console.log('Terminal disconnected');
};

// 连接监控 WebSocket
const monitoringWs = new WebSocket('ws://localhost:3000/ws/monitoring');

monitoringWs.onmessage = (event) => {
  const data = JSON.parse(event.data);
  updateMonitoringUI(data);
};
```

**Leptos 中使用 WebSocket:**

```rust
use web_sys::WebSocket;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[component]
fn TerminalComponent() -> impl IntoView {
    let (output, set_output) = create_signal(String::new());

    create_effect(move |_| {
        // 创建 WebSocket 连接
        let ws = WebSocket::new("ws://localhost:3000/ws/terminal/web-prod-01")
            .unwrap();

        // 设置消息处理器
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                let msg = String::from(txt);
                set_output.update(|output| output.push_str(&msg));
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
    });

    view! {
        <div class="terminal">
            <pre>{move || output.get()}</pre>
        </div>
    }
}
```

---

## 使用示例

访问 `http://localhost:3000/example` 查看完整的交互示例页面。

### 运行服务器

```bash
cd crates/server
cargo run
```

服务器启动后会显示：

```
╔══════════════════════════════════════════╗
║  Server started successfully!           ║
╠══════════════════════════════════════════╣
║  🌐 Web UI:       http://127.0.0.1:3000  ║
║  📡 HTTP API:     http://127.0.0.1:3000/api ║
║  🔌 WebSocket:    ws://127.0.0.1:3000/ws ║
╠══════════════════════════════════════════╣
║  Example Page:                           ║
║    http://127.0.0.1:3000/example         ║
╚══════════════════════════════════════════╝
```

---

## API 文档

### HTTP API 端点

#### GET /api/servers
获取服务器列表

**查询参数:**
- `env` (可选): 环境过滤 (生产, 测试)
- `status` (可选): 状态过滤 (online, offline, warning)

**响应:**
```json
{
  "code": 200,
  "message": "success",
  "data": [
    {
      "id": "1",
      "name": "web-prod-01",
      "ip": "192.168.1.101",
      "status": "online",
      "cpu": 45.2,
      "memory": 68.5,
      "disk": 55.0,
      "env": "生产"
    }
  ]
}
```

#### GET /api/servers/:id
获取单个服务器详情

**响应:**
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "1",
    "name": "web-prod-01",
    "ip": "192.168.1.101",
    "status": "online"
  }
}
```

#### POST /api/servers
创建服务器

**请求体:**
```json
{
  "id": "new-id",
  "name": "new-server",
  "ip": "192.168.1.200",
  "status": "online"
}
```

#### GET /api/servers/stats
获取服务器统计信息

**响应:**
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "online": 119,
    "warning": 5,
    "offline": 4,
    "total": 128
  }
}
```

### WebSocket 端点

#### /ws/terminal/:server_id
终端连接

**发送消息:** 文本命令
**接收消息:** 命令输出

#### /ws/monitoring
监控数据推送

**接收消息格式:**
```json
{
  "type": "MonitoringData",
  "server_id": "web-prod-01",
  "cpu": 45.2,
  "memory": 68.5,
  "disk": 55.0,
  "network": 124.5
}
```

---

## 选择指南

| 需求 | 推荐方式 | 原因 |
|------|---------|------|
| Leptos 应用内部通信 | Server Functions | 类型安全、简单易用 |
| 外部系统调用 | HTTP API | 标准化、跨平台 |
| 实时数据推送 | WebSocket | 低延迟、双向通信 |
| Web 终端 | WebSocket | 需要持续交互 |
| 批量数据获取 | HTTP API 或 Server Functions | 都可以，看具体需求 |
| 移动应用对接 | HTTP API | 更通用 |
| 第三方集成 | HTTP API | 文档化、标准化 |

---

## 最佳实践

1. **优先使用 Server Functions**
   - 对于 Leptos 应用，优先使用 Server Functions
   - 它提供了最好的开发体验和类型安全

2. **HTTP API 用于公开接口**
   - 需要被外部调用时使用 HTTP API
   - 提供清晰的 API 文档

3. **WebSocket 用于实时场景**
   - 只在真正需要实时通信时使用
   - 考虑连接管理和错误处理

4. **混合使用**
   - 可以在同一个应用中同时使用三种方式
   - 根据具体场景选择最合适的方式
