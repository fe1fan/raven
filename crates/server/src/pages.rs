use leptos::*;
use crate::components::Layout;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Layout>
            <div class="p-8">
                <div class="grid grid-cols-4 gap-6 mb-8">
                    <DashboardCard
                        value="128"
                        title="服务器总数"
                        badge="+12"
                        badge_type="success"
                        icon_bg="from-blue-500 to-blue-600"
                        icon_shadow="shadow-blue-500/30"
                    />
                    <DashboardCard
                        value="119"
                        title="在线服务器"
                        badge="93%"
                        badge_type="success"
                        icon_bg="from-emerald-500 to-emerald-600"
                        icon_shadow="shadow-emerald-500/30"
                    />
                    <DashboardCard
                        value="7"
                        title="活跃告警"
                        badge="-3"
                        badge_type="danger"
                        icon_bg="from-amber-500 to-orange-500"
                        icon_shadow="shadow-amber-500/30"
                    />
                    <DashboardCard
                        value="56"
                        title="容器运行中"
                        badge="+8"
                        badge_type="success"
                        icon_bg="from-violet-500 to-purple-600"
                        icon_shadow="shadow-violet-500/30"
                    />
                </div>

                <div class="grid grid-cols-3 gap-6 mb-8">
                    <ResourceTrendChart/>
                    <ServerStatusChart/>
                </div>

                <div class="grid grid-cols-2 gap-6">
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
    icon_shadow: &'static str,
) -> impl IntoView {
    let badge_class = match badge_type {
        "success" => "px-2.5 py-1 bg-success/10 text-success text-xs font-medium rounded-full",
        "danger" => "px-2.5 py-1 bg-danger/10 text-danger text-xs font-medium rounded-full",
        _ => "px-2.5 py-1 bg-default-100 text-default-600 text-xs font-medium rounded-full",
    };

    view! {
        <div class="bg-white rounded-2xl p-6 shadow-sm hover:shadow-md transition-shadow duration-300">
            <div class="flex items-center gap-4">
                <div class=format!("w-12 h-12 rounded-2xl bg-gradient-to-br {} flex items-center justify-center shadow-lg {}", icon_bg, icon_shadow)>
                    <svg class="w-6 h-6 text-white" fill="currentColor" viewBox="0 0 24 24">
                        <path d="M20 13H4c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1h16c.55 0 1-.45 1-1v-6c0-.55-.45-1-1-1z"/>
                    </svg>
                </div>
                <div class="flex-1">
                    <p class="text-3xl font-bold text-default-900">{value}</p>
                    <p class="text-sm text-default-500">{title}</p>
                </div>
                <span class=badge_class>{badge}</span>
            </div>
        </div>
    }
}

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="p-8">
                <h1 class="text-3xl font-bold mb-6">"仪表盘"</h1>
            </div>
        </Layout>
    }
}

#[component]
pub fn ServersPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="p-8">
                <div class="flex items-center justify-between mb-6">
                    <div class="flex items-center gap-3">
                        <select class="bg-white border border-default-200 rounded-xl px-4 py-2.5 text-sm text-default-700 focus:outline-none focus:ring-2 focus:ring-primary/20">
                            <option>"所有环境"</option>
                            <option>"生产环境"</option>
                        </select>
                        <select class="bg-white border border-default-200 rounded-xl px-4 py-2.5 text-sm text-default-700 focus:outline-none focus:ring-2 focus:ring-primary/20">
                            <option>"所有状态"</option>
                            <option>"在线"</option>
                        </select>
                    </div>
                    <button class="flex items-center gap-2 bg-primary hover:bg-primary/90 text-white px-5 py-2.5 rounded-xl text-sm font-semibold">
                        "添加服务器"
                    </button>
                </div>

                <div class="grid grid-cols-4 gap-4 mb-6">
                    <ServerStatCard value="119" label="在线" color="success"/>
                    <ServerStatCard value="5" label="告警中" color="warning"/>
                    <ServerStatCard value="4" label="离线" color="danger"/>
                    <ServerStatCard value="128" label="总计" color="primary"/>
                </div>

                <div class="bg-white rounded-2xl shadow-sm overflow-hidden">
                    <table class="w-full">
                        <thead class="bg-gradient-to-r from-default-50/80 to-default-50/50 border-b border-default-100/50">
                            <tr>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-600 uppercase">"服务器"</th>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-600 uppercase">"IP地址"</th>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-600 uppercase">"状态"</th>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-600 uppercase">"CPU"</th>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-600 uppercase">"内存"</th>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-600 uppercase">"磁盘"</th>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-600 uppercase">"环境"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-default-100/50">
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
                        </tbody>
                    </table>
                </div>
            </div>
        </Layout>
    }
}

