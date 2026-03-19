use leptos::prelude::*;

// =============================================================================
// Settings pages — settings, profile, notifications
// =============================================================================

/// Family portal settings with notification toggles and account management.
#[component]
pub fn SettingsPage() -> impl IntoView {
    let (push_enabled, set_push_enabled) = signal(true);
    let (email_enabled, set_email_enabled) = signal(true);

    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"설정"</h1>
                <p class="text-sm text-txt-secondary mt-1">"Bominal Family 설정을 관리하세요."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <h2 class="font-semibold text-txt-primary">"알림 설정"</h2>
                <div class="flex justify-between items-center">
                    <div>
                        <p class="text-sm font-medium text-txt-primary">"푸시 알림"</p>
                        <p class="text-xs text-txt-tertiary">"앱 푸시 알림을 받습니다."</p>
                    </div>
                    <button
                        class={move || format!("w-11 h-6 rounded-full transition-colors {}",
                            if push_enabled.get() { "bg-[var(--portal-accent)]" } else { "bg-gray-200" }
                        )}
                        on:click=move |_| set_push_enabled.update(|v| *v = !*v)
                    >
                        <span class={move || format!("block w-5 h-5 bg-surface-card rounded-full shadow transform transition-transform {}",
                            if push_enabled.get() { "translate-x-5" } else { "translate-x-0.5" }
                        )}></span>
                    </button>
                </div>
                <div class="flex justify-between items-center">
                    <div>
                        <p class="text-sm font-medium text-txt-primary">"이메일 알림"</p>
                        <p class="text-xs text-txt-tertiary">"이메일로 알림을 받습니다."</p>
                    </div>
                    <button
                        class={move || format!("w-11 h-6 rounded-full transition-colors {}",
                            if email_enabled.get() { "bg-[var(--portal-accent)]" } else { "bg-gray-200" }
                        )}
                        on:click=move |_| set_email_enabled.update(|v| *v = !*v)
                    >
                        <span class={move || format!("block w-5 h-5 bg-surface-card rounded-full shadow transform transition-transform {}",
                            if email_enabled.get() { "translate-x-5" } else { "translate-x-0.5" }
                        )}></span>
                    </button>
                </div>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <h2 class="font-semibold text-txt-primary">"계정"</h2>
                <a href="/family/profile" class="block text-sm text-[var(--portal-accent)] hover:underline">"프로필 관리 →"</a>
                <button
                    class="text-sm text-danger hover:underline"
                    on:click=move |_| {
                        leptos::task::spawn_local(async move {
                            let _ = crate::api::post_no_body("/api/auth/logout").await;
                            if let Some(window) = leptos::web_sys::window() {
                                let _ = window.location().set_href("/auth/signin");
                            }
                        });
                    }
                >"로그아웃"</button>
            </div>
        </div>
    }
}

/// Family member profile management page.
#[component]
pub fn ProfilePage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"내 프로필"</h1>
                <p class="text-sm text-txt-secondary mt-1">"가족 구성원 정보를 관리하세요."</p>
            </div>
            <div class="skeleton h-4 w-48"></div>
        </div>
    }
}

/// Displays recent notifications with a mark-all-read action.
#[component]
pub fn NotificationsPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-8">
            <div class="flex justify-between items-center">
                <div>
                    <h1 class="text-xl font-bold text-txt-primary">"알림"</h1>
                    <p class="text-sm text-txt-secondary mt-1">"최근 알림 내역입니다."</p>
                </div>
                <button class="text-sm text-[var(--portal-accent)] hover:underline">"모두 읽음"</button>
            </div>
            <div class="skeleton h-4 w-48"></div>
        </div>
    }
}
