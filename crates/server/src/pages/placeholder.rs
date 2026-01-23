use crate::components::*;
use leptos::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <Layout>
            <PageHeader
                title="系统概览"
                subtitle="查看所有节点的实时运行指标"
                action=view! {
                    <div class="flex gap-3">
                        <button class="px-4 py-2 bg-apple-gray-200 dark:bg-white/10 rounded-apple-xl text-sm font-medium text-apple-label dark:text-apple-darkLabel hover:bg-apple-gray-300 dark:hover:bg-white/20 transition-colors">
                            "刷新数据"
                        </button>
                    </div>
                }.into_view()
            />

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
                <DashboardCard
                    value="98.5%"
                    title="系统可用性"
                    badge="稳定"
                    badge_type="success"
                    icon_bg="bg-apple-blue"
                    icon_svg="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                />
                <DashboardCard
                    value="1.2k"
                    title="每秒请求数"
                    badge="+5%"
                    badge_type="primary"
                    icon_bg="bg-apple-indigo"
                    icon_svg="M13 10V3L4 14h7v7l9-11h-7z"
                />
                <DashboardCard
                    value="42ms"
                    title="平均响应时间"
                    badge="-2ms"
                    badge_type="success"
                    icon_bg="bg-apple-green"
                    icon_svg="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
                />
                <DashboardCard
                    value="15"
                    title="本周安全事件"
                    badge="常规"
                    badge_type="indigo"
                    icon_bg="bg-apple-red"
                    icon_svg="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944"
                />
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <GlassCard>
                    <h3 class="text-lg font-bold mb-4 text-apple-label dark:text-apple-darkLabel">"节点区域分布"</h3>
                    <div class="h-64 flex items-center justify-center border-2 border-dashed border-apple-gray-300 dark:border-white/10 rounded-apple-xl">
                        <span class="text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel">"Map Chart Placeholder"</span>
                    </div>
                </GlassCard>
                <GlassCard>
                    <h3 class="text-lg font-bold mb-4 text-apple-label dark:text-apple-darkLabel">"资源分配"</h3>
                    <div class="h-64 flex items-center justify-center border-2 border-dashed border-apple-gray-300 dark:border-white/10 rounded-apple-xl">
                        <span class="text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel">"Donut Chart Placeholder"</span>
                    </div>
                </GlassCard>
            </div>
        </Layout>
    }
}

#[component]
pub fn MonitoringPage() -> impl IntoView {
    view! {
        <Layout>
            <PageHeader
                title="监控中心"
                subtitle="多维度监控系统资源与服务状态"
            />

            <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                <MetricCard title="CPU 使用率" value="24%" trend="up" color="apple-blue" />
                <MetricCard title="内存使用率" value="62%" trend="down" color="apple-indigo" />
                <MetricCard title="网络流量" value="1.5 GB/s" trend="stable" color="apple-green" />
            </div>

            <GlassCard>
                <div class="flex items-center justify-between mb-6">
                    <h3 class="text-lg font-bold text-apple-label dark:text-apple-darkLabel">"实时性能监控"</h3>
                    <div class="flex gap-2">
                        <span class="px-3 py-1 bg-apple-blue/10 text-apple-blue text-xs font-bold rounded-full">"CPU"</span>
                        <span class="px-3 py-1 bg-apple-gray-200 dark:bg-white/10 text-apple-secondaryLabel text-xs font-bold rounded-full">"Memory"</span>
                    </div>
                </div>
                <div class="h-80 flex items-center justify-center border-2 border-dashed border-apple-gray-300 dark:border-white/10 rounded-apple-xl">
                    <span class="text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel">"Real-time Line Chart Placeholder"</span>
                </div>
            </GlassCard>
        </Layout>
    }
}

