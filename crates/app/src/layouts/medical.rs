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
        href: "/medical",
        label_key: "medical.nav.dashboard",
        icon_path: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6",
    },
    NavItem {
        href: "/medical/patients",
        label_key: "medical.nav.patients",
        icon_path: "M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z",
    },
    NavItem {
        href: "/medical/prescriptions",
        label_key: "medical.nav.prescriptions",
        icon_path: "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z",
    },
    NavItem {
        href: "/medical/appointments",
        label_key: "medical.nav.appointments",
        icon_path: "M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z",
    },
    NavItem {
        href: "/medical/history",
        label_key: "medical.nav.history",
        icon_path: "M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z",
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
// Medical layout — clinical sidebar layout with handoff indicator
// ---------------------------------------------------------------------------

/// Medical (clinician) portal layout.
///
/// Left sidebar (`w-60`) with nav items, a handoff session indicator bar
/// at the top, and user info at bottom. Uses `data-portal="medical"` to
/// activate the `--portal-accent` / `--portal-accent-light` CSS custom
/// properties (sky blue clinical color).
#[component]
pub fn MedicalLayout(children: Children) -> impl IntoView {
    view! {
        <div data-portal="medical" class="flex h-screen bg-gray-50">
            // Sidebar
            <aside class="w-60 flex-shrink-0 bg-white shadow-[2px_0_8px_rgba(0,0,0,0.04)] flex flex-col">
                // Logo
                <div class="h-16 flex items-center px-4 border-b border-gray-100">
                    <div class="flex items-center gap-2">
                        <div class="w-8 h-8 bg-[#0EA5E9] rounded-lg flex items-center justify-center">
                            <svg class="w-5 h-5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                            </svg>
                        </div>
                        <div>
                            <p class="text-sm font-bold text-gray-900">{t("nav.portal.medical")}</p>
                            <p class="text-xs text-gray-500">{t("medical.nav.subtitle")}</p>
                        </div>
                    </div>
                </div>

                // Handoff Session indicator
                <div class="mx-3 mt-3 px-3 py-2 bg-[#E0F2FE] rounded-xl">
                    <div class="flex items-center gap-2">
                        <div class="w-2 h-2 bg-[#0EA5E9] rounded-full animate-pulse"></div>
                        <p class="text-xs font-medium text-[#0284C7]">{t("medical.handoff.label")}</p>
                    </div>
                    <p class="text-xs text-[#0369A1] mt-0.5">{t("medical.handoff.no_patient")}</p>
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
                            <span class="text-xs font-semibold text-[var(--portal-accent)]">{t("medical.user.initial")}</span>
                        </div>
                        <div class="flex-1 min-w-0">
                            <p class="text-sm font-medium text-gray-900 truncate">{t("medical.user.name")}</p>
                            <p class="text-xs text-gray-500 truncate">{t("medical.user.role")}</p>
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
                        <h1 class="text-sm font-semibold text-gray-900">{t("medical.topbar.title")}</h1>
                        <p class="text-xs text-gray-500">{t("medical.topbar.subtitle")}</p>
                    </div>
                    <div class="flex items-center gap-3">
                        <a
                            href="/medical/patients"
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
