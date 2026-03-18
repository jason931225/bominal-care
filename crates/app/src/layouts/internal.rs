use leptos::prelude::*;
use leptos_router::components::A;

// ---------------------------------------------------------------------------
// Navigation data
// ---------------------------------------------------------------------------

struct NavItem {
    href: &'static str,
    label: &'static str,
    icon_path: &'static str,
}

struct NavSection {
    title: &'static str,
    items: &'static [NavItem],
}

const NAV_SECTIONS: &[NavSection] = &[
    NavSection {
        title: "",
        items: &[
            NavItem { href: "/internal", label: "대시보드", icon_path: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" },
        ],
    },
    NavSection {
        title: "이용자 관리",
        items: &[
            NavItem { href: "/internal/clients", label: "이용자", icon_path: "M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" },
            NavItem { href: "/internal/schedules", label: "일정", icon_path: "M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" },
        ],
    },
    NavSection {
        title: "인력 관리",
        items: &[
            NavItem { href: "/internal/caregivers", label: "요양보호사", icon_path: "M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" },
        ],
    },
    NavSection {
        title: "품질 및 의뢰",
        items: &[
            NavItem { href: "/internal/quality", label: "품질", icon_path: "M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" },
            NavItem { href: "/internal/referrals", label: "의뢰", icon_path: "M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4" },
        ],
    },
    NavSection {
        title: "행정",
        items: &[
            NavItem { href: "/internal/compliance", label: "규정 준수", icon_path: "M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z" },
            NavItem { href: "/internal/reports", label: "보고서", icon_path: "M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" },
            NavItem { href: "/internal/settings", label: "설정", icon_path: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" },
        ],
    },
];

// ---------------------------------------------------------------------------
// Sidebar nav item
// ---------------------------------------------------------------------------

#[component]
fn SidebarNavItem(
    href: &'static str,
    label: &'static str,
    icon_path: &'static str,
) -> impl IntoView {
    view! {
        <li>
            <A
                href=href
                attr:class="flex items-center gap-2 px-3 py-2 rounded-lg text-sm font-medium transition-colors text-gray-600 hover:bg-gray-100 hover:text-gray-900"
            >
                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d=icon_path />
                </svg>
                <span>{label}</span>
            </A>
        </li>
    }
}

// ---------------------------------------------------------------------------
// Internal layout — admin-focused compact layout with left sidebar
// ---------------------------------------------------------------------------

/// Internal (provider) portal layout.
/// Left sidebar (w-60) with 5 nav sections, top bar showing org name.
/// Admin-focused compact layout.
#[component]
pub fn InternalLayout(children: Children) -> impl IntoView {
    view! {
        <div class="flex h-screen bg-gray-50">
            // Sidebar
            <aside class="w-60 flex-shrink-0 bg-white border-r border-gray-200 flex flex-col">
                // Logo
                <div class="h-16 flex items-center px-4 border-b border-gray-200">
                    <div class="flex items-center gap-2">
                        <div class="w-8 h-8 bg-blue-600 rounded-lg flex items-center justify-center">
                            <svg class="w-5 h-5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                            </svg>
                        </div>
                        <div>
                            <p class="text-sm font-bold text-gray-900">"시니어케어"</p>
                            <p class="text-xs text-gray-500">"내부 관리"</p>
                        </div>
                    </div>
                </div>

                // Navigation
                <nav class="flex-1 overflow-y-auto py-4 px-3 space-y-6">
                    {NAV_SECTIONS.iter().map(|section| {
                        view! {
                            <div>
                                {(!section.title.is_empty()).then(|| view! {
                                    <p class="px-3 mb-1 text-xs font-semibold text-gray-400 uppercase tracking-wider">
                                        {section.title}
                                    </p>
                                })}
                                <ul class="space-y-0.5">
                                    {section.items.iter().map(|item| {
                                        view! {
                                            <SidebarNavItem
                                                href=item.href
                                                label=item.label
                                                icon_path=item.icon_path
                                            />
                                        }
                                    }).collect_view()}
                                </ul>
                            </div>
                        }
                    }).collect_view()}
                </nav>

                // User info
                <div class="p-4 border-t border-gray-200">
                    <div class="flex items-center gap-3">
                        <div class="w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center">
                            <span class="text-xs font-semibold text-blue-700">"관"</span>
                        </div>
                        <div class="flex-1 min-w-0">
                            <p class="text-sm font-medium text-gray-900 truncate">"관리자"</p>
                            <p class="text-xs text-gray-500 truncate">"시설장"</p>
                        </div>
                        <a
                            href="/auth/signin"
                            class="p-1.5 text-gray-400 hover:text-gray-600 hover:bg-gray-100 rounded-lg transition-colors"
                            aria-label="로그아웃"
                        >
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15m3 0l3-3m0 0l-3-3m3 3H9" />
                            </svg>
                        </a>
                    </div>
                </div>
            </aside>

            // Main area
            <div class="flex-1 flex flex-col overflow-hidden">
                // Top bar
                <header class="h-16 bg-white border-b border-gray-200 flex items-center justify-between px-6 flex-shrink-0">
                    <div>
                        <h1 class="text-sm font-semibold text-gray-900">"행복노인복지센터"</h1>
                        <p class="text-xs text-gray-500">"서울특별시 강남구 · 방문요양 / 방문목욕"</p>
                    </div>
                    <div class="flex items-center gap-3">
                        <button class="relative p-2 text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded-lg transition-colors">
                            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
                            </svg>
                            <span class="absolute top-1.5 right-1.5 w-2 h-2 bg-red-500 rounded-full" />
                        </button>
                    </div>
                </header>

                // Page content
                <main class="flex-1 overflow-y-auto p-6">
                    {children()}
                </main>
            </div>
        </div>
    }
}
