use leptos::prelude::*;
use leptos_router::components::A;

// ---------------------------------------------------------------------------
// Top bar — greeting, notifications, logout
// ---------------------------------------------------------------------------

#[component]
fn TopBar() -> impl IntoView {
    view! {
        <header class="fixed top-0 left-0 right-0 z-40 bg-white border-b border-gray-100 shadow-sm">
            <div class="max-w-lg mx-auto flex items-center justify-between px-4 h-14">
                // App logo + name
                <A href="/" attr:class="flex items-center gap-2 min-h-[44px]" attr:aria-label="홈으로 이동">
                    <div class="w-8 h-8 rounded-lg bg-primary-600 flex items-center justify-center">
                        <svg class="w-5 h-5 text-white" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 14H9V8h2v8zm4 0h-2V8h2v8z" />
                        </svg>
                    </div>
                    <span class="text-lg font-bold text-primary-700">"시니어 포털"</span>
                </A>

                <div class="flex items-center gap-1">
                    // Notification bell
                    <A
                        href="/notifications"
                        attr:class="relative flex items-center justify-center w-10 h-10 rounded-full hover:bg-gray-100 transition-colors"
                        attr:aria-label="알림"
                    >
                        <svg class="w-6 h-6 text-gray-600" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
                        </svg>
                        <span class="absolute top-1.5 right-1.5 w-2.5 h-2.5 bg-red-500 rounded-full border border-white" />
                    </A>

                    // Logout button
                    <a
                        href="/auth/signin"
                        class="flex items-center justify-center w-10 h-10 rounded-full hover:bg-gray-100 transition-colors"
                        aria-label="로그아웃"
                    >
                        <svg class="w-5 h-5 text-gray-500" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15m3 0l3-3m0 0l-3-3m3 3H9" />
                        </svg>
                    </a>
                </div>
            </div>
        </header>
    }
}

// ---------------------------------------------------------------------------
// Bottom navigation — 5 primary destinations
// ---------------------------------------------------------------------------

#[component]
fn BottomNavItem(
    href: &'static str,
    label: &'static str,
    /// SVG path(s) for the icon (stroke style)
    icon_path: &'static str,
) -> impl IntoView {
    view! {
        <A
            href=href
            attr:class="flex-1 flex flex-col items-center py-2 gap-0.5 text-gray-400"
        >
            <svg class="w-7 h-7" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" aria-hidden="true">
                <path stroke-linecap="round" stroke-linejoin="round" d=icon_path />
            </svg>
            <span class="text-xs font-medium">{label}</span>
        </A>
    }
}

#[component]
fn BottomNavBar() -> impl IntoView {
    view! {
        <nav
            class="fixed bottom-0 left-0 right-0 z-40 bg-white border-t border-gray-200 shadow-[0_-2px_8px_rgba(0,0,0,0.06)]"
            aria-label="주요 메뉴"
        >
            <div class="max-w-lg mx-auto flex items-stretch">
                <BottomNavItem
                    href="/"
                    label="홈"
                    icon_path="M2.25 12l8.954-8.955a1.126 1.126 0 011.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25"
                />
                <BottomNavItem
                    href="/appointments"
                    label="예약"
                    icon_path="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 7.5v11.25m-18 0A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75m-18 0v-7.5A2.25 2.25 0 015.25 9h13.5A2.25 2.25 0 0121 11.25v7.5"
                />
                <BottomNavItem
                    href="/medications"
                    label="약"
                    icon_path="M9.75 3.104v5.714a2.25 2.25 0 01-.659 1.591L5 14.5M9.75 3.104c-.251.023-.501.05-.75.082m.75-.082a24.301 24.301 0 014.5 0m0 0v5.714c0 .597.237 1.17.659 1.591L19.8 15.3M14.25 3.104c.251.023.501.05.75.082M19.8 15.3l-1.57.393A9.065 9.065 0 0112 15a9.065 9.065 0 00-6.23.693L5 14.5m14.8.8l1.402 1.402c1.232 1.232.65 3.318-1.067 3.611A48.309 48.309 0 0112 21c-2.773 0-5.491-.235-8.135-.687-1.718-.293-2.3-2.379-1.067-3.61L5 14.5"
                />
                <BottomNavItem
                    href="/care"
                    label="돌봄"
                    icon_path="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12z"
                />
                <BottomNavItem
                    href="/more"
                    label="더보기"
                    icon_path="M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25A2.25 2.25 0 0118 10.5h-2.25a2.25 2.25 0 01-2.25-2.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-2.25A2.25 2.25 0 0113.5 18v-2.25z"
                />
            </div>
        </nav>
    }
}

// ---------------------------------------------------------------------------
// Emergency button — fixed bottom-right, always accessible
// ---------------------------------------------------------------------------

#[component]
fn EmergencyButton() -> impl IntoView {
    view! {
        <A
            href="/emergency"
            attr:class="fixed bottom-24 right-4 z-50
                   w-16 h-16 rounded-full
                   bg-red-500 hover:bg-red-700 active:bg-red-700
                   shadow-lg shadow-red-500/40
                   flex flex-col items-center justify-center gap-0.5
                   text-white font-bold text-xs
                   border-2 border-white
                   transition-transform duration-150 active:scale-95
                   select-none"
            attr:aria-label="긴급 연락"
        >
            <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                <path d="M12 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4.22 2.22a1 1 0 011.42 1.42l-.71.7a1 1 0 11-1.41-1.41l.7-.71zM3.34 5.64a1 1 0 011.42-1.42l.7.71A1 1 0 114.05 6.35l-.71-.71zM21 11h-1a1 1 0 110-2h1a1 1 0 110 2zM4 11H3a1 1 0 110-2h1a1 1 0 110 2zm14 6H6a2 2 0 010-4h12a2 2 0 010 4zm-1-6a5 5 0 00-10 0h10zm-5 8a2 2 0 100 4 2 2 0 000-4z" />
            </svg>
            <span>"긴급"</span>
        </A>
    }
}

// ---------------------------------------------------------------------------
// Senior layout — mobile-first with large text for accessibility
// ---------------------------------------------------------------------------

/// Senior portal layout with top bar, bottom nav, and emergency button.
/// Mobile-first design with large text (`text-scale-senior`) for accessibility.
#[component]
pub fn SeniorLayout(children: Children) -> impl IntoView {
    view! {
        <div class="text-scale-senior min-h-screen bg-gray-50">
            <TopBar />
            <main class="pt-14 pb-20" id="main-content">
                {children()}
            </main>
            <BottomNavBar />
            <EmergencyButton />
        </div>
    }
}
