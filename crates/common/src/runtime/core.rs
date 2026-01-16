//! 核心 JavaScript 运行时实现
//!
//! 提供基础的 JS 执行环境和绑定管理，不包含特定应用逻辑。

use boa_engine::{
    js_string, object::ObjectInitializer, property::Attribute, Context, JsString, JsValue,
    NativeFunction, Source,
};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::cell::RefCell;

use super::bindings::{BindingRegistry, BindingValue, NativeBinding};
use super::import::{parse_imports, create_binding_from_module};

// 使用 thread_local 存储当前请求的绑定注册表
thread_local! {
    static CURRENT_BINDINGS: RefCell<Option<Arc<RwLock<BindingRegistry>>>> = const { RefCell::new(None) };
}

/// 设置当前线程的绑定注册表
pub fn set_current_bindings(registry: Arc<RwLock<BindingRegistry>>) {
    CURRENT_BINDINGS.with(|cell| {
        *cell.borrow_mut() = Some(registry);
    });
}

/// 获取当前线程的绑定注册表
pub fn get_current_bindings() -> Option<Arc<RwLock<BindingRegistry>>> {
    CURRENT_BINDINGS.with(|cell| cell.borrow().clone())
}

/// 调用绑定方法的辅助函数
pub fn call_binding(binding_name: &str, method: &str, args: Vec<BindingValue>) -> BindingValue {
    match get_current_bindings() {
        Some(registry) => {
            let registry = registry.read().unwrap();
            registry.call(binding_name, method, args)
        }
        None => BindingValue::Error("No binding registry available".to_string()),
    }
}

/// 将 JsValue 转换为 BindingValue
pub fn js_to_binding_value(value: &JsValue, context: &mut Context) -> BindingValue {
    if value.is_null_or_undefined() {
        BindingValue::Null
    } else if let Some(b) = value.as_boolean() {
        BindingValue::Bool(b)
    } else if let Some(n) = value.as_number() {
        if n.fract() == 0.0 {
            BindingValue::Int(n as i64)
        } else {
            BindingValue::Float(n)
        }
    } else if let Some(s) = value.as_string() {
        BindingValue::String(s.to_std_string_escaped())
    } else if let Some(obj) = value.as_object() {
        // 检查是否是数组
        if obj.is_array() {
            let length = obj
                .get(js_string!("length"), context)
                .ok()
                .and_then(|v| v.as_number())
                .map(|n| n as usize)
                .unwrap_or(0);

            let mut arr = Vec::with_capacity(length);
            for i in 0..length {
                if let Ok(item) = obj.get(i as u32, context) {
                    arr.push(js_to_binding_value(&item, context));
                }
            }
            BindingValue::Array(arr)
        } else {
            // 普通对象
            let mut map = HashMap::new();
            if let Ok(keys) = obj.own_property_keys(context) {
                for key in keys {
                    if let Ok(val) = obj.get(key.clone(), context) {
                        let key_str = key.to_string();
                        map.insert(key_str, js_to_binding_value(&val, context));
                    }
                }
            }
            BindingValue::Object(map)
        }
    } else {
        BindingValue::String(value.display().to_string())
    }
}

