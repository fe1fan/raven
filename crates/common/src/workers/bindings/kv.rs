//! KV 存储绑定
//!
//! 提供类似 Cloudflare Workers KV 的键值存储功能。
//!
//! # JS 使用方式
//!
//! ```javascript
//! // 获取值
//! const value = await env.KV.get("my-key");
//!
//! // 存储值
//! await env.KV.put("my-key", "my-value");
//!
//! // 存储带过期时间的值（秒）
//! await env.KV.put("temp-key", "temp-value", { expirationTtl: 3600 });
//!
//! // 删除值
//! await env.KV.delete("my-key");
//!
//! // 列出所有键
//! const keys = await env.KV.list();
//! ```

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::runtime::bindings::{BindingMethod, BindingValue, NativeBinding};

/// KV 存储后端 trait
///
/// 实现此 trait 可以提供不同的存储后端（内存、文件、Redis 等）
pub trait KvStore: Send + Sync {
    /// 获取值
    fn get(&self, key: &str) -> Option<Vec<u8>>;

    /// 存储值
    fn put(&self, key: &str, value: &[u8], ttl: Option<Duration>) -> Result<(), String>;

    /// 删除值
    fn delete(&self, key: &str) -> Result<bool, String>;

    /// 列出所有键
    fn list(&self, prefix: Option<&str>, limit: Option<usize>) -> Vec<String>;

    /// 检查键是否存在
    fn exists(&self, key: &str) -> bool {
        self.get(key).is_some()
    }
}

/// 内存 KV 存储实现
pub struct MemoryKvStore {
    data: Arc<RwLock<HashMap<String, KvEntry>>>,
}

struct KvEntry {
    value: Vec<u8>,
    expires_at: Option<Instant>,
}

impl Default for MemoryKvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryKvStore {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 清理过期的键
    pub fn cleanup_expired(&self) {
        if let Ok(mut data) = self.data.write() {
            let now = Instant::now();
            data.retain(|_, entry| {
                entry.expires_at.map(|exp| exp > now).unwrap_or(true)
            });
        }
    }
}

impl KvStore for MemoryKvStore {
    fn get(&self, key: &str) -> Option<Vec<u8>> {
        let data = self.data.read().ok()?;
        let entry = data.get(key)?;

        // 检查是否过期
        if let Some(expires_at) = entry.expires_at {
            if Instant::now() > expires_at {
                return None;
            }
        }

        Some(entry.value.clone())
    }

    fn put(&self, key: &str, value: &[u8], ttl: Option<Duration>) -> Result<(), String> {
        let mut data = self.data.write().map_err(|e| e.to_string())?;

        let expires_at = ttl.map(|d| Instant::now() + d);

        data.insert(
            key.to_string(),
            KvEntry {
                value: value.to_vec(),
                expires_at,
            },
        );

        Ok(())
    }

    fn delete(&self, key: &str) -> Result<bool, String> {
        let mut data = self.data.write().map_err(|e| e.to_string())?;
        Ok(data.remove(key).is_some())
    }

    fn list(&self, prefix: Option<&str>, limit: Option<usize>) -> Vec<String> {
        let data = match self.data.read() {
            Ok(d) => d,
            Err(_) => return vec![],
        };

        let now = Instant::now();
        let mut keys: Vec<String> = data
            .iter()
            .filter(|(_, entry)| {
                // 过滤掉过期的键
                entry.expires_at.map(|exp| exp > now).unwrap_or(true)
            })
            .map(|(k, _)| k.clone())
            .filter(|k| {
                // 按前缀过滤
                prefix.map(|p| k.starts_with(p)).unwrap_or(true)
            })
            .collect();

        keys.sort();

        if let Some(limit) = limit {
            keys.truncate(limit);
        }

        keys
    }
}

/// KV 绑定
///
/// 将 KV 存储暴露给 JS 环境
pub struct KvBinding {
    name: String,
    store: Box<dyn KvStore>,
}

impl KvBinding {
    /// 创建新的 KV 绑定
    pub fn new(name: &str, store: Box<dyn KvStore>) -> Self {
        Self {
            name: name.to_string(),
            store,
        }
    }

    /// 使用内存存储创建 KV 绑定
    pub fn memory(name: &str) -> Self {
        Self::new(name, Box::new(MemoryKvStore::new()))
    }

    /// 解析 TTL 参数
    fn parse_ttl(args: &[BindingValue]) -> Option<Duration> {
        // 第三个参数是 options 对象，包含 expirationTtl
        if args.len() > 2 {
            if let BindingValue::Object(opts) = &args[2] {
                if let Some(BindingValue::Int(ttl)) = opts.get("expirationTtl") {
                    return Some(Duration::from_secs(*ttl as u64));
                }
                if let Some(BindingValue::Float(ttl)) = opts.get("expirationTtl") {
                    return Some(Duration::from_secs(*ttl as u64));
                }
            }
            // 也支持直接传 TTL 秒数作为第三个参数
            if let BindingValue::Int(ttl) = &args[2] {
                return Some(Duration::from_secs(*ttl as u64));
            }
        }
        None
    }
}

impl NativeBinding for KvBinding {
    fn name(&self) -> &str {
        &self.name
    }

