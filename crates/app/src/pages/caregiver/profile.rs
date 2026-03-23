use chrono::Datelike;
use leptos::prelude::*;
use bominal_types::{Notification, PersonProfile, CaregiverCredential};

// =============================================================================
// 14. NotificationsPage — notification list from API
// =============================================================================

#[component]
pub fn NotificationsPage() -> impl IntoView {
    let notifications = LocalResource::new(|| {
        crate::api::get::<Vec<Notification>>("/api/notifications")
    });
    let mark_all_read = RwSignal::new(false);

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <div class="flex items-center justify-between">
                <h1 class="text-xl font-bold text-gray-900">"알림"</h1>
                <button
                    class="text-sm text-teal-600 font-medium hover:text-teal-700"
                    on:click=move |_| {
                        mark_all_read.set(true);
                        leptos::task::spawn_local(async move {
                            let _ = crate::api::post::<serde_json::Value, _>(
                                "/api/notifications/mark-all-read",
                                &serde_json::json!({}),
                            ).await;
                        });
                    }
                >"모두 읽음"</button>
            </div>

            <Suspense fallback=move || view! {
                <div class="animate-pulse space-y-2">
                    <div class="bg-gray-200 rounded-xl h-16" />
                    <div class="bg-gray-200 rounded-xl h-16" />
                    <div class="bg-gray-200 rounded-xl h-16" />
                </div>
            }>
                {move || Suspend::new(async move {
                    match notifications.await {
                        Ok(resp) if resp.success => {
                            let items = resp.data.unwrap_or_default();
                            if items.is_empty() {
                                view! {
                                    <p class="text-center text-gray-500 py-8">"알림이 없습니다."</p>
                                }.into_any()
                            } else {
                                let all_read = mark_all_read.get();
                                view! {
                                    <div class="space-y-2">
                                        {items.into_iter().map(|n| {
                                            let title = n.title.clone();
                                            let body = n.message.clone();
                                            let time = format_relative_time(&n.created_at);
                                            let unread = if all_read { false } else { !n.is_read };
                                            let icon_type = notification_icon_type(&n.notification_type);
                                            view! {
                                                <NotificationItem
                                                    title=title
                                                    body=body
                                                    time=time
                                                    unread=unread
                                                    icon_type=icon_type
                                                />
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            }
                        }
                        _ => view! {
                            <p class="text-center text-gray-500 py-8">"알림을 불러올 수 없습니다."</p>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

/// Map NotificationType to an icon type string.
fn notification_icon_type(nt: &bominal_types::NotificationType) -> String {
    match nt {
        bominal_types::NotificationType::Reminder => "schedule".to_string(),
        bominal_types::NotificationType::Warning => "medication".to_string(),
        bominal_types::NotificationType::Alert | bominal_types::NotificationType::Emergency => "care".to_string(),
        bominal_types::NotificationType::ActionRequired => "care".to_string(),
        _ => "info".to_string(),
    }
}

/// Format a datetime as relative time in Korean.
fn format_relative_time(dt: &chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let diff = now - *dt;
    let mins = diff.num_minutes();
    if mins < 1 {
        "방금 전".to_string()
    } else if mins < 60 {
        format!("{}분 전", mins)
    } else if mins < 1440 {
        format!("{}시간 전", mins / 60)
    } else if mins < 10080 {
        format!("{}일 전", mins / 1440)
    } else {
        crate::api::format_date_kr(dt)
    }
}

#[component]
fn NotificationItem(
    #[prop(into)] title: String,
    #[prop(into)] body: String,
    #[prop(into)] time: String,
    unread: bool,
    #[prop(into)] icon_type: String,
) -> impl IntoView {
    let bg = if unread { "bg-teal-50 border-teal-100" } else { "bg-white border-gray-100" };
    let icon_bg = match icon_type.as_str() {
        "schedule" => "bg-blue-100 text-blue-600",
        "medication" => "bg-red-100 text-red-600",
        "care" => "bg-green-100 text-green-600",
        _ => "bg-gray-100 text-gray-600",
    };
    let icon_path = match icon_type.as_str() {
        "schedule" => "M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z",
        "medication" => "M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z",
        "care" => "M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z",
        _ => "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
    };

    view! {
        <div class={format!("rounded-xl p-4 border {bg}")}>
            <div class="flex gap-3">
                <div class={format!("w-9 h-9 rounded-full flex items-center justify-center shrink-0 {icon_bg}")}>
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d=icon_path />
                    </svg>
                </div>
                <div class="flex-1 min-w-0">
                    <div class="flex items-center justify-between mb-0.5">
                        <p class="text-sm font-semibold text-gray-900">{title}</p>
                        <span class="text-xs text-gray-400 shrink-0">{time}</span>
                    </div>
                    <p class="text-sm text-gray-600">{body}</p>
                </div>
            </div>
        </div>
    }
}

// =============================================================================
// 15. ProfilePage — caregiver profile from API
// =============================================================================

#[component]
pub fn ProfilePage() -> impl IntoView {
    let profile = LocalResource::new(|| {
        crate::api::get::<PersonProfile>("/api/profile/me")
    });
    let credentials = LocalResource::new(|| {
        crate::api::get::<Vec<CaregiverCredential>>("/api/credentials")
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <h1 class="text-xl font-bold text-gray-900">"내 프로필"</h1>

            // Profile card — from API
            <Suspense fallback=move || view! {
                <div class="animate-pulse space-y-4">
                    <div class="bg-gray-200 rounded-xl h-40" />
                    <div class="bg-gray-200 rounded-xl h-24" />
                </div>
            }>
                {move || Suspend::new(async move {
                    match profile.await {
                        Ok(resp) if resp.success => {
                            match resp.data {
                                Some(p) => {
                                    let name = p.korean_name.clone().unwrap_or_else(|| "이름 없음".to_string());
                                    let initial = name.chars().next().unwrap_or('?').to_string();
                                    let phone = p.phone.as_deref().map(crate::api::format_phone_kr).unwrap_or_else(|| "등록되지 않음".to_string());
                                    let address = build_profile_address(&p);

                                    view! {
                                        <div class="space-y-5">
                                            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 text-center">
                                                <div class="w-20 h-20 bg-teal-100 rounded-full flex items-center justify-center mx-auto mb-3">
                                                    <span class="text-3xl font-bold text-teal-700">{initial}</span>
                                                </div>
                                                <h2 class="text-lg font-bold text-gray-900">{name}</h2>
                                                <p class="text-sm text-gray-500">"요양보호사"</p>
                                                <span class="inline-block mt-2 px-3 py-1 bg-green-100 text-green-700 text-xs font-medium rounded-full">"활동 중"</span>
                                            </div>

                                            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                                <h3 class="font-semibold text-gray-900 mb-3">"연락처 정보"</h3>
                                                <dl class="space-y-2 text-sm">
                                                    <div class="flex justify-between"><dt class="text-gray-500">"전화번호"</dt><dd class="font-medium text-gray-900">{phone}</dd></div>
                                                    <div class="flex justify-between"><dt class="text-gray-500">"주소"</dt><dd class="font-medium text-gray-900">{address}</dd></div>
                                                </dl>
                                            </div>
                                        </div>
                                    }.into_any()
                                }
                                None => {
                                    // Fallback: use auth context name
                                    let auth = crate::use_auth();
                                    let user_name = auth.get().map(|u| u.name.clone()).unwrap_or_else(|| "사용자".to_string());
                                    let user_email = auth.get().map(|u| u.email.clone()).unwrap_or_default();
                                    let initial = user_name.chars().next().unwrap_or('?').to_string();

                                    view! {
                                        <div class="space-y-5">
                                            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 text-center">
                                                <div class="w-20 h-20 bg-teal-100 rounded-full flex items-center justify-center mx-auto mb-3">
                                                    <span class="text-3xl font-bold text-teal-700">{initial}</span>
                                                </div>
                                                <h2 class="text-lg font-bold text-gray-900">{user_name}</h2>
                                                <p class="text-sm text-gray-500">"요양보호사"</p>
                                                <span class="inline-block mt-2 px-3 py-1 bg-green-100 text-green-700 text-xs font-medium rounded-full">"활동 중"</span>
                                            </div>

                                            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                                <h3 class="font-semibold text-gray-900 mb-3">"연락처 정보"</h3>
                                                <dl class="space-y-2 text-sm">
                                                    {if !user_email.is_empty() {
                                                        view! {
                                                            <div class="flex justify-between"><dt class="text-gray-500">"이메일"</dt><dd class="font-medium text-gray-900">{user_email}</dd></div>
                                                        }.into_any()
                                                    } else {
                                                        view! { <div></div> }.into_any()
                                                    }}
                                                </dl>
                                            </div>
                                        </div>
                                    }.into_any()
                                }
                            }
                        }
                        _ => {
                            // Fallback: use auth context
                            let auth = crate::use_auth();
                            let user_name = auth.get().map(|u| u.name.clone()).unwrap_or_else(|| "사용자".to_string());
                            let initial = user_name.chars().next().unwrap_or('?').to_string();
                            view! {
                                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 text-center">
                                    <div class="w-20 h-20 bg-teal-100 rounded-full flex items-center justify-center mx-auto mb-3">
                                        <span class="text-3xl font-bold text-teal-700">{initial}</span>
                                    </div>
                                    <h2 class="text-lg font-bold text-gray-900">{user_name}</h2>
                                    <p class="text-sm text-gray-500">"요양보호사"</p>
                                </div>
                            }.into_any()
                        }
                    }
                })}
            </Suspense>

            // Credentials — from API
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h3 class="font-semibold text-gray-900 mb-3">"자격 및 인증"</h3>
                <Suspense fallback=move || view! {
                    <p class="text-sm text-gray-400">"자격 정보를 불러오는 중..."</p>
                }>
                    {move || Suspend::new(async move {
                        match credentials.await {
                            Ok(resp) if resp.success => {
                                let items = resp.data.unwrap_or_default();
                                if items.is_empty() {
                                    view! {
                                        <p class="text-sm text-gray-500">"등록된 자격 정보가 없습니다."</p>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="space-y-3">
                                            {items.into_iter().map(|cred| {
                                                let name = format!("{}", cred.credential_type);
                                                let issuer = cred.issuer.unwrap_or_else(|| "발급기관 미등록".to_string());
                                                let expires = cred.expires_at
                                                    .map(|dt| format!("{}.{:02}.{:02}", dt.year(), dt.month(), dt.day()))
                                                    .unwrap_or_else(|| "만료일 없음".to_string());
                                                let valid = cred.expires_at
                                                    .map(|dt| dt > chrono::Utc::now())
                                                    .unwrap_or(true);
                                                view! {
                                                    <CredentialItem name=name issuer=issuer expires=expires valid=valid />
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_any()
                                }
                            }
                            _ => view! {
                                <p class="text-sm text-gray-500">"자격 정보를 불러올 수 없습니다."</p>
                            }.into_any(),
                        }
                    })}
                </Suspense>
            </div>

            // Links
            <div class="grid grid-cols-2 gap-3">
                <a href="/caregiver/profile/availability" class="bg-white rounded-xl p-4 shadow-sm border border-gray-100 text-center hover:shadow-md">
                    <p class="text-sm font-medium text-gray-900">"근무 가능 시간"</p>
                </a>
                <a href="/caregiver/settings" class="bg-white rounded-xl p-4 shadow-sm border border-gray-100 text-center hover:shadow-md">
                    <p class="text-sm font-medium text-gray-900">"설정"</p>
                </a>
            </div>
        </div>
    }
}

/// Build a display address from PersonProfile fields.
fn build_profile_address(p: &PersonProfile) -> String {
    let parts: Vec<&str> = [
        p.city.as_deref(),
        p.district.as_deref(),
        p.address.as_deref(),
    ]
    .iter()
    .filter_map(|&s| s)
    .collect();
    if parts.is_empty() {
        "주소 미등록".to_string()
    } else {
        parts.join(" ")
    }
}

#[component]
fn CredentialItem(
    #[prop(into)] name: String,
    #[prop(into)] issuer: String,
    #[prop(into)] expires: String,
    valid: bool,
) -> impl IntoView {
    let badge = if valid {
        ("bg-green-100 text-green-700", "유효")
    } else {
        ("bg-red-100 text-red-700", "만료")
    };

    view! {
        <div class="flex items-center justify-between py-2 border-b border-gray-50 last:border-0">
            <div>
                <p class="text-sm font-medium text-gray-900">{name}</p>
                <p class="text-xs text-gray-500">{issuer}" · 만료: "{expires}</p>
            </div>
            <span class={format!("text-xs font-medium px-2 py-0.5 rounded-full {}", badge.0)}>{badge.1}</span>
        </div>
    }
}

// =============================================================================
// 16. ProfileAvailabilityPage — API-driven weekly availability with exceptions
// =============================================================================

/// Day label + key used for the weekly grid.
const DAYS: [(&str, &str); 7] = [
    ("월요일", "monday"),
    ("화요일", "tuesday"),
    ("수요일", "wednesday"),
    ("목요일", "thursday"),
    ("금요일", "friday"),
    ("토요일", "saturday"),
    ("일요일", "sunday"),
];

/// Generate time options from 09:00 to 21:00 in 30-min steps.
fn time_options() -> Vec<String> {
    let mut opts = Vec::with_capacity(25);
    let mut hour = 9;
    let mut min = 0;
    while hour < 21 || (hour == 21 && min == 0) {
        opts.push(format!("{:02}:{:02}", hour, min));
        min += 30;
        if min >= 60 {
            min = 0;
            hour += 1;
        }
    }
    opts
}

/// Parse a slot from the API JSON for a given day key.
fn parse_slot_for_day(
    slots: &[serde_json::Value],
    day_key: &str,
) -> (bool, String, String) {
    for slot in slots {
        let day = slot.get("day_of_week")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if day.to_lowercase() == day_key {
            let start = slot.get("start_time")
                .and_then(|v| v.as_str())
                .unwrap_or("09:00")
                .to_string();
            let end = slot.get("end_time")
                .and_then(|v| v.as_str())
                .unwrap_or("18:00")
                .to_string();
            return (true, start, end);
        }
    }
    (false, "09:00".to_string(), "18:00".to_string())
}

#[component]
pub fn ProfileAvailabilityPage() -> impl IntoView {
    let saved_msg = RwSignal::new(None::<String>);
    let save_error = RwSignal::new(None::<String>);

    let availability = LocalResource::new(|| {
        crate::api::get::<Vec<serde_json::Value>>("/api/availability")
    });
    let exceptions = LocalResource::new(|| {
        crate::api::get::<Vec<serde_json::Value>>("/api/availability/exceptions")
    });

    // Signals for each day: (enabled, start, end)
    let day_signals: Vec<(&str, &str, RwSignal<bool>, RwSignal<String>, RwSignal<String>)> =
        DAYS.iter()
            .map(|(label, key)| {
                (
                    *label,
                    *key,
                    RwSignal::new(false),
                    RwSignal::new("09:00".to_string()),
                    RwSignal::new("18:00".to_string()),
                )
            })
            .collect();

    // Clone signals for populating from API
    let day_sigs_for_init: Vec<(&str, RwSignal<bool>, RwSignal<String>, RwSignal<String>)> =
        day_signals.iter().map(|(_, k, e, s, en)| (*k, *e, *s, *en)).collect();

    // Exception form state
    let show_exception_form = RwSignal::new(false);
    let exception_date = RwSignal::new(String::new());
    let exception_reason = RwSignal::new(String::new());
    let exception_error = RwSignal::new(None::<String>);

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <div class="flex items-center gap-3">
                <a href="/caregiver/profile" class="p-2 rounded-lg hover:bg-gray-100">
                    <svg class="w-5 h-5 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
                    </svg>
                </a>
                <h1 class="text-xl font-bold text-gray-900">"근무 가능 시간"</h1>
            </div>

            <p class="text-sm text-gray-600">"근무 가능한 요일과 시간을 설정해주세요."</p>

            // Weekly grid — populated from API
            <Suspense fallback=move || view! {
                <div class="animate-pulse space-y-2">
                    <div class="bg-gray-200 rounded-xl h-16" />
                    <div class="bg-gray-200 rounded-xl h-16" />
                    <div class="bg-gray-200 rounded-xl h-16" />
                </div>
            }>
                {
                    let sigs = day_sigs_for_init.clone();
                    move || Suspend::new({
                        let sigs = sigs.clone();
                        async move {
                            let slots: Vec<serde_json::Value> = availability.await
                                .ok()
                                .and_then(|r| if r.success { r.data } else { None })
                                .unwrap_or_default();
                            // Populate signals from API
                            for (key, enabled_sig, start_sig, end_sig) in &sigs {
                                let (on, start, end) = parse_slot_for_day(&slots, key);
                                enabled_sig.set(on);
                                start_sig.set(start);
                                end_sig.set(end);
                            }
                            view! { <div></div> }
                        }
                    })
                }
            </Suspense>

            <div class="space-y-3">
                {day_signals.iter().map(|(label, _key, enabled, start, end_time)| {
                    let is_on = *enabled;
                    let start_val = *start;
                    let end_val = *end_time;
                    let label = label.to_string();
                    let opts = time_options();
                    let start_opts = opts.clone();
                    let end_opts = opts;
                    view! {
                        <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
                            <div class="flex items-center justify-between">
                                <div class="flex items-center gap-3">
                                    <button
                                        class="w-10 h-6 rounded-full transition-colors relative"
                                        class=("bg-teal-600", move || is_on.get())
                                        class=("bg-gray-300", move || !is_on.get())
                                        on:click=move |_| is_on.update(|v| *v = !*v)
                                    >
                                        <span
                                            class="absolute top-0.5 w-5 h-5 bg-white rounded-full shadow transition-transform"
                                            class=("left-[1.125rem]", move || is_on.get())
                                            class=("left-0.5", move || !is_on.get())
                                        />
                                    </button>
                                    <span class="font-medium text-gray-900">{label}</span>
                                </div>
                                <Show when=move || is_on.get()>
                                    <div class="flex items-center gap-1 text-sm">
                                        <select
                                            class="px-2 py-1 border border-gray-300 rounded-lg text-sm"
                                            on:change=move |ev| {
                                                start_val.set(event_target_value(&ev));
                                            }
                                        >
                                            {start_opts.iter().map(|t| {
                                                let val = t.clone();
                                                let cmp = t.clone();
                                                let txt = t.clone();
                                                view! {
                                                    <option value={val} selected=move || start_val.get() == cmp>{txt}</option>
                                                }
                                            }).collect_view()}
                                        </select>
                                        <span class="text-gray-600">"~"</span>
                                        <select
                                            class="px-2 py-1 border border-gray-300 rounded-lg text-sm"
                                            on:change=move |ev| {
                                                end_val.set(event_target_value(&ev));
                                            }
                                        >
                                            {end_opts.iter().map(|t| {
                                                let val = t.clone();
                                                let cmp = t.clone();
                                                let txt = t.clone();
                                                view! {
                                                    <option value={val} selected=move || end_val.get() == cmp>{txt}</option>
                                                }
                                            }).collect_view()}
                                        </select>
                                    </div>
                                </Show>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>

            // Save feedback
            {move || saved_msg.get().map(|msg| view! {
                <p class="text-sm text-green-600 text-center">{msg}</p>
            })}
            {move || save_error.get().map(|msg| view! {
                <p class="text-sm text-red-600 text-center">{msg}</p>
            })}

            <button
                class="w-full py-3 bg-teal-600 text-white font-semibold rounded-xl hover:bg-teal-700"
                on:click={
                    let ds = day_signals.iter()
                        .map(|(_, key, en, st, ed)| (*key, *en, *st, *ed))
                        .collect::<Vec<_>>();
                    move |_| {
                        let slots: Vec<serde_json::Value> = ds.iter()
                            .filter(|(_, en, _, _)| en.get_untracked())
                            .map(|(key, _, st, ed)| {
                                serde_json::json!({
                                    "day_of_week": key,
                                    "start_time": st.get_untracked(),
                                    "end_time": ed.get_untracked()
                                })
                            })
                            .collect();
                        let body = serde_json::json!({ "slots": slots });
                        let saved = saved_msg;
                        let err = save_error;
                        leptos::task::spawn_local(async move {
                            match crate::api::put::<serde_json::Value, _>(
                                "/api/availability", &body
                            ).await {
                                Ok(resp) if resp.success => {
                                    err.set(None);
                                    saved.set(Some("저장되었습니다".to_string()));
                                }
                                Ok(resp) => {
                                    saved.set(None);
                                    err.set(resp.error);
                                }
                                Err(e) => {
                                    saved.set(None);
                                    err.set(Some(e));
                                }
                            }
                        });
                    }
                }
            >"저장"</button>

            // Exceptions section
            <div class="space-y-3 pt-4 border-t border-gray-200">
                <h2 class="text-lg font-semibold text-gray-900">"일정 변경"</h2>

                <Suspense fallback=move || view! {
                    <p class="text-sm text-gray-400">"일정 변경 내역을 불러오는 중..."</p>
                }>
                    {move || Suspend::new(async move {
                        match exceptions.await {
                            Ok(resp) if resp.success => {
                                let items = resp.data.unwrap_or_default();
                                if items.is_empty() {
                                    view! {
                                        <p class="text-sm text-gray-500">"등록된 일정 변경이 없습니다."</p>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="space-y-2">
                                            {items.into_iter().map(|exc| {
                                                let exc_id = exc.get("id")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("")
                                                    .to_string();
                                                let date = exc.get("date")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("")
                                                    .to_string();
                                                let reason = exc.get("reason")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("")
                                                    .to_string();
                                                let is_available = exc.get("is_available")
                                                    .and_then(|v| v.as_bool())
                                                    .unwrap_or(false);
                                                let badge = if is_available {
                                                    ("bg-green-100 text-green-700", "추가 가능")
                                                } else {
                                                    ("bg-red-100 text-red-700", "차단됨")
                                                };
                                                let del_id = exc_id.clone();
                                                view! {
                                                    <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100 flex items-center justify-between">
                                                        <div>
                                                            <div class="flex items-center gap-2">
                                                                <p class="text-sm font-medium text-gray-900">{date}</p>
                                                                <span class={format!("text-xs px-2 py-0.5 rounded-full {}", badge.0)}>{badge.1}</span>
                                                            </div>
                                                            {if !reason.is_empty() {
                                                                Some(view! { <p class="text-xs text-gray-500 mt-1">{reason}</p> })
                                                            } else { None }}
                                                        </div>
                                                        <button
                                                            class="text-sm text-red-600 font-medium hover:text-red-700"
                                                            on:click=move |_| {
                                                                let id = del_id.clone();
                                                                leptos::task::spawn_local(async move {
                                                                    let _ = crate::api::delete::<serde_json::Value>(
                                                                        &format!("/api/availability/exceptions/{}", id)
                                                                    ).await;
                                                                    if let Some(w) = leptos::web_sys::window() {
                                                                        let _ = w.location().reload();
                                                                    }
                                                                });
                                                            }
                                                        >"삭제"</button>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_any()
                                }
                            }
                            _ => view! {
                                <p class="text-sm text-gray-500">"일정 변경 내역을 불러올 수 없습니다."</p>
                            }.into_any(),
                        }
                    })}
                </Suspense>

                // Add exception button / form
                <Show when=move || !show_exception_form.get()>
                    <button
                        class="w-full py-2 bg-gray-100 text-gray-700 text-sm font-medium rounded-xl hover:bg-gray-200"
                        on:click=move |_| show_exception_form.set(true)
                    >"날짜 차단"</button>
                </Show>

                <Show when=move || show_exception_form.get()>
                    <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100 space-y-3">
                        <div class="space-y-1">
                            <label class="text-sm font-medium text-gray-700">"날짜"</label>
                            <input
                                type="date"
                                class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm"
                                on:input=move |ev| exception_date.set(event_target_value(&ev))
                            />
                        </div>
                        <div class="space-y-1">
                            <label class="text-sm font-medium text-gray-700">"사유"</label>
                            <input
                                type="text"
                                class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm"
                                placeholder="사유를 입력하세요"
                                on:input=move |ev| exception_reason.set(event_target_value(&ev))
                            />
                        </div>

                        {move || exception_error.get().map(|e| view! {
                            <p class="text-sm text-red-600">{e}</p>
                        })}

                        <div class="flex gap-2">
                            <button
                                class="flex-1 py-2 bg-teal-600 text-white text-sm font-medium rounded-xl hover:bg-teal-700"
                                on:click=move |_| {
                                    let date_val = exception_date.get_untracked();
                                    let reason_val = exception_reason.get_untracked();
                                    if date_val.is_empty() {
                                        exception_error.set(Some("날짜를 선택해주세요.".to_string()));
                                        return;
                                    }
                                    let body = serde_json::json!({
                                        "date": date_val,
                                        "reason": reason_val,
                                        "is_available": false
                                    });
                                    leptos::task::spawn_local(async move {
                                        match crate::api::post::<serde_json::Value, _>(
                                            "/api/availability/exceptions", &body
                                        ).await {
                                            Ok(resp) if resp.success => {
                                                if let Some(w) = leptos::web_sys::window() {
                                                    let _ = w.location().reload();
                                                }
                                            }
                                            Ok(resp) => exception_error.set(resp.error),
                                            Err(e) => exception_error.set(Some(e)),
                                        }
                                    });
                                }
                            >"저장"</button>
                            <button
                                class="flex-1 py-2 bg-gray-100 text-gray-700 text-sm font-medium rounded-xl hover:bg-gray-200"
                                on:click=move |_| {
                                    show_exception_form.set(false);
                                    exception_error.set(None);
                                }
                            >"취소"</button>
                        </div>
                    </div>
                </Show>
            </div>
        </div>
    }
}

// =============================================================================
// 28. SettingsPage — caregiver settings
// =============================================================================

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <h1 class="text-xl font-bold text-gray-900">"설정"</h1>

            // Notification settings
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h3 class="font-semibold text-gray-900 mb-4">"알림 설정"</h3>
                <div class="space-y-3">
                    <SettingsToggle label="스케줄 알림" desc="방문 시작 30분 전 알림" default_on=true />
                    <SettingsToggle label="투약 알림" desc="고객 투약 시간 알림" default_on=true />
                    <SettingsToggle label="긴급 알림" desc="긴급 상황 알림" default_on=true />
                    <SettingsToggle label="교육/공지 알림" desc="교육 일정 및 공지사항" default_on=false />
                </div>
            </div>

            // Display settings
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h3 class="font-semibold text-gray-900 mb-4">"화면 설정"</h3>
                <div class="space-y-3">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm font-medium text-gray-900">"글자 크기"</p>
                            <p class="text-xs text-gray-500">"앱 전체 글자 크기"</p>
                        </div>
                        <select class="px-3 py-1.5 border border-gray-300 rounded-lg text-sm">
                            <option>"보통"</option>
                            <option>"크게"</option>
                            <option>"매우 크게"</option>
                        </select>
                    </div>
                    <SettingsToggle label="다크 모드" desc="어두운 화면 모드" default_on=false />
                </div>
            </div>

            // Account
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h3 class="font-semibold text-gray-900 mb-4">"계정"</h3>
                <div class="space-y-2">
                    <a href="/caregiver/profile" class="flex items-center justify-between py-2">
                        <span class="text-sm text-gray-700">"프로필 수정"</span>
                        <svg class="w-4 h-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
                        </svg>
                    </a>
                    <a href="/caregiver/profile/availability" class="flex items-center justify-between py-2">
                        <span class="text-sm text-gray-700">"근무 가능 시간 변경"</span>
                        <svg class="w-4 h-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
                        </svg>
                    </a>
                    <div class="pt-2 border-t border-gray-100">
                        <button
                            class="flex items-center justify-between py-2 w-full text-left"
                            on:click=move |_| {
                                leptos::task::spawn_local(async move {
                                    let _ = crate::api::post_no_body("/api/auth/logout").await;
                                    if let Some(window) = leptos::web_sys::window() {
                                        let _ = window.location().set_href("/auth/signin");
                                    }
                                });
                            }
                        >
                            <span class="text-sm text-red-600">"로그아웃"</span>
                        </button>
                    </div>
                </div>
            </div>

            // App info
            <div class="text-center text-xs text-gray-400 pt-4">
                <p>"요양보호사 포털 v1.0.0"</p>
            </div>
        </div>
    }
}

#[component]
fn SettingsToggle(
    #[prop(into)] label: String,
    #[prop(into)] desc: String,
    default_on: bool,
) -> impl IntoView {
    let is_on = RwSignal::new(default_on);

    view! {
        <div class="flex items-center justify-between">
            <div>
                <p class="text-sm font-medium text-gray-900">{label}</p>
                <p class="text-xs text-gray-500">{desc}</p>
            </div>
            <button
                class="w-10 h-6 rounded-full transition-colors relative"
                class=("bg-teal-600", move || is_on.get())
                class=("bg-gray-300", move || !is_on.get())
                on:click=move |_| is_on.update(|v| *v = !*v)
            >
                <span
                    class="absolute top-0.5 w-5 h-5 bg-white rounded-full shadow transition-transform"
                    class=("left-[1.125rem]", move || is_on.get())
                    class=("left-0.5", move || !is_on.get())
                />
            </button>
        </div>
    }
}
