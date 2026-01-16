//! 核心 JavaScript 运行时
//!
//! 提供通用的 JS 执行环境和绑定注入机制，可以被不同的应用场景复用。
//!
//! # 应用场景
//!
//! - **Workers**: HTTP Server + fetch() 入口
//! - **Operator**: 其他场景 + 自定义入口

pub mod bindings;
mod core;
mod import;

pub use core::JsRuntime;
pub use import::{parse_imports, create_binding_from_module};
