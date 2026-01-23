use leptos::*;
use leptos_router::*;

// 通用按钮组件 - 简化版
#[component]
pub fn ButtonSimple(
    #[prop(optional, default = "primary")] variant: &'static str,
    #[prop(optional, default = "medium")] size: &'static str,
    #[prop(optional, default = "")] class: &'static str,
    children: Children,
) -> impl IntoView {
    let base_class = "inline-flex items-center justify-center font-medium transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed";

    let variant_class = match variant {
        "primary" => "bg-apple-blue hover:bg-apple-blue/90 text-white shadow-lg shadow-apple-blue/30 focus:ring-apple-blue/50",
        "secondary" => "bg-apple-gray-200 dark:bg-apple-gray-700 text-apple-label dark:text-apple-darkLabel hover:bg-apple-gray-300 dark:hover:bg-apple-gray-600 focus:ring-apple-gray-400",
        "danger" => "bg-apple-red hover:bg-apple-red/90 text-white shadow-lg shadow-apple-red/30 focus:ring-apple-red/50",
        "ghost" => "bg-transparent hover:bg-black/5 dark:hover:bg-white/10 text-apple-blue dark:text-apple-darkBlue",
        "success" => "bg-apple-green hover:bg-apple-green/90 text-white shadow-lg shadow-apple-green/30 focus:ring-apple-green/50",
        _ => "bg-apple-blue hover:bg-apple-blue/90 text-white shadow-lg shadow-apple-blue/30 focus:ring-apple-blue/50",
    };

    let size_class = match size {
        "small" => "px-3 py-1.5 text-xs rounded-lg",
        "medium" => "px-5 py-2.5 text-sm rounded-apple-xl",
        "large" => "px-6 py-3 text-base rounded-apple-2xl",
        _ => "px-5 py-2.5 text-sm rounded-apple-xl",
    };

    view! {
        <button class=format!("{} {} {} {}", base_class, variant_class, size_class, class)>
            {children()}
        </button>
    }
}

// 输入框组件
#[component]
pub fn Input(
    #[prop(optional, default = "text")] type_: &'static str,
    #[prop(optional, default = "")] placeholder: &'static str,
    #[prop(optional, default = "")] class: &'static str,
    #[prop(optional)] icon: Option<View>,
) -> impl IntoView {
    let icon_clone = icon.clone();
    view! {
        <div class="relative group">
            {move || icon_clone.clone().map(|i| view! {
                <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel group-focus-within:text-apple-blue transition-colors">
                    {i}
                </div>
            })}
            <input
                type=type_
                placeholder=placeholder
                class=format!("w-full bg-apple-gray-200/50 dark:bg-white/10 border-none rounded-apple-xl py-2 text-sm placeholder-apple-secondaryLabel dark:placeholder-apple-darkSecondaryLabel text-apple-label dark:text-apple-darkLabel focus:outline-none focus:ring-2 focus:ring-apple-blue/50 transition-all {} {}",
                    if icon.is_some() { "pl-9 pr-4" } else { "px-4" },
                    class
                )
            />
        </div>
    }
}

// 复选框组件
#[component]
pub fn Checkbox(
    #[prop(optional)] checked: bool,
    #[prop(optional)] label: Option<String>,
    #[prop(optional, default = "")] class: &'static str,
) -> impl IntoView {
    view! {
        <label class=format!("inline-flex items-center cursor-pointer group {}", class)>
            <div class="relative">
                <input type="checkbox" checked=checked class="peer sr-only"/>
                <div class="w-5 h-5 bg-apple-gray-200/50 dark:bg-white/10 rounded-md border border-apple-gray-200/50 dark:border-white/10 peer-checked:bg-apple-blue peer-checked:border-apple-blue transition-all duration-200 flex items-center justify-center backdrop-blur-md">
                    <svg class="w-3.5 h-3.5 text-white scale-0 peer-checked:scale-100 transition-transform duration-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7"/>
                    </svg>
                </div>
                <div class="absolute inset-0 rounded-md ring-0 peer-focus:ring-2 peer-focus:ring-apple-blue/50 transition-all"></div>
            </div>
            {match label {
                Some(text) => view! {
                    <span class="ml-2 text-sm font-medium text-apple-label dark:text-apple-darkLabel group-hover:text-apple-blue transition-colors">
                        {text}
                    </span>
                }.into_view(),
                None => view! {}.into_view()
            }}
        </label>
    }
}

