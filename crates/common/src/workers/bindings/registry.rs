//! 绑定注册表和核心 trait 定义

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

/// 绑定方法定义
#[derive(Debug, Clone)]
pub struct BindingMethod {
    /// 方法名称
    pub name: String,
    /// 参数数量（-1 表示可变参数）
    pub arity: i32,
    /// 是否异步
    pub is_async: bool,
}

impl BindingMethod {
    pub fn new(name: &str, arity: i32) -> Self {
        Self {
            name: name.to_string(),
            arity,
            is_async: false,
        }
    }

    pub fn async_method(name: &str, arity: i32) -> Self {
        Self {
            name: name.to_string(),
            arity,
            is_async: true,
        }
    }
}

/// Worker 绑定 trait
///
/// 所有可绑定到 JS 环境的模块都需要实现此 trait
pub trait WorkerBinding: Send + Sync {
    /// 绑定名称（在 JS 中通过 env.{name} 访问）
    fn name(&self) -> &str;

    /// 获取此绑定支持的所有方法
    fn methods(&self) -> Vec<BindingMethod>;

    /// 调用绑定的方法
    ///
    /// # Arguments
    /// * `method` - 方法名称
    /// * `args` - 参数列表
    ///
    /// # Returns
    /// 方法执行结果
    fn call(&self, method: &str, args: Vec<BindingValue>) -> BindingValue;

    /// 获取绑定的属性值
    fn get_property(&self, _name: &str) -> Option<BindingValue> {
        None
    }
}

/// 绑定注册表
///
/// 管理所有注册的 Worker 绑定
pub struct BindingRegistry {
    bindings: HashMap<String, Box<dyn WorkerBinding>>,
}

impl Default for BindingRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl BindingRegistry {
    /// 创建新的注册表
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    /// 注册一个绑定
    ///
    /// # Arguments
    /// * `name` - 绑定名称（env.{name}）
    /// * `binding` - 绑定实现
    pub fn register(&mut self, name: &str, binding: Box<dyn WorkerBinding>) {
        self.bindings.insert(name.to_string(), binding);
    }

    /// 获取绑定
    pub fn get(&self, name: &str) -> Option<&dyn WorkerBinding> {
        self.bindings.get(name).map(|b| b.as_ref())
    }

    /// 获取可变绑定引用
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Box<dyn WorkerBinding>> {
        self.bindings.get_mut(name)
    }

    /// 列出所有已注册的绑定名称
    pub fn list(&self) -> Vec<&str> {
        self.bindings.keys().map(|s| s.as_str()).collect()
    }

    /// 调用绑定方法
    pub fn call(&self, binding_name: &str, method: &str, args: Vec<BindingValue>) -> BindingValue {
        match self.bindings.get(binding_name) {
            Some(binding) => binding.call(method, args),
            None => BindingValue::Error(format!("Binding '{}' not found", binding_name)),
        }
    }

    /// 移除绑定
    pub fn remove(&mut self, name: &str) -> Option<Box<dyn WorkerBinding>> {
        self.bindings.remove(name)
    }

    /// 检查是否存在绑定
    pub fn contains(&self, name: &str) -> bool {
        self.bindings.contains_key(name)
    }

    /// 获取绑定数量
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockBinding;

    impl WorkerBinding for MockBinding {
        fn name(&self) -> &str {
            "MOCK"
        }

        fn methods(&self) -> Vec<BindingMethod> {
            vec![
                BindingMethod::new("test", 0),
                BindingMethod::new("echo", 1),
            ]
        }

        fn call(&self, method: &str, args: Vec<BindingValue>) -> BindingValue {
            match method {
                "test" => BindingValue::String("test result".to_string()),
                "echo" => args.into_iter().next().unwrap_or(BindingValue::Null),
                _ => BindingValue::Error(format!("Unknown method: {}", method)),
            }
        }
    }

    #[test]
    fn test_registry() {
        let mut registry = BindingRegistry::new();
        registry.register("MOCK", Box::new(MockBinding));

        assert!(registry.contains("MOCK"));
        assert_eq!(registry.len(), 1);

        let result = registry.call("MOCK", "test", vec![]);
        assert_eq!(result.as_string(), Some("test result"));

        let result = registry.call("MOCK", "echo", vec![BindingValue::String("hello".to_string())]);
        assert_eq!(result.as_string(), Some("hello"));
    }
}
