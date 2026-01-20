// Server Functions - Leptos 内置的前后端通信机制
use leptos::*;
use serde::{Deserialize, Serialize};

// 数据模型（与 API 共享）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
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
pub struct MonitoringMetrics {
    pub cpu: f32,
    pub memory: f32,
    pub disk: f32,
    pub network: f32,
}

// Server Function 1: 获取服务器列表
// 这个函数可以在客户端调用，但会在服务器端执行
#[server(GetServers, "/api")]
pub async fn get_servers_sf(
    env_filter: Option<String>,
    status_filter: Option<String>,
) -> Result<Vec<ServerInfo>, ServerFnError> {
    // 模拟数据库查询
    let all_servers = vec![
        ServerInfo {
            id: "1".to_string(),
            name: "web-prod-01".to_string(),
            ip: "192.168.1.101".to_string(),
            status: "online".to_string(),
            cpu: 45.2,
            memory: 68.5,
            disk: 55.0,
            env: "生产".to_string(),
        },
        ServerInfo {
            id: "2".to_string(),
            name: "web-prod-02".to_string(),
            ip: "192.168.1.102".to_string(),
            status: "online".to_string(),
            cpu: 32.1,
            memory: 54.3,
            disk: 48.0,
            env: "生产".to_string(),
        },
        ServerInfo {
            id: "3".to_string(),
            name: "db-master-01".to_string(),
            ip: "192.168.1.201".to_string(),
            status: "warning".to_string(),
            cpu: 78.5,
            memory: 89.2,
            disk: 92.0,
            env: "生产".to_string(),
        },
        ServerInfo {
            id: "4".to_string(),
            name: "test-server-01".to_string(),
            ip: "192.168.2.101".to_string(),
            status: "online".to_string(),
            cpu: 25.0,
            memory: 45.0,
            disk: 30.0,
            env: "测试".to_string(),
        },
    ];

    // 应用过滤器
    let filtered: Vec<ServerInfo> = all_servers
        .into_iter()
        .filter(|s| {
            let env_match = env_filter.as_ref().map_or(true, |env| &s.env == env);
            let status_match = status_filter.as_ref().map_or(true, |status| &s.status == status);
            env_match && status_match
        })
        .collect();

    Ok(filtered)
}

// Server Function 2: 获取单个服务器详情
#[server(GetServerById, "/api")]
pub async fn get_server_by_id_sf(id: String) -> Result<ServerInfo, ServerFnError> {
    // 模拟数据库查询
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let server = ServerInfo {
        id: id.clone(),
        name: format!("server-{}", id),
        ip: format!("192.168.1.{}", id),
        status: "online".to_string(),
        cpu: 45.2,
        memory: 68.5,
        disk: 55.0,
        env: "生产".to_string(),
    };

    Ok(server)
}

// Server Function 3: 获取监控指标
#[server(GetMonitoringMetrics, "/api")]
pub async fn get_monitoring_metrics_sf(server_id: String) -> Result<MonitoringMetrics, ServerFnError> {
    // 模拟获取实时监控数据
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let metrics = MonitoringMetrics {
        cpu: 40.0 + rng.gen::<f32>() * 20.0,
        memory: 60.0 + rng.gen::<f32>() * 20.0,
        disk: 50.0 + rng.gen::<f32>() * 10.0,
        network: 100.0 + rng.gen::<f32>() * 50.0,
    };

    println!("Fetching metrics for server: {}", server_id);

    Ok(metrics)
}

// Server Function 4: 执行命令
#[server(ExecuteCommand, "/api")]
pub async fn execute_command_sf(
    server_id: String,
    command: String,
) -> Result<String, ServerFnError> {
    println!("Executing command on {}: {}", server_id, command);

    // 模拟命令执行
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let output = if command.trim() == "systemctl status nginx" {
        "● nginx.service - A high performance web server
   Loaded: loaded (/lib/systemd/system/nginx.service; enabled)
   Active: active (running) since Mon 2024-01-15 08:00:00 CST
  Process: 1234 ExecStart=/usr/sbin/nginx (code=exited, status=0/SUCCESS)
 Main PID: 1235 (nginx)
    Tasks: 5 (limit: 4915)".to_string()
    } else if command.trim() == "ls -la" {
        "total 48
drwx------  6 root root 4096 Jan 15 10:00 .
drwxr-xr-x 23 root root 4096 Jan 10 08:00 ..
-rw-------  1 root root 1234 Jan 15 09:00 .bash_history
-rw-r--r--  1 root root  570 Jan  6 00:00 .bashrc
drwx------  2 root root 4096 Jan 10 08:00 .ssh".to_string()
    } else {
        format!("bash: {}: command not found", command)
    };

    Ok(output)
}

// Server Function 5: 创建服务器
#[server(CreateServer, "/api")]
pub async fn create_server_sf(server: ServerInfo) -> Result<ServerInfo, ServerFnError> {
    println!("Creating server: {:?}", server);

    // 模拟数据库插入
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    // 在实际应用中，这里会保存到数据库
    Ok(server)
}

// Server Function 6: 测试 HTTP API（用于演示）
#[server(TestHttpApi, "/api")]
pub async fn test_http_api_sf() -> Result<String, ServerFnError> {
    // 模拟调用内部 API
    let stats = serde_json::json!({
        "code": 200,
        "message": "success",
        "data": {
            "online": 119,
            "warning": 5,
            "offline": 4,
            "total": 128
        }
    });

    Ok(serde_json::to_string_pretty(&stats).unwrap())
}
