//! 工具函数绑定示例
//!
//! 展示如何创建自定义的 Worker 绑定，提供实用的工具函数。
//!
//! # JS 使用方式
//!
//! ```javascript
//! // 字符串操作
//! const reversed = env.UTILS.reverse("hello"); // "olleh"
//! const hash = env.UTILS.hash("data"); // SHA-256 哈希
//!
//! // 数学计算
//! const sum = env.UTILS.sum([1, 2, 3, 4, 5]); // 15
//! const avg = env.UTILS.average([10, 20, 30]); // 20
//!
//! // JSON 操作
//! const pretty = env.UTILS.prettyJson({name: "test", value: 123});
//!
//! // 时间操作
//! const timestamp = env.UTILS.timestamp(); // 当前时间戳
//! const formatted = env.UTILS.formatDate(timestamp, "YYYY-MM-DD");
//!
//! // Base64 编解码
//! const encoded = env.UTILS.base64Encode("hello");
//! const decoded = env.UTILS.base64Decode(encoded);
//! ```

use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};
use std::collections::HashMap;
use chrono::{Utc, TimeZone};

use super::registry::{BindingMethod, BindingValue, WorkerBinding};

/// 工具函数绑定
///
/// 提供各种实用的工具函数给 JS 环境使用
pub struct UtilsBinding {
    name: String,
}

impl UtilsBinding {
    /// 创建新的工具绑定
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    /// 反转字符串
    fn reverse(&self, args: &[BindingValue]) -> BindingValue {
        match args.first() {
            Some(BindingValue::String(s)) => {
                let reversed: String = s.chars().rev().collect();
                BindingValue::String(reversed)
            }
            _ => BindingValue::Error("reverse requires a string argument".to_string()),
        }
    }

    /// 计算 SHA-256 哈希
    fn hash(&self, args: &[BindingValue]) -> BindingValue {
        let data = match args.first() {
            Some(BindingValue::String(s)) => s.as_bytes().to_vec(),
            Some(BindingValue::Bytes(b)) => b.clone(),
            _ => return BindingValue::Error("hash requires string or bytes".to_string()),
        };

        let mut hasher = Sha256::new();
        hasher.update(&data);
        let result = hasher.finalize();
        BindingValue::String(format!("{:x}", result))
    }

    /// 计算数组和
    fn sum(&self, args: &[BindingValue]) -> BindingValue {
        match args.first() {
            Some(BindingValue::Array(arr)) => {
                let mut total = 0.0;
                for item in arr {
                    match item {
                        BindingValue::Int(i) => total += *i as f64,
                        BindingValue::Float(f) => total += f,
                        _ => continue,
                    }
                }
                BindingValue::Float(total)
            }
            _ => BindingValue::Error("sum requires an array argument".to_string()),
        }
    }

    /// 计算数组平均值
    fn average(&self, args: &[BindingValue]) -> BindingValue {
        match args.first() {
            Some(BindingValue::Array(arr)) => {
                if arr.is_empty() {
                    return BindingValue::Float(0.0);
                }
                
                let mut total = 0.0;
                let mut count = 0;
                for item in arr {
                    match item {
                        BindingValue::Int(i) => {
                            total += *i as f64;
                            count += 1;
                        }
                        BindingValue::Float(f) => {
                            total += f;
                            count += 1;
                        }
                        _ => continue,
                    }
                }
                
                if count > 0 {
                    BindingValue::Float(total / count as f64)
                } else {
                    BindingValue::Float(0.0)
                }
            }
            _ => BindingValue::Error("average requires an array argument".to_string()),
        }
    }

    /// 格式化 JSON（美化输出）
    fn pretty_json(&self, args: &[BindingValue]) -> BindingValue {
        match args.first() {
            Some(BindingValue::String(s)) => {
                // 尝试解析并重新格式化
                match serde_json::from_str::<serde_json::Value>(s) {
                    Ok(v) => match serde_json::to_string_pretty(&v) {
                        Ok(pretty) => BindingValue::String(pretty),
                        Err(e) => BindingValue::Error(format!("Failed to format JSON: {}", e)),
                    },
                    Err(e) => BindingValue::Error(format!("Invalid JSON: {}", e)),
                }
            }
            Some(BindingValue::Object(obj)) => {
                // 直接格式化对象
                match self.binding_value_to_json(obj) {
                    Ok(json) => BindingValue::String(json),
                    Err(e) => BindingValue::Error(e),
                }
            }
            _ => BindingValue::Error("prettyJson requires a string or object".to_string()),
        }
    }

