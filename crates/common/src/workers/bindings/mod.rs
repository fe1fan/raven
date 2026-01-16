//! Workers 绑定实现
//!
//! 提供具体的绑定实现（KV, UTILS 等）
//!
//! # 注意
//!
//! 核心的绑定系统（BindingRegistry, WorkerBinding trait 等）
//! 现在位于 `crate::runtime::bindings`，这里只包含具体实现。
//!
//! # Example
//!
//! ```ignore
//! use common::runtime::bindings::BindingRegistry;
//! use common::workers::bindings::KvBinding;
//!
//! let mut registry = BindingRegistry::new();
//! registry.register("KV", Box::new(KvBinding::memory("KV")));
//! ```

mod kv;
mod utils;

// 重新导出核心绑定系统（为了向后兼容）
pub use crate::runtime::bindings::{BindingRegistry, WorkerBinding, BindingMethod, BindingValue};

// 导出具体的绑定实现
pub use kv::{KvBinding, KvStore, MemoryKvStore};
pub use utils::UtilsBinding;