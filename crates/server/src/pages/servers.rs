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
                    <ButtonSimple>
                        "添加服务器"
                    </ButtonSimple>
                }.into_view()
            />

            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
                <ServerStatCard value="119" label="在线" color="success"/>
                <ServerStatCard value="5" label="告警中" color="warning"/>
                <ServerStatCard value="4" label="离线" color="danger"/>
                <ServerStatCard value="128" label="总计" color="primary"/>
            </div>

            <GlassCard class="!p-0">
                <div class="relative z-10 p-4 border-b border-apple-gray-200/50 dark:border-white/10 flex flex-wrap items-center gap-4">
                    <div class="flex gap-2 w-full sm:w-auto">
                        <div class="w-32">
                            <Select
                                placeholder="所有环境"
                                options=vec![
                                    ("all".to_string(), "所有环境".to_string()),
                                    ("prod".to_string(), "生产环境".to_string()),
                                    ("test".to_string(), "测试环境".to_string()),
                                ]
                            />
                        </div>
                        <div class="w-32">
                            <Select
                                placeholder="所有状态"
                                options=vec![
                                    ("all".to_string(), "所有状态".to_string()),
                                    ("online".to_string(), "在线".to_string()),
                                    ("offline".to_string(), "离线".to_string()),
                                ]
                            />
                        </div>
                    </div>
                    <div class="flex-1 min-w-[200px]">
                        <Input
                            placeholder="搜索服务器..."
                            icon=view! {
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>
                            }.into_view()
                        />
                    </div>
                </div>
                <div class="overflow-hidden rounded-b-apple-3xl">
                    <Table>
                        <TableHeader>
                        <TableHead>"服务器"</TableHead>
                        <TableHead>"IP地址"</TableHead>
                        <TableHead>"状态"</TableHead>
                        <TableHead>"资源占用"</TableHead>
                        <TableHead><div class="text-right">"操作"</div></TableHead>
                    </TableHeader>
                    <TableBody>
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
                    </TableBody>
                    </Table>
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
            <Badge text="运行中".to_string() variant="success" />
        },
        "warning" => view! {
            <Badge text="告警".to_string() variant="warning" />
        },
        _ => view! {
             <Badge text="离线".to_string() variant="danger" />
        },
    };

    view! {
        <TableRow>
            <TableCell>
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
            </TableCell>
            <TableCell>
                <DisplayText value=ip.to_string() class="text-xs" />
            </TableCell>
            <TableCell>{status_view}</TableCell>
            <TableCell>
                <div class="space-y-2 min-w-[140px]">
                    <ResourceBar label="CPU" value=cpu color="bg-apple-blue" />
                    <ResourceBar label="RAM" value=memory color="bg-apple-indigo" />
                    <ResourceBar label="DSK" value=disk color="bg-apple-green" />
                </div>
            </TableCell>
            <TableCell class="text-right">
                <div class="flex justify-end gap-2">
                    <ButtonSimple variant="ghost" size="small" class="!p-2">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>
                    </ButtonSimple>
                    <ButtonSimple variant="ghost" size="small" class="!p-2 text-apple-red hover:bg-apple-red/10">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>
                    </ButtonSimple>
                </div>
            </TableCell>
        </TableRow>
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
            <DisplayText value=format!("{}%", value) class="text-[10px] w-8 text-right text-apple-secondaryLabel" />
        </div>
    }
}