// 单选框组件
#[component]
pub fn Radio(
    #[prop(optional)] checked: bool,
    #[prop(optional)] name: &'static str,
    #[prop(optional)] label: Option<String>,
    #[prop(optional, default = "")] class: &'static str,
) -> impl IntoView {
    view! {
        <label class=format!("inline-flex items-center cursor-pointer group {}", class)>
            <div class="relative">
                <input type="radio" name=name checked=checked class="peer sr-only"/>
                <div class="w-5 h-5 bg-apple-gray-200/50 dark:bg-white/10 rounded-full border border-apple-gray-200/50 dark:border-white/10 peer-checked:bg-apple-blue peer-checked:border-apple-blue transition-all duration-200 flex items-center justify-center backdrop-blur-md">
                    <div class="w-2 h-2 bg-white rounded-full scale-0 peer-checked:scale-100 transition-transform duration-200 shadow-sm"></div>
                </div>
                <div class="absolute inset-0 rounded-full ring-0 peer-focus:ring-2 peer-focus:ring-apple-blue/50 transition-all"></div>
            </div>
            {match label {
                Some(text) => view! {
                    <span class="ml-2 text-sm font-medium text-apple-label dark:text-apple-darkLabel group-hover:text-apple-blue transition-colors">
                        {text}
                    </span>
                }.into_view(),
                None => view! {}.into_view()
            }}
        </label>
    }
}

// 文本域组件
#[component]
pub fn Textarea(
    #[prop(optional, default = "")] placeholder: &'static str,
    #[prop(optional, default = 4)] rows: usize,
    #[prop(optional, default = "")] class: &'static str,
) -> impl IntoView {
    view! {
        <textarea
            placeholder=placeholder
            rows=rows
            class=format!("w-full bg-apple-gray-200/50 dark:bg-white/10 border-none rounded-apple-xl px-4 py-3 text-sm placeholder-apple-secondaryLabel dark:placeholder-apple-darkSecondaryLabel text-apple-label dark:text-apple-darkLabel focus:outline-none focus:ring-2 focus:ring-apple-blue/50 transition-all resize-none shadow-inner {}", class)
        ></textarea>
    }
}