#[component]
fn ServerStatCard(value: &'static str, label: &'static str, color: &'static str) -> impl IntoView {
    let (bg_color, icon_path) = match color {
        "success" => ("from-success to-emerald-600", "M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"),
        "warning" => ("from-warning to-orange-500", "M12 22c1.1 0 2-.9 2-2h-4c0 1.1.9 2 2 2zm6-6v-5c0-3.07-1.63-5.64-4.5-6.32V4c0-.83-.67-1.5-1.5-1.5s-1.5.67-1.5 1.5v.68C7.64 5.36 6 7.92 6 11v5l-2 2v1h16v-1l-2-2z"),
        "danger" => ("from-danger to-pink-600", "M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"),
        _ => ("from-primary to-blue-600", "M20 13H4c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1h16c.55 0 1-.45 1-1v-6c0-.55-.45-1-1-1z"),
    };

    view! {
        <div class="bg-white rounded-xl p-4 shadow-sm border border-default-100/50">
            <div class="flex items-center gap-3">
                <div class=format!("w-10 h-10 rounded-lg bg-gradient-to-br {} flex items-center justify-center", bg_color)>
                    <svg class="w-5 h-5 text-white" fill="currentColor" viewBox="0 0 24 24">
                        <path d=icon_path/>
                    </svg>
                </div>
                <div>
                    <p class="text-2xl font-bold text-default-900">{value}</p>
                    <p class="text-xs text-default-500">{label}</p>
                </div>
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
    let (status_class, status_text) = match status {
        "online" => ("bg-success/10 text-success border-success/20", "在线"),
        "warning" => ("bg-warning/10 text-warning border-warning/20", "告警"),
        _ => ("bg-danger/10 text-danger border-danger/20", "离线"),
    };

    view! {
        <tr class="hover:bg-default-50/50 transition-all">
            <td class="px-6 py-4">
                <div class="flex items-center gap-3">
                    <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-blue-500 to-blue-600 flex items-center justify-center">
                        <svg class="w-4 h-4 text-white" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M20 13H4c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1h16c.55 0 1-.45 1-1v-6c0-.55-.45-1-1-1z"/>
                        </svg>
                    </div>
                    <div>
                        <span class="font-semibold text-default-900">{name}</span>
                        <p class="text-xs text-default-400">{os}</p>
                    </div>
                </div>
            </td>
            <td class="px-6 py-4">
                <span class="text-default-700 font-mono text-sm">{ip}</span>
            </td>
            <td class="px-6 py-4">
                <span class=format!("inline-flex items-center gap-1.5 px-2.5 py-1 text-xs font-semibold rounded-lg border {}", status_class)>
                    {status_text}
                </span>
            </td>
            <td class="px-6 py-4">
                <ProgressBar value=cpu/>
            </td>
            <td class="px-6 py-4">
                <ProgressBar value=memory/>
            </td>
            <td class="px-6 py-4">
                <ProgressBar value=disk/>
            </td>
            <td class="px-6 py-4">
                <span class="px-2.5 py-1 bg-primary/10 text-primary text-xs font-semibold rounded-lg border border-primary/20">{env}</span>
            </td>
        </tr>
    }
}

#[component]
fn ProgressBar(value: &'static str) -> impl IntoView {
    let val: i32 = value.parse().unwrap_or(0);
    let color = if val < 60 { "from-success to-emerald-500" } else if val < 80 { "from-warning to-orange-500" } else { "from-danger to-pink-600" };

    view! {
        <div class="flex items-center gap-2.5 min-w-[100px]">
            <div class="flex-1 h-2 bg-default-200 rounded-full overflow-hidden">
                <div class=format!("h-full bg-gradient-to-r {} rounded-full", color) style=format!("width:{}%", value)></div>
            </div>
            <span class="text-xs font-medium text-default-700 w-10 text-right">{value}"%"</span>
        </div>
    }
}

#[component]
fn ResourceTrendChart() -> impl IntoView {
    view! {
        <div class="col-span-2 bg-white rounded-2xl shadow-sm overflow-hidden">
            <div class="flex items-center justify-between px-6 py-4 border-b border-default-100">
                <h3 class="font-semibold text-default-900">"资源使用趋势"</h3>
                <select class="text-sm bg-default-100 border-0 rounded-lg px-3 py-1.5 focus:outline-none focus:ring-2 focus:ring-primary/20">
                    <option>"最近24小时"</option>
                    <option>"最近7天"</option>
                    <option>"最近30天"</option>
                </select>
            </div>
            <div class="p-6">
                <div class="h-48 bg-gradient-to-t from-primary/5 to-transparent rounded-xl flex items-end justify-between px-4 pb-4">
                    <div class="flex-1 flex items-end gap-2 h-full">
                        <ChartBar height="45%"/>
                        <ChartBar height="62%"/>
                        <ChartBar height="38%"/>
                        <ChartBar height="75%"/>
                        <ChartBar height="55%"/>
                        <ChartBar height="68%"/>
                        <ChartBar height="42%"/>
                        <ChartBar height="58%"/>
                        <ChartBar height="72%"/>
                        <ChartBar height="48%"/>
                        <ChartBar height="65%"/>
                        <ChartBar height="52%"/>
                    </div>
                </div>
                <div class="flex justify-between mt-3 text-xs text-default-400">
                    <span>"00:00"</span>
                    <span>"04:00"</span>
                    <span>"08:00"</span>
                    <span>"12:00"</span>
                    <span>"16:00"</span>
                    <span>"20:00"</span>
                    <span>"24:00"</span>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ChartBar(height: &'static str) -> impl IntoView {
    view! {
        <div class="flex-1 bg-gradient-to-t from-primary/60 to-primary/20 rounded-t-lg" style=format!("height:{}", height)></div>
    }
}

#[component]
fn ServerStatusChart() -> impl IntoView {
    view! {
        <div class="bg-white rounded-2xl shadow-sm overflow-hidden">
            <div class="px-6 py-4 border-b border-default-100">
                <h3 class="font-semibold text-default-900">"服务器状态分布"</h3>
            </div>
            <div class="p-6 flex flex-col items-center">
                <div class="relative w-36 h-36">
                    <svg class="w-full h-full -rotate-90" viewBox="0 0 36 36">
                        <circle cx="18" cy="18" r="15.9" fill="none" stroke="#e4e4e7" stroke-width="3"/>
                        <circle cx="18" cy="18" r="15.9" fill="none" stroke="#17c964" stroke-width="3" stroke-dasharray="87, 100" stroke-linecap="round"/>
                        <circle cx="18" cy="18" r="15.9" fill="none" stroke="#f5a524" stroke-width="3" stroke-dasharray="5, 100" stroke-dashoffset="-87" stroke-linecap="round"/>
                        <circle cx="18" cy="18" r="15.9" fill="none" stroke="#f31260" stroke-width="3" stroke-dasharray="4, 100" stroke-dashoffset="-92" stroke-linecap="round"/>
                    </svg>
                    <div class="absolute inset-0 flex flex-col items-center justify-center">
                        <span class="text-2xl font-bold text-default-900">"93%"</span>
                        <span class="text-xs text-default-500">"健康率"</span>
                    </div>
                </div>
                <div class="flex gap-6 mt-6">
                    <div class="flex items-center gap-2">
                        <div class="w-3 h-3 rounded-full bg-success"></div>
                        <span class="text-sm text-default-600">"在线 119"</span>
                    </div>
                    <div class="flex items-center gap-2">
                        <div class="w-3 h-3 rounded-full bg-warning"></div>
                        <span class="text-sm text-default-600">"告警 5"</span>
                    </div>
                    <div class="flex items-center gap-2">
                        <div class="w-3 h-3 rounded-full bg-danger"></div>
                        <span class="text-sm text-default-600">"离线 4"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn RecentAlerts() -> impl IntoView {
    view! {
        <div class="bg-white rounded-2xl shadow-sm overflow-hidden">
            <div class="flex items-center justify-between px-6 py-4 border-b border-default-100">
                <h3 class="font-semibold text-default-900">"最近告警"</h3>
                <a href="#" class="text-sm text-primary hover:underline">"查看全部"</a>
            </div>
            <div class="divide-y divide-default-100">
                <AlertItem priority="P1" severity="danger" message="CPU使用率超过90%" server="web-prod-01" time="2分钟前"/>
                <AlertItem priority="P2" severity="warning" message="磁盘空间不足20%" server="db-master-01" time="15分钟前"/>
                <AlertItem priority="P3" severity="default" message="内存使用率超过80%" server="api-server-03" time="1小时前"/>
            </div>
        </div>
    }
}

#[component]
fn AlertItem(
    priority: &'static str,
    severity: &'static str,
    message: &'static str,
    server: &'static str,
    time: &'static str,
) -> impl IntoView {
    let badge_class = match severity {
        "danger" => "px-2 py-1 bg-danger/10 text-danger text-xs font-semibold rounded-lg",
        "warning" => "px-2 py-1 bg-warning/10 text-warning text-xs font-semibold rounded-lg",
        _ => "px-2 py-1 bg-default-100 text-default-600 text-xs font-semibold rounded-lg",
    };

    view! {
        <div class="px-6 py-4 flex items-center gap-4 hover:bg-default-50 transition-colors">
            <span class=badge_class>{priority}</span>
            <div class="flex-1 min-w-0">
                <p class="text-sm font-medium text-default-900 truncate">{message}</p>
                <p class="text-xs text-default-500">{server}</p>
            </div>
            <span class="text-xs text-default-400">{time}</span>
        </div>
    }
}

#[component]
fn RecentActivities() -> impl IntoView {
    view! {
        <div class="bg-white rounded-2xl shadow-sm overflow-hidden">
            <div class="flex items-center justify-between px-6 py-4 border-b border-default-100">
                <h3 class="font-semibold text-default-900">"最近操作"</h3>
                <a href="#" class="text-sm text-primary hover:underline">"查看全部"</a>
            </div>
            <div class="divide-y divide-default-100">
                <ActivityItem 
                    icon_bg="bg-primary/10" 
                    icon_color="text-primary" 
                    user="Admin" 
                    user_color="text-primary"
                    action="执行了命令" 
                    detail="web-prod-01 · systemctl restart nginx" 
                    time="5分钟前"
                />
                <ActivityItem 
                    icon_bg="bg-success/10" 
                    icon_color="text-success" 
                    user="DevOps" 
                    user_color="text-success"
                    action="部署了应用" 
                    detail="api-server · v2.3.1" 
                    time="30分钟前"
                />
                <ActivityItem 
                    icon_bg="bg-violet-500/10" 
                    icon_color="text-violet-500" 
                    user="Admin" 
                    user_color="text-violet-500"
                    action="添加了用户" 
                    detail="developer@company.com" 
                    time="2小时前"
                />
            </div>
        </div>
    }
}

#[component]
fn ActivityItem(
    icon_bg: &'static str,
    icon_color: &'static str,
    user: &'static str,
    user_color: &'static str,
    action: &'static str,
    detail: &'static str,
    time: &'static str,
) -> impl IntoView {
    view! {
        <div class="px-6 py-4 flex items-center gap-4 hover:bg-default-50 transition-colors">
            <div class=format!("w-9 h-9 rounded-xl {} flex items-center justify-center", icon_bg)>
                <svg class=format!("w-4 h-4 {}", icon_color) fill="currentColor" viewBox="0 0 24 24">
                    <path d="M9.4 16.6L4.8 12l4.6-4.6L8 6l-6 6 6 6 1.4-1.4z"/>
                </svg>
            </div>
            <div class="flex-1 min-w-0">
                <p class="text-sm font-medium text-default-900">
                    <span class=user_color>{user}</span>
                    {" "}
                    {action}
                </p>
                <p class="text-xs text-default-500 truncate">{detail}</p>
            </div>
            <span class="text-xs text-default-400">{time}</span>
        </div>
    }
}

#[component]
pub fn MonitoringPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="p-8">
                <div class="grid grid-cols-4 gap-6 mb-8">
                    <MonitoringCard
                        title="CPU 使用率"
                        server="web-prod-01"
                        value="45.2%"
                        color="primary"
                        bars=vec![45, 52, 38, 65, 42, 45]
                    />
                    <MonitoringCard
                        title="内存使用率"
                        server="web-prod-01"
                        value="68.5%"
                        color="warning"
                        bars=vec![62, 65, 68, 66, 70, 68]
                    />
                    <MonitoringCard
                        title="磁盘使用率"
                        server="web-prod-01"
                        value="55.0%"
                        color="success"
                        bars=vec![54, 54, 55, 55, 55, 55]
                    />
                    <MonitoringCard
                        title="网络流量"
                        server="web-prod-01"
                        value="124 MB/s"
                        color="violet"
                        bars=vec![35, 48, 72, 58, 85, 62]
                    />
                </div>
                <div class="bg-white rounded-2xl shadow-hero overflow-hidden">
                    <div class="flex items-center justify-between px-6 py-4 border-b border-default-100">
                        <h3 class="font-semibold text-default-900">"进程监控"</h3>
                        <select class="text-sm bg-default-100 border-0 rounded-lg px-3 py-1.5 focus:outline-none">
                            <option>"web-prod-01"</option>
                            <option>"web-prod-02"</option>
                            <option>"db-master-01"</option>
                        </select>
                    </div>
                    <table class="w-full">
                        <thead class="bg-default-50">
                            <tr>
                                <th class="text-left px-6 py-3 text-xs font-semibold text-default-500">"进程名"</th>
                                <th class="text-left px-6 py-3 text-xs font-semibold text-default-500">"PID"</th>
                                <th class="text-left px-6 py-3 text-xs font-semibold text-default-500">"CPU"</th>
                                <th class="text-left px-6 py-3 text-xs font-semibold text-default-500">"内存"</th>
                                <th class="text-left px-6 py-3 text-xs font-semibold text-default-500">"状态"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-default-100">
                            <tr class="hover:bg-default-50">
                                <td class="px-6 py-3 font-medium">"nginx"</td>
                                <td class="px-6 py-3 text-default-500">"1234"</td>
                                <td class="px-6 py-3">"2.3%"</td>
                                <td class="px-6 py-3">"128MB"</td>
                                <td class="px-6 py-3"><span class="text-success">"Running"</span></td>
                            </tr>
                            <tr class="hover:bg-default-50">
                                <td class="px-6 py-3 font-medium">"node"</td>
                                <td class="px-6 py-3 text-default-500">"5678"</td>
                                <td class="px-6 py-3">"15.6%"</td>
                                <td class="px-6 py-3">"512MB"</td>
                                <td class="px-6 py-3"><span class="text-success">"Running"</span></td>
                            </tr>
                            <tr class="hover:bg-default-50">
                                <td class="px-6 py-3 font-medium">"redis-server"</td>
                                <td class="px-6 py-3 text-default-500">"9012"</td>
                                <td class="px-6 py-3">"0.5%"</td>
                                <td class="px-6 py-3">"64MB"</td>
                                <td class="px-6 py-3"><span class="text-success">"Running"</span></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </Layout>
    }
}

#[component]
pub fn AlertsPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="p-8">
                <div class="flex items-center justify-between mb-6">
                    <div class="flex items-center gap-3">
                        <select class="bg-default-100 border-0 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-primary/20">
                            <option>"所有级别"</option>
                            <option>"P1 紧急"</option>
                            <option>"P2 严重"</option>
                            <option>"P3 警告"</option>
                        </select>
                        <select class="bg-default-100 border-0 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-primary/20">
                            <option>"所有状态"</option>
                            <option>"活跃"</option>
                            <option>"已确认"</option>
                            <option>"已解决"</option>
                        </select>
                    </div>
                    <button class="flex items-center gap-2 bg-primary hover:bg-primary/90 text-white px-5 py-2.5 rounded-xl text-sm font-medium transition-colors shadow-lg shadow-primary/30">
                        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
                        </svg>
                        "新建规则"
                    </button>
                </div>
                <div class="bg-white rounded-2xl shadow-hero overflow-hidden">
                    <div class="divide-y divide-default-100">
                        <div class="px-6 py-5 flex items-center gap-4 hover:bg-default-50 transition-colors">
                            <span class="px-3 py-1.5 bg-danger/10 text-danger text-xs font-bold rounded-lg">"P1"</span>
                            <div class="flex-1">
                                <p class="font-medium text-default-900">"CPU使用率超过90%持续5分钟"</p>
                                <p class="text-sm text-default-500 mt-1">"服务器: web-prod-01 · 触发条件: cpu_usage > 90%"</p>
                            </div>
                            <span class="inline-flex items-center gap-1.5 px-3 py-1.5 bg-warning/10 text-warning text-xs font-medium rounded-full">
                                <span class="w-1.5 h-1.5 rounded-full bg-warning animate-pulse"></span>
                                "活跃"
                            </span>
                            <span class="text-sm text-default-400">"2分钟前"</span>
                            <button class="px-4 py-2 bg-default-100 hover:bg-default-200 rounded-xl text-sm font-medium transition-colors">"确认"</button>
                        </div>
                        <div class="px-6 py-5 flex items-center gap-4 hover:bg-default-50 transition-colors">
                            <span class="px-3 py-1.5 bg-warning/10 text-warning text-xs font-bold rounded-lg">"P2"</span>
                            <div class="flex-1">
                                <p class="font-medium text-default-900">"磁盘空间不足20%"</p>
                                <p class="text-sm text-default-500 mt-1">"服务器: db-master-01 · 当前使用: 92%"</p>
                            </div>
                            <span class="inline-flex items-center gap-1.5 px-3 py-1.5 bg-warning/10 text-warning text-xs font-medium rounded-full">
                                <span class="w-1.5 h-1.5 rounded-full bg-warning animate-pulse"></span>
                                "活跃"
                            </span>
                            <span class="text-sm text-default-400">"15分钟前"</span>
                            <button class="px-4 py-2 bg-default-100 hover:bg-default-200 rounded-xl text-sm font-medium transition-colors">"确认"</button>
                        </div>
                        <div class="px-6 py-5 flex items-center gap-4 hover:bg-default-50 transition-colors">
                            <span class="px-3 py-1.5 bg-default-100 text-default-600 text-xs font-bold rounded-lg">"P3"</span>
                            <div class="flex-1">
                                <p class="font-medium text-default-900">"内存使用率超过80%"</p>
                                <p class="text-sm text-default-500 mt-1">"服务器: api-server-03 · 当前使用: 82%"</p>
                            </div>
                            <span class="inline-flex items-center gap-1.5 px-3 py-1.5 bg-success/10 text-success text-xs font-medium rounded-full">"已确认"</span>
                            <span class="text-sm text-default-400">"1小时前"</span>
                            <button class="px-4 py-2 bg-success hover:bg-success/90 text-white rounded-xl text-sm font-medium transition-colors">"解决"</button>
                        </div>
                    </div>
                </div>
            </div>
        </Layout>
    }
}

#[component]
pub fn TerminalPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="p-8">
                <div class="flex items-center justify-between mb-6">
                    <div class="flex items-center gap-3">
                        <select class="bg-default-100 border-0 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-primary/20">
                            <option>"web-prod-01"</option>
                            <option>"web-prod-02"</option>
                            <option>"db-master-01"</option>
                        </select>
                    </div>
                    <div class="flex items-center gap-2">
                        <button class="px-4 py-2 bg-default-100 hover:bg-default-200 rounded-xl text-sm font-medium transition-colors">"新建标签页"</button>
                        <button class="px-4 py-2 bg-default-100 hover:bg-default-200 rounded-xl text-sm font-medium transition-colors">"同步输入"</button>
                    </div>
                </div>
                <div class="bg-[#1a1a1a] rounded-2xl shadow-hero-lg overflow-hidden">
                    <div class="flex items-center justify-between px-4 py-3 bg-[#2a2a2a] border-b border-white/10">
                        <div class="flex items-center gap-2">
                            <div class="flex items-center gap-2 px-4 py-2 bg-primary/20 rounded-lg">
                                <span class="text-sm text-white">"web-prod-01"</span>
                                <button class="text-white/50 hover:text-white">"×"</button>
                            </div>
                            <div class="flex items-center gap-2 px-4 py-2 hover:bg-white/5 rounded-lg cursor-pointer">
                                <span class="text-sm text-white/60">"web-prod-02"</span>
                                <button class="text-white/30 hover:text-white">"×"</button>
                            </div>
                            <button class="px-3 py-2 text-white/40 hover:text-white hover:bg-white/5 rounded-lg">"+ 新建"</button>
                        </div>
                        <div class="flex items-center gap-2">
                            <button class="p-2 text-white/50 hover:text-white hover:bg-white/10 rounded-lg" title="上传">
                                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M9 16h6v-6h4l-7-7-7 7h4zm-4 2h14v2H5z"/>
                                </svg>
                            </button>
                            <button class="p-2 text-white/50 hover:text-white hover:bg-white/10 rounded-lg" title="下载">
                                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z"/>
                                </svg>
                            </button>
                        </div>
                    </div>
                    <div class="p-6 font-mono text-sm h-96 overflow-y-auto">
                        <div class="mb-1">
                            <span class="text-emerald-400">"root@web-prod-01"</span>
                            <span class="text-white">":"</span>
                            <span class="text-blue-400">"~"</span>
                            <span class="text-white">"# "</span>
                            <span class="text-white">"ls -la"</span>
                        </div>
                        <div class="text-gray-400 mb-1">"total 48"</div>
                        <div class="text-gray-400 mb-1">"drwx------  6 root root 4096 Jan 15 10:00 ."</div>
                        <div class="text-gray-400 mb-1">"drwxr-xr-x 23 root root 4096 Jan 10 08:00 .."</div>
                        <div class="text-gray-400 mb-1">"-rw-------  1 root root 1234 Jan 15 09:00 .bash_history"</div>
                        <div class="text-gray-400 mb-1">"-rw-r--r--  1 root root  570 Jan  6 00:00 .bashrc"</div>
                        <div class="text-gray-400 mb-1">"drwx------  2 root root 4096 Jan 10 08:00 .ssh"</div>
                        <div class="mt-4">
                            <span class="text-emerald-400">"root@web-prod-01"</span>
                            <span class="text-white">":"</span>
                            <span class="text-blue-400">"~"</span>
                            <span class="text-white">"# "</span>
                            <span class="text-white">"df -h"</span>
                        </div>
                        <div class="text-gray-400 mb-1">"Filesystem      Size  Used Avail Use% Mounted on"</div>
                        <div class="text-gray-400 mb-1">"/dev/sda1       100G   55G   45G  55% /"</div>
                        <div class="text-gray-400 mb-1">"/dev/sdb1       500G  234G  266G  47% /data"</div>
                        <div class="mt-4">
                            <span class="text-emerald-400">"root@web-prod-01"</span>
                            <span class="text-white">":"</span>
                            <span class="text-blue-400">"~"</span>
                            <span class="text-white">"# "</span>
                            <span class="inline-block w-2 h-4 bg-white animate-pulse"></span>
                        </div>
                    </div>
                </div>
            </div>
        </Layout>
    }
}

