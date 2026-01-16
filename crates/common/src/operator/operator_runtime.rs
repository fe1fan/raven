//! Operator 运行时
//!
//! 基于核心 JsRuntime，提供 Operator 场景的脚本执行支持

use crate::runtime::JsRuntime;

/// Operator 运行时
/// 
/// 封装 JsRuntime，提供适用于 Operator 场景的脚本加载和执行
pub struct OperatorRuntime {
    runtime: JsRuntime,
}

impl OperatorRuntime {
    /// 创建新的 Operator 运行时
    pub fn new() -> Self {
        Self {
            runtime: JsRuntime::new(),
        }
    }

    /// 加载并执行 Operator 脚本
    /// 
    /// 脚本会被包装在一个异步立即执行函数 (async IIFE) 中，
    /// 允许在顶层使用 await 关键字
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use common::operator::OperatorRuntime;
    /// 
    /// let mut runtime = OperatorRuntime::new();
    /// let script = r#"
    ///     import { UserManager } from 'raven/identity'
    ///     
    ///     console.log("Hello from operator!");
    ///     await UserManager.addUser({ username: "john", password: "secret" });
    /// "#;
    /// 
    /// runtime.execute(script).expect("Failed to execute script");
    /// ```
    pub fn execute(&mut self, script: &str) -> Result<(), String> {
        self.runtime.load_script(script, |cleaned_script| {
            // 使用 async IIFE 包装脚本，允许顶层 await
            format!(
                r#"
                (async function() {{
                    {}
                }})();
                "#,
                cleaned_script
            )
        })
    }

    /// 获取底层的 JsRuntime 引用（只读）
    /// 
    /// 用于访问底层运行时的其他功能，如绑定注册等
    pub fn runtime(&self) -> &JsRuntime {
        &self.runtime
    }

    /// 获取底层的 JsRuntime 可变引用
    /// 
    /// 用于直接操作底层运行时
    pub fn runtime_mut(&mut self) -> &mut JsRuntime {
        &mut self.runtime
    }
}

impl Default for OperatorRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_runtime() {
        let _runtime = OperatorRuntime::new();
    }

    #[test]
    fn test_execute_simple_script() {
        let mut runtime = OperatorRuntime::new();
        let script = r#"
            console.log("Hello from operator!");
        "#;
        
        assert!(runtime.execute(script).is_ok());
    }

    #[test]
    fn test_execute_with_import() {
        let mut runtime = OperatorRuntime::new();
        let script = r#"
            import { UserManager } from 'raven/identity'
            console.log("UserManager loaded!");
        "#;
        
        assert!(runtime.execute(script).is_ok());
    }
}
