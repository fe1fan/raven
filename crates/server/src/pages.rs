use crate::components::Layout;
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Layout>
            <div class="max-w-7xl mx-auto">
                <div class="mb-8">
                    <h1 class="text-3xl font-bold tracking-tight text-apple-label dark:text-apple-darkLabel mb-2">"仪表盘"</h1>
                    <p class="text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel">"概览您的系统运行状态"</p>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
                    <DashboardCard
                        value="128"
                        title="服务器总数"
                        badge="+12"
                        badge_type="primary"
                        icon_bg="bg-apple-blue"
                        icon_svg="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01"
                    />
                    <DashboardCard
                        value="119"
                        title="在线服务器"
                        badge="93%"
                        badge_type="success"
                        icon_bg="bg-apple-green"
                        icon_svg="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                    />
                    <DashboardCard
                        value="7"
                        title="活跃告警"
                        badge="-3"
                        badge_type="danger"
                        icon_bg="bg-apple-red"
                        icon_svg="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                    />
                    <DashboardCard
                        value="56"
                        title="容器运行中"
                        badge="+8"
                        badge_type="indigo"
                        icon_bg="bg-apple-indigo"
                        icon_svg="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
                    />
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
                    <div class="lg:col-span-2 glass-card p-6 rounded-apple-3xl">
                        <h3 class="text-lg font-bold mb-4 text-apple-label dark:text-apple-darkLabel">"资源趋势"</h3>
                        <div class="h-64 flex items-center justify-center border-2 border-dashed border-apple-gray-300 dark:border-white/10 rounded-apple-xl">
                            <span class="text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel">"Chart Placeholder"</span>
                        </div>
                    </div>
                    <div class="glass-card p-6 rounded-apple-3xl">
                        <h3 class="text-lg font-bold mb-4 text-apple-label dark:text-apple-darkLabel">"服务器状态"</h3>
                        <div class="h-64 flex items-center justify-center border-2 border-dashed border-apple-gray-300 dark:border-white/10 rounded-apple-xl">
                            <span class="text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel">"Pie Chart Placeholder"</span>
                        </div>
                    </div>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                    <RecentAlerts/>
                    <RecentActivities/>
                </div>
            </div>
        </Layout>
    }
}

#[component]
fn DashboardCard(
    value: &'static str,
    title: &'static str,
    badge: &'static str,
    badge_type: &'static str,
    icon_bg: &'static str,
    icon_svg: &'static str,
) -> impl IntoView {
    let badge_class = match badge_type {
        "success" => "px-2 py-0.5 bg-apple-green/10 text-apple-green dark:text-apple-darkGreen text-xs font-bold rounded-full",
        "danger" => "px-2 py-0.5 bg-apple-red/10 text-apple-red dark:text-apple-darkRed text-xs font-bold rounded-full",
        "primary" => "px-2 py-0.5 bg-apple-blue/10 text-apple-blue text-xs font-bold rounded-full",
        "indigo" => "px-2 py-0.5 bg-apple-indigo/10 text-apple-indigo text-xs font-bold rounded-full",
        _ => "px-2 py-0.5 bg-apple-gray-200 text-apple-gray-600 text-xs font-bold rounded-full",
    };

    view! {
        <div class="glass-card p-6 rounded-apple-3xl group transition-all duration-300 hover:scale-[1.02]">
            <div class="flex items-center justify-between mb-4">
                <div class=format!("w-12 h-12 {} rounded-apple-xl flex items-center justify-center text-white shadow-lg", icon_bg)>
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d=icon_svg/>
                    </svg>
                </div>
                <span class=badge_class>{badge}</span>
            </div>
            <p class="text-3xl font-bold tracking-tight text-apple-label dark:text-apple-darkLabel">{value}</p>
            <p class="text-sm text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel font-medium mt-1">{title}</p>
        </div>
    }
}

