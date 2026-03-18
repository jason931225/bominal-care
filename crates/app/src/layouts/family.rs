use leptos::prelude::*;
use leptos_router::components::A;

// ---------------------------------------------------------------------------
// Navigation data
// ---------------------------------------------------------------------------

struct NavItem {
    href: &'static str,
    label: &'static str,
    icon_path: &'static str,
    badge: u32,
}

struct NavSection {
    title: &'static str,
    items: &'static [NavItem],
}

const NAV_SECTIONS: &[NavSection] = &[
    NavSection {
        title: "홈",
        items: &[
            NavItem { href: "/family", label: "대시보드", icon_path: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6", badge: 0 },
            NavItem { href: "/family/timeline", label: "케어 타임라인", icon_path: "M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01", badge: 0 },
            NavItem { href: "/family/notifications", label: "알림", icon_path: "M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9", badge: 3 },
            NavItem { href: "/family/observability", label: "모니터링", icon_path: "M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z", badge: 0 },
        ],
    },
    NavSection {
        title: "케어 관리",
        items: &[
            NavItem { href: "/family/care", label: "케어 플랜", icon_path: "M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z", badge: 0 },
            NavItem { href: "/family/medications", label: "복약 현황", icon_path: "M9.75 3.104v5.714a2.25 2.25 0 01-.659 1.591L5 14.5M14.25 3.104v5.714c0 .597.237 1.17.659 1.591L19.8 15.3", badge: 0 },
            NavItem { href: "/family/approvals", label: "승인 대기", icon_path: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z", badge: 2 },
        ],
    },
    NavSection {
        title: "매칭 & 계약",
        items: &[
            NavItem { href: "/family/matching", label: "매칭 요청", icon_path: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z", badge: 0 },
            NavItem { href: "/family/payments", label: "결제 내역", icon_path: "M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z", badge: 0 },
        ],
    },
    NavSection {
        title: "지원",
        items: &[
            NavItem { href: "/family/settings", label: "설정", icon_path: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z", badge: 0 },
        ],
    },
];

struct BottomTab {
    href: &'static str,
    label: &'static str,
    icon_path: &'static str,
}

const BOTTOM_TABS: &[BottomTab] = &[
    BottomTab { href: "/family", label: "홈", icon_path: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" },
    BottomTab { href: "/family/timeline", label: "타임라인", icon_path: "M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01" },
    BottomTab { href: "/family/notifications", label: "알림", icon_path: "M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" },
    BottomTab { href: "/family/matching", label: "매칭", icon_path: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" },
    BottomTab { href: "/family/settings", label: "프로필", icon_path: "M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" },
];

// ---------------------------------------------------------------------------
// Sidebar nav item
// ---------------------------------------------------------------------------

#[component]
fn SidebarNavItem(
    href: &'static str,
    label: &'static str,
    icon_path: &'static str,
    badge: u32,
) -> impl IntoView {
    view! {
        <A
            href=href
            attr:class="flex items-center gap-2.5 px-3 py-2 rounded-lg mb-0.5 text-sm font-medium transition-colors text-gray-600 hover:bg-gray-100 hover:text-gray-900"
        >
            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d=icon_path />
            </svg>
            <span class="flex-1">{label}</span>
            {(badge > 0).then(|| view! {
                <span class="w-5 h-5 bg-red-500 text-white text-xs rounded-full flex items-center justify-center font-bold">
                    {badge}
                </span>
            })}
        </A>
    }
}

// ---------------------------------------------------------------------------
// Sidebar
// ---------------------------------------------------------------------------

#[component]
fn Sidebar() -> impl IntoView {
    view! {
        <aside class="hidden lg:flex w-64 bg-white border-r border-gray-200 flex-col fixed inset-y-0 left-0 top-14 z-30">
            <nav class="flex-1 overflow-y-auto py-4 px-3">
                {NAV_SECTIONS.iter().map(|section| {
                    view! {
                        <div class="mb-5">
                            <p class="text-xs font-semibold text-gray-400 uppercase tracking-wider px-3 mb-1.5">
                                {section.title}
                            </p>
                            {section.items.iter().map(|item| {
                                view! {
                                    <SidebarNavItem
                                        href=item.href
                                        label=item.label
                                        icon_path=item.icon_path
                                        badge=item.badge
                                    />
                                }
                            }).collect_view()}
                        </div>
                    }
                }).collect_view()}
            </nav>

            // Sidebar footer
            <div class="border-t border-gray-100 p-4">
                <div class="flex items-center gap-3">
                    <div class="w-8 h-8 rounded-full bg-blue-600 text-white text-sm font-bold flex items-center justify-center">
                        "가"
                    </div>
                    <div class="flex-1 min-w-0">
                        <p class="text-sm font-medium text-gray-900 truncate">"가족 사용자"</p>
                    </div>
                </div>
            </div>
        </aside>
    }
}

// ---------------------------------------------------------------------------
// Top bar
// ---------------------------------------------------------------------------

#[component]
fn TopBar() -> impl IntoView {
    view! {
        <header class="bg-white border-b border-gray-200 sticky top-0 z-40">
            <div class="flex items-center justify-between px-4 h-14">
                // Left: Logo
                <div class="flex items-center gap-3">
                    <A href="/family" attr:class="flex items-center gap-2">
                        <svg class="w-6 h-6 text-blue-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" />
                        </svg>
                        <span class="font-bold text-gray-900 hidden sm:block">"가족 케어 포털"</span>
                    </A>
                </div>

                // Center: Senior selector
                <div class="relative">
                    <button class="flex items-center gap-2 px-3 py-1.5 bg-blue-50 border border-blue-200 rounded-full text-sm font-medium text-blue-700 hover:bg-blue-100 transition-colors">
                        <span class="w-6 h-6 rounded-full bg-blue-200 flex items-center justify-center text-xs">
                            "👴"
                        </span>
                        <span class="max-w-32 truncate">"김복순 (모)"</span>
                        <span class="text-blue-400">"▾"</span>
                    </button>
                </div>

                // Right: Notifications + Avatar
                <div class="flex items-center gap-2">
                    <A
                        href="/family/notifications"
                        attr:class="relative p-2 rounded-full text-gray-500 hover:bg-gray-100"
                    >
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
                        </svg>
                        <span class="absolute top-1 right-1 w-4 h-4 bg-red-500 text-white text-xs rounded-full flex items-center justify-center font-bold">
                            "5"
                        </span>
                    </A>
                    <a href="/auth/signin"
                       class="p-2 rounded-full text-gray-400 hover:text-gray-600 hover:bg-gray-100 transition-colors"
                       aria-label="로그아웃">
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15m3 0l3-3m0 0l-3-3m3 3H9" />
                        </svg>
                    </a>
                </div>
            </div>
        </header>
    }
}

// ---------------------------------------------------------------------------
// Bottom tab bar (mobile)
// ---------------------------------------------------------------------------

#[component]
fn BottomTabBar() -> impl IntoView {
    view! {
        <nav class="lg:hidden fixed bottom-0 inset-x-0 bg-white border-t border-gray-200 z-40">
            <div class="flex">
                {BOTTOM_TABS.iter().map(|tab| {
                    view! {
                        <A
                            href=tab.href
                            attr:class="flex-1 flex flex-col items-center py-2 text-xs font-medium text-gray-500"
                        >
                            <svg class="w-5 h-5 mb-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d=tab.icon_path />
                            </svg>
                            <span>{tab.label}</span>
                        </A>
                    }
                }).collect_view()}
            </div>
        </nav>
    }
}

// ---------------------------------------------------------------------------
// Family layout
// ---------------------------------------------------------------------------

/// Family portal layout.
/// Desktop: sidebar with nav sections + content area.
/// Mobile: hamburger menu + bottom tab bar (5 tabs).
/// Includes a senior selector dropdown in the top bar.
#[component]
pub fn FamilyLayout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50 flex flex-col">
            <TopBar />

            <div class="flex flex-1 overflow-hidden">
                <Sidebar />

                // Main content — offset for sidebar on desktop
                <main class="flex-1 overflow-y-auto pb-16 lg:pb-0 lg:ml-64">
                    {children()}
                </main>
            </div>

            <BottomTabBar />
        </div>
    }
}