#[component]
pub fn CommandsPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="p-8">
                <div class="flex items-center justify-between mb-6">
                    <div class="flex items-center gap-3">
                        <select class="bg-default-100 border-0 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-primary/20">
                            <option>"选择服务器"</option>
                            <option>"web-prod-01"</option>
                            <option>"web-prod-02"</option>
                            <option>"db-master-01"</option>
                            <option>"批量选择..."</option>
                        </select>
                    </div>
                    <button class="px-4 py-2 bg-default-100 hover:bg-default-200 rounded-xl text-sm font-medium transition-colors">"保存为脚本"</button>
                </div>
                <div class="bg-white rounded-2xl shadow-hero overflow-hidden mb-6">
                    <div class="px-6 py-4 border-b border-default-100">
                        <h3 class="font-semibold text-default-900">"命令执行"</h3>
                    </div>
                    <div class="p-6">
                        <div class="mb-4">
                            <label class="block text-sm font-medium text-default-700 mb-2">"输入命令"</label>
                            <input
                                type="text"
                                value="systemctl status nginx"
                                class="w-full px-4 py-3 bg-default-50 border border-default-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-primary/20 focus:border-primary font-mono"
                            />
                        </div>
                        <button class="flex items-center gap-2 bg-primary hover:bg-primary/90 text-white px-6 py-3 rounded-xl text-sm font-medium transition-colors shadow-lg shadow-primary/30">
                            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M8 5v14l11-7z"/>
                            </svg>
                            "执行命令"
                        </button>
                    </div>
                </div>
                <div class="bg-[#1a1a1a] rounded-2xl shadow-hero-lg overflow-hidden">
                    <div class="px-4 py-3 bg-[#2a2a2a] border-b border-white/10">
                        <span class="text-sm text-white/80">"执行结果 - web-prod-01"</span>
                    </div>
                    <div class="p-6 font-mono text-sm">
                        <div class="text-gray-400 mb-1">"● nginx.service - A high performance web server"</div>
                        <div class="text-gray-400 mb-1">"   Loaded: loaded (/lib/systemd/system/nginx.service; enabled)"</div>
                        <div class="text-emerald-400 mb-1">"   Active: active (running) since Mon 2024-01-15 08:00:00 CST"</div>
                        <div class="text-gray-400 mb-1">"  Process: 1234 ExecStart=/usr/sbin/nginx (code=exited, status=0/SUCCESS)"</div>
                        <div class="text-gray-400 mb-1">" Main PID: 1235 (nginx)"</div>
                        <div class="text-gray-400">"    Tasks: 5 (limit: 4915)"</div>
                    </div>
                </div>
            </div>
        </Layout>
    }
}