#[component]
fn RecentAlerts() -> impl IntoView {
    view! {
        <div class="glass-card p-6 rounded-apple-3xl">
            <h3 class="text-lg font-bold mb-4 text-apple-label dark:text-apple-darkLabel">"最近告警"</h3>
            <div class="space-y-4">
                <AlertItem
                    title="CPU 使用率过高"
                    server="web-prod-01"
                    time="10分钟前"
                    level="danger"
                />
                <AlertItem
                    title="磁盘空间不足"
                    server="db-master-01"
                    time="30分钟前"
                    level="warning"
                />
                <AlertItem
                    title="内存使用率警告"
                    server="cache-01"
                    time="1小时前"
                    level="warning"
                />
            </div>
        </div>
    }
}

#[component]
fn AlertItem(
    title: &'static str,
    server: &'static str,
    time: &'static str,
    level: &'static str,
) -> impl IntoView {
    let (icon_bg, icon_color) = match level {
        "danger" => ("bg-apple-red/10", "text-apple-red"),
        "warning" => ("bg-apple-yellow/10", "text-apple-yellow"),
        _ => ("bg-apple-gray-200", "text-apple-gray-600"),
    };

    view! {
        <div class="flex items-start gap-3 p-3 rounded-apple-2xl hover:bg-apple-gray-200/50 dark:hover:bg-white/5 transition-colors">
            <div class=format!("w-8 h-8 rounded-full flex items-center justify-center flex-shrink-0 {}", icon_bg)>
                <svg class=format!("w-4 h-4 {}", icon_color) fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
                </svg>
            </div>
            <div class="flex-1 min-w-0">
                <p class="text-sm font-medium text-apple-label dark:text-apple-darkLabel truncate">{title}</p>
                <div class="flex items-center gap-2 mt-0.5">
                    <span class="text-xs text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel">{server}</span>
                    <span class="text-xs text-apple-gray-400">.</span>
                    <span class="text-xs text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel">{time}</span>
                </div>
            </div>
        </div>
    }
}

#[component]
fn RecentActivities() -> impl IntoView {
    view! {
        <div class="glass-card p-6 rounded-apple-3xl">
            <h3 class="text-lg font-bold mb-4 text-apple-label dark:text-apple-darkLabel">"最近活动"</h3>
             <div class="space-y-4">
                <ActivityItem
                    user="Admin"
                    action="重启了服务器"
                    target="web-prod-02"
                    time="5分钟前"
                />
                <ActivityItem
                    user="DevOps"
                    action="部署了新版本"
                    target="api-service"
                    time="15分钟前"
                />
                <ActivityItem
                    user="System"
                    action="自动备份完成"
                    target="db-backup"
                    time="2小时前"
                />
            </div>
        </div>
    }
}

#[component]
fn ActivityItem(
    user: &'static str,
    action: &'static str,
    target: &'static str,
    time: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex items-start gap-3 p-3 rounded-apple-2xl hover:bg-apple-gray-200/50 dark:hover:bg-white/5 transition-colors">
            <div class="w-8 h-8 rounded-full bg-apple-blue/10 flex items-center justify-center flex-shrink-0 text-apple-blue font-bold text-xs">
                {move || user.chars().next().unwrap().to_string()}
            </div>
            <div class="flex-1 min-w-0">
                <p class="text-sm text-apple-label dark:text-apple-darkLabel">
                    <span class="font-medium">{user}</span>
                    " "
                    {action}
                    " "
                    <span class="font-medium text-apple-blue">{target}</span>
                </p>
                <p class="text-xs text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel mt-0.5">{time}</p>
            </div>
        </div>
    }
}

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="p-8">
                <h1 class="text-3xl font-bold mb-6 text-apple-label dark:text-apple-darkLabel">"仪表盘"</h1>
            </div>
        </Layout>
    }
}

