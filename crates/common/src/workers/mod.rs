//! Workers 模块 - HTTP Server + Fetch 入口
//!
//! 基于核心 runtime，提供 Cloudflare Workers 风格的 HTTP 服务器。
//! 使用 `export default { fetch() }` 作为入口。

pub mod bindings;
mod http;
mod fetch_runtime;
mod server;

pub use http::{HttpRequest, HttpResponse};
pub use fetch_runtime::FetchRuntime;
pub use server::{serve, serve_script, ServerConfig, WorkerServer};
