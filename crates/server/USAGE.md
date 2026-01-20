# 快速开始指南

## 启动服务器

```bash
cd crates/server
cargo run
```

服务器将在 `http://127.0.0.1:3000` 启动。

## 访问示例页面

打开浏览器访问 `http://127.0.0.1:3000/example`，这个页面展示了三种前后端交互方式的示例。

## 测试 HTTP API

### 使用 curl

```bash
# 添加执行权限
chmod +x test_api.sh

# 运行测试脚本
./test_api.sh
```

### 手动测试

```bash
# 健康检查
curl http://127.0.0.1:3000/health

# 获取所有服务器
curl http://127.0.0.1:3000/api/servers | jq '.'

# 获取生产环境服务器
curl "http://127.0.0.1:3000/api/servers?env=生产" | jq '.'

# 获取在线服务器
curl "http://127.0.0.1:3000/api/servers?status=online" | jq '.'

# 获取单个服务器
curl http://127.0.0.1:3000/api/servers/1 | jq '.'

# 获取服务器统计
curl http://127.0.0.1:3000/api/servers/stats | jq '.'

# 创建新服务器
curl -X POST http://127.0.0.1:3000/api/servers \
  -H "Content-Type: application/json" \
  -d '{
    "id": "100",
    "name": "new-server",
    "ip": "192.168.1.100",
    "status": "online",
    "cpu": 10.0,
    "memory": 20.0,
    "disk": 30.0,
    "env": "测试"
  }' | jq '.'
```

## 测试 WebSocket

### 使用 HTML 测试页面

在浏览器中打开 `test_websocket.html` 文件：

```bash
open test_websocket.html
# 或
firefox test_websocket.html
```

这个页面提供了两个 WebSocket 测试界面：
- **终端 WebSocket**: 模拟交互式终端
- **监控 WebSocket**: 接收实时监控数据推送

### 使用 JavaScript

```javascript
// 终端连接
const terminalWs = new WebSocket('ws://127.0.0.1:3000/ws/terminal/web-prod-01');

terminalWs.onopen = () => {
    console.log('Terminal connected');
    terminalWs.send('ls -la'); // 发送命令
};

terminalWs.onmessage = (event) => {
    console.log('Output:', event.data);
};

// 监控数据连接
const monitoringWs = new WebSocket('ws://127.0.0.1:3000/ws/monitoring');

monitoringWs.onmessage = (event) => {
    const data = JSON.parse(event.data);
    console.log('Monitoring data:', data);
    // { type: "MonitoringData", server_id: "...", cpu: 45.2, ... }
};
```

### 使用 wscat (命令行工具)

```bash
# 安装 wscat
npm install -g wscat

# 连接终端
wscat -c ws://127.0.0.1:3000/ws/terminal/web-prod-01

# 连接监控
wscat -c ws://127.0.0.1:3000/ws/monitoring
```

## 在 Leptos 组件中使用

### 1. Server Functions (推荐)

```rust
use leptos::*;
use crate::server_functions::*;

#[component]
fn MyComponent() -> impl IntoView {
    // 自动加载数据
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
                        Ok(data) => view! {
                            <ul>
                                {data.iter().map(|s| view! {
                                    <li>{&s.name}</li>
                                }).collect::<Vec<_>>()}
                            </ul>
                        },
                        Err(e) => view! { <p>"错误"</p> }
                    }
                })
            }}
        </Suspense>
    }
}
```

### 2. HTTP API (在客户端使用 fetch)

```rust
#[component]
fn ApiComponent() -> impl IntoView {
    let (data, set_data) = create_signal(String::new());

    let fetch_data = move |_| {
        spawn_local(async move {
            let resp = reqwasm::http::Request::get("/api/servers")
                .send()
                .await
                .unwrap();
            let text = resp.text().await.unwrap();
            set_data.set(text);
        });
    };

    view! {
        <button on:click=fetch_data>"获取数据"</button>
        <pre>{move || data.get()}</pre>
    }
}
```

### 3. WebSocket

```rust
use web_sys::WebSocket;

#[component]
fn WebSocketComponent() -> impl IntoView {
    let (messages, set_messages) = create_signal(Vec::<String>::new());

    create_effect(move |_| {
        let ws = WebSocket::new("ws://127.0.0.1:3000/ws/monitoring").unwrap();

        // 设置消息处理器
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            // 处理消息
        }) as Box<dyn FnMut(MessageEvent)>);

        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    });

    view! {
        // UI
    }
}
```

## API 端点总览

### HTTP API

| 方法 | 端点 | 描述 |
|------|------|------|
| GET | /health | 健康检查 |
| GET | /api/servers | 获取服务器列表 |
| GET | /api/servers/:id | 获取单个服务器 |
| POST | /api/servers | 创建服务器 |
| GET | /api/servers/stats | 获取统计信息 |

### WebSocket

| 端点 | 描述 |
|------|------|
| /ws/terminal/:server_id | 终端连接 |
| /ws/monitoring | 监控数据推送 |

### Server Functions

Server Functions 自动注册在 `/api/*` 路径下，无需手动管理端点。

## 故障排除

### 端口已被占用

```bash
# 查找占用端口的进程
lsof -i :3000

# 终止进程
kill -9 <PID>
```

### WebSocket 连接失败

确保：
1. 服务器正在运行
2. 使用正确的协议（ws:// 而不是 http://）
3. 检查浏览器控制台的错误消息

### CORS 错误

服务器已配置 `CorsLayer::permissive()`，应该不会有 CORS 问题。如果仍然遇到问题，检查请求头设置。

## 下一步

- 查看 `README_INTERACTIONS.md` 了解详细的交互方式说明
- 查看 `src/api.rs` 了解如何添加新的 HTTP API
- 查看 `src/websocket.rs` 了解如何添加新的 WebSocket 端点
- 查看 `src/server_functions.rs` 了解如何添加新的 Server Functions
