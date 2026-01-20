use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::pages::*;
use crate::example_page::ExamplePage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html lang="zh-CN"/>
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Title text="Raven - 服务器管理系统"/>
        <Script src="https://cdn.tailwindcss.com"/>

        <Router>
            <Routes>
                <Route path="/" view=HomePage/>
                <Route path="/dashboard" view=DashboardPage/>
                <Route path="/servers" view=ServersPage/>
                <Route path="/monitoring" view=MonitoringPage/>
                <Route path="/alerts" view=AlertsPage/>
                <Route path="/terminal" view=TerminalPage/>
                <Route path="/commands" view=CommandsPage/>
                <Route path="/cronjobs" view=CronjobsPage/>
                <Route path="/docker" view=DockerPage/>
                <Route path="/database" view=DatabasePage/>
                <Route path="/files" view=FilesPage/>
                <Route path="/firewall" view=FirewallPage/>
                <Route path="/audit" view=AuditPage/>
                <Route path="/users" view=UsersPage/>
                <Route path="/settings" view=SettingsPage/>
                <Route path="/example" view=ExamplePage/>
            </Routes>
        </Router>
    }
}
