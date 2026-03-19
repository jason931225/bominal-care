use leptos::prelude::*;

use bominal_types::Notification;
use crate::components::data_display::EmptyState;
use crate::components::layout::PageHeader;

/// Notification list with read/unread status.
#[component]
pub fn NotificationsPage() -> impl IntoView {
    let notifications = LocalResource::new(|| {
        crate::api::get::<Vec<Notification>>("/api/notifications?page=1&limit=50")
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="알림" />
            <Suspense fallback=move || view! { <div class="skeleton h-8 w-20"></div> }>
                {move || Suspend::new(async move {
                    match notifications.await {
                        Ok(resp) if resp.success => {
                            let items = resp.data.unwrap_or_default();
                            if items.is_empty() {
                                view! { <EmptyState message="알림이 없습니다." /> }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-3">
                                        {items.into_iter().map(|notif| {
                                            let border_class = if notif.is_read {
                                                ""
                                            } else {
                                                "border border-primary bg-primary-light/30"
                                            };
                                            let type_class = match format!("{}", notif.notification_type).as_str() {
                                                "WARNING" | "ALERT" => "bg-warning-light text-warning",
                                                "ACTION_REQUIRED" => "bg-danger-light text-danger",
                                                _ => "bg-surface-subtle text-txt-secondary",
                                            };
                                            view! {
                                                <div class={format!("bg-surface-card rounded-2xl p-5 shadow-sm {border_class}")}>
                                                    <div class="flex items-center justify-between mb-1">
                                                        <p class="text-lg font-medium text-txt-primary">{notif.title}</p>
                                                        <span class={format!("text-xs px-2 py-1 rounded-full {type_class}")}>
                                                            {format!("{}", notif.notification_type)}
                                                        </span>
                                                    </div>
                                                    <p class="text-base text-txt-secondary">{notif.message}</p>
                                                    <p class="text-sm text-txt-disabled mt-2">{crate::api::format_datetime_kr(&notif.created_at)}</p>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            }
                        }
                        Ok(resp) => view! { <p class="text-danger">{resp.error.unwrap_or_default()}</p> }.into_any(),
                        Err(e) => view! { <p class="text-danger">{e}</p> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

/// Settings menu.
#[component]
pub fn SettingsPage() -> impl IntoView {
    let menu_items = vec![
        ("/profile", "내 프로필", "개인정보 수정"),
        ("/consent", "동의 관리", "데이터 공유 설정"),
        ("/notifications", "알림 설정", "알림 수신 관리"),
    ];

    let on_logout = move |_: leptos::ev::MouseEvent| {
        leptos::task::spawn_local(async move {
            let _ = crate::api::post_no_body("/api/auth/logout").await;
            if let Some(window) = leptos::web_sys::window() {
                let _ = window.location().set_href("/auth/signin");
            }
        });
    };

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="설정" />
            <div class="space-y-2">
                {menu_items.into_iter().map(|(href, title, desc)| {
                    view! {
                        <a href=href
                           class="flex items-center justify-between bg-surface-card rounded-2xl p-5 \
                                  shadow-sm hover:shadow-md transition-shadow duration-200">
                            <div>
                                <p class="text-lg font-medium text-txt-primary">{title}</p>
                                <p class="text-base text-txt-tertiary">{desc}</p>
                            </div>
                            <span class="text-txt-disabled text-xl">">"</span>
                        </a>
                    }
                }).collect_view()}
                <button
                    on:click=on_logout
                    class="w-full flex items-center justify-between bg-surface-card rounded-2xl p-5 \
                           shadow-sm hover:shadow-md transition-shadow duration-200"
                >
                    <div class="text-left">
                        <p class="text-lg font-medium text-danger">"로그아웃"</p>
                        <p class="text-base text-txt-tertiary">"안전하게 로그아웃"</p>
                    </div>
                    <span class="text-txt-disabled text-xl">">"</span>
                </button>
            </div>
        </div>
    }
}

/// Additional menu items (More page).
#[component]
pub fn MorePage() -> impl IntoView {
    let sections = vec![
        ("건강", vec![
            ("/medications", "약물 관리", "\u{1f48a}"),
            ("/medication-log", "복약 기록", "\u{1f4cb}"),
            ("/medical-history", "병력 기록", "\u{1f3e5}"),
        ]),
        ("돌봄", vec![
            ("/care", "케어 플랜", "\u{1f49c}"),
            ("/appointments", "진료 예약", "\u{1f4c5}"),
        ]),
        ("생활", vec![
            ("/services", "서비스", "\u{1f527}"),
            ("/housing", "주거 서비스", "\u{1f3e0}"),
            ("/opportunities", "사회 참여", "\u{1f31f}"),
        ]),
        ("계정", vec![
            ("/profile", "내 프로필", "\u{1f464}"),
            ("/consent", "동의 관리", "\u{1f512}"),
            ("/emergency", "긴급 연락", "\u{1f6a8}"),
            ("/settings", "설정", "\u{2699}\u{fe0f}"),
        ]),
    ];

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-8">
            <PageHeader title="더보기" />
            {sections.into_iter().map(|(section_title, items)| {
                view! {
                    <section>
                        <h2 class="text-lg font-semibold text-txt-tertiary mb-2">{section_title}</h2>
                        <div class="space-y-1">
                            {items.into_iter().map(|(href, label, icon)| {
                                view! {
                                    <a href=href
                                       class="flex items-center gap-4 bg-surface-card rounded-2xl px-5 py-4 \
                                              shadow-sm hover:shadow-md transition-shadow duration-200">
                                        <span class="text-2xl">{icon}</span>
                                        <span class="text-lg font-medium text-txt-primary">{label}</span>
                                    </a>
                                }
                            }).collect_view()}
                        </div>
                    </section>
                }
            }).collect_view()}
        </div>
    }
}
