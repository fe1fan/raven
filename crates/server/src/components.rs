// Apple 风格的 UI 组件
use leptos::*;
use leptos_router::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gradient-to-br from-gray-50 via-blue-50 to-purple-50">
            <Sidebar/>
            <div class="lg:ml-72">
                <Header/>
                <main class="p-6">
                    {children()}
                </main>
            </div>
        </div>
    }
}

#[component]
fn Sidebar() -> impl IntoView {
    let location = use_location();

    view! {
        <aside class="fixed inset-y-0 left-0 z-50 w-72 bg-white/80 backdrop-blur-xl border-r border-gray-200 shadow-2xl">
            // Logo 区域
            <div class="p-6 border-b border-gray-200">
                <div class="flex items-center gap-3">
                    <div class="w-12 h-12 bg-gradient-to-br from-blue-500 via-purple-500 to-pink-500 rounded-2xl flex items-center justify-center shadow-lg">
                        <svg class="w-7 h-7 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M12 5l7 7-7 7"/>
                        </svg>
                    </div>
                    <div>
                        <h1 class="text-xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                            "Raven"
                        </h1>
                        <p class="text-xs text-gray-500">"服务器管理系统"</p>
                    </div>
                </div>
            </div>

            // 导航菜单
            <nav class="p-4 space-y-1 overflow-y-auto h-[calc(100vh-120px)]">
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
fn NavSection(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="mb-6">
            <div class="px-3 mb-2">
                <span class="text-xs font-bold text-gray-400 uppercase tracking-wider">
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
            let base = "group flex items-center gap-3 px-3 py-3 rounded-xl font-medium transition-all duration-200";
            if is_active() {
                format!("{} bg-gradient-to-r from-blue-500 to-purple-600 text-white shadow-lg shadow-blue-500/30", base)
            } else {
                format!("{} text-gray-600 hover:bg-gray-100 hover:text-gray-900", base)
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
            <span class="text-sm">{label}</span>
        </A>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="sticky top-0 z-40 bg-white/80 backdrop-blur-xl border-b border-gray-200 shadow-sm">
            <div class="flex items-center justify-between px-8 py-4">
                // 搜索框
                <div class="flex-1 max-w-xl">
                    <div class="relative">
                        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                            <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                            </svg>
                        </div>
                        <input
                            type="text"
                            placeholder="搜索服务器、任务..."
                            class="w-full pl-10 pr-4 py-2.5 bg-gray-100 border-0 rounded-2xl text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:bg-white transition-all"
                        />
                    </div>
                </div>

                // 右侧操作
                <div class="flex items-center gap-3 ml-6">
                    // 通知
                    <button class="relative p-2.5 text-gray-600 hover:bg-gray-100 rounded-xl transition-colors">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"/>
                        </svg>
                        <span class="absolute top-1.5 right-1.5 w-2 h-2 bg-red-500 rounded-full"></span>
                    </button>

                    // 用户头像
                    <div class="flex items-center gap-3 pl-3 ml-3 border-l border-gray-200">
                        <div class="text-right hidden sm:block">
                            <p class="text-sm font-semibold text-gray-900">"Admin"</p>
                            <p class="text-xs text-gray-500">"超级管理员"</p>
                        </div>
                        <div class="w-10 h-10 bg-gradient-to-br from-blue-500 to-purple-600 rounded-xl flex items-center justify-center text-white font-bold shadow-lg cursor-pointer hover:scale-105 transition-transform">
                            "A"
                        </div>
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
