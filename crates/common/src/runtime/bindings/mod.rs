//! 绑定系统 - 通用基础设施
//!
//! 提供绑定注册、管理和调用机制，可被所有应用场景使用

mod registry;
mod value;

pub use registry::{BindingMethod, BindingRegistry, NativeBinding};
pub use value::BindingValue;
