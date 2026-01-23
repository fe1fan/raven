use crate::components::*;
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Layout>
            <PageHeader
                title="仪表盘"
                subtitle="概览您的系统运行状态"
                action=view! {
                    <button class="bg-apple-blue hover:bg-apple-blue/90 text-white px-5 py-2.5 rounded-apple-xl text-sm font-semibold shadow-lg shadow-apple-blue/30 transition-all hover:scale-105 active:scale-95">
                        "系统自检"
                    </button>
                }.into_view()
            />

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
                <div class="lg:col-span-2">
                    <GlassCard>
                        <h3 class="text-lg font-bold mb-4 text-apple-label dark:text-apple-darkLabel">"资源趋势"</h3>
                        <div class="h-64 flex items-center justify-center border-2 border-dashed border-apple-gray-300 dark:border-white/10 rounded-apple-xl">
                            <span class="text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel">"Chart Placeholder"</span>
                        </div>
                    </GlassCard>
                </div>
                <div>
                    <GlassCard>
                        <h3 class="text-lg font-bold mb-4 text-apple-label dark:text-apple-darkLabel">"服务器状态"</h3>
                        <div class="h-64 flex items-center justify-center border-2 border-dashed border-apple-gray-300 dark:border-white/10 rounded-apple-xl">
                            <span class="text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel">"Pie Chart Placeholder"</span>
                        </div>
                    </GlassCard>
                </div>
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <RecentAlerts/>
                <RecentActivities/>
            </div>
        </Layout>
    }
}

#[component]
fn RecentAlerts() -> impl IntoView {
    view! {
        <GlassCard>
            <div class="flex items-center justify-between mb-4">
                <h3 class="text-lg font-bold text-apple-label dark:text-apple-darkLabel">"最近告警"</h3>
                <a href="/alerts" class="text-xs text-apple-blue font-medium hover:underline">"查看全部"</a>
            </div>
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
        </GlassCard>
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
        <div class="flex items-start gap-3 p-3 rounded-apple-2xl hover:bg-apple-gray-200/50 dark:hover:bg-white/5 transition-colors group">
            <div class=format!("w-8 h-8 rounded-full flex items-center justify-center flex-shrink-0 transition-transform group-hover:scale-110 {}", icon_bg)>
                <svg class=format!("w-4 h-4 {}", icon_color) fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
                </svg>
            </div>
            <div class="flex-1 min-w-0">
                <p class="text-sm font-medium text-apple-label dark:text-apple-darkLabel truncate">{title}</p>
                <div class="flex items-center gap-2 mt-0.5">
                    <span class="text-xs text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel">{server}</span>
                    <span class="text-xs text-apple-gray-400">"•"</span>
                    <span class="text-xs text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel">{time}</span>
                </div>
            </div>
        </div>
    }
}

#[component]
fn RecentActivities() -> impl IntoView {
    view! {
        <GlassCard>
            <div class="flex items-center justify-between mb-4">
                <h3 class="text-lg font-bold text-apple-label dark:text-apple-darkLabel">"最近活动"</h3>
                <a href="/audit" class="text-xs text-apple-blue font-medium hover:underline">"查看全部"</a>
            </div>
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
        </GlassCard>
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
        <div class="flex items-start gap-3 p-3 rounded-apple-2xl hover:bg-apple-gray-200/50 dark:hover:bg-white/5 transition-colors group">
            <div class="w-8 h-8 rounded-full bg-apple-blue/10 flex items-center justify-center flex-shrink-0 text-apple-blue font-bold text-xs group-hover:scale-110 transition-transform">
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