#[component]
pub fn CronjobsPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="p-8">
                <div class="flex items-center justify-between mb-6">
                    <div class="flex items-center gap-3">
                        <select class="bg-default-100 border-0 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-primary/20">
                            <option>"所有状态"</option>
                            <option>"启用"</option>
                            <option>"禁用"</option>
                        </select>
                    </div>
                    <button class="flex items-center gap-2 bg-primary hover:bg-primary/90 text-white px-5 py-2.5 rounded-xl text-sm font-medium transition-colors shadow-lg shadow-primary/30">
                        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
                        </svg>
                        "新建任务"
                    </button>
                </div>
                <div class="bg-white rounded-2xl shadow-hero overflow-hidden">
                    <table class="w-full">
                        <thead class="bg-default-50 border-b border-default-100">
                            <tr>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-500 uppercase">"任务名称"</th>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-500 uppercase">"Cron表达式"</th>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-500 uppercase">"目标服务器"</th>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-500 uppercase">"状态"</th>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-500 uppercase">"下次执行"</th>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-500 uppercase">"操作"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-default-100">
                            <tr class="hover:bg-default-50 transition-colors">
                                <td class="px-6 py-4 font-medium text-default-900">"日志清理"</td>
                                <td class="px-6 py-4">
                                    <code class="px-2 py-1 bg-default-100 rounded text-xs font-mono">"0 3 * * *"</code>
                                </td>
                                <td class="px-6 py-4 text-default-600">"全部服务器"</td>
                                <td class="px-6 py-4">
                                    <span class="inline-flex items-center gap-1.5 text-success text-sm">
                                        <span class="w-2 h-2 rounded-full bg-success"></span>
                                        "启用"
                                    </span>
                                </td>
                                <td class="px-6 py-4 text-default-600">"明天 03:00"</td>
                                <td class="px-6 py-4">
                                    <div class="flex items-center gap-1">
                                        <button class="p-2 hover:bg-default-100 rounded-lg">
                                            <svg class="w-4 h-4 text-default-500" fill="currentColor" viewBox="0 0 24 24">
                                                <path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25z"/>
                                            </svg>
                                        </button>
                                        <button class="p-2 hover:bg-default-100 rounded-lg">
                                            <svg class="w-4 h-4 text-default-500" fill="currentColor" viewBox="0 0 24 24">
                                                <path d="M8 5v14l11-7z"/>
                                            </svg>
                                        </button>
                                    </div>
                                </td>
                            </tr>
                            <tr class="hover:bg-default-50 transition-colors">
                                <td class="px-6 py-4 font-medium text-default-900">"数据库备份"</td>
                                <td class="px-6 py-4">
                                    <code class="px-2 py-1 bg-default-100 rounded text-xs font-mono">"0 2 * * *"</code>
                                </td>
                                <td class="px-6 py-4 text-default-600">"db-master-01"</td>
                                <td class="px-6 py-4">
                                    <span class="inline-flex items-center gap-1.5 text-success text-sm">
                                        <span class="w-2 h-2 rounded-full bg-success"></span>
                                        "启用"
                                    </span>
                                </td>
                                <td class="px-6 py-4 text-default-600">"明天 02:00"</td>
                                <td class="px-6 py-4">
                                    <div class="flex items-center gap-1">
                                        <button class="p-2 hover:bg-default-100 rounded-lg">
                                            <svg class="w-4 h-4 text-default-500" fill="currentColor" viewBox="0 0 24 24">
                                                <path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25z"/>
                                            </svg>
                                        </button>
                                        <button class="p-2 hover:bg-default-100 rounded-lg">
                                            <svg class="w-4 h-4 text-default-500" fill="currentColor" viewBox="0 0 24 24">
                                                <path d="M8 5v14l11-7z"/>
                                            </svg>
                                        </button>
                                    </div>
                                </td>
                            </tr>
                            <tr class="hover:bg-default-50 transition-colors">
                                <td class="px-6 py-4 font-medium text-default-900">"健康检查"</td>
                                <td class="px-6 py-4">
                                    <code class="px-2 py-1 bg-default-100 rounded text-xs font-mono">"*/5 * * * *"</code>
                                </td>
                                <td class="px-6 py-4 text-default-600">"全部服务器"</td>
                                <td class="px-6 py-4">
                                    <span class="inline-flex items-center gap-1.5 text-success text-sm">
                                        <span class="w-2 h-2 rounded-full bg-success"></span>
                                        "启用"
                                    </span>
                                </td>
                                <td class="px-6 py-4 text-default-600">"5分钟后"</td>
                                <td class="px-6 py-4">
                                    <div class="flex items-center gap-1">
                                        <button class="p-2 hover:bg-default-100 rounded-lg">
                                            <svg class="w-4 h-4 text-default-500" fill="currentColor" viewBox="0 0 24 24">
                                                <path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25z"/>
                                            </svg>
                                        </button>
                                        <button class="p-2 hover:bg-default-100 rounded-lg">
                                            <svg class="w-4 h-4 text-default-500" fill="currentColor" viewBox="0 0 24 24">
                                                <path d="M8 5v14l11-7z"/>
                                            </svg>
                                        </button>
                                    </div>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </Layout>
    }
}

