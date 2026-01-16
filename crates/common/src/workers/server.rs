//! Worker HTTP 服务器
//!
//! 使用标准库 TcpListener 实现简单的 HTTP 服务器。
//! 注意：由于 boa_engine 的 Context 不是线程安全的，服务器采用单线程模式。

use std::fs;
use std::net::{TcpListener, TcpStream};

use super::http::{HttpRequest, HttpResponse};
use super::runtime::JsRuntime;

/// Worker 服务器配置
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// 监听地址
    pub host: String,
    /// 监听端口
    pub port: u16,
    /// Worker 脚本路径
    pub script_path: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8787,
            script_path: "worker.js".to_string(),
        }
    }
}

impl ServerConfig {
    pub fn new(host: &str, port: u16, script_path: &str) -> Self {
        Self {
            host: host.to_string(),
            port,
            script_path: script_path.to_string(),
        }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// Worker HTTP 服务器
pub struct WorkerServer {
    config: ServerConfig,
    runtime: JsRuntime,
}

impl WorkerServer {
    /// 创建新的 Worker 服务器
    pub fn new(config: ServerConfig) -> Result<Self, String> {
        let mut runtime = JsRuntime::new();

        // 加载 Worker 脚本（会自动解析 import 并加载所需的绑定）
        let script = fs::read_to_string(&config.script_path)
            .map_err(|e| format!("Failed to read script {}: {}", config.script_path, e))?;

        runtime.load_worker(&script)?;

        Ok(Self { config, runtime })
    }

    /// 从脚本内容创建服务器
    pub fn from_script(script: &str, host: &str, port: u16) -> Result<Self, String> {
        let mut runtime = JsRuntime::new();
        
        // 加载脚本（会自动解析 import 并加载所需的绑定）
        runtime.load_worker(script)?;

        Ok(Self {
            config: ServerConfig {
                host: host.to_string(),
                port,
                script_path: String::new(),
            },
            runtime,
        })
    }

    /// 从现有的 JsRuntime 创建服务器
    pub fn from_runtime(runtime: JsRuntime, config: ServerConfig) -> Self {
        Self { config, runtime }
    }

    /// 启动服务器（阻塞，单线程模式）
    pub fn run(&mut self) -> Result<(), String> {
        let addr = self.config.addr();
        let listener =
            TcpListener::bind(&addr).map_err(|e| format!("Failed to bind to {}: {}", addr, e))?;

        println!("Worker server listening on http://{}", addr);
        println!("Press Ctrl+C to stop");

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    if let Err(e) = self.handle_connection(&mut stream) {
                        eprintln!("Error handling connection: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }

        Ok(())
    }

    /// 处理单个连接
    fn handle_connection(&mut self, stream: &mut TcpStream) -> Result<(), String> {
        // 解析请求
        let request = HttpRequest::from_stream(stream)
            .map_err(|e| format!("Failed to parse request: {}", e))?;

        println!("{} {} {}", request.method, request.path, request.version);

        // 调用 Worker 处理请求
        let response = self.runtime.handle_request(&request, &self.config.addr()).unwrap_or_else(|e| {
            eprintln!("Worker error: {}", e);
            HttpResponse::error(500, &format!("Worker error: {}", e))
        });

        // 发送响应
        response
            .write_to(stream)
            .map_err(|e| format!("Failed to write response: {}", e))?;

        Ok(())
    }

    /// 处理单个请求（用于测试）
    pub fn handle_request(&mut self, request: &HttpRequest) -> Result<HttpResponse, String> {
        self.runtime.handle_request(request, &self.config.addr())
    }
}

/// 快速启动 Worker 服务器
pub fn serve(script_path: &str, port: u16) -> Result<(), String> {
    let config = ServerConfig::new("127.0.0.1", port, script_path);
    let mut server = WorkerServer::new(config)?;
    server.run()
}

/// 从脚本内容启动服务器
pub fn serve_script(script: &str, port: u16) -> Result<(), String> {
    let mut server = WorkerServer::from_script(script, "127.0.0.1", port)?;
    server.run()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_server_from_script() {
        let script = r#"
            export default {
                fetch(request, env, ctx) {
                    return new Response("Test Response", { status: 200 });
                }
            }
        "#;

        let mut server = WorkerServer::from_script(script, "127.0.0.1", 0).unwrap();

        let request = HttpRequest {
            method: "GET".to_string(),
            path: "/test".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
            body: Vec::new(),
        };

        let response = server.handle_request(&request).unwrap();
        assert_eq!(response.status, 200);
        assert_eq!(String::from_utf8_lossy(&response.body), "Test Response");
    }

    #[test]
    fn test_request_properties() {
        let script = r#"
            export default {
                fetch(request, env, ctx) {
                    var url = request.url;
                    var method = request.method;
                    return new Response(method + " " + url, { status: 200 });
                }
            }
        "#;

        let mut server = WorkerServer::from_script(script, "127.0.0.1", 8080).unwrap();

        let request = HttpRequest {
            method: "POST".to_string(),
            path: "/api/data".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
            body: Vec::new(),
        };

        let response = server.handle_request(&request).unwrap();
        assert_eq!(response.status, 200);
        assert!(String::from_utf8_lossy(&response.body).contains("POST"));
        assert!(String::from_utf8_lossy(&response.body).contains("/api/data"));
    }
}
