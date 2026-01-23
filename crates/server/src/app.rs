use crate::example_page::ExamplePage;
use crate::pages::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let tailwind_config = r#"
        tailwind.config = {
            darkMode: 'class',
            theme: {
                extend: {
                    colors: {
                        apple: {
                            bg: '#F5F5F7',
                            card: 'rgba(255, 255, 255, 0.8)',
                            label: '#000000',
                            secondaryLabel: 'rgba(60, 60, 67, 0.6)',
                            tertiaryLabel: 'rgba(60, 60, 67, 0.3)',
                            border: 'rgba(0, 0, 0, 0.1)',

                            darkBg: '#000000',
                            darkCard: 'rgba(28, 28, 30, 0.7)',
                            darkLabel: '#FFFFFF',
                            darkSecondaryLabel: 'rgba(235, 235, 245, 0.6)',
                            darkTertiaryLabel: 'rgba(235, 235, 245, 0.3)',
                            darkBorder: 'rgba(255, 255, 255, 0.12)',

                            blue: '#007AFF',
                            darkBlue: '#0A84FF',
                            green: '#34C759',
                            darkGreen: '#30D158',
                            red: '#FF3B30',
                            darkRed: '#FF453A',
                            yellow: '#FFCC00',
                            indigo: '#5856D6',
                            gray: {
                                100: '#F5F5F7',
                                200: '#E5E5EA',
                                300: '#C7C7CC',
                                400: '#AEAEB2',
                                500: '#8E8E93',
                                600: '#636366',
                                700: '#48484A',
                                800: '#2C2C2E',
                                900: '#1C1C1E',
                            },
                        }
                    },
                    borderRadius: {
                        'apple-xl': '12px',
                        'apple-2xl': '16px',
                        'apple-3xl': '22px',
                    },
                    fontFamily: {
                        sans: ['Inter', '-apple-system', 'BlinkMacSystemFont', 'SF Pro Text', 'system-ui', 'sans-serif'],
                    }
                }
            }
        }
    "#;

    view! {
        <Html lang="zh-CN" class="antialiased"/>
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Title text="Raven - 服务器管理系统"/>

        // 1. Google Fonts
        <Link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap"/>

        // 2. Tailwind CDN
        <Script src="https://cdn.tailwindcss.com"/>

        // 3. Tailwind Configuration
        <Script>
            {tailwind_config}
        </Script>

        // 4. Theme initialization script (runs before render to prevent flash)
        <Script>
            r#"
            (function() {
                var theme = localStorage.getItem('theme');
                if (theme === 'dark' || (!theme && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
                    document.documentElement.classList.add('dark');
                    document.documentElement.classList.remove('light');
                } else {
                    document.documentElement.classList.add('light');
                    document.documentElement.classList.remove('dark');
                }
                window.toggleTheme = function() {
                    var html = document.documentElement;
                    if (html.classList.contains('dark')) {
                        html.classList.remove('dark');
                        html.classList.add('light');
                        localStorage.setItem('theme', 'light');
                    } else {
                        html.classList.remove('light');
                        html.classList.add('dark');
                        localStorage.setItem('theme', 'dark');
                    }
                };
            })();
            "#
        </Script>

        // 5. Global Styles for Glassmorphism
        <Style>
            ".glass-card {
                backdrop-filter: saturate(180%) blur(20px);
                -webkit-backdrop-filter: saturate(180%) blur(20px);
            }
            .dark .glass-card {
                background: rgba(28, 28, 30, 0.7);
                border: 1px solid rgba(255, 255, 255, 0.08);
            }
            html:not(.dark) .glass-card {
                background: rgba(255, 255, 255, 0.7);
                border: 1px solid rgba(0, 0, 0, 0.05);
                box-shadow: 0 4px 24px rgba(0, 0, 0, 0.04);
            }"
        </Style>

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
