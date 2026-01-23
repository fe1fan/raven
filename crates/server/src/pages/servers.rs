use crate::components::*;
use leptos::*;

#[component]
pub fn ServersPage() -> impl IntoView {
    view! {
        <Layout>
            <PageHeader
                title="服务器管理"
                subtitle="管理所有节点的运行状态"
                action=view! {
                    <button class="bg-apple-blue hover:bg-apple-blue/90 text-white px-5 py-2.5 rounded-apple-xl text-sm font-semibold shadow-lg shadow-apple-blue/30 transition-all hover:scale-105 active:scale-95">
                        "添加服务器"
                    </button>
                }.into_view()
            />

            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
                <ServerStatCard value="119" label="在线" color="success"/>
                <ServerStatCard value="5" label="告警中" color="warning"/>
                <ServerStatCard value="4" label="离线" color="danger"/>
                <ServerStatCard value="128" label="总计" color="primary"/>
            </div>

            <GlassCard class="!p-0 overflow-hidden">
                <div class="p-4 border-b border-apple-gray-200/50 dark:border-white/10 flex flex-wrap items-center gap-4">
                    <div class="flex gap-2">
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
                    </div>
                    <div class="flex-1 min-w-[200px]">
                        <div class="relative group">
                            <input
                                type="text"
                                placeholder="搜索服务器..."
                                class="w-full bg-apple-gray-200/50 dark:bg-white/10 border-none rounded-apple-xl pl-10 pr-4 py-2 text-sm text-apple-label dark:text-apple-darkLabel focus:outline-none focus:ring-2 focus:ring-apple-blue/50"
                            />
                            <div class="absolute inset-y-0 left-3 flex items-center pointer-events-none text-apple-secondaryLabel group-focus-within:text-apple-blue transition-colors">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="overflow-x-auto">
                    <table class="w-full text-sm text-left">
                        <thead class="bg-apple-gray-100/50 dark:bg-white/5 border-b border-apple-gray-200/50 dark:border-white/10 text-xs uppercase text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel font-semibold">
                            <tr>
                                <th class="px-6 py-4">"服务器"</th>
                                <th class="px-6 py-4">"IP地址"</th>
                                <th class="px-6 py-4">"状态"</th>
                                <th class="px-6 py-4">"资源占用"</th>
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
            </GlassCard>
        </Layout>
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
                    <div class="w-10 h-10 rounded-xl bg-apple-gray-200 dark:bg-white/10 flex items-center justify-center text-apple-secondaryLabel transition-transform group-hover:scale-110">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01"/>
                        </svg>
                    </div>
                    <div>
                        <div class="font-bold text-apple-label dark:text-apple-darkLabel flex items-center gap-2">
                            {name}
                            <span class="px-1.5 py-0.5 rounded text-[10px] bg-apple-gray-200/50 dark:bg-white/10 text-apple-secondaryLabel">{env}</span>
                        </div>
                        <div class="text-xs text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel">{os}</div>
                    </div>
                </div>
            </td>
            <td class="px-6 py-4">
                <div class="font-mono text-xs text-apple-label dark:text-apple-darkLabel">{ip}</div>
            </td>
            <td class="px-6 py-4">{status_view}</td>
            <td class="px-6 py-4 min-w-[200px]">
                <div class="space-y-2">
                    <ResourceBar label="CPU" value=cpu color="bg-apple-blue" />
                    <ResourceBar label="RAM" value=memory color="bg-apple-indigo" />
                    <ResourceBar label="DSK" value=disk color="bg-apple-green" />
                </div>
            </td>
            <td class="px-6 py-4 text-right">
                <div class="flex justify-end gap-2">
                    <button class="p-2 text-apple-secondaryLabel hover:text-apple-blue hover:bg-apple-blue/10 rounded-lg transition-all" title="终端">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>
                    </button>
                    <button class="p-2 text-apple-secondaryLabel hover:text-apple-red hover:bg-apple-red/10 rounded-lg transition-all" title="更多">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>
                    </button>
                </div>
            </td>
        </tr>
    }
}

#[component]
fn ResourceBar(label: &'static str, value: &'static str, color: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-center gap-2">
            <span class="text-[10px] font-bold text-apple-secondaryLabel w-8">{label}</span>
            <div class="flex-1 h-1.5 bg-apple-gray-200 dark:bg-white/5 rounded-full overflow-hidden">
                <div class=format!("h-full {} rounded-full transition-all duration-500", color) style=format!("width: {}%", value)></div>
            </div>
            <span class="text-[10px] font-mono text-apple-secondaryLabel w-8 text-right">{value}"%"</span>
        </div>
    }
}
