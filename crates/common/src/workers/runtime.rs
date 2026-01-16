//! JavaScript 运行时
//!
//! 使用 boa_engine 提供类 Cloudflare Workers 的 JS 执行环境。

use boa_engine::{
    class::{Class, ClassBuilder},
    js_string,
    object::ObjectInitializer,
    property::Attribute,
    Context, JsArgs, JsData, JsObject, JsString, JsValue, NativeFunction, Source,
};
use boa_gc::{Finalize, Trace};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::bindings::{BindingRegistry, BindingValue, WorkerBinding, KvBinding, UtilsBinding};
use super::http::{HttpRequest, HttpResponse};

// 使用 thread_local 存储当前请求的绑定注册表
thread_local! {
    static CURRENT_BINDINGS: RefCell<Option<Arc<RwLock<BindingRegistry>>>> = const { RefCell::new(None) };
}

/// 设置当前线程的绑定注册表
fn set_current_bindings(registry: Arc<RwLock<BindingRegistry>>) {
    CURRENT_BINDINGS.with(|cell| {
        *cell.borrow_mut() = Some(registry);
    });
}

/// 获取当前线程的绑定注册表
fn get_current_bindings() -> Option<Arc<RwLock<BindingRegistry>>> {
    CURRENT_BINDINGS.with(|cell| cell.borrow().clone())
}

/// 调用绑定方法的辅助函数
fn call_binding(binding_name: &str, method: &str, args: Vec<BindingValue>) -> BindingValue {
    match get_current_bindings() {
        Some(registry) => {
            let registry = registry.read().unwrap();
            registry.call(binding_name, method, args)
        }
        None => BindingValue::Error("No binding registry available".to_string()),
    }
}