/// 将 BindingValue 转换为 JsValue
pub fn binding_value_to_js(value: BindingValue, context: &mut Context) -> JsValue {
    match value {
        BindingValue::Null => JsValue::null(),
        BindingValue::Bool(b) => JsValue::from(b),
        BindingValue::Int(i) => JsValue::from(i as f64),
        BindingValue::Float(f) => JsValue::from(f),
        BindingValue::String(s) => JsValue::from(js_string!(s)),
        BindingValue::Bytes(b) => {
            // 转换为 Uint8Array 或者字符串
            match String::from_utf8(b) {
                Ok(s) => JsValue::from(js_string!(s)),
                Err(e) => JsValue::from(js_string!(format!(
                    "<binary data: {} bytes>",
                    e.into_bytes().len()
                ))),
            }
        }
        BindingValue::Json(j) => JsValue::from(js_string!(j)),
        BindingValue::Array(arr) => {
            let length = arr.len();
            let js_arr = ObjectInitializer::new(context).build();
            for (i, item) in arr.into_iter().enumerate() {
                js_arr
                    .set(i as u32, binding_value_to_js(item, context), false, context)
                    .ok();
            }
            js_arr
                .set(
                    js_string!("length"),
                    JsValue::from(length as f64),
                    false,
                    context,
                )
                .ok();
            JsValue::from(js_arr)
        }
        BindingValue::Object(obj) => {
            let js_obj = ObjectInitializer::new(context).build();
            for (k, v) in obj {
                js_obj
                    .set(
                        JsString::from(k),
                        binding_value_to_js(v, context),
                        false,
                        context,
                    )
                    .ok();
            }
            JsValue::from(js_obj)
        }
        BindingValue::Error(e) => {
            // 返回 Error 对象或者 null
            JsValue::from(js_string!(format!("Error: {}", e)))
        }
    }
}

/// 核心 JavaScript 运行时
///
/// 提供基础的 JS 执行环境，不包含特定应用的入口逻辑（如 fetch）
pub struct JsRuntime {
    /// boa_engine 上下文
    pub context: Context,
    /// 已加载的模块
    pub loaded_module: Option<JsValue>,
    /// 绑定注册表
    bindings: Arc<RwLock<BindingRegistry>>,
    /// 记录被 import 的绑定（用于按需加载）
    imported_bindings: Vec<String>,
}

impl JsRuntime {
    /// 创建新的 JS 运行时
    pub fn new() -> Self {
        let mut context = Context::default();

        // 注入全局 API
        Self::inject_console(&mut context);

        Self {
            context,
            loaded_module: None,
            bindings: Arc::new(RwLock::new(BindingRegistry::new())),
            imported_bindings: Vec::new(),
        }
    }