#[component]
pub fn ServersPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="max-w-7xl mx-auto">
                <div class="flex items-center justify-between mb-8">
                    <div>
                        <h1 class="text-3xl font-bold tracking-tight text-apple-label dark:text-apple-darkLabel mb-2">"服务器管理"</h1>
                        <p class="text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel">"管理所有节点的运行状态"</p>
                    </div>
                    <button class="bg-apple-blue hover:bg-apple-blue/90 text-white px-5 py-2.5 rounded-apple-xl text-sm font-semibold shadow-lg shadow-apple-blue/30 transition-all hover:scale-105 active:scale-95">
                        "添加服务器"
                    </button>
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
                    <ServerStatCard value="119" label="在线" color="success"/>
                    <ServerStatCard value="5" label="告警中" color="warning"/>
                    <ServerStatCard value="4" label="离线" color="danger"/>
                    <ServerStatCard value="128" label="总计" color="primary"/>
                </div>

                <div class="glass-card rounded-apple-3xl overflow-hidden">
                    <div class="p-4 border-b border-apple-gray-200/50 dark:border-white/10 flex items-center gap-4">
                        <select class="bg-apple-gray-200/50 dark:bg-white/10 border-none rounded-apple-xl px-4 py-2 text-sm text-apple-label dark:text-apple-darkLabel focus:outline-none focus:ring-2 focus:ring-apple-blue/50">
                            <option>"所有环境"</option>
                            <option>"生产环境"</option>
                            <option>"测试环境"</option>
                        </select>
                        <select class="bg-apple-gray-200/50 dark:bg-white/10 border-none rounded-apple-xl px-4 py-2 text-sm text-apple-label dark:text-apple-darkLabel focus:outline-none focus:ring-2 focus:ring-apple-blue/50">
                            <option>"所有状态"</option>
                            <option>"在线"</option>
                            <option>"离线"</option>
                        </select>
                        <div class="flex-1"></div>
                        <input
                            type="text"
                            placeholder="搜索服务器..."
                            class="bg-apple-gray-200/50 dark:bg-white/10 border-none rounded-apple-xl px-4 py-2 text-sm text-apple-label dark:text-apple-darkLabel focus:outline-none focus:ring-2 focus:ring-apple-blue/50 w-64"
                        />
                    </div>
                    <div class="overflow-x-auto">
                        <table class="w-full text-sm text-left">
                            <thead class="bg-apple-gray-100/50 dark:bg-white/5 border-b border-apple-gray-200/50 dark:border-white/10 text-xs uppercase text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel font-semibold">
                                <tr>
                                    <th class="px-6 py-4">"服务器"</th>
                                    <th class="px-6 py-4">"IP地址"</th>
                                    <th class="px-6 py-4">"状态"</th>
                                    <th class="px-6 py-4">"CPU"</th>
                                    <th class="px-6 py-4">"内存"</th>
                                    <th class="px-6 py-4">"磁盘"</th>
                                    <th class="px-6 py-4">"环境"</th>
                                    <th class="px-6 py-4 text-right">"操作"</th>
                                </tr>
                            </thead>
                            <tbody class="divide-y divide-apple-gray-200/50 dark:divide-white/5">
                                <ServerTableRow
                                    name="web-prod-01"
                                    os="Ubuntu 22.04"
                                    ip="192.168.1.101"
                                    status="online"
                                    cpu="45"
                                    memory="68"
                                    disk="55"
                                    env="生产"
                                />
                                <ServerTableRow
                                    name="db-master-01"
                                    os="CentOS 7"
                                    ip="192.168.1.201"
                                    status="warning"
                                    cpu="78"
                                    memory="85"
                                    disk="92"
                                    env="生产"
                                />
                                <ServerTableRow
                                    name="cache-redis-01"
                                    os="Debian 11"
                                    ip="192.168.1.50"
                                    status="online"
                                    cpu="12"
                                    memory="45"
                                    disk="20"
                                    env="生产"
                                />
                                <ServerTableRow
                                    name="test-worker-01"
                                    os="Ubuntu 22.04"
                                    ip="192.168.2.10"
                                    status="offline"
                                    cpu="0"
                                    memory="0"
                                    disk="0"
                                    env="测试"
                                />
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </Layout>
    }
}

