// 示例页面 - 展示三种前后端交互方式（Apple 风格）
use crate::components::Layout;
use crate::server_functions::*;
use leptos::*;

#[component]
pub fn ExamplePage() -> impl IntoView {
    view! {
        <Layout>
            <div class="bg-gradient-to-br from-gray-50 to-gray-100 p-8">
                <div class="max-w-7xl mx-auto">
                    // 页面标题
                    <div class="mb-12 text-center">
                        <h1 class="text-5xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent mb-4">
                            "前后端交互示例"
                        </h1>
                        <p class="text-xl text-gray-600">
                            "三种交互方式的完整演示"
                        </p>
                    </div>

                    <div class="space-y-8">
                        // 示例 1: Server Functions
                        <ServerFunctionExample/>

                        // 示例 2: HTTP API
                        <HttpApiExample/>

                        // 示例 3: WebSocket
                        <WebSocketExample/>
                    </div>
                </div>
            </div>
        </Layout>
    }
}

// ==================== 示例 1: Server Functions ====================
#[component]
fn ServerFunctionExample() -> impl IntoView {
    let (servers, set_servers) = create_signal(Vec::<ServerInfo>::new());
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);

    let load_servers = create_action(move |_input: &()| async move {
        set_loading.set(true);
        set_error.set(None);

        match get_servers_sf(None, None).await {
            Ok(data) => {
                set_servers.set(data);
                set_loading.set(false);
            }
            Err(e) => {
                set_error.set(Some(e.to_string()));
                set_loading.set(false);
            }
        }
    });

    view! {
        <div class="bg-white rounded-3xl shadow-2xl overflow-hidden hover:shadow-3xl transition-all duration-300">
            <div class="bg-gradient-to-r from-blue-500 to-blue-600 p-8">
                <div class="flex items-center justify-between">
                    <div>
                        <div class="flex items-center gap-3 mb-2">
                            <div class="w-12 h-12 bg-white/20 backdrop-blur-lg rounded-2xl flex items-center justify-center">
                                <svg class="w-7 h-7 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                                </svg>
                            </div>
                            <h2 class="text-3xl font-bold text-white">"Server Functions"</h2>
                        </div>
                        <p class="text-blue-100 text-lg">"Leptos 内置的类型安全 RPC 机制"</p>
                    </div>
                    <div class="hidden md:flex items-center gap-2 px-4 py-2 bg-white/20 backdrop-blur-lg rounded-full">
                        <div class="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
                        <span class="text-white text-sm font-medium">"推荐使用"</span>
                    </div>
                </div>
            </div>

            <div class="p-8">
                <div class="mb-6">
                    <button
                        on:click=move |_| load_servers.dispatch(())
                        disabled=move || loading.get()
                        class="group relative inline-flex items-center gap-2 px-8 py-4 bg-gradient-to-r from-blue-500 to-blue-600 text-white rounded-2xl font-semibold shadow-lg hover:shadow-xl hover:scale-105 active:scale-95 transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100"
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
                        </svg>
                        {move || if loading.get() { "加载中..." } else { "加载服务器列表" }}
                    </button>
                </div>

                {move || {
                    if loading.get() {
                        view! {
                            <div class="flex items-center justify-center py-12">
                                <div class="relative">
                                    <div class="w-16 h-16 border-4 border-blue-200 border-t-blue-500 rounded-full animate-spin"></div>
                                    <div class="absolute inset-0 flex items-center justify-center">
                                        <div class="w-8 h-8 bg-blue-500 rounded-full animate-pulse"></div>
                                    </div>
                                </div>
                            </div>
                        }.into_view()
                    } else if let Some(err) = error.get() {
                        view! {
                            <div class="bg-red-50 border-l-4 border-red-500 p-6 rounded-2xl">
                                <div class="flex items-center gap-3">
                                    <svg class="w-6 h-6 text-red-500" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"/>
                                    </svg>
                                    <span class="text-red-800 font-medium">"错误: " {err}</span>
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        let server_list = servers.get();
                        if server_list.is_empty() {
                            view! {
                                <div class="text-center py-12">
                                    <svg class="w-20 h-20 text-gray-300 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M12 5l7 7-7 7"/>
                                    </svg>
                                    <p class="text-gray-500 text-lg">"点击按钮加载数据"</p>
                                </div>
                            }.into_view()
                        } else {
                            view! {
                                <div class="grid gap-4">
                                    {server_list.iter().map(|server| {
                                        let status_color = match server.status.as_str() {
                                            "online" => "bg-green-100 text-green-800 border-green-200",
                                            "warning" => "bg-yellow-100 text-yellow-800 border-yellow-200",
                                            _ => "bg-gray-100 text-gray-800 border-gray-200",
                                        };

                                        view! {
                                            <div class="group bg-gray-50 hover:bg-white border-2 border-gray-100 hover:border-blue-200 rounded-2xl p-6 transition-all duration-200 hover:shadow-lg">
                                                <div class="flex items-center justify-between mb-4">
                                                    <div class="flex items-center gap-3">
                                                        <div class="w-12 h-12 bg-gradient-to-br from-blue-500 to-purple-600 rounded-xl flex items-center justify-center text-white font-bold text-lg shadow-lg">
                                                            {server.name.chars().next().unwrap_or('S').to_uppercase().to_string()}
                                                        </div>
                                                        <div>
                                                            <h3 class="font-bold text-gray-900 text-lg">{&server.name}</h3>
                                                            <p class="text-gray-500 text-sm">{&server.ip}</p>
                                                        </div>
                                                    </div>
                                                    <span class={format!("px-4 py-2 rounded-full text-sm font-semibold border-2 {}", status_color)}>
                                                        {&server.status}
                                                    </span>
                                                </div>
                                                <div class="grid grid-cols-3 gap-4">
                                                    <MetricBadge label="CPU" value=format!("{:.1}%", server.cpu) color="blue"/>
                                                    <MetricBadge label="内存" value=format!("{:.1}%", server.memory) color="purple"/>
                                                    <MetricBadge label="磁盘" value=format!("{:.1}%", server.disk) color="pink"/>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            }.into_view()
                        }
                    }
                }}

                <div class="mt-8 p-6 bg-gradient-to-br from-blue-50 to-purple-50 rounded-2xl border-2 border-blue-100">
                    <div class="flex items-start gap-3 mb-3">
                        <svg class="w-6 h-6 text-blue-500 flex-shrink-0 mt-1" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"/>
                        </svg>
                        <div class="flex-1">
                            <h4 class="font-bold text-gray-900 mb-2">"代码示例"</h4>
                            <pre class="text-sm text-gray-700 bg-white/50 rounded-xl p-4 overflow-x-auto">
"#[server(GetServers, \"/api\")]
pub async fn get_servers_sf() -> Result<Vec<ServerInfo>, ServerFnError> {{
    // 服务器端代码
    Ok(servers)
}}

// 客户端调用
let servers = get_servers_sf().await?;"
                            </pre>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn MetricBadge(label: &'static str, value: String, color: &'static str) -> impl IntoView {
    let color_classes = match color {
        "blue" => "bg-blue-100 text-blue-700 border-blue-200",
        "purple" => "bg-purple-100 text-purple-700 border-purple-200",
        "pink" => "bg-pink-100 text-pink-700 border-pink-200",
        _ => "bg-gray-100 text-gray-700 border-gray-200",
    };

    view! {
        <div class={format!("border-2 rounded-xl p-3 {}", color_classes)}>
            <div class="text-xs font-medium opacity-75 mb-1">{label}</div>
            <div class="text-lg font-bold">{value}</div>
        </div>
    }
}

// ==================== 示例 2: HTTP API ====================
#[component]
fn HttpApiExample() -> impl IntoView {
    let (api_response, set_api_response) = create_signal(String::new());
    let (loading, set_loading) = create_signal(false);

    let fetch_api = create_action(move |_input: &()| async move {
        set_loading.set(true);
        set_api_response.set("".to_string());

        match test_http_api_sf().await {
            Ok(data) => {
                set_api_response.set(data);
                set_loading.set(false);
            }
            Err(e) => {
                set_api_response.set(format!("错误: {}", e));
                set_loading.set(false);
            }
        }
    });

    view! {
        <div class="bg-white rounded-3xl shadow-2xl overflow-hidden hover:shadow-3xl transition-all duration-300">
            <div class="bg-gradient-to-r from-emerald-500 to-teal-600 p-8">
                <div class="flex items-center justify-between">
                    <div>
                        <div class="flex items-center gap-3 mb-2">
                            <div class="w-12 h-12 bg-white/20 backdrop-blur-lg rounded-2xl flex items-center justify-center">
                                <svg class="w-7 h-7 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                                </svg>
                            </div>
                            <h2 class="text-3xl font-bold text-white">"HTTP REST API"</h2>
                        </div>
                        <p class="text-emerald-100 text-lg">"标准化的 RESTful 接口"</p>
                    </div>
                </div>
            </div>

            <div class="p-8">
                <div class="mb-6">
                    <button
                        on:click=move |_| fetch_api.dispatch(())
                        disabled=move || loading.get()
                        class="group relative inline-flex items-center gap-2 px-8 py-4 bg-gradient-to-r from-emerald-500 to-teal-600 text-white rounded-2xl font-semibold shadow-lg hover:shadow-xl hover:scale-105 active:scale-95 transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16l2.879-2.879m0 0a3 3 0 104.243-4.242 3 3 0 00-4.243 4.242zM21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                        </svg>
                        {move || if loading.get() { "请求中..." } else { "调用 API 获取统计" }}
                    </button>
                </div>

                {move || {
                    if loading.get() {
                        view! {
                            <div class="flex items-center justify-center py-12">
                                <div class="w-16 h-16 border-4 border-emerald-200 border-t-emerald-500 rounded-full animate-spin"></div>
                            </div>
                        }.into_view()
                    } else if !api_response.get().is_empty() {
                        view! {
                            <div class="bg-gradient-to-br from-gray-50 to-gray-100 rounded-2xl p-6 border-2 border-gray-200">
                                <div class="flex items-center gap-2 mb-3">
                                    <svg class="w-5 h-5 text-emerald-500" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                                    </svg>
                                    <span class="font-semibold text-gray-900">"API 响应"</span>
                                </div>
                                <pre class="text-sm text-gray-700 overflow-x-auto whitespace-pre-wrap font-mono bg-white rounded-xl p-4 border border-gray-200">
                                    {api_response.get()}
                                </pre>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="text-center py-12">
                                <svg class="w-20 h-20 text-gray-300 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z"/>
                                </svg>
                                <p class="text-gray-500 text-lg">"点击按钮调用 API"</p>
                            </div>
                        }.into_view()
                    }
                }}

                <div class="mt-8 p-6 bg-gradient-to-br from-emerald-50 to-teal-50 rounded-2xl border-2 border-emerald-100">
                    <div class="flex items-start gap-3">
                        <svg class="w-6 h-6 text-emerald-500 flex-shrink-0 mt-1" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"/>
                        </svg>
                        <div class="flex-1">
                            <h4 class="font-bold text-gray-900 mb-2">"代码示例"</h4>
                            <pre class="text-sm text-gray-700 bg-white/50 rounded-xl p-4 overflow-x-auto">
"// Axum 路由定义
async fn get_server_stats(State(state): State<AppState>) {{
    Json(ApiResponse::success(stats))
}}

// 客户端调用
fetch('/api/servers/stats')
  .then(res => res.json())"
                            </pre>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

// ==================== 示例 3: WebSocket ====================
#[component]
fn WebSocketExample() -> impl IntoView {
    let (connected, set_connected) = create_signal(false);
    let (message_count, set_message_count) = create_signal(0);

    let simulate_ws = create_action(move |_: &()| async move {
        set_connected.set(true);
        set_message_count.set(0);

        for i in 1..=10 {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            set_message_count.set(i);
        }
        set_connected.set(false);
    });

    view! {
        <div class="bg-white rounded-3xl shadow-2xl overflow-hidden hover:shadow-3xl transition-all duration-300">
            <div class="bg-gradient-to-r from-violet-500 to-purple-600 p-8">
                <div class="flex items-center justify-between">
                    <div>
                        <div class="flex items-center gap-3 mb-2">
                            <div class="w-12 h-12 bg-white/20 backdrop-blur-lg rounded-2xl flex items-center justify-center">
                                <svg class="w-7 h-7 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                                </svg>
                            </div>
                            <h2 class="text-3xl font-bold text-white">"WebSocket"</h2>
                        </div>
                        <p class="text-violet-100 text-lg">"实时双向通信"</p>
                    </div>
                    <div class="hidden md:block">
                        {move || if connected.get() {
                            view! {
                                <div class="flex items-center gap-2 px-4 py-2 bg-green-500/20 backdrop-blur-lg rounded-full">
                                    <div class="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
                                    <span class="text-white text-sm font-medium">"已连接"</span>
                                </div>
                            }.into_view()
                        } else {
                            view! {
                                <div class="flex items-center gap-2 px-4 py-2 bg-white/20 backdrop-blur-lg rounded-full">
                                    <div class="w-2 h-2 bg-gray-400 rounded-full"></div>
                                    <span class="text-white text-sm font-medium">"未连接"</span>
                                </div>
                            }.into_view()
                        }}
                    </div>
                </div>
            </div>

            <div class="p-8">
                <div class="mb-6">
                    <button
                        on:click=move |_| simulate_ws.dispatch(())
                        disabled=move || connected.get()
                        class="inline-flex items-center gap-2 px-8 py-4 bg-gradient-to-r from-violet-500 to-purple-600 text-white rounded-2xl font-semibold shadow-lg hover:shadow-xl hover:scale-105 active:scale-95 transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                        </svg>
                        {move || if connected.get() { "连接中..." } else { "模拟 WebSocket 连接" }}
                    </button>
                </div>

                <div class="bg-gradient-to-br from-gray-900 to-gray-800 rounded-2xl p-6 min-h-[300px] shadow-inner">
                    <div class="flex items-center gap-2 mb-4">
                        <div class="w-3 h-3 bg-red-500 rounded-full"></div>
                        <div class="w-3 h-3 bg-yellow-500 rounded-full"></div>
                        <div class="w-3 h-3 bg-green-500 rounded-full"></div>
                        <span class="ml-4 text-gray-400 text-sm font-mono">"WebSocket Terminal"</span>
                    </div>
                    <div class="font-mono text-sm space-y-2">
                        {move || if connected.get() {
                            view! {
                                <div class="space-y-2">
                                    <div class="text-green-400">> "WebSocket 连接已建立..."</div>
                                    <div class="text-blue-400">> "开始接收实时数据..."</div>
                                    <div class="text-gray-300">
                                        > "已接收 "
                                        <span class="text-yellow-400 font-bold">{message_count}</span>
                                        " 条消息"
                                    </div>
                                    <div class="flex items-center gap-2">
                                        <span class="text-green-400">">"</span>
                                        <div class="w-2 h-4 bg-green-400 animate-pulse"></div>
                                    </div>
                                </div>
                            }.into_view()
                        } else if message_count.get() > 0 {
                            view! {
                                <div class="space-y-2">
                                    <div class="text-yellow-400">> "连接已关闭"</div>
                                    <div class="text-gray-400">> "总共接收了 " {message_count} " 条消息"</div>
                                </div>
                            }.into_view()
                        } else {
                            view! {
                                <div class="text-gray-500">> "等待连接..."</div>
                            }.into_view()
                        }}
                    </div>
                </div>

                <div class="mt-8 p-6 bg-gradient-to-br from-violet-50 to-purple-50 rounded-2xl border-2 border-violet-100">
                    <div class="flex items-start gap-3">
                        <svg class="w-6 h-6 text-violet-500 flex-shrink-0 mt-1" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"/>
                        </svg>
                        <div class="flex-1">
                            <h4 class="font-bold text-gray-900 mb-2">"使用说明"</h4>
                            <div class="text-sm text-gray-700 space-y-2">
                                <p>"WebSocket 提供实时双向通信，适合："</p>
                                <ul class="list-disc list-inside space-y-1 ml-4">
                                    <li>"Web 终端 (SSH/Shell)"</li>
                                    <li>"实时监控数据推送"</li>
                                    <li>"聊天应用"</li>
                                    <li>"协作编辑"</li>
                                </ul>
                                <p class="mt-3 text-xs text-gray-500">
                                    "💡 提示：打开浏览器访问 "
                                    <code class="px-2 py-1 bg-white rounded">"test_websocket.html"</code>
                                    " 查看完整的 WebSocket 示例"
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
