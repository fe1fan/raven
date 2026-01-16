//! 绑定值类型定义

use std::collections::HashMap;
use std::fmt;

/// 绑定方法的返回值类型
#[derive(Debug, Clone)]
pub enum BindingValue {
    /// 空值
    Null,
    /// 布尔值
    Bool(bool),
    /// 整数
    Int(i64),
    /// 浮点数
    Float(f64),
    /// 字符串
    String(String),
    /// 字节数组
    Bytes(Vec<u8>),
    /// JSON 对象（序列化为字符串）
    Json(String),
    /// 数组
    Array(Vec<BindingValue>),
    /// 对象/Map
    Object(HashMap<String, BindingValue>),
    /// 错误
    Error(String),
}

impl BindingValue {
    pub fn is_error(&self) -> bool {
        matches!(self, BindingValue::Error(_))
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            BindingValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            BindingValue::String(s) => Some(s),
            _ => None,
        }
    }
}

impl fmt::Display for BindingValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BindingValue::Null => write!(f, "null"),
            BindingValue::Bool(b) => write!(f, "{}", b),
            BindingValue::Int(i) => write!(f, "{}", i),
            BindingValue::Float(n) => write!(f, "{}", n),
            BindingValue::String(s) => write!(f, "{}", s),
            BindingValue::Bytes(b) => write!(f, "<bytes: {} bytes>", b.len()),
            BindingValue::Json(j) => write!(f, "{}", j),
            BindingValue::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            BindingValue::Object(obj) => {
                write!(f, "{{")?;
                for (i, (k, v)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
            BindingValue::Error(e) => write!(f, "Error: {}", e),
        }
    }
}