    /// 获取当前时间戳（毫秒）
    fn timestamp(&self, _args: &[BindingValue]) -> BindingValue {
        let now = Utc::now();
        BindingValue::Int(now.timestamp_millis())
    }

    /// 格式化日期
    fn format_date(&self, args: &[BindingValue]) -> BindingValue {
        let timestamp = match args.first() {
            Some(BindingValue::Int(ts)) => *ts,
            Some(BindingValue::Float(ts)) => *ts as i64,
            _ => return BindingValue::Error("formatDate requires a timestamp".to_string()),
        };

        let format = match args.get(1) {
            Some(BindingValue::String(f)) => f.clone(),
            _ => "%Y-%m-%d %H:%M:%S".to_string(), // 默认格式
        };

        // 将时间戳（毫秒）转换为 DateTime
        match Utc.timestamp_millis_opt(timestamp) {
            chrono::LocalResult::Single(dt) => {
                let formatted = dt.format(&format).to_string();
                BindingValue::String(formatted)
            }
            _ => BindingValue::Error("Invalid timestamp".to_string()),
        }
    }

    /// Base64 编码
    fn base64_encode(&self, args: &[BindingValue]) -> BindingValue {
        let data = match args.first() {
            Some(BindingValue::String(s)) => s.as_bytes().to_vec(),
            Some(BindingValue::Bytes(b)) => b.clone(),
            _ => return BindingValue::Error("base64Encode requires string or bytes".to_string()),
        };

        let encoded = general_purpose::STANDARD.encode(&data);
        BindingValue::String(encoded)
    }

    /// Base64 解码
    fn base64_decode(&self, args: &[BindingValue]) -> BindingValue {
        let encoded = match args.first() {
            Some(BindingValue::String(s)) => s,
            _ => return BindingValue::Error("base64Decode requires a string".to_string()),
        };

        match general_purpose::STANDARD.decode(encoded.as_bytes()) {
            Ok(decoded) => {
                // 尝试转换为 UTF-8 字符串
                match String::from_utf8(decoded.clone()) {
                    Ok(s) => BindingValue::String(s),
                    Err(_) => BindingValue::Bytes(decoded),
                }
            }
            Err(e) => BindingValue::Error(format!("Failed to decode base64: {}", e)),
        }
    }

    /// 生成随机字符串
    fn random_string(&self, args: &[BindingValue]) -> BindingValue {
        use rand::{Rng, distributions::Alphanumeric};
        
        let length = match args.first() {
            Some(BindingValue::Int(n)) => *n as usize,
            Some(BindingValue::Float(n)) => *n as usize,
            _ => 16, // 默认长度
        };

        let random_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();

        BindingValue::String(random_string)
    }

    /// 将 BindingValue 对象转换为 JSON 字符串
    fn binding_value_to_json(&self, obj: &HashMap<String, BindingValue>) -> Result<String, String> {
        let json_value = self.binding_value_to_serde_json(&BindingValue::Object(obj.clone()));
        serde_json::to_string_pretty(&json_value)
            .map_err(|e| format!("Failed to serialize: {}", e))
    }

    /// 将 BindingValue 转换为 serde_json::Value
    fn binding_value_to_serde_json(&self, value: &BindingValue) -> serde_json::Value {
        match value {
            BindingValue::Null => serde_json::Value::Null,
            BindingValue::Bool(b) => serde_json::Value::Bool(*b),
            BindingValue::Int(i) => serde_json::Value::Number((*i).into()),
            BindingValue::Float(f) => {
                serde_json::Value::Number(
                    serde_json::Number::from_f64(*f).unwrap_or(0.into())
                )
            }
            BindingValue::String(s) => serde_json::Value::String(s.clone()),
            BindingValue::Array(arr) => {
                let items: Vec<_> = arr.iter()
                    .map(|v| self.binding_value_to_serde_json(v))
                    .collect();
                serde_json::Value::Array(items)
            }
            BindingValue::Object(obj) => {
                let map: serde_json::Map<_, _> = obj.iter()
                    .map(|(k, v)| (k.clone(), self.binding_value_to_serde_json(v)))
                    .collect();
                serde_json::Value::Object(map)
            }
            _ => serde_json::Value::String(format!("{}", value)),
        }
    }
}

