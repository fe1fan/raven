//! Worker 绑定模块系统
//!
//! 提供可扩展的模块架构，让 JS 代码可以调用 Rust 实现的功能。
//!
//! # 架构
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │                     JS Runtime                          │
//! │  ┌─────────────────────────────────────────────────┐   │
//! │  │ env.KV.get("key")                               │   │
//! │  │ env.KV.put("key", "value")                      │   │
//! │  │ env.DB.execute("SELECT * FROM users")          │   │
//! │  └─────────────────────────────────────────────────┘   │
//! └─────────────────────┬───────────────────────────────────┘
//!                       │
//!                       ▼
//! ┌─────────────────────────────────────────────────────────┐
//! │                 BindingRegistry                         │
//! │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐   │
//! │  │    KV    │ │    D1    │ │    R2    │ │  Custom  │   │
//! │  │ Binding  │ │ Binding  │ │ Binding  │ │ Binding  │   │
//! │  └──────────┘ └──────────┘ └──────────┘ └──────────┘   │
//! └─────────────────────────────────────────────────────────┘
//! ```
//!
//! # Example
//!
//! ```ignore
//! use common::workers::bindings::{BindingRegistry, KvBinding};
//!
//! let mut registry = BindingRegistry::new();
//! registry.register("KV", Box::new(KvBinding::memory()));
//!
//! // JS 中可以这样使用:
//! // const value = await env.KV.get("my-key");
//! // await env.KV.put("my-key", "my-value");
//! ```

mod registry;
mod kv;
mod utils;

pub use registry::{BindingRegistry, WorkerBinding, BindingMethod, BindingValue};
pub use kv::{KvBinding, KvStore, MemoryKvStore};
pub use utils::UtilsBinding;