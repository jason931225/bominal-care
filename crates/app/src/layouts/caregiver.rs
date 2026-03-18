use leptos::prelude::*;
use leptos_router::components::A;

// ---------------------------------------------------------------------------
// Bottom navigation — 5 primary destinations for caregiver
// ---------------------------------------------------------------------------

struct BottomNavItem {
    href: &'static str,
    label: &'static str,
    icon_path: &'static str,
}

const BOTTOM_NAV: &[BottomNavItem] = &[
    BottomNavItem {
        href: "/caregiver/schedule",
        label: "스케줄",
        icon_path: "M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z",
    },
    BottomNavItem {
        href: "/caregiver/clients",
        label: "고객",
        icon_path: "M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z",
    },
    BottomNavItem {
        href: "/caregiver/tasks",
        label: "업무",
        icon_path: "M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4",
    },
    BottomNavItem {
        href: "/caregiver/notifications",
        label: "알림",
        icon_path: "M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9",
    },
    BottomNavItem {
        href: "/caregiver/profile",
        label: "프로필",
        icon_path: "M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z",
    },
];

// ---------------------------------------------------------------------------
// Top bar
// ---------------------------------------------------------------------------

#[component]
fn TopBar() -> impl IntoView {
    view! {
        <header class="sticky top-0 z-40 bg-white border-b border-gray-200 shadow-sm">
            <div class="flex items-center justify-between px-4 h-14">
                // Logo
                <A href="/caregiver" attr:class="flex items-center gap-2">
                    <div class="w-8 h-8 bg-teal-600 rounded-lg flex items-center justify-center">
                        <svg class="w-5 h-5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                        </svg>
                    </div>
                    <span class="font-bold text-gray-900">"요양보호사 포털"</span>
                </A>

                // Right actions
                <div class="flex items-center gap-2">
                    <A
                        href="/caregiver/notifications"
                        attr:class="relative p-2 rounded-full text-gray-500 hover:bg-gray-100"
                    >
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
                        </svg>
                        <span class="absolute top-1 right-1 w-2 h-2 bg-red-500 rounded-full" />
                    </A>
                    <a
                        href="/auth/signin"
                        class="p-2 rounded-full text-gray-400 hover:text-gray-600 hover:bg-gray-100 transition-colors"
                        aria-label="로그아웃"
                    >
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
// Bottom navigation bar
// ---------------------------------------------------------------------------

#[component]
fn BottomNavBar() -> impl IntoView {
    view! {
        <nav
            class="fixed bottom-0 left-0 right-0 z-40 bg-white border-t border-gray-200 shadow-[0_-2px_8px_rgba(0,0,0,0.06)]"
            aria-label="주요 메뉴"
        >
            <div class="flex items-stretch">
                {BOTTOM_NAV.iter().map(|item| {
                    view! {
                        <A
                            href=item.href
                            attr:class="flex-1 flex flex-col items-center py-2 text-xs font-medium text-gray-400"
                        >
                            <svg class="w-6 h-6 mb-0.5" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" aria-hidden="true">
                                <path stroke-linecap="round" stroke-linejoin="round" d=item.icon_path />
                            </svg>
                            <span>{item.label}</span>
                        </A>
                    }
                }).collect_view()}
            </div>
        </nav>
    }
}

// ---------------------------------------------------------------------------
// Caregiver layout — mobile-first
// ---------------------------------------------------------------------------

/// Caregiver portal layout.
/// Mobile-first design with top bar and bottom nav (5 items).
#[component]
pub fn CaregiverLayout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50">
            <TopBar />
            <main class="pb-20" id="main-content">
                {children()}
            </main>
            <BottomNavBar />
        </div>
    }
}
