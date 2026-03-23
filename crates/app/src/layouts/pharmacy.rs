use leptos::prelude::*;
use leptos_router::components::A;

use crate::i18n::t;

// ---------------------------------------------------------------------------
// Navigation data
// ---------------------------------------------------------------------------

/// A single navigation item with route, display label, and SVG icon path.
struct NavItem {
    href: &'static str,
    label_key: &'static str,
    icon_path: &'static str,
}

const NAV_ITEMS: &[NavItem] = &[
    NavItem {
        href: "/pharmacy",
        label_key: "pharmacy.nav.dashboard",
        icon_path: "M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zm10 0a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zm10 0a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z",
    },
    NavItem {
        href: "/pharmacy/queue",
        label_key: "pharmacy.nav.queue",
        icon_path: "M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01",
    },
    NavItem {
        href: "/pharmacy/inventory",
        label_key: "pharmacy.nav.inventory",
        icon_path: "M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4",
    },
    NavItem {
        href: "/pharmacy/fulfillment",
        label_key: "pharmacy.nav.fulfillment",
        icon_path: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z",
    },
];

// ---------------------------------------------------------------------------
// Sidebar nav item
// ---------------------------------------------------------------------------

/// A single sidebar navigation link with icon and label.
///
/// Active state uses portal accent CSS variables for consistent theming.
#[component]
fn SidebarNavItem(
    href: &'static str,
    label_key: &'static str,
    icon_path: &'static str,
) -> impl IntoView {
    view! {
        <li>
            <A
                href=href
                attr:class="flex items-center gap-2 px-3 py-2 rounded-xl text-sm font-medium transition-colors text-gray-600 hover:bg-[var(--portal-accent-light)] hover:text-[var(--portal-accent)]"
            >
                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d=icon_path />
                </svg>
                <span>{t(label_key)}</span>
            </A>
        </li>
    }
}

// ---------------------------------------------------------------------------
// Pharmacy layout — sidebar layout for pharmacy staff
// ---------------------------------------------------------------------------

/// Pharmacy portal layout.
///
/// Left sidebar (`w-60`) with nav items and user info at bottom.
/// Uses `data-portal="pharmacy"` to activate the `--portal-accent` /
/// `--portal-accent-light` CSS custom properties (emerald green).
#[component]
pub fn PharmacyLayout(children: Children) -> impl IntoView {
    view! {
        <div data-portal="pharmacy" class="flex h-screen bg-gray-50">
            // Sidebar
            <aside class="w-60 flex-shrink-0 bg-white shadow-[2px_0_8px_rgba(0,0,0,0.04)] flex flex-col">
                // Logo
                <div class="h-16 flex items-center px-4 border-b border-gray-100">
                    <div class="flex items-center gap-2">
                        <div class="w-8 h-8 bg-[#059669] rounded-lg flex items-center justify-center">
                            <svg class="w-5 h-5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z" />
                            </svg>
                        </div>
                        <div>
                            <p class="text-sm font-bold text-gray-900">{t("nav.portal.pharmacy")}</p>
                            <p class="text-xs text-gray-500">{t("pharmacy.nav.subtitle")}</p>
                        </div>
                    </div>
                </div>

                // Navigation
                <nav class="flex-1 overflow-y-auto py-4 px-3">
                    <ul class="space-y-0.5">
                        {NAV_ITEMS.iter().map(|item| {
                            view! {
                                <SidebarNavItem
                                    href=item.href
                                    label_key=item.label_key
                                    icon_path=item.icon_path
                                />
                            }
                        }).collect_view()}
                    </ul>
                </nav>

                // User info
                <div class="p-4 border-t border-gray-100">
                    <div class="flex items-center gap-3">
                        <div class="w-8 h-8 bg-[var(--portal-accent-light)] rounded-full flex items-center justify-center">
                            <span class="text-xs font-semibold text-[var(--portal-accent)]">{t("pharmacy.user.initial")}</span>
                        </div>
                        <div class="flex-1 min-w-0">
                            <p class="text-sm font-medium text-gray-900 truncate">{t("pharmacy.user.name")}</p>
                            <p class="text-xs text-gray-500 truncate">{t("pharmacy.user.role")}</p>
                        </div>
                        <button
                            class="p-1.5 text-gray-400 hover:text-gray-600 hover:bg-gray-100 rounded-lg transition-colors min-w-[44px] min-h-[44px] flex items-center justify-center"
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
                // Top bar
                <header class="h-16 bg-white shadow-sm flex items-center justify-between px-6 flex-shrink-0">
                    <div>
                        <h1 class="text-sm font-semibold text-gray-900">{t("pharmacy.topbar.title")}</h1>
                        <p class="text-xs text-gray-500">{t("pharmacy.topbar.subtitle")}</p>
                    </div>
                    <div class="flex items-center gap-3">
                        <a
                            href="/pharmacy/queue"
                            class="relative p-2 text-[var(--portal-accent)] hover:text-gray-700 hover:bg-[var(--portal-accent-light)] rounded-lg transition-colors min-w-[44px] min-h-[44px] flex items-center justify-center"
                        >
                            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
                            </svg>
                        </a>
                    </div>
                </header>

                // Page content
                <main class="flex-1 overflow-y-auto p-6 bg-surface-page animate-fade-in">
                    {children()}
                </main>
            </div>
        </div>
    }
}
