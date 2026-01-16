//! Worker 运行时示例
//!
//! 运行方式:
//! ```bash
//! cargo run --example worker
//! ```
//!
//! 然后访问 http://127.0.0.1:8787/

use common::workers::{ServerConfig, WorkerServer};

fn main() {
    println!("=== Cloudflare Workers 风格的 Rust 运行时 ===\n");

    println!("📋 可用测试路由:");
    println!("  - http://127.0.0.1:8787/                (路由列表)");
    println!("  - http://127.0.0.1:8787/hello           (基本响应)");
    println!("  - http://127.0.0.1:8787/test-kv         (测试 KV 绑定)");
    println!("  - http://127.0.0.1:8787/test-utils      (测试 UTILS 绑定)");
    println!("  - http://127.0.0.1:8787/test-combo      (测试组合功能)");
    println!("  - http://127.0.0.1:8787/test-no-import  (测试未导入的绑定)");
    println!();
    let mut conf = ServerConfig::default();
    conf.script_path = "crates/common/examples/workers.js".to_string();
    if let Ok(mut server) = WorkerServer::new(conf) {
        if let Err(e) = server.run() {
            eprintln!("服务器错误: {}", e);
        }
    }
}
