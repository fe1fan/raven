//! Workers Fetch 运行时
//!
//! 基于核心 JsRuntime，添加 Cloudflare Workers 风格的 fetch() 入口支持

use boa_engine::{
    class::{Class, ClassBuilder},
    js_string, object::ObjectInitializer, property::Attribute, Context, JsArgs, JsData, JsObject,
    JsString, JsValue, NativeFunction,
};
use boa_gc::{Finalize, Trace};
use std::collections::HashMap;

use crate::runtime::JsRuntime;
use super::http::{HttpRequest, HttpResponse};

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

/// Workers Fetch 运行时
///
/// 基于核心 JsRuntime，添加 fetch() 入口支持
pub struct WorkersRuntime {
    runtime: JsRuntime,
}

impl WorkersRuntime {
    /// 创建新的 Fetch 运行时
    pub fn new() -> Self {
        let mut runtime = JsRuntime::new();
        
        // 注册 Response 类
        runtime.context.register_global_class::<JsResponseClass>().unwrap();
        
        Self { runtime }
    }

    /// 获取底层运行时的可变引用
    pub fn runtime_mut(&mut self) -> &mut JsRuntime {
        &mut self.runtime
    }

    /// 加载 Worker 脚本（使用 fetch 入口的包装）
    pub fn load_worker(&mut self, script: &str) -> Result<(), String> {
        self.runtime.load_script(script, |cleaned_script| {
            format!(
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
            )
        })
    }

    /// 处理 HTTP 请求（调用 fetch 入口）
    pub fn handle_request(
        &mut self,
        request: &HttpRequest,
        host: &str,
    ) -> Result<HttpResponse, String> {
        // 设置当前线程的绑定注册表
        self.runtime.set_bindings_context();

        let module = self
            .runtime
            .loaded_module
            .as_ref()
            .ok_or("Worker not loaded")?
            .clone();

        let module_obj = module.as_object().ok_or("Invalid worker module")?;

        let default_export = module_obj
            .get(js_string!("default"), &mut self.runtime.context)
            .map_err(|e| format!("Failed to get default export: {}", e))?;

        let worker_obj = default_export
            .as_object()
            .ok_or("Default export is not an object")?;

        let fetch_fn = worker_obj
            .get(js_string!("fetch"), &mut self.runtime.context)
            .map_err(|e| format!("Failed to get fetch function: {}", e))?;

        let fetch_callable = fetch_fn.as_callable().ok_or("fetch is not a function")?;

        // 构建 Request 对象
        let js_request = self.create_js_request(request, host)?;

        // 构建 env 对象（空对象，保持兼容性）
        let env = ObjectInitializer::new(&mut self.runtime.context).build();

        // 构建 context 对象
        let wait_until_fn = NativeFunction::from_fn_ptr(|_, _, _| Ok(JsValue::undefined()));
        let pass_through_fn = NativeFunction::from_fn_ptr(|_, _, _| Ok(JsValue::undefined()));

        let ctx_obj = ObjectInitializer::new(&mut self.runtime.context)
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
                &mut self.runtime.context,
            )
            .map_err(|e| format!("Failed to call fetch: {}", e))?;

        self.js_response_to_http(result)
    }

    /// 创建 JS Request 对象
    fn create_js_request(&mut self, request: &HttpRequest, host: &str) -> Result<JsObject, String> {
        let url = request.url(host);

        let headers_data = ObjectInitializer::new(&mut self.runtime.context).build();
        for (key, value) in &request.headers {
            headers_data
                .set(
                    JsString::from(key.as_str()),
                    JsValue::from(js_string!(value.as_str())),
                    false,
                    &mut self.runtime.context,
                )
                .ok();
        }

        let headers = ObjectInitializer::new(&mut self.runtime.context)
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
                get_fn.to_js_function(self.runtime.context.realm()),
                false,
                &mut self.runtime.context,
            )
            .ok();

        let body_text = request.body_text().unwrap_or_default();

        let js_request = ObjectInitializer::new(&mut self.runtime.context)
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
            .get(js_string!("status"), &mut self.runtime.context)
            .ok()
            .and_then(|v| v.as_number())
            .map(|n| n as u16)
            .unwrap_or(200);

        let body = response_obj
            .get(js_string!("body"), &mut self.runtime.context)
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
        if let Ok(js_headers) = response_obj.get(js_string!("headers"), &mut self.runtime.context) {
            if let Some(headers_obj) = js_headers.as_object() {
                if let Ok(keys) = headers_obj.own_property_keys(&mut self.runtime.context) {
                    for key in keys {
                        if let Ok(value) = headers_obj.get(key.clone(), &mut self.runtime.context) {
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

impl Default for WorkersRuntime {
    fn default() -> Self {
        Self::new()
    }
}