#[component]
pub fn DockerPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="p-8">
                <div class="flex items-center justify-between mb-6">
                    <div class="flex items-center gap-3">
                        <select class="bg-default-100 border-0 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-primary/20">
                            <option>"所有容器"</option>
                            <option>"运行中"</option>
                            <option>"已停止"</option>
                        </select>
                        <select class="bg-default-100 border-0 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-primary/20">
                            <option>"所有服务器"</option>
                            <option>"web-prod-01"</option>
                            <option>"web-prod-02"</option>
                        </select>
                    </div>
                    <button class="flex items-center gap-2 bg-primary hover:bg-primary/90 text-white px-5 py-2.5 rounded-xl text-sm font-medium transition-colors shadow-lg shadow-primary/30">
                        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
                        </svg>
                        "创建容器"
                    </button>
                </div>
                <div class="grid grid-cols-3 gap-6">
                    <DockerContainer
                        name="nginx-proxy"
                        image="nginx:1.24"
                        cpu="2.3%"
                        memory="128MB"
                        ports="80, 443"
                        uptime="7天"
                        icon_bg="from-blue-500 to-blue-600"
                        icon_shadow="shadow-blue-500/30"
                    />
                    <DockerContainer
                        name="api-service"
                        image="node:18-alpine"
                        cpu="15.6%"
                        memory="512MB"
                        ports="3000"
                        uptime="3天"
                        icon_bg="from-emerald-500 to-emerald-600"
                        icon_shadow="shadow-emerald-500/30"
                    />
                    <DockerContainer
                        name="redis-cache"
                        image="redis:7-alpine"
                        cpu="0.5%"
                        memory="64MB"
                        ports="6379"
                        uptime="14天"
                        icon_bg="from-red-500 to-red-600"
                        icon_shadow="shadow-red-500/30"
                    />
                </div>
            </div>
        </Layout>
    }
}

#[component]
pub fn DatabasePage() -> impl IntoView {
    view! {
        <Layout>
            <div class="p-8">
                <div class="grid grid-cols-3 gap-6 mb-8">
                    <div class="bg-white rounded-2xl p-6 shadow-hero">
                        <div class="flex items-center gap-4">
                            <div class="w-12 h-12 rounded-2xl bg-gradient-to-br from-blue-500 to-blue-600 flex items-center justify-center shadow-lg shadow-blue-500/30">
                                <svg class="w-6 h-6 text-white" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M12 3C7.58 3 4 4.79 4 7v10c0 2.21 3.59 4 8 4s8-1.79 8-4V7c0-2.21-3.58-4-8-4z"/>
                                </svg>
                            </div>
                            <div>
                                <p class="text-3xl font-bold text-default-900">"8"</p>
                                <p class="text-sm text-default-500">"数据库连接"</p>
                            </div>
                        </div>
                    </div>
                    <div class="bg-white rounded-2xl p-6 shadow-hero">
                        <div class="flex items-center gap-4">
                            <div class="w-12 h-12 rounded-2xl bg-gradient-to-br from-emerald-500 to-emerald-600 flex items-center justify-center shadow-lg shadow-emerald-500/30">
                                <svg class="w-6 h-6 text-white" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
                                </svg>
                            </div>
                            <div>
                                <p class="text-3xl font-bold text-default-900">"6"</p>
                                <p class="text-sm text-default-500">"正常连接"</p>
                            </div>
                        </div>
                    </div>
                    <div class="bg-white rounded-2xl p-6 shadow-hero">
                        <div class="flex items-center gap-4">
                            <div class="w-12 h-12 rounded-2xl bg-gradient-to-br from-amber-500 to-orange-500 flex items-center justify-center shadow-lg shadow-amber-500/30">
                                <svg class="w-6 h-6 text-white" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z"/>
                                </svg>
                            </div>
                            <div>
                                <p class="text-3xl font-bold text-default-900">"2"</p>
                                <p class="text-sm text-default-500">"需要关注"</p>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="bg-white rounded-2xl shadow-hero overflow-hidden">
                    <div class="px-6 py-4 border-b border-default-100 flex items-center justify-between">
                        <h3 class="font-semibold text-default-900">"数据库列表"</h3>
                        <button class="flex items-center gap-2 bg-primary hover:bg-primary/90 text-white px-4 py-2 rounded-xl text-sm font-medium transition-colors">"添加连接"</button>
                    </div>
                    <div class="divide-y divide-default-100">
                        <div class="px-6 py-5 flex items-center gap-4 hover:bg-default-50 transition-colors">
                            <div class="w-10 h-10 rounded-xl bg-blue-500/10 flex items-center justify-center">
                                <span class="text-blue-500 font-bold text-sm">"M"</span>
                            </div>
                            <div class="flex-1">
                                <p class="font-medium text-default-900">"prod-mysql"</p>
                                <p class="text-sm text-default-500">"192.168.1.201:3306 · MySQL 8.0"</p>
                            </div>
                            <span class="inline-flex items-center gap-1.5 text-success text-sm">
                                <span class="w-2 h-2 rounded-full bg-success"></span>
                                "正常"
                            </span>
                            <div class="text-right">
                                <p class="text-sm font-medium">"45/100"</p>
                                <p class="text-xs text-default-400">"连接数"</p>
                            </div>
                            <div class="text-right">
                                <p class="text-sm font-medium">"1,234"</p>
                                <p class="text-xs text-default-400">"QPS"</p>
                            </div>
                            <button class="px-4 py-2 bg-default-100 hover:bg-default-200 rounded-xl text-sm font-medium transition-colors">"SQL客户端"</button>
                        </div>
                        <div class="px-6 py-5 flex items-center gap-4 hover:bg-default-50 transition-colors">
                            <div class="w-10 h-10 rounded-xl bg-violet-500/10 flex items-center justify-center">
                                <span class="text-violet-500 font-bold text-sm">"P"</span>
                            </div>
                            <div class="flex-1">
                                <p class="font-medium text-default-900">"prod-postgres"</p>
                                <p class="text-sm text-default-500">"192.168.1.202:5432 · PostgreSQL 15"</p>
                            </div>
                            <span class="inline-flex items-center gap-1.5 text-success text-sm">
                                <span class="w-2 h-2 rounded-full bg-success"></span>
                                "正常"
                            </span>
                            <div class="text-right">
                                <p class="text-sm font-medium">"23/50"</p>
                                <p class="text-xs text-default-400">"连接数"</p>
                            </div>
                            <div class="text-right">
                                <p class="text-sm font-medium">"856"</p>
                                <p class="text-xs text-default-400">"QPS"</p>
                            </div>
                            <button class="px-4 py-2 bg-default-100 hover:bg-default-200 rounded-xl text-sm font-medium transition-colors">"SQL客户端"</button>
                        </div>
                        <div class="px-6 py-5 flex items-center gap-4 hover:bg-default-50 transition-colors">
                            <div class="w-10 h-10 rounded-xl bg-red-500/10 flex items-center justify-center">
                                <span class="text-red-500 font-bold text-sm">"R"</span>
                            </div>
                            <div class="flex-1">
                                <p class="font-medium text-default-900">"cache-redis"</p>
                                <p class="text-sm text-default-500">"192.168.1.203:6379 · Redis 7.2"</p>
                            </div>
                            <span class="inline-flex items-center gap-1.5 text-warning text-sm">
                                <span class="w-2 h-2 rounded-full bg-warning"></span>
                                "高负载"
                            </span>
                            <div class="text-right">
                                <p class="text-sm font-medium">"1,024"</p>
                                <p class="text-xs text-default-400">"连接数"</p>
                            </div>
                            <div class="text-right">
                                <p class="text-sm font-medium">"5,678"</p>
                                <p class="text-xs text-default-400">"QPS"</p>
                            </div>
                            <button class="px-4 py-2 bg-default-100 hover:bg-default-200 rounded-xl text-sm font-medium transition-colors">"Key管理"</button>
                        </div>
                    </div>
                </div>
            </div>
        </Layout>
    }
}