    fn methods(&self) -> Vec<BindingMethod> {
        vec![
            BindingMethod::async_method("get", 1),
            BindingMethod::async_method("put", 2),
            BindingMethod::async_method("delete", 1),
            BindingMethod::async_method("list", 0),
            BindingMethod::new("getWithMetadata", 1),
        ]
    }

    fn call(&self, method: &str, args: Vec<BindingValue>) -> BindingValue {
        match method {
            "get" => {
                let key = match args.first() {
                    Some(BindingValue::String(k)) => k,
                    _ => return BindingValue::Error("get requires a string key".to_string()),
                };

                match self.store.get(key) {
                    Some(data) => {
                        // 尝试转换为 UTF-8 字符串
                        match String::from_utf8(data.clone()) {
                            Ok(s) => BindingValue::String(s),
                            Err(_) => BindingValue::Bytes(data),
                        }
                    }
                    None => BindingValue::Null,
                }
            }

            "put" => {
                let key = match args.first() {
                    Some(BindingValue::String(k)) => k.clone(),
                    _ => return BindingValue::Error("put requires a string key".to_string()),
                };

                let value = match args.get(1) {
                    Some(BindingValue::String(v)) => v.as_bytes().to_vec(),
                    Some(BindingValue::Bytes(v)) => v.clone(),
                    Some(BindingValue::Int(v)) => v.to_string().into_bytes(),
                    Some(BindingValue::Float(v)) => v.to_string().into_bytes(),
                    Some(BindingValue::Json(v)) => v.as_bytes().to_vec(),
                    _ => return BindingValue::Error("put requires a value".to_string()),
                };

                let ttl = Self::parse_ttl(&args);

                match self.store.put(&key, &value, ttl) {
                    Ok(_) => BindingValue::Null,
                    Err(e) => BindingValue::Error(e),
                }
            }

            "delete" => {
                let key = match args.first() {
                    Some(BindingValue::String(k)) => k,
                    _ => return BindingValue::Error("delete requires a string key".to_string()),
                };

                match self.store.delete(key) {
                    Ok(deleted) => BindingValue::Bool(deleted),
                    Err(e) => BindingValue::Error(e),
                }
            }

            "list" => {
                let prefix = args.first().and_then(|v| {
                    if let BindingValue::String(s) = v {
                        Some(s.as_str())
                    } else if let BindingValue::Object(opts) = v {
                        opts.get("prefix").and_then(|p| p.as_string())
                    } else {
                        None
                    }
                });

                let limit = args.first().and_then(|v| {
                    if let BindingValue::Object(opts) = v {
                        if let Some(BindingValue::Int(l)) = opts.get("limit") {
                            return Some(*l as usize);
                        }
                    }
                    None
                });

                let keys = self.store.list(prefix, limit);
                let result: Vec<BindingValue> = keys
                    .into_iter()
                    .map(BindingValue::String)
                    .collect();

                // 返回 { keys: [...] } 格式
                let mut obj = HashMap::new();
                obj.insert("keys".to_string(), BindingValue::Array(result));
                BindingValue::Object(obj)
            }

            _ => BindingValue::Error(format!("Unknown method: {}", method)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_kv_store() {
        let store = MemoryKvStore::new();

        // 测试 put 和 get
        store.put("key1", b"value1", None).unwrap();
        assert_eq!(store.get("key1"), Some(b"value1".to_vec()));

        // 测试不存在的键
        assert_eq!(store.get("nonexistent"), None);

        // 测试 delete
        assert!(store.delete("key1").unwrap());
        assert_eq!(store.get("key1"), None);

        // 测试 list
        store.put("prefix:a", b"1", None).unwrap();
        store.put("prefix:b", b"2", None).unwrap();
        store.put("other:c", b"3", None).unwrap();

        let all_keys = store.list(None, None);
        assert_eq!(all_keys.len(), 3);

        let prefix_keys = store.list(Some("prefix:"), None);
        assert_eq!(prefix_keys.len(), 2);
    }

    #[test]
    fn test_kv_binding() {
        let binding = KvBinding::memory("KV");

        // 测试 put
        let result = binding.call(
            "put",
            vec![
                BindingValue::String("test-key".to_string()),
                BindingValue::String("test-value".to_string()),
            ],
        );
        assert!(!result.is_error());

        // 测试 get
        let result = binding.call("get", vec![BindingValue::String("test-key".to_string())]);
        assert_eq!(result.as_string(), Some("test-value"));

        // 测试 list
        let result = binding.call("list", vec![]);
        if let BindingValue::Object(obj) = result {
            assert!(obj.contains_key("keys"));
        } else {
            panic!("Expected object result");
        }

        // 测试 delete
        let result = binding.call("delete", vec![BindingValue::String("test-key".to_string())]);
        if let BindingValue::Bool(deleted) = result {
            assert!(deleted);
        } else {
            panic!("Expected bool result");
        }
    }

    #[test]
    fn test_kv_with_ttl() {
        let store = MemoryKvStore::new();

        // 设置一个已过期的条目（TTL = 0）
        store.put("expired", b"value", Some(Duration::from_secs(0))).unwrap();

        // 等待一小段时间确保过期
        std::thread::sleep(Duration::from_millis(10));

        // 应该返回 None
        assert_eq!(store.get("expired"), None);
    }
}
