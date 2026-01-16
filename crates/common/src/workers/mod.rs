//! Worker 运行时模块
//!
//! 提供类 Cloudflare Workers 的 JavaScript 运行时环境，
//! 支持加载并执行 JS 文件，启动 HTTP 服务器运行 Worker 代码。
//!
//! # Example
//!
//! ```ignore
//! use common::workers::{serve_script, WorkerServer, JsRuntime};
//! use common::workers::bindings::{BindingRegistry, KvBinding};
//!
//! // 创建运行时并注册 KV 绑定
//! let mut runtime = JsRuntime::new();
//! runtime.register_binding(Box::new(KvBinding::memory("KV")));
//!
//! let script = r#"
//!     export default {
//!         async fetch(request, env, ctx) {
//!             // 使用 KV 存储
//!             await env.KV.put("key", "value");
//!             const value = await env.KV.get("key");
//!             return new Response(value, { status: 200 });
//!         }
//!     }
//! "#;
//!
//! runtime.load_worker(script)?;
//! ```

pub mod bindings;
mod http;
mod runtime;
mod server;

pub use http::{HttpRequest, HttpResponse};
pub use runtime::JsRuntime;
pub use server::{serve, serve_script, ServerConfig, WorkerServer};
