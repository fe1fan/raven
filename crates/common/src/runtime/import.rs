//! Import 语句解析和绑定创建

use super::bindings::NativeBinding;
use crate::workers::bindings::{KvBinding, UtilsBinding};
use crate::operator::{
    UserManagerBinding, 
    GroupManagerBinding, 
    PermissionManagerBinding, 
    SudoManagerBinding
};

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

/// 根据导入名称和模块路径创建绑定实例
///
/// # 支持的模块
///
/// - `raven/kv` -> `KV` 键值存储
/// - `raven/utils` -> `UTILS` 工具函数
/// - `raven/identity` -> `UserManager`, `GroupManager`, `PermissionManager`, `SudoManager` 用户和权限管理
/// - `raven/db` -> `DB` 数据库（未实现）
pub fn create_binding_from_module(imported_name: &str, module_path: &str) -> Option<Box<dyn NativeBinding>> {
    match module_path {
        "raven/kv" => {
            let binding = Box::new(KvBinding::memory(imported_name));
            Some(binding)
        },
        "raven/utils" => {
            let binding = Box::new(UtilsBinding::new(imported_name));
            Some(binding)
        },
        "raven/identity" => {
            // 根据导入的名称创建相应的 Manager
            match imported_name {
                "UserManager" => Some(Box::new(UserManagerBinding)),
                "GroupManager" => Some(Box::new(GroupManagerBinding)),
                "PermissionManager" => Some(Box::new(PermissionManagerBinding)),
                "SudoManager" => Some(Box::new(SudoManagerBinding)),
                _ => None,
            }
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
        let result = create_binding_from_module("KV", "raven/kv");
        assert!(result.is_some());
    }

    #[test]
    fn test_create_utils_binding() {
        let result = create_binding_from_module("UTILS", "raven/utils");
        assert!(result.is_some());
    }

    #[test]
    fn test_create_identity_bindings() {
        assert!(create_binding_from_module("UserManager", "raven/identity").is_some());
        assert!(create_binding_from_module("GroupManager", "raven/identity").is_some());
        assert!(create_binding_from_module("PermissionManager", "raven/identity").is_some());
        assert!(create_binding_from_module("SudoManager", "raven/identity").is_some());
    }

    #[test]
    fn test_unknown_module() {
        let result = create_binding_from_module("Unknown", "raven/unknown");
        assert!(result.is_none());
    }
    
    #[test]
    fn test_unknown_identity_binding() {
        let result = create_binding_from_module("UnknownManager", "raven/identity");
        assert!(result.is_none());
    }
}