#[component]
fn ServerStatCard(value: &'static str, label: &'static str, color: &'static str) -> impl IntoView {
    let (text_color, bg_color) = match color {
        "success" => ("text-apple-green", "bg-apple-green/10"),
        "warning" => ("text-apple-yellow", "bg-apple-yellow/10"),
        "danger" => ("text-apple-red", "bg-apple-red/10"),
        _ => ("text-apple-blue", "bg-apple-blue/10"),
    };

    view! {
        <div class="glass-card p-4 rounded-apple-2xl flex items-center justify-between">
            <div>
                <p class="text-2xl font-bold text-apple-label dark:text-apple-darkLabel">{value}</p>
                <p class="text-xs font-medium text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel mt-0.5">{label}</p>
            </div>
            <div class=format!("w-10 h-10 rounded-apple-xl flex items-center justify-center {}", bg_color)>
                <div class=format!("w-3 h-3 rounded-full {}", text_color.replace("text-", "bg-"))></div>
            </div>
        </div>
    }
}

#[component]
fn ServerTableRow(
    name: &'static str,
    os: &'static str,
    ip: &'static str,
    status: &'static str,
    cpu: &'static str,
    memory: &'static str,
    disk: &'static str,
    env: &'static str,
) -> impl IntoView {
    let status_view = match status {
        "online" => view! {
            <span class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-apple-green/10 text-apple-green text-xs font-bold border border-apple-green/20">
                <span class="w-1.5 h-1.5 rounded-full bg-apple-green animate-pulse"></span>
                "运行中"
            </span>
        },
        "warning" => view! {
            <span class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-apple-yellow/10 text-apple-yellow text-xs font-bold border border-apple-yellow/20">
                <span class="w-1.5 h-1.5 rounded-full bg-apple-yellow"></span>
                "告警"
            </span>
        },
        _ => view! {
            <span class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-apple-red/10 text-apple-red text-xs font-bold border border-apple-red/20">
                <span class="w-1.5 h-1.5 rounded-full bg-apple-red"></span>
                "离线"
            </span>
        },
    };

    view! {
        <tr class="group hover:bg-apple-gray-200/50 dark:hover:bg-white/5 transition-colors">
            <td class="px-6 py-4">
                <div class="flex items-center gap-3">
                    <div class="w-8 h-8 rounded-lg bg-apple-gray-200 dark:bg-white/10 flex items-center justify-center text-apple-secondaryLabel">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01"/>
                        </svg>
                    </div>
                    <div>
                        <div class="font-semibold text-apple-label dark:text-apple-darkLabel">{name}</div>
                        <div class="text-xs text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel">{os}</div>
                    </div>
                </div>
            </td>
            <td class="px-6 py-4 text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel font-mono text-xs">{ip}</td>
            <td class="px-6 py-4">{status_view}</td>
            <td class="px-6 py-4">
                <div class="flex items-center gap-2">
                    <div class="flex-1 w-20 h-1.5 bg-apple-gray-200 dark:bg-white/10 rounded-full overflow-hidden">
                        <div class="h-full bg-apple-blue rounded-full" style=format!("width: {}%", cpu)></div>
                    </div>
                    <span class="text-xs font-medium text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel">{cpu}"%"</span>
                </div>
            </td>
            <td class="px-6 py-4">
                <div class="flex items-center gap-2">
                    <div class="flex-1 w-20 h-1.5 bg-apple-gray-200 dark:bg-white/10 rounded-full overflow-hidden">
                        <div class="h-full bg-apple-indigo rounded-full" style=format!("width: {}%", memory)></div>
                    </div>
                    <span class="text-xs font-medium text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel">{memory}"%"</span>
                </div>
            </td>
            <td class="px-6 py-4">
                <div class="flex items-center gap-2">
                    <div class="flex-1 w-20 h-1.5 bg-apple-gray-200 dark:bg-white/10 rounded-full overflow-hidden">
                        <div class="h-full bg-apple-green rounded-full" style=format!("width: {}%", disk)></div>
                    </div>
                    <span class="text-xs font-medium text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel">{disk}"%"</span>
                </div>
            </td>
            <td class="px-6 py-4">
                <span class="px-2 py-1 rounded-md bg-apple-gray-200/50 dark:bg-white/10 text-xs font-medium text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel">
                    {env}
                </span>
            </td>
            <td class="px-6 py-4 text-right">
                <button class="p-1 text-apple-secondaryLabel hover:text-apple-blue transition-colors">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z"/>
                    </svg>
                </button>
            </td>
        </tr>
    }
}