#[component]
pub fn FilesPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="p-8">
                <div class="flex items-center justify-between mb-6">
                    <div class="flex items-center gap-3">
                        <select class="bg-default-100 border-0 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-primary/20">
                            <option>"web-prod-01"</option>
                            <option>"web-prod-02"</option>
                            <option>"db-master-01"</option>
                        </select>
                        <div class="flex items-center gap-2 px-4 py-2 bg-default-100 rounded-xl text-sm text-default-600">
                            <span>"/"</span>
                            <span class="text-default-400">"/"</span>
                            <span>"var"</span>
                            <span class="text-default-400">"/"</span>
                            <span>"www"</span>
                        </div>
                    </div>
                    <div class="flex items-center gap-2">
                        <button class="flex items-center gap-2 px-4 py-2 bg-default-100 hover:bg-default-200 rounded-xl text-sm font-medium transition-colors">
                            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M9 16h6v-6h4l-7-7-7 7h4zm-4 2h14v2H5z"/>
                            </svg>
                            "上传"
                        </button>
                        <button class="flex items-center gap-2 px-4 py-2 bg-default-100 hover:bg-default-200 rounded-xl text-sm font-medium transition-colors">
                            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
                            </svg>
                            "新建文件夹"
                        </button>
                    </div>
                </div>
                <div class="bg-white rounded-2xl shadow-hero overflow-hidden">
                    <div class="grid grid-cols-4 border-b border-default-100">
                        <div class="col-span-1 border-r border-default-100 p-4">
                            <div class="space-y-1">
                                <div class="flex items-center gap-2 px-3 py-2 bg-primary/10 text-primary rounded-lg cursor-pointer">
                                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M10 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z"/>
                                    </svg>
                                    <span class="text-sm font-medium">"/"</span>
                                </div>
                                <div class="flex items-center gap-2 px-3 py-2 hover:bg-default-100 rounded-lg cursor-pointer ml-4">
                                    <svg class="w-4 h-4 text-default-400" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M10 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z"/>
                                    </svg>
                                    <span class="text-sm text-default-600">"etc"</span>
                                </div>
                                <div class="flex items-center gap-2 px-3 py-2 hover:bg-default-100 rounded-lg cursor-pointer ml-4">
                                    <svg class="w-4 h-4 text-default-400" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M10 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z"/>
                                    </svg>
                                    <span class="text-sm text-default-600">"var"</span>
                                </div>
                                <div class="flex items-center gap-2 px-3 py-2 hover:bg-default-100 rounded-lg cursor-pointer ml-4">
                                    <svg class="w-4 h-4 text-default-400" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M10 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z"/>
                                    </svg>
                                    <span class="text-sm text-default-600">"home"</span>
                                </div>
                            </div>
                        </div>
                        <div class="col-span-3 p-4">
                            <div class="grid grid-cols-6 gap-4">
                                <div class="flex flex-col items-center gap-2 p-4 hover:bg-default-50 rounded-xl cursor-pointer">
                                    <svg class="w-12 h-12 text-warning" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M10 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z"/>
                                    </svg>
                                    <span class="text-xs text-default-600 text-center">"nginx"</span>
                                </div>
                                <div class="flex flex-col items-center gap-2 p-4 hover:bg-default-50 rounded-xl cursor-pointer">
                                    <svg class="w-12 h-12 text-warning" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M10 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z"/>
                                    </svg>
                                    <span class="text-xs text-default-600 text-center">"ssl"</span>
                                </div>
                                <div class="flex flex-col items-center gap-2 p-4 hover:bg-default-50 rounded-xl cursor-pointer">
                                    <svg class="w-12 h-12 text-primary" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"/>
                                    </svg>
                                    <span class="text-xs text-default-600 text-center">"nginx.conf"</span>
                                </div>
                                <div class="flex flex-col items-center gap-2 p-4 hover:bg-default-50 rounded-xl cursor-pointer">
                                    <svg class="w-12 h-12 text-primary" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"/>
                                    </svg>
                                    <span class="text-xs text-default-600 text-center">"hosts"</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </Layout>
    }
}

#[component]
pub fn FirewallPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="p-8">
                <div class="grid grid-cols-3 gap-6 mb-8">
                    <div class="bg-white rounded-2xl p-6 shadow-hero">
                        <div class="flex items-center gap-4">
                            <div class="w-12 h-12 rounded-2xl bg-gradient-to-br from-emerald-500 to-emerald-600 flex items-center justify-center shadow-lg shadow-emerald-500/30">
                                <svg class="w-6 h-6 text-white" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
                                </svg>
                            </div>
                            <div>
                                <p class="text-3xl font-bold text-default-900">"24"</p>
                                <p class="text-sm text-default-500">"允许规则"</p>
                            </div>
                        </div>
                    </div>
                    <div class="bg-white rounded-2xl p-6 shadow-hero">
                        <div class="flex items-center gap-4">
                            <div class="w-12 h-12 rounded-2xl bg-gradient-to-br from-red-500 to-red-600 flex items-center justify-center shadow-lg shadow-red-500/30">
                                <svg class="w-6 h-6 text-white" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
                                </svg>
                            </div>
                            <div>
                                <p class="text-3xl font-bold text-default-900">"156"</p>
                                <p class="text-sm text-default-500">"拒绝规则"</p>
                            </div>
                        </div>
                    </div>
                    <div class="bg-white rounded-2xl p-6 shadow-hero">
                        <div class="flex items-center gap-4">
                            <div class="w-12 h-12 rounded-2xl bg-gradient-to-br from-amber-500 to-orange-500 flex items-center justify-center shadow-lg shadow-amber-500/30">
                                <svg class="w-6 h-6 text-white" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4z"/>
                                </svg>
                            </div>
                            <div>
                                <p class="text-3xl font-bold text-default-900">"1,234"</p>
                                <p class="text-sm text-default-500">"今日拦截"</p>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="bg-white rounded-2xl shadow-hero overflow-hidden">
                    <div class="px-6 py-4 border-b border-default-100 flex items-center justify-between">
                        <h3 class="font-semibold text-default-900">"防火墙规则 - web-prod-01"</h3>
                        <button class="flex items-center gap-2 bg-primary hover:bg-primary/90 text-white px-4 py-2 rounded-xl text-sm font-medium transition-colors">"添加规则"</button>
                    </div>
                    <table class="w-full">
                        <thead class="bg-default-50">
                            <tr>
                                <th class="text-left px-6 py-3 text-xs font-semibold text-default-500">"动作"</th>
                                <th class="text-left px-6 py-3 text-xs font-semibold text-default-500">"协议"</th>
                                <th class="text-left px-6 py-3 text-xs font-semibold text-default-500">"端口"</th>
                                <th class="text-left px-6 py-3 text-xs font-semibold text-default-500">"来源"</th>
                                <th class="text-left px-6 py-3 text-xs font-semibold text-default-500">"描述"</th>
                                <th class="text-left px-6 py-3 text-xs font-semibold text-default-500">"操作"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-default-100">
                            <tr class="hover:bg-default-50">
                                <td class="px-6 py-4">
                                    <span class="px-2 py-1 bg-success/10 text-success text-xs font-semibold rounded-lg">"允许"</span>
                                </td>
                                <td class="px-6 py-4">"TCP"</td>
                                <td class="px-6 py-4">"80, 443"</td>
                                <td class="px-6 py-4 text-default-600">"0.0.0.0/0"</td>
                                <td class="px-6 py-4 text-default-600">"HTTP/HTTPS"</td>
                                <td class="px-6 py-4">
                                    <button class="p-2 hover:bg-default-100 rounded-lg">
                                        <svg class="w-4 h-4 text-default-500" fill="currentColor" viewBox="0 0 24 24">
                                            <path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25z"/>
                                        </svg>
                                    </button>
                                </td>
                            </tr>
                            <tr class="hover:bg-default-50">
                                <td class="px-6 py-4">
                                    <span class="px-2 py-1 bg-success/10 text-success text-xs font-semibold rounded-lg">"允许"</span>
                                </td>
                                <td class="px-6 py-4">"TCP"</td>
                                <td class="px-6 py-4">"22"</td>
                                <td class="px-6 py-4 text-default-600">"10.0.0.0/8"</td>
                                <td class="px-6 py-4 text-default-600">"SSH内网"</td>
                                <td class="px-6 py-4">
                                    <button class="p-2 hover:bg-default-100 rounded-lg">
                                        <svg class="w-4 h-4 text-default-500" fill="currentColor" viewBox="0 0 24 24">
                                            <path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25z"/>
                                        </svg>
                                    </button>
                                </td>
                            </tr>
                            <tr class="hover:bg-default-50">
                                <td class="px-6 py-4">
                                    <span class="px-2 py-1 bg-danger/10 text-danger text-xs font-semibold rounded-lg">"拒绝"</span>
                                </td>
                                <td class="px-6 py-4">"ALL"</td>
                                <td class="px-6 py-4">"*"</td>
                                <td class="px-6 py-4 text-default-600">"192.168.100.0/24"</td>
                                <td class="px-6 py-4 text-default-600">"黑名单网段"</td>
                                <td class="px-6 py-4">
                                    <button class="p-2 hover:bg-default-100 rounded-lg">
                                        <svg class="w-4 h-4 text-default-500" fill="currentColor" viewBox="0 0 24 24">
                                            <path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25z"/>
                                        </svg>
                                    </button>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </Layout>
    }
}

