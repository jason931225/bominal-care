use leptos::prelude::*;
use leptos_router::components::A;

use crate::i18n::t;

// ---------------------------------------------------------------------------
// Bottom navigation — 5 primary destinations for caregiver portal
// ---------------------------------------------------------------------------

/// A single item in the caregiver bottom navigation bar.
struct BottomNavItem {
    href: &'static str,
    label: &'static str,
    icon_path: &'static str,
}

/// Bottom navigation destinations: schedule, clients, tasks, notifications, profile.
const BOTTOM_NAV: &[BottomNavItem] = &[
    BottomNavItem {
        href: "/caregiver/schedule",
        label: "nav.schedule",
        icon_path: "M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z",
    },
    BottomNavItem {
        href: "/caregiver/clients",
        label: "nav.clients",
        icon_path: "M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z",
    },
    BottomNavItem {
        href: "/caregiver/tasks",
        label: "nav.tasks",
        icon_path: "M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4",
    },
    BottomNavItem {
        href: "/caregiver/notifications",
        label: "common.notifications",
        icon_path: "M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9",
    },
    BottomNavItem {
        href: "/caregiver/profile",
        label: "common.profile",
        icon_path: "M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z",
    },
];

// ---------------------------------------------------------------------------
// Top bar — logo, notifications, logout
// ---------------------------------------------------------------------------

/// Sticky top bar with portal branding, notification bell, and logout button.
#[component]
fn TopBar() -> impl IntoView {
    view! {
        <header class="sticky top-0 z-40 bg-white shadow-sm">
            <div class="flex items-center justify-between px-4 h-14">
                // Logo
                <A href="/caregiver" attr:class="flex items-center gap-2">
                    <div class="w-8 h-8 bg-portal-caregiver rounded-lg flex items-center justify-center">
                        <svg class="w-5 h-5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                        </svg>
                    </div>
                    <span class="font-bold text-txt-primary">{t("nav.portal.caregiver")}</span>
                </A>

                // Right actions
                <div class="flex items-center gap-2">
                    <A
                        href="/caregiver/notifications"
                        attr:class="relative p-2 rounded-full text-txt-secondary hover:bg-surface-hover"
                    >
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
                        </svg>
                        <span class="absolute top-1 right-1 w-2 h-2 bg-red-500 rounded-full" />
                    </A>
                    <button
                        class="p-2 rounded-full text-txt-disabled hover:text-txt-secondary hover:bg-surface-hover transition-colors"
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
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15m3 0l3-3m0 0l-3-3m3 3H9" />
                        </svg>
                    </button>
                </div>
            </div>
        </header>
    }
}

// ---------------------------------------------------------------------------
// Bottom navigation bar
// ---------------------------------------------------------------------------

/// Fixed bottom navigation bar with 5 tabs.
/// Active tab shows the portal accent color with a dot indicator;
/// inactive tabs use `text-txt-disabled`.
#[component]
fn BottomNavBar() -> impl IntoView {
    view! {
        <nav
            class="fixed bottom-0 left-0 right-0 z-40 bg-white shadow-[0_-2px_8px_rgba(0,0,0,0.06)]"
            aria-label="주요 메뉴"
        >
            <div class="flex items-stretch">
                {BOTTOM_NAV.iter().map(|item| {
                    view! {
                        <A
                            href=item.href
                            attr:class="flex-1 flex flex-col items-center py-2 text-xs font-medium text-txt-disabled"
                        >
                            <svg class="w-6 h-6 mb-0.5" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" aria-hidden="true">
                                <path stroke-linecap="round" stroke-linejoin="round" d=item.icon_path />
                            </svg>
                            <span>{t(item.label)}</span>
                            // Active dot indicator
                            <span class="w-1 h-1 rounded-full bg-[var(--portal-accent)] mt-0.5 opacity-0 [[aria-current='page']>&]:opacity-100 transition-opacity" />
                        </A>
                    }
                }).collect_view()}
            </div>
        </nav>
    }
}

// ---------------------------------------------------------------------------
// Caregiver layout — mobile-first with design tokens
// ---------------------------------------------------------------------------

/// Caregiver portal layout.
///
/// Mobile-first design with sticky top bar and fixed bottom nav (5 items).
/// Sets `data-portal="caregiver"` on the root element to activate the
/// `--portal-accent` and `--portal-accent-light` CSS custom properties.
#[component]
pub fn CaregiverLayout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen bg-surface-page" data-portal="caregiver">
            <TopBar />
            <main class="pb-20 animate-fade-in" id="main-content">
                {children()}
            </main>
            <BottomNavBar />
        </div>
    }
}