#[component]
fn MetricCard(title: &'static str, value: &'static str, trend: &'static str, color: &'static str) -> impl IntoView {
    view! {
        <div class="glass-card p-5 rounded-apple-2xl">
            <p class="text-sm font-medium text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel mb-1">{title}</p>
            <div class="flex items-end justify-between">
                <h4 class=format!("text-2xl font-bold text-{}", color)>{value}</h4>
                <div class="flex items-center gap-1 text-xs font-bold text-apple-green">
                    {match trend {
                        "up" => "↑ 12%",
                        "down" => "↓ 5%",
                        _ => "→ 0%",
                    }}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn AlertsPage() -> impl IntoView {
    view! {
        <Layout>
            <PageHeader
                title="告警中心"
                subtitle="集中管理系统异常与预警信息"
            />

            <div class="flex gap-4 mb-6">
                <div class="flex-1 relative">
                    <input type="text" placeholder="搜索告警..." class="w-full pl-10 pr-4 py-2 bg-apple-gray-200/50 dark:bg-white/10 border-none rounded-apple-xl text-sm focus:ring-2 focus:ring-apple-blue/50" />
                    <div class="absolute inset-y-0 left-3 flex items-center pointer-events-none text-apple-secondaryLabel">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>
                    </div>
                </div>
                <select class="bg-apple-gray-200/50 dark:bg-white/10 border-none rounded-apple-xl px-4 py-2 text-sm">
                    <option>"所有级别"</option>
                    <option>"严重"</option>
                    <option>"警告"</option>
                </select>
            </div>

            <GlassCard class="!p-0 overflow-hidden">
                <table class="w-full text-sm text-left">
                    <thead class="bg-apple-gray-100/50 dark:bg-white/5 border-b border-apple-gray-200/50 dark:border-white/10 text-xs uppercase text-apple-secondaryLabel font-semibold">
                        <tr>
                            <th class="px-6 py-4">"级别"</th>
                            <th class="px-6 py-4">"告警内容"</th>
                            <th class="px-6 py-4">"来源"</th>
                            <th class="px-6 py-4">"状态"</th>
                            <th class="px-6 py-4">"时间"</th>
                            <th class="px-6 py-4 text-right">"操作"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-apple-gray-200/50 dark:divide-white/5">
                        <AlertRow level="danger" content="CPU 使用率超过 95%" source="web-prod-01" status="未处理" time="2分钟前" />
                        <AlertRow level="warning" content="内存使用率过高 (85%)" source="db-master-01" status="已确认" time="15分钟前" />
                        <AlertRow level="warning" content="磁盘空间不足 (剩余 10%)" source="cache-01" status="未处理" time="1小时前" />
                    </tbody>
                </table>
            </GlassCard>
        </Layout>
    }
}

#[component]
fn AlertRow(level: &'static str, content: &'static str, source: &'static str, status: &'static str, time: &'static str) -> impl IntoView {
    view! {
        <tr class="hover:bg-apple-gray-200/50 dark:hover:bg-white/5 transition-colors">
            <td class="px-6 py-4">
                <Badge text=level.to_uppercase() variant=level />
            </td>
            <td class="px-6 py-4 font-medium text-apple-label dark:text-apple-darkLabel">{content}</td>
            <td class="px-6 py-4 text-apple-secondaryLabel">{source}</td>
            <td class="px-6 py-4 text-apple-secondaryLabel">{status}</td>
            <td class="px-6 py-4 text-apple-secondaryLabel">{time}</td>
            <td class="px-6 py-4 text-right">
                <button class="text-apple-blue font-medium hover:underline">"处理"</button>
            </td>
        </tr>
    }
}

#[component]
pub fn TerminalPage() -> impl IntoView {
    view! {
        <Layout>
            <PageHeader
                title="Web 终端"
                subtitle="通过浏览器安全访问您的服务器终端"
            />

            <div class="flex gap-4 mb-6">
                <select class="flex-1 bg-apple-gray-200/50 dark:bg-white/10 border-none rounded-apple-xl px-4 py-2 text-sm">
                    <option>"选择服务器..."</option>
                    <option>"web-prod-01 (192.168.1.101)"</option>
                    <option>"db-master-01 (192.168.1.201)"</option>
                </select>
                <button class="bg-apple-blue text-white px-6 py-2 rounded-apple-xl text-sm font-bold">"连接"</button>
            </div>

            <div class="bg-[#1C1C1E] rounded-apple-3xl p-6 h-[600px] shadow-2xl border border-white/5 font-mono text-apple-green flex flex-col">
                <div class="flex gap-1.5 mb-4">
                    <div class="w-3 h-3 rounded-full bg-[#FF5F56]"></div>
                    <div class="w-3 h-3 rounded-full bg-[#FFBD2E]"></div>
                    <div class="w-3 h-3 rounded-full bg-[#27C93F]"></div>
                    <span class="ml-4 text-xs text-apple-gray-400">"ssh root@web-prod-01"</span>
                </div>
                <div class="flex-1 overflow-y-auto">
                    <p>"Last login: Fri Jan 23 10:24:15 2026 from 10.0.4.12"</p>
                    <p class="mt-2"><span class="text-apple-indigo">"root@web-prod-01"</span>":"<span class="text-apple-blue">"~"</span>"# ls -la"</p>
                    <p>"total 48"</p>
                    <p>"drwxr-x---  5 root root 4096 Jan 23 10:00 ."</p>
                    <p>"drwxr-xr-x 20 root root 4096 Jan 20 15:30 .."</p>
                    <p class="mt-2"><span class="text-apple-indigo">"root@web-prod-01"</span>":"<span class="text-apple-blue">"~"</span>"# "<span class="animate-pulse">"_"</span></p>
                </div>
            </div>
        </Layout>
    }
}

#[component]
pub fn CommandsPage() -> impl IntoView {
    view! {
        <Layout>
            <PageHeader
                title="命令执行"
                subtitle="在多台服务器上批量执行维护命令"
            />

            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                <GlassCard class="lg:col-span-1">
                    <h3 class="text-lg font-bold mb-4 text-apple-label dark:text-apple-darkLabel">"配置参数"</h3>
                    <div class="space-y-4">
                        <div>
                            <label class="block text-xs font-bold text-apple-secondaryLabel uppercase mb-1">"目标服务器"</label>
                            <div class="p-3 bg-apple-gray-200/50 dark:bg-white/10 rounded-apple-xl space-y-2">
                                <label class="flex items-center gap-2 text-sm"><input type="checkbox" checked /> "生产环境所有节点"</label>
                                <label class="flex items-center gap-2 text-sm"><input type="checkbox" /> "web-prod-01"</label>
                                <label class="flex items-center gap-2 text-sm"><input type="checkbox" /> "web-prod-02"</label>
                            </div>
                        </div>
                        <div>
                            <label class="block text-xs font-bold text-apple-secondaryLabel uppercase mb-1">"超时设置"</label>
                            <input type="number" value="30" class="w-full bg-apple-gray-200/50 dark:bg-white/10 border-none rounded-apple-xl px-4 py-2 text-sm" />
                        </div>
                    </div>
                </GlassCard>

                <GlassCard class="lg:col-span-2">
                    <h3 class="text-lg font-bold mb-4 text-apple-label dark:text-apple-darkLabel">"脚本编辑"</h3>
                    <textarea class="w-full h-48 bg-[#1C1C1E] text-white font-mono p-4 rounded-apple-xl border border-white/5 focus:outline-none" placeholder="#!/bin/bash\n\napt-get update\napt-get upgrade -y"></textarea>
                    <div class="mt-4 flex justify-end gap-3">
                        <button class="px-6 py-2 bg-apple-gray-200 dark:bg-white/10 rounded-apple-xl text-sm font-bold">"保存模版"</button>
                        <button class="px-6 py-2 bg-apple-blue text-white rounded-apple-xl text-sm font-bold">"立即执行"</button>
                    </div>
                </GlassCard>
            </div>
        </Layout>
    }
}

#[component]
pub fn CronjobsPage() -> impl IntoView {
    view! {
        <Layout>
            <PageHeader
                title="定时任务"
                subtitle="管理系统自动化脚本与定时作业"
                action=view! {
                    <button class="bg-apple-blue text-white px-5 py-2.5 rounded-apple-xl text-sm font-bold">"创建任务"</button>
                }.into_view()
            />

            <GlassCard class="!p-0 overflow-hidden">
                <table class="w-full text-sm text-left">
                    <thead class="bg-apple-gray-100/50 dark:bg-white/5 border-b border-apple-gray-200/50 dark:border-white/10 text-xs uppercase text-apple-secondaryLabel font-semibold">
                        <tr>
                            <th class="px-6 py-4">"任务名称"</th>
                            <th class="px-6 py-4">"执行计划"</th>
                            <th class="px-6 py-4">"最近运行"</th>
                            <th class="px-6 py-4">"状态"</th>
                            <th class="px-6 py-4 text-right">"操作"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-apple-gray-200/50 dark:divide-white/5">
                        <CronjobRow name="每日数据库备份" schedule="0 0 * * *" last_run="22小时前" status="success" />
                        <CronjobRow name="清理临时文件" schedule="0 2 * * *" last_run="20小时前" status="success" />
                        <CronjobRow name="证书自动更新" schedule="0 0 1 * *" last_run="20天前" status="warning" />
                    </tbody>
                </table>
            </GlassCard>
        </Layout>
    }
}

#[component]
fn CronjobRow(name: &'static str, schedule: &'static str, last_run: &'static str, status: &'static str) -> impl IntoView {
    view! {
        <tr class="hover:bg-apple-gray-200/50 dark:hover:bg-white/5 transition-colors">
            <td class="px-6 py-4 font-medium text-apple-label dark:text-apple-darkLabel">{name}</td>
            <td class="px-6 py-4 font-mono text-xs">{schedule}</td>
            <td class="px-6 py-4 text-apple-secondaryLabel">{last_run}</td>
            <td class="px-6 py-4">
                <Badge text=status.to_uppercase() variant=status />
            </td>
            <td class="px-6 py-4 text-right">
                <button class="text-apple-secondaryLabel hover:text-apple-blue transition-colors">"编辑"</button>
            </td>
        </tr>
    }
}

#[component]
pub fn DockerPage() -> impl IntoView {
    view! {
        <Layout>
            <PageHeader
                title="容器管理"
                subtitle="监控容器运行状态与镜像分发"
            />

            <div class="flex gap-1 p-1 bg-apple-gray-200/50 dark:bg-white/10 w-fit rounded-apple-2xl mb-8">
                <button class="px-6 py-2 bg-white dark:bg-white/10 shadow-sm rounded-apple-xl text-sm font-bold">"容器 (12)"</button>
                <button class="px-6 py-2 text-apple-secondaryLabel text-sm font-medium">"镜像 (45)"</button>
                <button class="px-6 py-2 text-apple-secondaryLabel text-sm font-medium">"网络 & 卷"</button>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                <ContainerCard name="api-gateway" image="nginx:latest" status="running" cpu="0.5%" ram="42MB" />
                <ContainerCard name="redis-cache" image="redis:7-alpine" status="running" cpu="1.2%" ram="128MB" />
                <ContainerCard name="worker-node" image="raven-worker:v1.2" status="stopped" cpu="0%" ram="0MB" />
            </div>
        </Layout>
    }
}

#[component]
fn ContainerCard(name: &'static str, image: &'static str, status: &'static str, cpu: &'static str, ram: &'static str) -> impl IntoView {
    view! {
        <GlassCard>
            <div class="flex items-center justify-between mb-4">
                <div class="flex items-center gap-3">
                    <div class="w-10 h-10 bg-apple-blue/10 rounded-apple-xl flex items-center justify-center text-apple-blue">
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>
                    </div>
                    <div>
                        <h4 class="font-bold text-apple-label dark:text-apple-darkLabel">{name}</h4>
                        <p class="text-[10px] text-apple-secondaryLabel font-mono">{image}</p>
                    </div>
                </div>
                <Badge text=status variant=if status == "running" { "success" } else { "danger" } />
            </div>
            <div class="flex gap-4 border-t border-apple-gray-200/50 dark:border-white/10 pt-4">
                <div class="flex-1 text-center">
                    <p class="text-[10px] uppercase font-bold text-apple-secondaryLabel">"CPU"</p>
                    <p class="text-sm font-bold text-apple-label dark:text-apple-darkLabel">{cpu}</p>
                </div>
                <div class="flex-1 text-center border-x border-apple-gray-200/50 dark:border-white/10">
                    <p class="text-[10px] uppercase font-bold text-apple-secondaryLabel">"RAM"</p>
                    <p class="text-sm font-bold text-apple-label dark:text-apple-darkLabel">{ram}</p>
                </div>
                <div class="flex-1 text-center">
                    <p class="text-[10px] uppercase font-bold text-apple-secondaryLabel">"UPTIME"</p>
                    <p class="text-sm font-bold text-apple-label dark:text-apple-darkLabel">"12h"</p>
                </div>
            </div>
        </GlassCard>
    }
}

#[component]
pub fn DatabasePage() -> impl IntoView {
    view! {
        <Layout>
            <PageHeader
                title="数据库"
                subtitle="查看数据库实例健康状况与存储占用"
            />

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <DatabaseCard type_="PostgreSQL" version="15.2" status="active" storage="45.2 GB" connections="128" />
                <DatabaseCard type_="Redis" version="7.0" status="active" storage="1.2 GB" connections="1,054" />
            </div>
        </Layout>
    }
}

#[component]
fn DatabaseCard(type_: &'static str, version: &'static str, status: &'static str, storage: &'static str, connections: &'static str) -> impl IntoView {
    view! {
        <GlassCard>
            <div class="flex items-start justify-between">
                <div class="flex gap-4">
                    <div class="w-12 h-12 bg-apple-indigo/10 rounded-apple-xl flex items-center justify-center text-apple-indigo">
                        <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>
                    </div>
                    <div>
                        <h4 class="text-xl font-bold text-apple-label dark:text-apple-darkLabel">{type_}</h4>
                        <p class="text-xs text-apple-secondaryLabel">"Version "{version}</p>
                    </div>
                </div>
                <Badge text=status variant="success" />
            </div>
            <div class="grid grid-cols-2 gap-4 mt-6">
                <div class="p-3 bg-apple-gray-200/50 dark:bg-white/10 rounded-apple-xl">
                    <p class="text-[10px] font-bold text-apple-secondaryLabel">"存储占用"</p>
                    <p class="text-lg font-bold text-apple-label dark:text-apple-darkLabel">{storage}</p>
                </div>
                <div class="p-3 bg-apple-gray-200/50 dark:bg-white/10 rounded-apple-xl">
                    <p class="text-[10px] font-bold text-apple-secondaryLabel">"当前连接"</p>
                    <p class="text-lg font-bold text-apple-label dark:text-apple-darkLabel">{connections}</p>
                </div>
            </div>
        </GlassCard>
    }
}

#[component]
pub fn FilesPage() -> impl IntoView {
    view! {
        <Layout>
            <PageHeader
                title="文件管理"
                subtitle="浏览、上传与管理服务器文件资源"
            />

            <GlassCard class="!p-0 overflow-hidden">
                <div class="p-4 border-b border-apple-gray-200/50 dark:border-white/10 flex items-center gap-2">
                    <button class="p-2 hover:bg-apple-gray-200 rounded-lg">"←"</button>
                    <div class="flex-1 bg-apple-gray-200/50 dark:bg-white/10 px-4 py-2 rounded-apple-xl text-sm font-mono text-apple-secondaryLabel">
                        "/var/www/raven/crates/server/src"
                    </div>
                    <button class="bg-apple-blue text-white px-4 py-2 rounded-apple-xl text-sm font-bold">"上传"</button>
                </div>
                <table class="w-full text-sm text-left">
                    <thead class="bg-apple-gray-100/50 dark:bg-white/5 text-xs uppercase text-apple-secondaryLabel font-semibold">
                        <tr>
                            <th class="px-6 py-4">"文件名"</th>
                            <th class="px-6 py-4">"大小"</th>
                            <th class="px-6 py-4">"修改时间"</th>
                            <th class="px-6 py-4 text-right">"权限"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-apple-gray-200/50 dark:divide-white/5">
                        <FileRow name="main.rs" size="12 KB" date="2小时前" perms="644" />
                        <FileRow name="app.rs" size="8 KB" date="5小时前" perms="644" />
                        <FileRow name="assets/" size="-" date="1天前" perms="755" is_dir=true />
                    </tbody>
                </table>
            </GlassCard>
        </Layout>
    }
}

#[component]
fn FileRow(name: &'static str, size: &'static str, date: &'static str, perms: &'static str, #[prop(optional)] is_dir: bool) -> impl IntoView {
    view! {
        <tr class="hover:bg-apple-gray-200/50 dark:hover:bg-white/5 transition-colors cursor-pointer">
            <td class="px-6 py-4 flex items-center gap-3 font-medium text-apple-label dark:text-apple-darkLabel">
                {if is_dir { "📁" } else { "📄" }} {name}
            </td>
            <td class="px-6 py-4 text-apple-secondaryLabel">{size}</td>
            <td class="px-6 py-4 text-apple-secondaryLabel">{date}</td>
            <td class="px-6 py-4 text-right font-mono text-xs">{perms}</td>
        </tr>
    }
}

#[component]
pub fn FirewallPage() -> impl IntoView {
    view! {
        <Layout>
            <PageHeader
                title="防火墙"
                subtitle="配置安全组规则与端口转发"
            />
            <GlassCard>
                <div class="h-64 flex items-center justify-center border-2 border-dashed border-apple-gray-300 dark:border-white/10 rounded-apple-xl">
                    <span class="text-apple-secondaryLabel dark:text-apple-darkSecondaryLabel">"Firewall Rules Table Placeholder"</span>
                </div>
            </GlassCard>
        </Layout>
    }
}

#[component]
pub fn AuditPage() -> impl IntoView {
    view! {
        <Layout>
            <PageHeader
                title="操作审计"
                subtitle="追踪系统关键操作与配置变更"
            />
            <GlassCard class="!p-0 overflow-hidden">
                <table class="w-full text-sm text-left">
                    <thead class="bg-apple-gray-100/50 dark:bg-white/5 border-b border-apple-gray-200/50 dark:border-white/10 text-xs uppercase text-apple-secondaryLabel font-semibold">
                        <tr>
                            <th class="px-6 py-4">"时间"</th>
                            <th class="px-6 py-4">"用户"</th>
                            <th class="px-6 py-4">"操作"</th>
                            <th class="px-6 py-4">"目标"</th>
                            <th class="px-6 py-4 text-right">"结果"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-apple-gray-200/50 dark:divide-white/5 text-apple-secondaryLabel">
                        <tr>
                            <td class="px-6 py-4 text-xs">"2026-01-23 10:45"</td>
                            <td class="px-6 py-4 font-bold text-apple-label dark:text-apple-darkLabel">"Admin"</td>
                            <td class="px-6 py-4">"重启服务"</td>
                            <td class="px-6 py-4">"web-prod-01"</td>
                            <td class="px-6 py-4 text-right"><Badge text="成功" variant="success" /></td>
                        </tr>
                    </tbody>
                </table>
            </GlassCard>
        </Layout>
    }
}

#[component]
pub fn UsersPage() -> impl IntoView {
    view! {
        <Layout>
            <PageHeader
                title="用户权限"
                subtitle="管理团队成员与系统访问控制"
                action=view! {
                    <button class="bg-apple-blue text-white px-5 py-2.5 rounded-apple-xl text-sm font-bold">"添加用户"</button>
                }.into_view()
            />

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                <UserCard name="Admin" role="Owner" email="admin@raven.io" />
                <UserCard name="Developer" role="Editor" email="dev@raven.io" />
                <UserCard name="Guest" role="Viewer" email="guest@raven.io" />
            </div>
        </Layout>
    }
}

#[component]
fn UserCard(name: &'static str, role: &'static str, email: &'static str) -> impl IntoView {
    view! {
        <GlassCard>
            <div class="flex items-center gap-4">
                <div class="w-14 h-14 bg-gradient-to-br from-apple-blue to-apple-indigo rounded-full flex items-center justify-center text-white text-xl font-bold">
                    {name.chars().next().unwrap().to_string()}
                </div>
                <div>
                    <h4 class="font-bold text-lg text-apple-label dark:text-apple-darkLabel">{name}</h4>
                    <p class="text-xs text-apple-secondaryLabel mb-2">{email}</p>
                    <Badge text=role variant=if role == "Owner" { "primary" } else { "indigo" } />
                </div>
            </div>
        </GlassCard>
    }
}

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <Layout>
            <PageHeader
                title="系统设置"
                subtitle="配置全局参数与安全策略"
            />

            <div class="max-w-3xl space-y-6">
                <GlassCard>
                    <h3 class="text-lg font-bold mb-4 text-apple-label dark:text-apple-darkLabel">"通用设置"</h3>
                    <div class="space-y-4">
                        <div class="flex items-center justify-between">
                            <div>
                                <p class="font-medium text-apple-label dark:text-apple-darkLabel">"站点名称"</p>
                                <p class="text-xs text-apple-secondaryLabel">"显示在浏览器标签页与侧边栏"</p>
                            </div>
                            <input type="text" value="Raven" class="bg-apple-gray-200/50 dark:bg-white/10 border-none rounded-apple-xl px-4 py-2 text-sm w-48" />
                        </div>
                        <div class="flex items-center justify-between">
                            <div>
                                <p class="font-medium text-apple-label dark:text-apple-darkLabel">"自动备份"</p>
                                <p class="text-xs text-apple-secondaryLabel">"每24小时备份一次数据库"</p>
                            </div>
                            <div class="w-12 h-6 bg-apple-blue rounded-full relative"><div class="absolute right-1 top-1 w-4 h-4 bg-white rounded-full"></div></div>
                        </div>
                    </div>
                </GlassCard>

                <GlassCard>
                    <h3 class="text-lg font-bold mb-4 text-apple-label dark:text-apple-darkLabel">"安全设置"</h3>
                    <div class="space-y-4">
                        <button class="w-full py-3 bg-apple-red/10 text-apple-red font-bold rounded-apple-xl">"重置所有 API 密钥"</button>
                    </div>
                </GlassCard>
            </div>
        </Layout>
    }
}