#[component]
pub fn AuditPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="p-8">
                <div class="flex items-center justify-between mb-6">
                    <div class="flex items-center gap-3">
                        <select class="bg-default-100 border-0 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-primary/20">
                            <option>"所有用户"</option>
                            <option>"Admin"</option>
                            <option>"DevOps"</option>
                        </select>
                        <select class="bg-default-100 border-0 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-primary/20">
                            <option>"所有操作"</option>
                            <option>"登录"</option>
                            <option>"命令执行"</option>
                            <option>"文件操作"</option>
                        </select>
                        <input
                            type="text"
                            placeholder="搜索..."
                            class="px-4 py-2.5 bg-default-100 border-0 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-primary/20"
                        />
                    </div>
                    <button class="px-4 py-2 bg-default-100 hover:bg-default-200 rounded-xl text-sm font-medium transition-colors">"导出日志"</button>
                </div>
                <div class="bg-white rounded-2xl shadow-hero overflow-hidden">
                    <table class="w-full">
                        <thead class="bg-default-50 border-b border-default-100">
                            <tr>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-500 uppercase">"时间"</th>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-500 uppercase">"用户"</th>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-500 uppercase">"操作类型"</th>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-500 uppercase">"目标"</th>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-500 uppercase">"详情"</th>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-500 uppercase">"IP地址"</th>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-500 uppercase">"状态"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-default-100">
                            <tr class="hover:bg-default-50">
                                <td class="px-6 py-4 text-default-600">"10:32:15"</td>
                                <td class="px-6 py-4 font-medium">"Admin"</td>
                                <td class="px-6 py-4">
                                    <span class="px-2 py-1 bg-primary/10 text-primary text-xs font-medium rounded-lg">"命令执行"</span>
                                </td>
                                <td class="px-6 py-4">"web-prod-01"</td>
                                <td class="px-6 py-4 text-default-600 font-mono text-xs">"systemctl restart nginx"</td>
                                <td class="px-6 py-4 text-default-500">"10.0.0.100"</td>
                                <td class="px-6 py-4">
                                    <span class="text-success">"成功"</span>
                                </td>
                            </tr>
                            <tr class="hover:bg-default-50">
                                <td class="px-6 py-4 text-default-600">"10:28:42"</td>
                                <td class="px-6 py-4 font-medium">"DevOps"</td>
                                <td class="px-6 py-4">
                                    <span class="px-2 py-1 bg-success/10 text-success text-xs font-medium rounded-lg">"部署"</span>
                                </td>
                                <td class="px-6 py-4">"api-service"</td>
                                <td class="px-6 py-4 text-default-600">"部署 v2.3.1"</td>
                                <td class="px-6 py-4 text-default-500">"10.0.0.101"</td>
                                <td class="px-6 py-4">
                                    <span class="text-success">"成功"</span>
                                </td>
                            </tr>
                            <tr class="hover:bg-default-50">
                                <td class="px-6 py-4 text-default-600">"09:30:00"</td>
                                <td class="px-6 py-4 font-medium text-danger">"Unknown"</td>
                                <td class="px-6 py-4">
                                    <span class="px-2 py-1 bg-danger/10 text-danger text-xs font-medium rounded-lg">"登录"</span>
                                </td>
                                <td class="px-6 py-4">"系统"</td>
                                <td class="px-6 py-4 text-default-600">"SSH登录尝试"</td>
                                <td class="px-6 py-4 text-default-500">"185.123.45.67"</td>
                                <td class="px-6 py-4">
                                    <span class="text-danger">"失败"</span>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </Layout>
    }
}

#[component]
pub fn UsersPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="p-8">
                <div class="flex items-center justify-between mb-6">
                    <div class="flex items-center gap-3">
                        <select class="bg-default-100 border-0 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-primary/20">
                            <option>"所有角色"</option>
                            <option>"超级管理员"</option>
                            <option>"运维人员"</option>
                            <option>"开发人员"</option>
                        </select>
                    </div>
                    <button class="flex items-center gap-2 bg-primary hover:bg-primary/90 text-white px-5 py-2.5 rounded-xl text-sm font-medium transition-colors shadow-lg shadow-primary/30">
                        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
                        </svg>
                        "添加用户"
                    </button>
                </div>
                <div class="bg-white rounded-2xl shadow-hero overflow-hidden">
                    <table class="w-full">
                        <thead class="bg-default-50 border-b border-default-100">
                            <tr>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-500 uppercase">"用户"</th>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-500 uppercase">"邮箱"</th>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-500 uppercase">"角色"</th>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-500 uppercase">"状态"</th>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-500 uppercase">"最后登录"</th>
                                <th class="text-left px-6 py-4 text-xs font-semibold text-default-500 uppercase">"操作"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-default-100">
                            <tr class="hover:bg-default-50">
                                <td class="px-6 py-4">
                                    <div class="flex items-center gap-3">
                                        <div class="w-9 h-9 rounded-full bg-gradient-to-br from-primary to-purple-600 flex items-center justify-center text-white text-sm font-medium">"A"</div>
                                        <span class="font-medium text-default-900">"admin"</span>
                                    </div>
                                </td>
                                <td class="px-6 py-4 text-default-600">"admin@company.com"</td>
                                <td class="px-6 py-4">
                                    <span class="px-2.5 py-1 bg-violet-500/10 text-violet-500 text-xs font-medium rounded-lg">"超级管理员"</span>
                                </td>
                                <td class="px-6 py-4">
                                    <span class="inline-flex items-center gap-1.5 text-success text-sm">
                                        <span class="w-2 h-2 rounded-full bg-success"></span>
                                        "活跃"
                                    </span>
                                </td>
                                <td class="px-6 py-4 text-default-500">"10分钟前"</td>
                                <td class="px-6 py-4">
                                    <button class="px-4 py-2 bg-default-100 hover:bg-default-200 rounded-xl text-sm font-medium transition-colors">"编辑"</button>
                                </td>
                            </tr>
                            <tr class="hover:bg-default-50">
                                <td class="px-6 py-4">
                                    <div class="flex items-center gap-3">
                                        <div class="w-9 h-9 rounded-full bg-gradient-to-br from-emerald-500 to-emerald-600 flex items-center justify-center text-white text-sm font-medium">"D"</div>
                                        <span class="font-medium text-default-900">"devops"</span>
                                    </div>
                                </td>
                                <td class="px-6 py-4 text-default-600">"devops@company.com"</td>
                                <td class="px-6 py-4">
                                    <span class="px-2.5 py-1 bg-primary/10 text-primary text-xs font-medium rounded-lg">"运维人员"</span>
                                </td>
                                <td class="px-6 py-4">
                                    <span class="inline-flex items-center gap-1.5 text-success text-sm">
                                        <span class="w-2 h-2 rounded-full bg-success"></span>
                                        "活跃"
                                    </span>
                                </td>
                                <td class="px-6 py-4 text-default-500">"30分钟前"</td>
                                <td class="px-6 py-4">
                                    <button class="px-4 py-2 bg-default-100 hover:bg-default-200 rounded-xl text-sm font-medium transition-colors">"编辑"</button>
                                </td>
                            </tr>
                            <tr class="hover:bg-default-50">
                                <td class="px-6 py-4">
                                    <div class="flex items-center gap-3">
                                        <div class="w-9 h-9 rounded-full bg-gradient-to-br from-amber-500 to-orange-500 flex items-center justify-center text-white text-sm font-medium">"J"</div>
                                        <span class="font-medium text-default-900">"john.dev"</span>
                                    </div>
                                </td>
                                <td class="px-6 py-4 text-default-600">"john@company.com"</td>
                                <td class="px-6 py-4">
                                    <span class="px-2.5 py-1 bg-success/10 text-success text-xs font-medium rounded-lg">"开发人员"</span>
                                </td>
                                <td class="px-6 py-4">
                                    <span class="inline-flex items-center gap-1.5 text-success text-sm">
                                        <span class="w-2 h-2 rounded-full bg-success"></span>
                                        "活跃"
                                    </span>
                                </td>
                                <td class="px-6 py-4 text-default-500">"2小时前"</td>
                                <td class="px-6 py-4">
                                    <button class="px-4 py-2 bg-default-100 hover:bg-default-200 rounded-xl text-sm font-medium transition-colors">"编辑"</button>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </Layout>
    }
}

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="p-8">
                <div class="grid grid-cols-3 gap-6">
                    <div class="col-span-2 space-y-6">
                        <div class="bg-white rounded-2xl shadow-hero overflow-hidden">
                            <div class="px-6 py-4 border-b border-default-100">
                                <h3 class="font-semibold text-default-900">"基本设置"</h3>
                            </div>
                            <div class="p-6 space-y-6">
                                <div>
                                    <label class="block text-sm font-medium text-default-700 mb-2">"系统名称"</label>
                                    <input
                                        type="text"
                                        value="ServerGuard"
                                        class="w-full px-4 py-3 bg-default-50 border border-default-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-primary/20 focus:border-primary"
                                    />
                                </div>
                                <div>
                                    <label class="block text-sm font-medium text-default-700 mb-2">"时区设置"</label>
                                    <select class="w-full px-4 py-3 bg-default-50 border border-default-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-primary/20">
                                        <option>"Asia/Shanghai (UTC+8)"</option>
                                        <option>"UTC"</option>
                                    </select>
                                </div>
                                <div>
                                    <label class="block text-sm font-medium text-default-700 mb-2">"语言"</label>
                                    <select class="w-full px-4 py-3 bg-default-50 border border-default-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-primary/20">
                                        <option>"简体中文"</option>
                                        <option>"English"</option>
                                    </select>
                                </div>
                                <button class="bg-primary hover:bg-primary/90 text-white px-6 py-3 rounded-xl text-sm font-medium transition-colors shadow-lg shadow-primary/30">"保存设置"</button>
                            </div>
                        </div>
                        <div class="bg-white rounded-2xl shadow-hero overflow-hidden">
                            <div class="px-6 py-4 border-b border-default-100">
                                <h3 class="font-semibold text-default-900">"邮件配置"</h3>
                            </div>
                            <div class="p-6 space-y-6">
                                <div>
                                    <label class="block text-sm font-medium text-default-700 mb-2">"SMTP 服务器"</label>
                                    <input
                                        type="text"
                                        placeholder="smtp.example.com"
                                        class="w-full px-4 py-3 bg-default-50 border border-default-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-primary/20"
                                    />
                                </div>
                                <div class="grid grid-cols-2 gap-4">
                                    <div>
                                        <label class="block text-sm font-medium text-default-700 mb-2">"端口"</label>
                                        <input
                                            type="text"
                                            placeholder="587"
                                            class="w-full px-4 py-3 bg-default-50 border border-default-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-primary/20"
                                        />
                                    </div>
                                    <div>
                                        <label class="block text-sm font-medium text-default-700 mb-2">"加密方式"</label>
                                        <select class="w-full px-4 py-3 bg-default-50 border border-default-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-primary/20">
                                            <option>"TLS"</option>
                                            <option>"SSL"</option>
                                            <option>"无"</option>
                                        </select>
                                    </div>
                                </div>
                                <button class="bg-default-100 hover:bg-default-200 px-6 py-3 rounded-xl text-sm font-medium transition-colors">"测试连接"</button>
                            </div>
                        </div>
                    </div>
                    <div class="space-y-6">
                        <div class="bg-white rounded-2xl shadow-hero overflow-hidden">
                            <div class="px-6 py-4 border-b border-default-100">
                                <h3 class="font-semibold text-default-900">"Agent 配置"</h3>
                            </div>
                            <div class="p-6 space-y-4">
                                <p class="text-sm text-default-600">"下载 Agent 安装包以监控您的服务器"</p>
                                <div class="space-y-2">
                                    <button class="w-full px-4 py-3 bg-default-100 hover:bg-default-200 rounded-xl text-sm font-medium transition-colors text-left">"Linux x64"</button>
                                    <button class="w-full px-4 py-3 bg-default-100 hover:bg-default-200 rounded-xl text-sm font-medium transition-colors text-left">"Linux ARM64"</button>
                                    <button class="w-full px-4 py-3 bg-default-100 hover:bg-default-200 rounded-xl text-sm font-medium transition-colors text-left">"Windows x64"</button>
                                </div>
                            </div>
                        </div>
                        <div class="bg-white rounded-2xl shadow-hero overflow-hidden">
                            <div class="px-6 py-4 border-b border-default-100">
                                <h3 class="font-semibold text-default-900">"注册 Token"</h3>
                            </div>
                            <div class="p-6">
                                <p class="text-xs text-default-500 mb-3">"Agent 注册时需要使用此 Token"</p>
                                <div class="p-3 bg-default-50 rounded-xl font-mono text-xs text-default-600 break-all mb-3">"sg_token_abc123xyz456..."</div>
                                <div class="flex gap-2">
                                    <button class="flex-1 px-4 py-2 bg-default-100 hover:bg-default-200 rounded-xl text-sm font-medium transition-colors">"复制"</button>
                                    <button class="flex-1 px-4 py-2 bg-default-100 hover:bg-default-200 rounded-xl text-sm font-medium transition-colors">"重新生成"</button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </Layout>
    }
}