/// 将 JsValue 转换为 BindingValue
fn js_to_binding_value(value: &JsValue, context: &mut Context) -> BindingValue {
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
fn binding_value_to_js(value: BindingValue, context: &mut Context) -> JsValue {
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

/// JavaScript Response 类
#[derive(Debug, Trace, Finalize, JsData)]
struct JsResponseClass {
    body: String,
    status: u16,
    headers: HashMap<String, String>,
}

impl Class for JsResponseClass {
    const NAME: &'static str = "Response";
    const LENGTH: usize = 2;

    fn data_constructor(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<Self> {
        let body = args
            .get_or_undefined(0)
            .to_string(context)?
            .to_std_string_escaped();

        let mut status = 200u16;
        let mut headers = HashMap::new();

        if let Some(init) = args.get_or_undefined(1).as_object() {
            if let Ok(s) = init.get(js_string!("status"), context) {
                if let Some(n) = s.as_number() {
                    status = n as u16;
                }
            }
            if let Ok(h) = init.get(js_string!("headers"), context) {
                if let Some(h_obj) = h.as_object() {
                    if let Ok(keys) = h_obj.own_property_keys(context) {
                        for key in keys {
                            if let Ok(value) = h_obj.get(key.clone(), context) {
                                let key_str = key.to_string().to_lowercase();
                                let value_str = if let Some(s) = value.as_string() {
                                    s.to_std_string_escaped()
                                } else {
                                    value.display().to_string()
                                };
                                headers.insert(key_str, value_str);
                            }
                        }
                    }
                }
            }
        }

        Ok(JsResponseClass {
            body,
            status,
            headers,
        })
    }

    fn init(class: &mut ClassBuilder<'_>) -> boa_engine::JsResult<()> {
        class.method(
            js_string!("getBody"),
            0,
            NativeFunction::from_fn_ptr(|this, _, _| {
                if let Some(obj) = this.as_object() {
                    if let Some(response) = obj.downcast_ref::<JsResponseClass>() {
                        return Ok(JsValue::from(js_string!(response.body.clone())));
                    }
                }
                Ok(JsValue::undefined())
            }),
        );

        class.method(
            js_string!("getStatus"),
            0,
            NativeFunction::from_fn_ptr(|this, _, _| {
                if let Some(obj) = this.as_object() {
                    if let Some(response) = obj.downcast_ref::<JsResponseClass>() {
                        return Ok(JsValue::from(response.status as i32));
                    }
                }
                Ok(JsValue::from(200))
            }),
        );

        Ok(())
    }

    fn object_constructor(
        instance: &JsObject,
        args: &[JsValue],
        context: &mut Context,
    ) -> boa_engine::JsResult<()> {
        let data = Self::data_constructor(&JsValue::from(instance.clone()), args, context)?;

        instance.set(
            js_string!("body"),
            JsValue::from(js_string!(data.body.clone())),
            false,
            context,
        )?;
        instance.set(
            js_string!("status"),
            JsValue::from(data.status as i32),
            false,
            context,
        )?;
        instance.set(
            js_string!("ok"),
            JsValue::from(data.status >= 200 && data.status < 300),
            false,
            context,
        )?;

        let headers_obj = ObjectInitializer::new(context).build();
        for (k, v) in &data.headers {
            headers_obj
                .set(
                    JsString::from(k.as_str()),
                    JsValue::from(js_string!(v.as_str())),
                    false,
                    context,
                )
                .ok();
        }
        instance.set(js_string!("headers"), headers_obj, false, context)?;

        Ok(())
    }
}

/// JavaScript 运行时
pub struct JsRuntime {
    context: Context,
    worker_module: Option<JsValue>,
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

        // 注册 Response 类
        context.register_global_class::<JsResponseClass>().unwrap();

        Self {
            context,
            worker_module: None,
            bindings: Arc::new(RwLock::new(BindingRegistry::new())),
            imported_bindings: Vec::new(),
        }
    }

    /// 注册一个绑定模块
    pub fn register_binding(&mut self, binding: Box<dyn WorkerBinding>) {
        let mut registry = self.bindings.write().unwrap();
        let name = binding.name().to_string();
        registry.register(&name, binding);
    }

    /// 获取绑定注册表
    pub fn bindings(&self) -> Arc<RwLock<BindingRegistry>> {
        Arc::clone(&self.bindings)
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

    /// 注入导入的绑定到全局作用域
    fn inject_imported_bindings(&mut self) {
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

    /// 解析脚本中的 import 语句，提取需要的绑定
    fn parse_imports(script: &str) -> Vec<(String, String)> {
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
    fn create_binding_from_module(module_path: &str) -> Option<(String, Box<dyn WorkerBinding>)> {
        // 根据模块路径创建对应的绑定实例
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
                // 未来实现
                None
            },
            _ => None,
        }
    }

    /// 加载并执行 Worker 脚本
    pub fn load_worker(&mut self, script: &str) -> Result<(), String> {
        // 解析 import 语句
        let imports = Self::parse_imports(script);
        
        println!("📦 检测到 {} 个 import 语句", imports.len());
        for (name, module) in &imports {
            println!("  - import {{ {} }} from '{}'", name, module);
        }
        
        // 根据 import 自动加载和注册绑定
        println!("\n🔧 自动加载绑定模块...");
        self.imported_bindings.clear();
        
        for (imported_name, module_path) in &imports {
            // 根据模块路径创建绑定实例
            if let Some((binding_name, binding)) = Self::create_binding_from_module(module_path) {
                // 验证导入的名称是否与绑定名称匹配
                if imported_name != &binding_name {
                    return Err(format!(
                        "Import name '{}' does not match expected binding '{}' for module '{}'",
                        imported_name, binding_name, module_path
                    ));
                }
                
                // 注册绑定
                if !self.imported_bindings.contains(&binding_name) {
                    let mut registry = self.bindings.write().unwrap();
                    registry.register(&binding_name, binding);
                    drop(registry);
                    
                    self.imported_bindings.push(binding_name.clone());
                    println!("  ✓ {} 模块已加载并注册", binding_name);
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
        
        let wrapped_script = format!(
            r#"
            var __worker_module__ = (function() {{
                var module = {{ exports: {{}} }};
                var exports = module.exports;

                function __export_default__(obj) {{
                    module.exports.default = obj;
                }}

                {}

                return module.exports;
            }})();
            __worker_module__;
            "#,
            cleaned_script.replace("export default", "__export_default__(") + ")"
        );

        let result = self
            .context
            .eval(Source::from_bytes(&wrapped_script))
            .map_err(|e| format!("Failed to load worker: {}", e))?;

        self.worker_module = Some(result);
        
        // 注入导入的绑定到全局作用域
        self.inject_imported_bindings();
        
        Ok(())
    }

    /// 构建 env 对象（现在绑定已经在全局作用域，env 对象只是一个空对象）
    fn build_env_object(&mut self) -> JsObject {
        // 创建一个空的 env 对象，用于保持与 Cloudflare Workers API 的兼容性
        // 实际的绑定已经通过 import 注入到全局作用域
        ObjectInitializer::new(&mut self.context).build()
    }

    /// 处理 HTTP 请求
    pub fn handle_request(
        &mut self,
        request: &HttpRequest,
        host: &str,
    ) -> Result<HttpResponse, String> {
        // 设置当前线程的绑定注册表
        set_current_bindings(Arc::clone(&self.bindings));

        let module = self
            .worker_module
            .as_ref()
            .ok_or("Worker not loaded")?
            .clone();

        let module_obj = module.as_object().ok_or("Invalid worker module")?;

        let default_export = module_obj
            .get(js_string!("default"), &mut self.context)
            .map_err(|e| format!("Failed to get default export: {}", e))?;

        let worker_obj = default_export
            .as_object()
            .ok_or("Default export is not an object")?;

        let fetch_fn = worker_obj
            .get(js_string!("fetch"), &mut self.context)
            .map_err(|e| format!("Failed to get fetch function: {}", e))?;

        let fetch_callable = fetch_fn.as_callable().ok_or("fetch is not a function")?;

        // 构建 Request 对象
        let js_request = self.create_js_request(request, host)?;

        // 构建 env 对象（包含绑定）
        let env = self.build_env_object();

        // 构建 context 对象
        let wait_until_fn = NativeFunction::from_fn_ptr(|_, _, _| Ok(JsValue::undefined()));
        let pass_through_fn = NativeFunction::from_fn_ptr(|_, _, _| Ok(JsValue::undefined()));

        let ctx_obj = ObjectInitializer::new(&mut self.context)
            .function(wait_until_fn, js_string!("waitUntil"), 1)
            .function(pass_through_fn, js_string!("passThroughOnException"), 0)
            .build();

        // 调用 fetch 函数
        let result = fetch_callable
            .call(
                &JsValue::from(worker_obj.clone()),
                &[
                    JsValue::from(js_request),
                    JsValue::from(env),
                    JsValue::from(ctx_obj),
                ],
                &mut self.context,
            )
            .map_err(|e| format!("Failed to call fetch: {}", e))?;

        self.js_response_to_http(result)
    }

    /// 创建 JS Request 对象
    fn create_js_request(&mut self, request: &HttpRequest, host: &str) -> Result<JsObject, String> {
        let url = request.url(host);

        let headers_data = ObjectInitializer::new(&mut self.context).build();
        for (key, value) in &request.headers {
            headers_data
                .set(
                    JsString::from(key.as_str()),
                    JsValue::from(js_string!(value.as_str())),
                    false,
                    &mut self.context,
                )
                .ok();
        }

        let headers = ObjectInitializer::new(&mut self.context)
            .property(js_string!("_data"), headers_data, Attribute::all())
            .build();

        let get_fn = NativeFunction::from_fn_ptr(|this, args, ctx| {
            let key = args
                .get_or_undefined(0)
                .to_string(ctx)?
                .to_std_string_escaped()
                .to_lowercase();

            if let Some(obj) = this.as_object() {
                if let Ok(data) = obj.get(js_string!("_data"), ctx) {
                    if let Some(data_obj) = data.as_object() {
                        return data_obj.get(JsString::from(key), ctx);
                    }
                }
            }
            Ok(JsValue::null())
        });

        headers
            .set(
                js_string!("get"),
                get_fn.to_js_function(self.context.realm()),
                false,
                &mut self.context,
            )
            .ok();

        let body_text = request.body_text().unwrap_or_default();

        let js_request = ObjectInitializer::new(&mut self.context)
            .property(
                js_string!("url"),
                JsValue::from(js_string!(url.as_str())),
                Attribute::all(),
            )
            .property(
                js_string!("method"),
                JsValue::from(js_string!(request.method.as_str())),
                Attribute::all(),
            )
            .property(js_string!("headers"), headers, Attribute::all())
            .property(
                js_string!("body"),
                if body_text.is_empty() {
                    JsValue::null()
                } else {
                    JsValue::from(js_string!(body_text.as_str()))
                },
                Attribute::all(),
            )
            .build();

        Ok(js_request)
    }

    /// 将 JS Response 转换为 HTTP Response
    fn js_response_to_http(&mut self, js_response: JsValue) -> Result<HttpResponse, String> {
        let response_obj = js_response.as_object().ok_or("Response is not an object")?;

        let status = response_obj
            .get(js_string!("status"), &mut self.context)
            .ok()
            .and_then(|v| v.as_number())
            .map(|n| n as u16)
            .unwrap_or(200);

        let body = response_obj
            .get(js_string!("body"), &mut self.context)
            .ok()
            .map(|v| {
                if v.is_null_or_undefined() {
                    String::new()
                } else if let Some(s) = v.as_string() {
                    s.to_std_string_escaped()
                } else {
                    v.display().to_string()
                }
            })
            .unwrap_or_default();

        let mut headers = HashMap::new();
        if let Ok(js_headers) = response_obj.get(js_string!("headers"), &mut self.context) {
            if let Some(headers_obj) = js_headers.as_object() {
                if let Ok(keys) = headers_obj.own_property_keys(&mut self.context) {
                    for key in keys {
                        if let Ok(value) = headers_obj.get(key.clone(), &mut self.context) {
                            let key_str = key.to_string();
                            if !key_str.starts_with('_') {
                                let value_str = if let Some(s) = value.as_string() {
                                    s.to_std_string_escaped()
                                } else {
                                    value.display().to_string()
                                };
                                headers.insert(key_str, value_str);
                            }
                        }
                    }
                }
            }
        }

        let status_text = match status {
            200 => "OK",
            201 => "Created",
            204 => "No Content",
            301 => "Moved Permanently",
            302 => "Found",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "Unknown",
        };

        let mut response = HttpResponse::new(status, status_text);
        response.body = body.as_bytes().to_vec();
        response.headers.insert(
            "content-length".to_string(),
            response.body.len().to_string(),
        );
        for (k, v) in headers {
            response.headers.insert(k, v);
        }

        Ok(response)
    }
}

impl Default for JsRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workers::bindings::KvBinding;

    #[test]
    fn test_runtime_creation() {
        let runtime = JsRuntime::new();
        assert!(runtime.worker_module.is_none());
    }

    #[test]
    fn test_simple_worker() {
        let mut runtime = JsRuntime::new();

        let script = r#"
            export default {
                fetch(request, env, ctx) {
                    return new Response("Hello from Worker!", {
                        status: 200,
                        headers: { "Content-Type": "text/plain" }
                    });
                }
            }
        "#;

        runtime.load_worker(script).expect("Failed to load worker");

        let request = HttpRequest {
            method: "GET".to_string(),
            path: "/".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
            body: Vec::new(),
        };

        let response = runtime
            .handle_request(&request, "localhost:8080")
            .expect("Failed to handle request");

        assert_eq!(response.status, 200);
        assert_eq!(
            String::from_utf8_lossy(&response.body),
            "Hello from Worker!"
        );
    }

    #[test]
    fn test_worker_with_kv() {
        let mut runtime = JsRuntime::new();

        // 注册 KV 绑定
        runtime.register_binding(Box::new(KvBinding::memory("KV")));

        let script = r#"
            export default {
                fetch(request, env, ctx) {
                    // 存储值
                    env.KV.put("test-key", "test-value");

                    // 获取值
                    var value = env.KV.get("test-key");

                    return new Response("KV value: " + value, {
                        status: 200,
                        headers: { "Content-Type": "text/plain" }
                    });
                }
            }
        "#;

        runtime.load_worker(script).expect("Failed to load worker");

        let request = HttpRequest {
            method: "GET".to_string(),
            path: "/".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
            body: Vec::new(),
        };

        let response = runtime
            .handle_request(&request, "localhost:8080")
            .expect("Failed to handle request");

        assert_eq!(response.status, 200);
        assert!(String::from_utf8_lossy(&response.body).contains("test-value"));
    }
}