// 自定义下拉选择组件
#[component]
pub fn Select(
    #[prop(optional)] options: Vec<(String, String)>, // (value, label)
    #[prop(optional)] selected: Option<RwSignal<String>>,
    #[prop(optional, default = "请选择")] placeholder: &'static str,
    #[prop(optional, default = "")] class: &'static str,
) -> impl IntoView {
    // Use local signal if no external signal provided (for demo/stateless usage)
    let internal_selected = create_rw_signal(String::new());
    let selected_signal = selected.unwrap_or(internal_selected);

    let _container_ref = create_node_ref::<html::Details>();

    // Compute display label
    let options_for_label = options.clone();
    let display_label = move || {
        let val = selected_signal.get();
        if val.is_empty() {
            placeholder.to_string()
        } else {
            options_for_label.iter()
                .find(|(v, _)| *v == val)
                .map(|(_, l)| l.clone())
                .unwrap_or(val)
        }
    };

    view! {
        <details class=format!("relative min-w-[140px] group/select {}", class) node_ref=_container_ref>
            <summary class="list-none w-full flex items-center justify-between bg-apple-gray-200/50 dark:bg-white/10 border border-transparent hover:border-apple-gray-300 dark:hover:border-white/20 rounded-apple-xl px-4 py-2 text-sm text-apple-label dark:text-apple-darkLabel focus:outline-none focus:ring-2 focus:ring-apple-blue/50 transition-all duration-200 cursor-pointer">
                <span class="truncate">{display_label}</span>
                <svg
                    class="w-4 h-4 text-apple-secondaryLabel transition-transform duration-200 group-open/select:rotate-180"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                </svg>
            </summary>

            <div class="absolute z-50 mt-2 w-full glass-card rounded-apple-xl shadow-xl border border-apple-gray-200/50 dark:border-white/10 overflow-hidden">
                <div class="max-h-60 overflow-y-auto py-1">
                    {options.clone().into_iter().map(|(val, label)| {
                        let val_clone = val.clone();
                        let label_clone = label.clone();
                        view! {
                            <div
                                class="px-4 py-2 text-sm text-apple-label dark:text-apple-darkLabel hover:bg-apple-blue hover:text-white cursor-pointer transition-colors flex items-center justify-between group"
                                on:click=move |_| {
                                    selected_signal.set(val_clone.clone());
                                    // Close details on selection
                                    if let Some(details) = _container_ref.get() {
                                        let _ = details.remove_attribute("open");
                                    }
                                }
                                attr:onclick="this.closest('details').removeAttribute('open'); this.closest('details').querySelector('summary .truncate').textContent = this.firstElementChild.textContent;"
                            >
                                <span>{label_clone}</span>
                                {move || if selected_signal.get() == val {
                                    view! { <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg> }.into_view()
                                } else {
                                    view! {}.into_view()
                                }}
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </details>
    }
}

// 开关组件
#[component]
pub fn Switch(
    #[prop(optional)] checked: bool,
) -> impl IntoView {
    let (is_checked, set_checked) = create_signal(checked);

    view! {
        <button
            type="button"
            role="switch"
            aria-checked=move || is_checked.get().to_string()
            on:click=move |_| set_checked.update(|v| *v = !*v)
            class=move || format!(
                "relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-apple-blue/50 focus:ring-offset-2 {}",
                if is_checked.get() { "bg-apple-green" } else { "bg-apple-gray-200 dark:bg-apple-gray-600" }
            )
        >
            <span
                aria-hidden="true"
                class=move || format!(
                    "pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {}",
                    if is_checked.get() { "translate-x-5" } else { "translate-x-0" }
                )
            />
        </button>
    }
}

// 分段控制器组件
#[component]
pub fn SegmentedControl(
    options: Vec<&'static str>,
    #[prop(optional)] active_index: usize,
) -> impl IntoView {
    let (current, set_current) = create_signal(active_index);

    view! {
        <div class="flex gap-1 p-1 bg-apple-gray-200/50 dark:bg-white/10 w-fit rounded-apple-2xl">
            {options.into_iter().enumerate().map(|(idx, label)| {
                view! {
                    <button
                        on:click=move |_| set_current.set(idx)
                        class=move || {
                            if current.get() == idx {
                                "px-6 py-2 bg-white dark:bg-white/10 shadow-sm rounded-apple-xl text-sm font-bold text-apple-label dark:text-apple-darkLabel transition-all"
                            } else {
                                "px-6 py-2 text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel text-sm font-medium hover:text-apple-label dark:hover:text-apple-darkLabel transition-all"
                            }
                        }
                    >
                        {label}
                    </button>
                }
            }).collect_view()}
        </div>
    }
}

// 头像组件
#[component]
pub fn Avatar(
    #[prop(into)] name: String,
    #[prop(optional, default = "medium")] size: &'static str,
    #[prop(optional, default = "")] class: &'static str,
) -> impl IntoView {
    let initial = name.chars().next().unwrap_or('?').to_uppercase().to_string();

    let size_class = match size {
        "small" => "w-8 h-8 text-xs",
        "medium" => "w-9 h-9 text-sm",
        "large" => "w-14 h-14 text-xl",
        _ => "w-10 h-10 text-base",
    };

    view! {
        <div class=format!("{} bg-gradient-to-br from-apple-blue to-apple-indigo flex items-center justify-center text-white font-bold shadow-md rounded-full {}", size_class, class)>
            {initial}
        </div>
    }
}

// 用户资料组件
#[component]
pub fn UserProfile(
    name: &'static str,
    role: &'static str,
    #[prop(optional, default = "")] class: &'static str,
) -> impl IntoView {
    view! {
        <div class=format!("flex items-center gap-3 {}", class)>
            <div class="text-right hidden sm:block">
                <p class="text-xs font-semibold text-apple-label dark:text-apple-darkLabel">{name}</p>
                <p class="text-[10px] text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel">{role}</p>
            </div>
            <Avatar name=name.to_string() size="medium" class="cursor-pointer hover:scale-105 transition-transform"/>
        </div>
    }
}

// 数据展示组件 (等宽字体)
#[component]
pub fn DisplayText(
    #[prop(into)] value: String,
    #[prop(optional, default = "")] class: &'static str,
) -> impl IntoView {
    view! {
        <span class=format!("font-mono text-sm text-apple-label dark:text-apple-darkLabel {}", class)>
            {value}
        </span>
    }
}

// 代码编辑器组件 (带行号)
#[component]
pub fn CodeEditor(
    #[prop(optional, into)] value: String,
    #[prop(optional, default = "")] placeholder: &'static str,
    #[prop(optional, default = "")] class: &'static str,
) -> impl IntoView {
    let (content, set_content) = create_signal(value);

    let line_numbers = move || {
        let lines = content.get().lines().count().max(1);
        (1..=lines).map(|n| n.to_string()).collect::<Vec<_>>().join("\n")
    };

    view! {
        <div class=format!("relative flex bg-[#1e1e1e] rounded-apple-xl overflow-hidden border border-white/10 shadow-inner {}", class)>
            // Line numbers
            <div class="py-3 pl-3 pr-2 text-right bg-[#252525] border-r border-white/10 select-none min-w-[2.5rem]">
                <pre class="text-sm font-mono text-apple-gray-500 leading-6">{line_numbers}</pre>
            </div>
            // Editor area
            <textarea
                class="flex-1 bg-transparent text-gray-300 p-3 text-sm font-mono leading-6 focus:outline-none resize-none"
                placeholder=placeholder
                on:input=move |ev| set_content.set(event_target_value(&ev))
                prop:value=content
                spellcheck="false"
            ></textarea>
        </div>
    }
}

// 表格组件系列
#[component]
pub fn Table(children: Children) -> impl IntoView {
    view! {
        <div class="overflow-x-auto">
            <table class="w-full text-left border-collapse">
                {children()}
            </table>
        </div>
    }
}

#[component]
pub fn TableHeader(children: Children) -> impl IntoView {
    view! {
        <thead>
            <tr class="border-b border-apple-gray-200/50 dark:border-white/10">
                {children()}
            </tr>
        </thead>
    }
}

#[component]
pub fn TableHead(children: Children) -> impl IntoView {
    view! {
        <th class="py-3 px-4 text-xs font-semibold text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel uppercase tracking-wider">
            {children()}
        </th>
    }
}

#[component]
pub fn TableBody(children: Children) -> impl IntoView {
    view! {
        <tbody class="divide-y divide-apple-gray-200/50 dark:divide-white/10">
            {children()}
        </tbody>
    }
}

#[component]
pub fn TableRow(children: Children) -> impl IntoView {
    view! {
        <tr class="group hover:bg-apple-blue/5 dark:hover:bg-white/5 transition-colors">
            {children()}
        </tr>
    }
}

#[component]
pub fn TableCell(
    #[prop(optional, default = "")] class: &'static str,
    children: Children
) -> impl IntoView {
    view! {
        <td class=format!("py-3 px-4 text-sm text-apple-label dark:text-apple-darkLabel {}", class)>
            {children()}
        </td>
    }
}

// 状态点组件
#[component]
pub fn StatusDot(
    #[prop(optional, default = "success")] variant: &'static str,
    #[prop(optional, default = true)] animate: bool,
) -> impl IntoView {
    let color_class = match variant {
        "success" => "bg-apple-green",
        "danger" => "bg-apple-red",
        "warning" => "bg-apple-yellow",
        "gray" | "offline" => "bg-apple-gray-500",
        _ => "bg-apple-blue",
    };

    view! {
        <span class=format!("w-1.5 h-1.5 rounded-full {} {}", color_class, if animate { "animate-pulse" } else { "" })></span>
    }
}

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen flex bg-apple-gray-100 dark:bg-[#000000] transition-colors duration-300">
            <Sidebar/>
            <div class="flex-1 flex flex-col min-w-0">
                <Header/>
                <main class="flex-1 overflow-y-auto p-6 md:p-8">
                    <div class="max-w-7xl mx-auto">
                        {children()}
                    </div>
                </main>
            </div>
        </div>
    }
}

#[component]
pub fn PageHeader(
    title: &'static str,
    subtitle: &'static str,
    #[prop(optional)] action: Option<View>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col md:flex-row md:items-center justify-between gap-4 mb-8">
            <div>
                <h1 class="text-3xl font-bold tracking-tight text-apple-label dark:text-apple-darkLabel mb-2">{title}</h1>
                <p class="text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel">{subtitle}</p>
            </div>
            {action}
        </div>
    }
}

#[component]
pub fn GlassCard(
    #[prop(optional, default = "")] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("glass-card p-6 rounded-apple-3xl {}", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn Badge(
    #[prop(into)] text: String,
    #[prop(optional, default = "primary")] variant: &'static str,
) -> impl IntoView {
    let base_class = "px-2.5 py-1 text-xs font-bold rounded-full border";
    let variant_class = match variant {
        "success" => "bg-apple-green/10 text-apple-green dark:bg-apple-green/20 dark:text-apple-darkGreen border-apple-green/20",
        "danger" => "bg-apple-red/10 text-apple-red dark:bg-apple-red/20 dark:text-apple-darkRed border-apple-red/20",
        "warning" => "bg-apple-yellow/10 text-apple-yellow dark:bg-apple-yellow/20 dark:text-apple-darkYellow border-apple-yellow/20",
        "indigo" => "bg-apple-indigo/10 text-apple-indigo dark:bg-apple-indigo/20 dark:text-apple-darkIndigo border-apple-indigo/20",
        _ => "bg-apple-blue/10 text-apple-blue dark:bg-apple-blue/20 dark:text-apple-darkBlue border-apple-blue/20",
    };

    view! {
        <span class=format!("{} {}", base_class, variant_class)>
            {text}
        </span>
    }
}

#[component]
pub fn DashboardCard(
    value: &'static str,
    title: &'static str,
    badge: &'static str,
    badge_type: &'static str,
    icon_bg: &'static str,
    icon_svg: &'static str,
) -> impl IntoView {
    view! {
        <div class="glass-card p-6 rounded-apple-3xl group transition-all duration-300 hover:scale-[1.02] hover:shadow-xl hover:shadow-black/5 dark:hover:shadow-white/5">
            <div class="flex items-center justify-between mb-4">
                <div class=format!("w-12 h-12 {} rounded-apple-xl flex items-center justify-center text-white shadow-lg", icon_bg)>
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d=icon_svg/>
                    </svg>
                </div>
                <Badge text=badge.to_string() variant=badge_type/>
            </div>
            <p class="text-3xl font-bold tracking-tight text-apple-label dark:text-apple-darkLabel">
                <DisplayText value=value.to_string() class="text-3xl font-bold" />
            </p>
            <p class="text-sm text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel font-medium mt-1">{title}</p>
        </div>
    }
}

#[component]
pub fn ServerStatCard(value: &'static str, label: &'static str, color: &'static str) -> impl IntoView {
    let (text_color, bg_color) = match color {
        "success" => ("text-apple-green", "bg-apple-green/10 dark:bg-apple-green/20"),
        "warning" => ("text-apple-yellow", "bg-apple-yellow/10 dark:bg-apple-yellow/20"),
        "danger" => ("text-apple-red", "bg-apple-red/10 dark:bg-apple-red/20"),
        _ => ("text-apple-blue", "bg-apple-blue/10 dark:bg-apple-blue/20"),
    };

    view! {
        <div class="glass-card p-4 rounded-apple-2xl flex items-center justify-between">
            <div>
                <p class="text-2xl font-bold text-apple-label dark:text-apple-darkLabel">
                    <DisplayText value=value.to_string() class="text-2xl font-bold" />
                </p>
                <p class="text-xs font-medium text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel mt-0.5">{label}</p>
            </div>
            <div class=format!("w-10 h-10 rounded-apple-xl flex items-center justify-center {}", bg_color)>
                <div class=format!("w-3 h-3 rounded-full {}", text_color.replace("text-", "bg-"))></div>
            </div>
        </div>
    }
}

#[component]
fn Sidebar() -> impl IntoView {
    let location = use_location();

    view! {
        <aside class="flex-shrink-0 glass-card border-r border-apple-gray-200/50 dark:border-white/10 z-50 h-screen sticky top-0 transition-all duration-300">
            <div class="h-16 flex items-center justify-between px-6 border-b border-apple-gray-200/50 dark:border-white/10">
                <div class="flex items-center gap-3 sidebar-content whitespace-nowrap overflow-hidden">
                    <img src="/assets/logo.png" class="w-8 h-8 rounded-lg shadow-sm" alt="Raven Logo"
                         onerror="this.style.display='none';this.nextElementSibling.style.display='flex'"/>
                    <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-apple-gray-800 to-black hidden items-center justify-center text-white font-bold text-lg">
                        "R"
                    </div>
                    <span class="text-lg font-bold tracking-tight text-apple-label dark:text-apple-darkLabel">"Raven"</span>
                </div>

                <button
                    onclick="toggleSidebar()"
                    class="p-2 text-apple-secondaryLabel hover:bg-black/5 dark:hover:bg-white/10 rounded-lg transition-all"
                >
                    <svg class="w-5 h-5 collapse-icon transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 19l-7-7 7-7m8 14l-7-7 7-7"/>
                    </svg>
                </button>
            </div>

            <nav class="sidebar-nav p-4 space-y-6 overflow-y-auto h-[calc(100vh-64px)]">
                <NavSection title="概览">
                    <NavItem href="/" icon=Icon::Dashboard label="仪表盘" is_active=move || location.pathname.get() == "/"/>
                    <NavItem href="/servers" icon=Icon::Server label="服务器管理" is_active=move || location.pathname.get() == "/servers"/>
                </NavSection>

                <NavSection title="监控与告警">
                    <NavItem href="/monitoring" icon=Icon::Monitor label="监控中心" is_active=move || location.pathname.get() == "/monitoring"/>
                    <NavItem href="/alerts" icon=Icon::Alert label="告警中心" is_active=move || location.pathname.get() == "/alerts"/>
                </NavSection>

                <NavSection title="运维工具">
                    <NavItem href="/terminal" icon=Icon::Terminal label="Web终端" is_active=move || location.pathname.get() == "/terminal"/>
                    <NavItem href="/commands" icon=Icon::Command label="命令执行" is_active=move || location.pathname.get() == "/commands"/>
                    <NavItem href="/cronjobs" icon=Icon::Cronjob label="定时任务" is_active=move || location.pathname.get() == "/cronjobs"/>
                </NavSection>

                <NavSection title="服务管理">
                    <NavItem href="/docker" icon=Icon::Docker label="容器管理" is_active=move || location.pathname.get() == "/docker"/>
                    <NavItem href="/database" icon=Icon::Database label="数据库" is_active=move || location.pathname.get() == "/database"/>
                    <NavItem href="/files" icon=Icon::Files label="文件管理" is_active=move || location.pathname.get() == "/files"/>
                </NavSection>

                <NavSection title="安全与审计">
                    <NavItem href="/firewall" icon=Icon::Firewall label="防火墙" is_active=move || location.pathname.get() == "/firewall"/>
                    <NavItem href="/audit" icon=Icon::Audit label="操作审计" is_active=move || location.pathname.get() == "/audit"/>
                    <NavItem href="/users" icon=Icon::Users label="用户权限" is_active=move || location.pathname.get() == "/users"/>
                </NavSection>

                <NavSection title="系统">
                    <NavItem href="/settings" icon=Icon::Settings label="系统设置" is_active=move || location.pathname.get() == "/settings"/>
                    <NavItem href="/example" icon=Icon::Example label="交互示例" is_active=move || location.pathname.get() == "/example"/>
                </NavSection>
            </nav>
        </aside>
    }
}

#[component]
fn NavSection(
    title: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div>
            <div class="px-3 mb-2 sidebar-content">
                <span class="text-xs font-semibold text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel uppercase tracking-wider">
                    {title}
                </span>
            </div>
            <div class="space-y-1">
                {children()}
            </div>
        </div>
    }
}

#[component]
fn NavItem<F>(
    href: &'static str,
    icon: Icon,
    label: &'static str,
    is_active: F,
) -> impl IntoView
where
    F: Fn() -> bool + Clone + 'static,
{
    let is_active_clone = is_active.clone();

    view! {
        <A href=href class=move || {
            let base = "group flex items-center gap-3 px-3 py-2 rounded-apple-xl font-medium transition-all duration-200 nav-item";
            if is_active() {
                format!("{} bg-apple-blue text-white shadow-md shadow-apple-blue/20", base)
            } else {
                format!("{} text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel hover:bg-black/5 dark:hover:bg-white/10 hover:text-apple-label dark:hover:text-apple-darkLabel", base)
            }
        }>
            <div class=move || {
                if is_active_clone() {
                    "w-5 h-5 flex-shrink-0"
                } else {
                    "w-5 h-5 flex-shrink-0 group-hover:scale-110 transition-transform"
                }
            }>
                {icon.svg()}
            </div>
            <span class="text-sm nav-label whitespace-nowrap overflow-hidden transition-all duration-300">{label}</span>
        </A>
    }
}

#[component]
fn Header() -> impl IntoView {

    view! {
        <header class="sticky top-0 z-40 glass-card border-b border-apple-gray-200/50 dark:border-white/10 h-16 transition-all duration-300">
            <div class="flex items-center justify-between px-6 h-full">
                // 搜索框
                <div class="flex-1 max-w-md">
                    <Input
                        placeholder="搜索..."
                        icon=view! {
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                            </svg>
                        }.into_view()
                    />
                </div>

                // 右侧操作
                <div class="flex items-center gap-4">
                    // 主题切换
                    <button
                        onclick="toggleTheme()"
                        class="p-2 text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel hover:bg-black/5 dark:hover:bg-white/10 rounded-full transition-colors"
                        title="切换主题"
                    >
                        <svg class="w-5 h-5 hidden dark:block" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"/>
                        </svg>
                        <svg class="w-5 h-5 block dark:hidden" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"/>
                        </svg>
                    </button>

                    // 通知
                    <button class="relative p-2 text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel hover:bg-black/5 dark:hover:bg-white/10 rounded-full transition-colors">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"/>
                        </svg>
                        <span class="absolute top-1.5 right-1.5 w-2 h-2 bg-apple-red rounded-full border border-white dark:border-black"></span>
                    </button>

                    // 用户头像
                    <div class="pl-4 ml-2 border-l border-apple-gray-200/50 dark:border-white/10">
                        <UserProfile name="Admin" role="超级管理员" />
                    </div>
                </div>
            </div>
        </header>
    }
}

// 图标枚举
#[derive(Clone, Copy)]
pub enum Icon {
    Dashboard,
    Server,
    Monitor,
    Alert,
    Terminal,
    Command,
    Cronjob,
    Docker,
    Database,
    Files,
    Firewall,
    Audit,
    Users,
    Settings,
    Example,
}

impl Icon {
    fn svg(&self) -> impl IntoView {
        let path = match self {
            Icon::Dashboard => "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6",
            Icon::Server => "M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01",
            Icon::Monitor => "M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z",
            Icon::Alert => "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z",
            Icon::Terminal => "M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z",
            Icon::Command => "M8 16l2.879-2.879m0 0a3 3 0 104.243-4.242 3 3 0 00-4.243 4.242zM21 12a9 9 0 11-18 0 9 9 0 0118 0z",
            Icon::Cronjob => "M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z",
            Icon::Docker => "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10",
            Icon::Database => "M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4",
            Icon::Files => "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z",
            Icon::Firewall => "M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z",
            Icon::Audit => "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z",
            Icon::Users => "M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z",
            Icon::Settings => "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z M15 12a3 3 0 11-6 0 3 3 0 016 0z",
            Icon::Example => "M13 10V3L4 14h7v7l9-11h-7z",
        };

        view! {
            <svg class="w-full h-full" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d=path/>
            </svg>
        }
    }
}
