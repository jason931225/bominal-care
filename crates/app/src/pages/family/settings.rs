use leptos::prelude::*;
use wasm_bindgen::JsCast;

// =============================================================================
// Settings pages — settings, profile, notifications
// =============================================================================

/// Family portal settings with notification toggles and account management.
#[component]
pub fn SettingsPage() -> impl IntoView {
    let (push_enabled, set_push_enabled) = signal(true);
    let (email_enabled, set_email_enabled) = signal(true);
    let (saved_msg, set_saved_msg) = signal(Option::<String>::None);

    let show_saved = move || {
        set_saved_msg.set(Some("설정이 저장되었습니다".to_string()));
        // Auto-dismiss after 2 seconds using wasm_bindgen closure + setTimeout
        let cb = wasm_bindgen::closure::Closure::once_into_js(move || {
            set_saved_msg.set(None);
        });
        if let Some(window) = leptos::web_sys::window() {
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                2_000,
            );
        }
    };

    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"설정"</h1>
                <p class="text-sm text-txt-secondary mt-1">"Bominal Family 설정을 관리하세요."</p>
            </div>

            // Saved confirmation toast
            {move || saved_msg.get().map(|msg| view! {
                <div class="bg-success-light rounded-xl px-4 py-2 text-sm font-medium text-success text-center transition-opacity">
                    {msg}
                </div>
            })}

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
                        on:click=move |_| {
                            set_push_enabled.update(|v| *v = !*v);
                            show_saved();
                        }
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
                        on:click=move |_| {
                            set_email_enabled.update(|v| *v = !*v);
                            show_saved();
                        }
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
/// Fetches profile data from `/api/profile/me` and displays editable form.
#[component]
pub fn ProfilePage() -> impl IntoView {
    let profile = LocalResource::new(|| {
        crate::api::get::<serde_json::Value>("/api/profile/me")
    });
    let saving = RwSignal::new(false);
    let save_msg = RwSignal::new(None::<String>);
    let save_error = RwSignal::new(None::<String>);
    let name_val = RwSignal::new(String::new());
    let phone_val = RwSignal::new(String::new());
    let address_val = RwSignal::new(String::new());

    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"내 프로필"</h1>
                <p class="text-sm text-txt-secondary mt-1">"가족 구성원 정보를 관리하세요."</p>
            </div>

            <Suspense fallback=move || view! { <div class="animate-pulse bg-gray-200 rounded-xl h-40" /> }>
                {move || Suspend::new(async move {
                    match profile.await {
                        Ok(resp) if resp.success => {
                            let data = resp.data.unwrap_or(serde_json::Value::Null);
                            let name = data.get("name")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();
                            let phone = data.get("phone")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();
                            let address = data.get("address")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();
                            let email = data.get("email")
                                .and_then(|v| v.as_str())
                                .unwrap_or("-")
                                .to_string();
                            name_val.set(name.clone());
                            phone_val.set(phone.clone());
                            address_val.set(address.clone());
                            view! {
                                <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                                    <div>
                                        <p class="text-sm text-txt-tertiary">"이메일"</p>
                                        <p class="text-sm text-txt-secondary">{email}</p>
                                    </div>
                                    <div>
                                        <label class="block text-sm font-medium text-txt-secondary mb-1">"이름"</label>
                                        <input
                                            type="text"
                                            class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-2 focus:ring-[var(--portal-accent)]/30 focus:border-[var(--portal-accent)]"
                                            prop:value=move || name_val.get()
                                            on:input=move |ev| name_val.set(event_target_value(&ev))
                                        />
                                    </div>
                                    <div>
                                        <label class="block text-sm font-medium text-txt-secondary mb-1">"전화번호"</label>
                                        <input
                                            type="tel"
                                            class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-2 focus:ring-[var(--portal-accent)]/30 focus:border-[var(--portal-accent)]"
                                            prop:value=move || phone_val.get()
                                            on:input=move |ev| phone_val.set(event_target_value(&ev))
                                        />
                                    </div>
                                    <div>
                                        <label class="block text-sm font-medium text-txt-secondary mb-1">"주소"</label>
                                        <input
                                            type="text"
                                            class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-2 focus:ring-[var(--portal-accent)]/30 focus:border-[var(--portal-accent)]"
                                            prop:value=move || address_val.get()
                                            on:input=move |ev| address_val.set(event_target_value(&ev))
                                        />
                                    </div>

                                    {move || save_error.get().map(|msg| view! {
                                        <p class="text-sm text-danger">{msg}</p>
                                    })}
                                    {move || save_msg.get().map(|msg| view! {
                                        <div class="bg-success-light rounded-xl p-3">
                                            <p class="text-sm font-medium text-success">{msg}</p>
                                        </div>
                                    })}

                                    <button
                                        class="w-full bg-[var(--portal-accent)] text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all disabled:opacity-50"
                                        prop:disabled=move || saving.get()
                                        on:click=move |_| {
                                            leptos::task::spawn_local(async move {
                                                saving.set(true);
                                                save_error.set(None);
                                                save_msg.set(None);
                                                let body = serde_json::json!({
                                                    "name": name_val.get(),
                                                    "phone": phone_val.get(),
                                                    "address": address_val.get(),
                                                });
                                                match crate::api::patch::<serde_json::Value, _>("/api/profile/me", &body).await {
                                                    Ok(resp) if resp.success => {
                                                        save_msg.set(Some("프로필이 저장되었습니다".to_string()));
                                                    }
                                                    Ok(resp) => save_error.set(resp.error),
                                                    Err(e) => save_error.set(Some(e)),
                                                }
                                                saving.set(false);
                                            });
                                        }
                                    >
                                        {move || if saving.get() { "저장 중..." } else { "저장" }}
                                    </button>
                                </div>
                            }.into_any()
                        }
                        _ => view! {
                            <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                                <p class="text-sm text-txt-tertiary">"프로필 정보를 불러올 수 없습니다."</p>
                            </div>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

/// Displays recent notifications fetched from `/api/notifications` with mark-read support.
#[component]
pub fn NotificationsPage() -> impl IntoView {
    let data = LocalResource::new(|| {
        crate::api::get::<Vec<bominal_types::Notification>>("/api/notifications")
    });
    let marking = RwSignal::new(false);
    let mark_error = RwSignal::new(None::<String>);

    view! {
        <div class="p-6 space-y-8">
            <div class="flex justify-between items-center">
                <div>
                    <h1 class="text-xl font-bold text-txt-primary">"알림"</h1>
                    <p class="text-sm text-txt-secondary mt-1">"최근 알림 내역입니다."</p>
                </div>
            </div>

            {move || mark_error.get().map(|msg| view! {
                <p class="text-sm text-danger">{msg}</p>
            })}

            <div class="space-y-3">
                <Suspense fallback=move || view! { <div class="animate-pulse bg-gray-200 rounded-xl h-20" /> }>
                    {move || Suspend::new(async move {
                        match data.await {
                            Ok(resp) if resp.success => {
                                let items = resp.data.unwrap_or_default();
                                if items.is_empty() {
                                    view! {
                                        <p class="text-center text-txt-secondary py-8">"알림이 없습니다."</p>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="space-y-3">
                                            {items.into_iter().map(|notif| {
                                                let notif_id = notif.id;
                                                let title = notif.title.clone();
                                                let message = notif.message.clone();
                                                let created = notif.created_at.format("%m-%d %H:%M").to_string();
                                                let is_read = notif.is_read;
                                                let bg_class = if is_read {
                                                    "bg-surface-card"
                                                } else {
                                                    "bg-[var(--portal-accent-light)]"
                                                };
                                                view! {
                                                    <div class={format!("{} rounded-2xl p-5 shadow-sm", bg_class)}>
                                                        <div class="flex justify-between items-start">
                                                            <div class="flex-1 min-w-0">
                                                                <p class="font-medium text-txt-primary text-sm">{title}</p>
                                                                <p class="text-sm text-txt-secondary mt-1 line-clamp-2">{message}</p>
                                                                <p class="text-xs text-txt-disabled mt-2">{created}</p>
                                                            </div>
                                                            {(!is_read).then(|| {
                                                                view! {
                                                                    <button
                                                                        class="text-xs text-[var(--portal-accent)] hover:underline ml-2 whitespace-nowrap"
                                                                        prop:disabled=move || marking.get()
                                                                        on:click=move |_| {
                                                                            leptos::task::spawn_local(async move {
                                                                                marking.set(true);
                                                                                mark_error.set(None);
                                                                                let url = format!("/api/notifications/{}/read", notif_id);
                                                                                match crate::api::patch::<serde_json::Value, _>(&url, &serde_json::json!({})).await {
                                                                                    Ok(_resp) => {
                                                                                        // Reload page to reflect changes
                                                                                        if let Some(window) = leptos::web_sys::window() {
                                                                                            let _ = window.location().reload();
                                                                                        }
                                                                                    }
                                                                                    Err(e) => mark_error.set(Some(e)),
                                                                                }
                                                                                marking.set(false);
                                                                            });
                                                                        }
                                                                    >
                                                                        "읽음"
                                                                    </button>
                                                                }
                                                            })}
                                                        </div>
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    }.into_any()
                                }
                            }
                            _ => view! {
                                <p class="text-center text-txt-secondary py-8">"데이터를 불러올 수 없습니다."</p>
                            }.into_any(),
                        }
                    })}
                </Suspense>
            </div>
        </div>
    }
}
