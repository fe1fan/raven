// WebSocket 接口示例
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use std::collections::HashMap;

// WebSocket 消息类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    // 终端消息
    TerminalInput { data: String },
    TerminalOutput { data: String },

    // 监控数据
    MonitoringData {
        server_id: String,
        cpu: f32,
        memory: f32,
        disk: f32,
        network: f32,
    },

    // 系统消息
    Ping,
    Pong,
    Error { message: String },
}

// WebSocket 状态
#[derive(Clone)]
pub struct WsState {
    // 用于广播监控数据
    pub monitoring_tx: broadcast::Sender<WsMessage>,
    // 存储终端会话
    pub terminals: Arc<RwLock<HashMap<String, broadcast::Sender<String>>>>,
}

impl WsState {
    pub fn new() -> Self {
        let (monitoring_tx, _) = broadcast::channel(100);

        // 启动模拟监控数据发送任务
        let tx_clone = monitoring_tx.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(2));
            loop {
                interval.tick().await;

                // 模拟发送监控数据
                let msg = WsMessage::MonitoringData {
                    server_id: "web-prod-01".to_string(),
                    cpu: 40.0 + rand::random::<f32>() * 20.0,
                    memory: 60.0 + rand::random::<f32>() * 20.0,
                    disk: 50.0 + rand::random::<f32>() * 10.0,
                    network: 100.0 + rand::random::<f32>() * 50.0,
                };

                let _ = tx_clone.send(msg);
            }
        });

        Self {
            monitoring_tx,
            terminals: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

// WebSocket 路由
pub fn ws_routes() -> Router<WsState> {
    Router::new()
        .route("/terminal/:server_id", get(terminal_handler))
        .route("/monitoring", get(monitoring_handler))
}

// WebSocket 处理器 - 终端连接
async fn terminal_handler(
    ws: WebSocketUpgrade,
    Path(server_id): Path<String>,
    State(state): State<WsState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_terminal_socket(socket, server_id, state))
}

async fn handle_terminal_socket(socket: WebSocket, server_id: String, state: WsState) {
    let (mut sender, mut receiver) = socket.split();

    // 为这个终端会话创建一个广播通道
    let (tx, mut rx) = broadcast::channel::<String>(100);

    // 将会话添加到状态中
    {
        let mut terminals = state.terminals.write().await;
        terminals.insert(server_id.clone(), tx.clone());
    }

    println!("Terminal connected: {}", server_id);

    // 发送欢迎消息
    let welcome = format!("Connected to {}\nroot@{}:~# ", server_id, server_id);
    let _ = sender.send(Message::Text(welcome)).await;

    // 创建一个任务来接收广播消息并发送到客户端
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // 创建一个任务来接收客户端消息
    let tx_clone = tx.clone();
    let server_id_clone = server_id.clone(); // 克隆 server_id 用于异步任务
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    // 处理终端输入
                    println!("Terminal input: {}", text);

                    // 模拟命令执行（实际应该执行真实命令）
                    let response = if text.trim() == "ls" {
                        "file1.txt  file2.txt  directory1/\n"
                    } else if text.trim().starts_with("echo") {
                        &format!("{}\n", text.trim().strip_prefix("echo ").unwrap_or(""))
                    } else {
                        "bash: command not found\n"
                    };

                    // 广播输出
                    let output = format!("{}root@{}:~# ", response, server_id_clone);
                    let _ = tx_clone.send(output);
                }
                Message::Close(_) => {
                    break;
                }
                _ => {}
            }
        }
    });

    // 等待任务完成
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // 清理会话
    let mut terminals = state.terminals.write().await;
    terminals.remove(&server_id);
    println!("Terminal disconnected: {}", server_id);
}

// WebSocket 处理器 - 监控数据推送
async fn monitoring_handler(
    ws: WebSocketUpgrade,
    State(state): State<WsState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_monitoring_socket(socket, state))
}

async fn handle_monitoring_socket(socket: WebSocket, state: WsState) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.monitoring_tx.subscribe();

    println!("Monitoring client connected");

    // 发送任务
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });

    // 接收任务（处理 ping/pong）
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                        match ws_msg {
                            WsMessage::Ping => {
                                println!("Received ping");
                            }
                            _ => {}
                        }
                    }
                }
                Message::Close(_) => {
                    break;
                }
                _ => {}
            }
        }
    });

    // 等待任务完成
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    println!("Monitoring client disconnected");
}