// Placeholder pages
#[component]
pub fn MonitoringPage() -> impl IntoView {
    view! { <Layout><div class="p-8"><h1 class="text-3xl font-bold mb-6 text-apple-label dark:text-apple-darkLabel">"监控中心"</h1></div></Layout> }
}

#[component]
pub fn AlertsPage() -> impl IntoView {
    view! { <Layout><div class="p-8"><h1 class="text-3xl font-bold mb-6 text-apple-label dark:text-apple-darkLabel">"告警中心"</h1></div></Layout> }
}

#[component]
pub fn TerminalPage() -> impl IntoView {
    view! { <Layout><div class="p-8"><h1 class="text-3xl font-bold mb-6 text-apple-label dark:text-apple-darkLabel">"Web终端"</h1></div></Layout> }
}

#[component]
pub fn CommandsPage() -> impl IntoView {
    view! { <Layout><div class="p-8"><h1 class="text-3xl font-bold mb-6 text-apple-label dark:text-apple-darkLabel">"命令执行"</h1></div></Layout> }
}

#[component]
pub fn CronjobsPage() -> impl IntoView {
    view! { <Layout><div class="p-8"><h1 class="text-3xl font-bold mb-6 text-apple-label dark:text-apple-darkLabel">"定时任务"</h1></div></Layout> }
}

#[component]
pub fn DockerPage() -> impl IntoView {
    view! { <Layout><div class="p-8"><h1 class="text-3xl font-bold mb-6 text-apple-label dark:text-apple-darkLabel">"容器管理"</h1></div></Layout> }
}

#[component]
pub fn DatabasePage() -> impl IntoView {
    view! { <Layout><div class="p-8"><h1 class="text-3xl font-bold mb-6 text-apple-label dark:text-apple-darkLabel">"数据库"</h1></div></Layout> }
}

#[component]
pub fn FilesPage() -> impl IntoView {
    view! { <Layout><div class="p-8"><h1 class="text-3xl font-bold mb-6 text-apple-label dark:text-apple-darkLabel">"文件管理"</h1></div></Layout> }
}

#[component]
pub fn FirewallPage() -> impl IntoView {
    view! { <Layout><div class="p-8"><h1 class="text-3xl font-bold mb-6 text-apple-label dark:text-apple-darkLabel">"防火墙"</h1></div></Layout> }
}

#[component]
pub fn AuditPage() -> impl IntoView {
    view! { <Layout><div class="p-8"><h1 class="text-3xl font-bold mb-6 text-apple-label dark:text-apple-darkLabel">"操作审计"</h1></div></Layout> }
}

#[component]
pub fn UsersPage() -> impl IntoView {
    view! { <Layout><div class="p-8"><h1 class="text-3xl font-bold mb-6 text-apple-label dark:text-apple-darkLabel">"用户权限"</h1></div></Layout> }
}

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! { <Layout><div class="p-8"><h1 class="text-3xl font-bold mb-6 text-apple-label dark:text-apple-darkLabel">"系统设置"</h1></div></Layout> }
}
