use common::runtime::JsRuntime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Raven Operator Identity 模块测试\n");
    
    // 读取测试脚本
    let script = include_str!("operator-test.js");
    
    // 创建 JS 运行时
    let mut runtime = JsRuntime::new();
    
    // 加载并执行脚本（使用简单的包装，直接执行）
    runtime.load_script(script, |cleaned_script| {
        format!(
            r#"
            (async function() {{
                {}
            }})();
            "#,
            cleaned_script
        )
    })?;
    
    println!("\n✅ 测试执行完成！");
    
    Ok(())
}