impl WorkerBinding for UtilsBinding {
    fn name(&self) -> &str {
        &self.name
    }

    fn methods(&self) -> Vec<BindingMethod> {
        vec![
            // 字符串操作
            BindingMethod::new("reverse", 1),
            BindingMethod::new("hash", 1),
            
            // 数学计算
            BindingMethod::new("sum", 1),
            BindingMethod::new("average", 1),
            
            // JSON 操作
            BindingMethod::new("prettyJson", 1),
            
            // 时间操作
            BindingMethod::new("timestamp", 0),
            BindingMethod::new("formatDate", 2),
            
            // 编码操作
            BindingMethod::new("base64Encode", 1),
            BindingMethod::new("base64Decode", 1),
            
            // 随机
            BindingMethod::new("randomString", 1),
        ]
    }

    fn call(&self, method: &str, args: Vec<BindingValue>) -> BindingValue {
        match method {
            "reverse" => self.reverse(&args),
            "hash" => self.hash(&args),
            "sum" => self.sum(&args),
            "average" => self.average(&args),
            "prettyJson" => self.pretty_json(&args),
            "timestamp" => self.timestamp(&args),
            "formatDate" => self.format_date(&args),
            "base64Encode" => self.base64_encode(&args),
            "base64Decode" => self.base64_decode(&args),
            "randomString" => self.random_string(&args),
            _ => BindingValue::Error(format!("Unknown method: {}", method)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        let utils = UtilsBinding::new("UTILS");
        let result = utils.call("reverse", vec![BindingValue::String("hello".to_string())]);
        assert_eq!(result.as_string(), Some("olleh"));
    }

    #[test]
    fn test_hash() {
        let utils = UtilsBinding::new("UTILS");
        let result = utils.call("hash", vec![BindingValue::String("test".to_string())]);
        // SHA-256 of "test"
        assert!(result.as_string().is_some());
        assert_eq!(
            result.as_string().unwrap(),
            "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
        );
    }

    #[test]
    fn test_sum() {
        let utils = UtilsBinding::new("UTILS");
        let result = utils.call(
            "sum",
            vec![BindingValue::Array(vec![
                BindingValue::Int(1),
                BindingValue::Int(2),
                BindingValue::Int(3),
            ])],
        );
        if let BindingValue::Float(sum) = result {
            assert_eq!(sum, 6.0);
        } else {
            panic!("Expected Float result");
        }
    }

    #[test]
    fn test_average() {
        let utils = UtilsBinding::new("UTILS");
        let result = utils.call(
            "average",
            vec![BindingValue::Array(vec![
                BindingValue::Int(10),
                BindingValue::Int(20),
                BindingValue::Int(30),
            ])],
        );
        if let BindingValue::Float(avg) = result {
            assert_eq!(avg, 20.0);
        } else {
            panic!("Expected Float result");
        }
    }

    #[test]
    fn test_base64() {
        let utils = UtilsBinding::new("UTILS");
        
        // 编码
        let encoded = utils.call("base64Encode", vec![BindingValue::String("hello".to_string())]);
        let encoded_str = encoded.as_string().unwrap();
        
        // 解码
        let decoded = utils.call("base64Decode", vec![BindingValue::String(encoded_str.to_string())]);
        assert_eq!(decoded.as_string(), Some("hello"));
    }

    #[test]
    fn test_timestamp() {
        let utils = UtilsBinding::new("UTILS");
        let result = utils.call("timestamp", vec![]);
        
        if let BindingValue::Int(ts) = result {
            assert!(ts > 0);
        } else {
            panic!("Expected Int result");
        }
    }

    #[test]
    fn test_random_string() {
        let utils = UtilsBinding::new("UTILS");
        
        // 默认长度
        let result = utils.call("randomString", vec![]);
        assert!(result.as_string().is_some());
        assert_eq!(result.as_string().unwrap().len(), 16);
        
        // 自定义长度
        let result = utils.call("randomString", vec![BindingValue::Int(10)]);
        assert_eq!(result.as_string().unwrap().len(), 10);
    }
}
