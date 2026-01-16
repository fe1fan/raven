//! Import 语句解析和绑定创建

use super::bindings::WorkerBinding;
use crate::workers::bindings::{KvBinding, UtilsBinding};

/// 解析脚本中的 import 语句，提取需要的绑定
pub fn parse_imports(script: &str) -> Vec<(String, String)> {
    let mut imports = Vec::new();
    
    for line in script.lines() {
        let trimmed = line.trim();
        
        // 匹配: import { KV } from 'raven/kv'
        // 或: import { UTILS } from "raven/utils"
        if trimmed.starts_with("import") && trimmed.contains("from") {
            // 简单的正则解析
            if let Some(from_pos) = trimmed.find("from") {
                let import_part = &trimmed[6..from_pos].trim(); // "import" 后面的部分
                let module_part = &trimmed[from_pos + 4..].trim(); // "from" 后面的部分
                
                // 提取导入的名称 (在 {} 中)
                if let (Some(start), Some(end)) = (import_part.find('{'), import_part.find('}')) {
                    let names = &import_part[start + 1..end];
                    for name in names.split(',') {
                        let name = name.trim().to_string();
                        
                        // 提取模块路径 (在引号中)
                        let module = module_part
                            .trim_start_matches('\'')
                            .trim_start_matches('"')
                            .trim_end_matches('\'')
                            .trim_end_matches('"')
                            .trim_end_matches(';')
                            .trim()
                            .to_string();
                        
                        if !name.is_empty() && !module.is_empty() {
                            imports.push((name, module));
                        }
                    }
                }
            }
        }
    }
    
    imports
}

/// 根据模块路径创建绑定实例
///
/// # 支持的模块
///
/// - `raven/kv` -> `KV` 键值存储
/// - `raven/utils` -> `UTILS` 工具函数
/// - `raven/db` -> `DB` 数据库（未实现）
pub fn create_binding_from_module(module_path: &str) -> Option<(String, Box<dyn WorkerBinding>)> {
    match module_path {
        "raven/kv" => {
            let binding = Box::new(KvBinding::memory("KV"));
            Some(("KV".to_string(), binding))
        },
        "raven/utils" => {
            let binding = Box::new(UtilsBinding::new("UTILS"));
            Some(("UTILS".to_string(), binding))
        },
        "raven/db" => {
            // 未来实现数据库绑定
            None
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_import() {
        let script = r#"
            import { KV } from 'raven/kv'
            
            export default {
                fetch() {}
            }
        "#;
        
        let imports = parse_imports(script);
        assert_eq!(imports.len(), 1);
        assert_eq!(imports[0], ("KV".to_string(), "raven/kv".to_string()));
    }

    #[test]
    fn test_parse_multiple_imports() {
        let script = r#"
            import { KV } from 'raven/kv'
            import { UTILS } from 'raven/utils'
            
            export default {
                fetch() {}
            }
        "#;
        
        let imports = parse_imports(script);
        assert_eq!(imports.len(), 2);
        assert_eq!(imports[0], ("KV".to_string(), "raven/kv".to_string()));
        assert_eq!(imports[1], ("UTILS".to_string(), "raven/utils".to_string()));
    }

    #[test]
    fn test_create_kv_binding() {
        let result = create_binding_from_module("raven/kv");
        assert!(result.is_some());
        let (name, _binding) = result.unwrap();
        assert_eq!(name, "KV");
    }

    #[test]
    fn test_create_utils_binding() {
        let result = create_binding_from_module("raven/utils");
        assert!(result.is_some());
        let (name, _binding) = result.unwrap();
        assert_eq!(name, "UTILS");
    }

    #[test]
    fn test_unknown_module() {
        let result = create_binding_from_module("raven/unknown");
        assert!(result.is_none());
    }
}