    /// 注入 console 对象
    fn inject_console(context: &mut Context) {
        let log_fn = NativeFunction::from_fn_ptr(|_, args, _| {
            let message = args
                .iter()
                .map(|v| {
                    if let Some(s) = v.as_string() {
                        s.to_std_string_escaped()
                    } else {
                        v.display().to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            println!("[JS] {}", message);
            Ok(JsValue::undefined())
        });

        let error_fn = NativeFunction::from_fn_ptr(|_, args, _| {
            let message = args
                .iter()
                .map(|v| {
                    if let Some(s) = v.as_string() {
                        s.to_std_string_escaped()
                    } else {
                        v.display().to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            eprintln!("[JS ERROR] {}", message);
            Ok(JsValue::undefined())
        });

        let console = ObjectInitializer::new(context)
            .function(log_fn, js_string!("log"), 1)
            .function(error_fn, js_string!("error"), 1)
            .build();

        context
            .register_global_property(js_string!("console"), console, Attribute::all())
            .expect("Failed to register console");
    }

    /// 手动注册一个绑定模块
    pub fn register_binding(&mut self, binding: Box<dyn NativeBinding>) {
        let mut registry = self.bindings.write().unwrap();
        let name = binding.name().to_string();
        registry.register(&name, binding);
    }

    /// 获取绑定注册表
    pub fn bindings(&self) -> Arc<RwLock<BindingRegistry>> {
        Arc::clone(&self.bindings)
    }

    /// 获取已导入的绑定列表
    pub fn imported_bindings(&self) -> &[String] {
        &self.imported_bindings
    }

    /// 加载并执行脚本（通用版本）
    ///
    /// 会自动解析 import 语句并加载所需的绑定
    pub fn load_script(&mut self, script: &str, wrapper_fn: impl FnOnce(&str) -> String) -> Result<(), String> {
        // 解析 import 语句
        let imports = parse_imports(script);
        
        if !imports.is_empty() {
            println!("📦 检测到 {} 个 import 语句", imports.len());
            for (name, module) in &imports {
                println!("  - import {{ {} }} from '{}'", name, module);
            }
        }
        
        // 根据 import 自动加载和注册绑定
        if !imports.is_empty() {
            println!("\n🔧 自动加载绑定模块...");
        }
        
        self.imported_bindings.clear();
        
        for (imported_name, module_path) in &imports {
            // 根据导入名称和模块路径创建绑定实例
            if let Some(binding) = create_binding_from_module(imported_name, module_path) {
                // 注册绑定（使用导入的名称）
                if !self.imported_bindings.contains(imported_name) {
                    let mut registry = self.bindings.write().unwrap();
                    registry.register(imported_name, binding);
                    drop(registry);
                    
                    self.imported_bindings.push(imported_name.clone());
                    println!("  ✓ {} 模块已加载并注册", imported_name);
                }
            } else {
                return Err(format!("Unknown or unsupported module: '{}'", module_path));
            }
        }
        
        // 移除 import 语句（因为 boa 不原生支持 ES6 import）
        let cleaned_script = script
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.starts_with("import") || !trimmed.contains("from")
            })
            .collect::<Vec<_>>()
            .join("\n");
        
        // 应用包装函数
        let wrapped_script = wrapper_fn(&cleaned_script);

        let result = self
            .context
            .eval(Source::from_bytes(&wrapped_script))
            .map_err(|e| format!("Failed to load script: {}", e))?;

        self.loaded_module = Some(result);
        
        // 注入导入的绑定到全局作用域
        self.inject_imported_bindings();
        
        Ok(())
    }

    /// 注入导入的绑定到全局作用域
    fn inject_imported_bindings(&mut self) {
        if self.imported_bindings.is_empty() {
            return;
        }
        
        println!("\n🔧 注入导入的绑定到全局作用域...");
        
        for binding_name in &self.imported_bindings {
            // 获取绑定的所有方法
            let methods: Vec<(String, i32)> = {
                let registry = self.bindings.read().unwrap();
                if let Some(binding) = registry.get(binding_name) {
                    binding
                        .methods()
                        .iter()
                        .map(|m| (m.name.clone(), m.arity))
                        .collect()
                } else {
                    vec![]
                }
            };

            if methods.is_empty() {
                eprintln!("⚠️  绑定 {} 没有方法", binding_name);
                continue;
            }

            // 创建绑定对象
            let binding_obj = ObjectInitializer::new(&mut self.context).build();

            // 为每个方法创建 JS 函数
            for (method_name, _arity) in methods {
                let binding_name_clone = binding_name.clone();
                let method_name_clone = method_name.clone();

                // 使用 from_closure 创建捕获闭包的原生函数
                let method_fn = unsafe {
                    NativeFunction::from_closure(move |_, args, ctx| {
                        // 将 JS 参数转换为 BindingValue
                        let binding_args: Vec<BindingValue> = args
                            .iter()
                            .map(|arg| js_to_binding_value(arg, ctx))
                            .collect();

                        // 调用绑定方法
                        let result =
                            call_binding(&binding_name_clone, &method_name_clone, binding_args);

                        // 将结果转换回 JsValue
                        Ok(binding_value_to_js(result, ctx))
                    })
                };

                binding_obj
                    .set(
                        JsString::from(method_name.as_str()),
                        method_fn.to_js_function(self.context.realm()),
                        false,
                        &mut self.context,
                    )
                    .ok();
            }

            // 注册到全局作用域
            self.context
                .register_global_property(
                    JsString::from(binding_name.as_str()),
                    binding_obj,
                    Attribute::all(),
                )
                .expect(&format!("Failed to register global binding: {}", binding_name));

            println!("  ✓ {} 已注入全局作用域", binding_name);
        }
    }

    /// 设置当前线程的绑定上下文
    pub fn set_bindings_context(&self) {
        set_current_bindings(Arc::clone(&self.bindings));
    }
}

impl Default for JsRuntime {
    fn default() -> Self {
        Self::new()
    }
}