#[component]
fn MonitoringCard(
    title: &'static str,
    server: &'static str,
    value: &'static str,
    color: &'static str,
    bars: Vec<i32>,
) -> impl IntoView {
    let (bg_class, bar_class) = match color {
        "primary" => ("bg-gradient-to-t from-primary/10 to-transparent", "bg-primary"),
        "warning" => ("bg-gradient-to-t from-warning/10 to-transparent", "bg-warning"),
        "success" => ("bg-gradient-to-t from-success/10 to-transparent", "bg-success"),
        "violet" => ("bg-gradient-to-t from-violet-500/10 to-transparent", "bg-violet-500"),
        _ => ("bg-gradient-to-t from-primary/10 to-transparent", "bg-primary"),
    };

    view! {
        <div class="bg-white rounded-2xl p-6 shadow-hero">
            <div class="flex items-center justify-between mb-4">
                <span class="text-sm font-medium text-default-600">{title}</span>
                <span class="text-xs text-default-400">{server}</span>
            </div>
            <p class="text-3xl font-bold text-default-900 mb-3">{value}</p>
            <div class={format!("h-16 {} rounded-lg flex items-end px-1", bg_class)}>
                {bars.iter().enumerate().map(|(i, height)| {
                    let opacity = if i == bars.len() - 1 { "60" } else { "40" };
                    let bar_style = format!("height:{}%", height);
                    view! {
                        <div
                            class={format!("{}/{} rounded-t mx-0.5 flex-1", bar_class, opacity)}
                            style={bar_style}
                        ></div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
fn DockerContainer(
    name: &'static str,
    image: &'static str,
    cpu: &'static str,
    memory: &'static str,
    ports: &'static str,
    uptime: &'static str,
    icon_bg: &'static str,
    icon_shadow: &'static str,
) -> impl IntoView {
    let icon_path = match name {
        "nginx-proxy" => "M21 8c-1.45 0-2.26 1.44-1.93 2.51l-3.55 3.56c-.3-.09-.74-.09-1.04 0l-2.55-2.55C12.27 10.45 11.46 9 10 9c-1.45 0-2.27 1.44-1.93 2.52l-4.56 4.55C2.44 15.74 1 16.55 1 18c0 1.1.9 2 2 2 1.45 0 2.26-1.44 1.93-2.51l4.55-4.56c.3.09.74.09 1.04 0l2.55 2.55z",
        _ => "M12 3C7.58 3 4 4.79 4 7v10c0 2.21 3.59 4 8 4s8-1.79 8-4V7c0-2.21-3.58-4-8-4z",
    };

    view! {
        <div class="bg-white rounded-2xl shadow-hero p-6 hover:shadow-hero-lg transition-shadow">
            <div class="flex items-start justify-between mb-4">
                <div class={format!("w-12 h-12 rounded-2xl bg-gradient-to-br {} flex items-center justify-center shadow-lg {}", icon_bg, icon_shadow)}>
                    <svg class="w-6 h-6 text-white" fill="currentColor" viewBox="0 0 24 24">
                        <path d={icon_path}/>
                    </svg>
                </div>
                <span class="inline-flex items-center gap-1.5 text-success text-sm">
                    <span class="w-2 h-2 rounded-full bg-success animate-pulse"></span>
                    "运行中"
                </span>
            </div>
            <h4 class="font-semibold text-default-900 mb-1">{name}</h4>
            <p class="text-sm text-default-500 mb-4">{image}</p>
            <div class="grid grid-cols-2 gap-4 text-sm">
                <div>
                    <span class="text-default-400">"CPU"</span>
                    <p class="font-medium">{cpu}</p>
                </div>
                <div>
                    <span class="text-default-400">"内存"</span>
                    <p class="font-medium">{memory}</p>
                </div>
                <div>
                    <span class="text-default-400">"端口"</span>
                    <p class="font-medium">{ports}</p>
                </div>
                <div>
                    <span class="text-default-400">"运行时间"</span>
                    <p class="font-medium">{uptime}</p>
                </div>
            </div>
            <div class="flex gap-2 mt-4 pt-4 border-t border-default-100">
                <button class="flex-1 py-2 bg-default-100 hover:bg-default-200 rounded-xl text-sm font-medium transition-colors">"终端"</button>
                <button class="flex-1 py-2 bg-default-100 hover:bg-default-200 rounded-xl text-sm font-medium transition-colors">"日志"</button>
                <button class="flex-1 py-2 bg-default-100 hover:bg-default-200 rounded-xl text-sm font-medium transition-colors">"重启"</button>
            </div>
        </div>
    }
}
