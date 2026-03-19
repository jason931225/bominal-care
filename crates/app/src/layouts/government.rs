use leptos::prelude::*;
use leptos_router::components::A;

use crate::i18n::t;

// ---------------------------------------------------------------------------
// Navigation data — 7 items for government portal
// ---------------------------------------------------------------------------

/// A single navigation entry in the government sidebar.
struct NavItem {
    href: &'static str,
    label: &'static str,
    icon_path: &'static str,
}

/// All navigation items displayed in the government portal sidebar.
const NAV_ITEMS: &[NavItem] = &[
    NavItem { href: "/gov", label: "대시보드", icon_path: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" },
    NavItem { href: "/gov/providers", label: "기관", icon_path: "M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" },
    NavItem { href: "/gov/programs", label: "프로그램", icon_path: "M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" },
    NavItem { href: "/gov/eligibility", label: "수급 자격", icon_path: "M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z" },
    NavItem { href: "/gov/audit", label: "감사 로그", icon_path: "M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01" },
    NavItem { href: "/gov/observability", label: "모니터링", icon_path: "M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" },
    NavItem { href: "/gov/settings", label: "설정", icon_path: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" },
];

// ---------------------------------------------------------------------------
// Government layout — purple/violet color scheme, left sidebar
// ---------------------------------------------------------------------------

/// Government portal layout with purple/violet accent (`portal-government`).
///
/// Uses CSS custom properties `--portal-accent` and `--portal-accent-light`
/// scoped via `data-portal="government"` on the root element.
///
/// Structure:
/// - Left sidebar (w-60) with 7 nav items, logo, and user info
/// - Top bar showing district/department context
/// - Scrollable main content area with fade-in animation
#[component]
pub fn GovernmentLayout(children: Children) -> impl IntoView {
    view! {
        <div data-portal="government" class="flex h-screen bg-surface-page">
            // Sidebar
            <aside class="w-60 flex-shrink-0 bg-white shadow-md flex flex-col">
                // Logo
                <div class="h-16 flex items-center px-4">
                    <div class="flex items-center gap-2">
                        <div class="w-8 h-8 bg-portal-government rounded-lg flex items-center justify-center">
                            <svg class="w-5 h-5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M3 6l3 1m0 0l-3 9a5.002 5.002 0 006.001 0M6 7l3 9M6 7l6-2m6 2l3-1m-3 1l-3 9a5.002 5.002 0 006.001 0M18 7l3 9m-3-9l-6-2m0-2v2m0 16V5m0 16H9m3 0h3" />
                            </svg>
                        </div>
                        <div>
                            <p class="text-sm font-bold text-gray-900">{t("nav.portal.government")}</p>
                            <p class="text-xs text-gray-500">{t("nav.gov_portal")}</p>
                        </div>
                    </div>
                </div>

                // Navigation
                <nav class="flex-1 overflow-y-auto py-4 px-3">
                    <p class="px-3 mb-2 text-xs font-semibold text-gray-400 uppercase tracking-wider">"메뉴"</p>
                    <ul class="space-y-0.5">
                        {NAV_ITEMS.iter().map(|item| {
                            view! {
                                <li>
                                    <A
                                        href=item.href
                                        attr:class="flex items-center gap-2 px-3 py-2 rounded-xl text-sm font-medium transition-colors text-gray-600 hover:bg-[var(--portal-accent-light)] hover:text-[var(--portal-accent)]"
                                    >
                                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d=item.icon_path />
                                        </svg>
                                        <span>{item.label}</span>
                                    </A>
                                </li>
                            }
                        }).collect_view()}
                    </ul>
                </nav>

                // User info
                <div class="p-4">
                    <div class="flex items-center gap-3">
                        <div class="w-8 h-8 bg-[var(--portal-accent-light)] rounded-full flex items-center justify-center">
                            <span class="text-xs font-semibold text-[var(--portal-accent)]">"담"</span>
                        </div>
                        <div class="flex-1 min-w-0">
                            <p class="text-sm font-medium text-gray-900 truncate">"담당자"</p>
                            <p class="text-xs text-gray-500 truncate">"노인복지팀장"</p>
                        </div>
                        <button
                            class="p-1.5 text-gray-400 hover:text-gray-600 hover:bg-gray-100 rounded-lg transition-colors"
                            aria-label=t("common.logout")
                            on:click=move |_| {
                                leptos::task::spawn_local(async move {
                                    let _ = crate::api::post_no_body("/api/auth/logout").await;
                                    if let Some(window) = leptos::web_sys::window() {
                                        let _ = window.location().set_href("/auth/signin");
                                    }
                                });
                            }
                        >
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15m3 0l3-3m0 0l-3-3m3 3H9" />
                            </svg>
                        </button>
                    </div>
                </div>
            </aside>

            // Main area
            <div class="flex-1 flex flex-col overflow-hidden">
                // Top bar with district/department info
                <header class="h-16 bg-white shadow-sm flex items-center justify-between px-6 flex-shrink-0">
                    <div class="flex items-center gap-3">
                        <div class="flex items-center gap-2">
                            <div class="w-2 h-2 bg-[var(--portal-accent)] rounded-full" />
                            <span class="text-sm font-semibold text-gray-900">"서울특별시 강남구청"</span>
                        </div>
                        <span class="text-gray-300">"|"</span>
                        <span class="text-sm text-gray-500">"노인복지과 장기요양팀"</span>
                    </div>
                    <div class="flex items-center gap-3">
                        <a href="/gov/audit" class="p-2 text-gray-500 hover:text-[var(--portal-accent)] hover:bg-[var(--portal-accent-light)] rounded-lg transition-colors">
                            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
                            </svg>
                        </a>
                    </div>
                </header>

                // Page content
                <main class="flex-1 overflow-y-auto p-6 animate-fade-in bg-surface-page">
                    {children()}
                </main>
            </div>
        </div>
    }
}
