use common::operator::OperatorRuntime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Raven Operator Identity 模块测试\n");
    
    // 读取测试脚本
    let script = include_str!("operator-test.js");
    
    // 创建 Operator 运行时
    let mut runtime = OperatorRuntime::new();
    
    // 执行脚本
    runtime.execute(script)?;
    
    println!("\n✅ 测试执行完成！");
    
    Ok(())
}
