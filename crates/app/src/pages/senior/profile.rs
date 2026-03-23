use leptos::prelude::*;
use uuid::Uuid;

use bominal_types::PersonProfile;
use crate::components::data_display::EmptyState;
use crate::components::layout::PageHeader;
use super::InfoRow;

/// Profile API response -- server returns PersonProfile directly.
type ProfileResponse = PersonProfile;

/// Personal info, health baseline, and family contacts.
#[component]
pub fn ProfilePage() -> impl IntoView {
    let profile = LocalResource::new(|| {
        crate::api::get::<ProfileResponse>("/api/profile/me")
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="내 프로필" subtitle="개인정보 및 건강 기본 정보" />
            <Suspense fallback=move || view! { <div class="skeleton h-8 w-20"></div> }>
                {move || Suspend::new(async move {
                    match profile.await {
                        Ok(resp) if resp.success => {
                            match resp.data {
                                Some(data) => {
                                    let p = data;
                                    view! {
                                        <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-3">
                                            <InfoRow
                                                label="한국 이름".to_string()
                                                value=p.korean_name.unwrap_or_else(|| "\u{2014}".to_string())
                                            />
                                            <InfoRow
                                                label="전화번호".to_string()
                                                value={
                                                    let raw = p.phone.unwrap_or_else(|| "\u{2014}".to_string());
                                                    if raw != "\u{2014}" { crate::api::format_phone_kr(&raw) } else { raw }
                                                }
                                            />
                                            <InfoRow
                                                label="주소".to_string()
                                                value=p.address.unwrap_or_else(|| "\u{2014}".to_string())
                                            />
                                            <InfoRow
                                                label="긴급 연락처 (이름)".to_string()
                                                value=p.emergency_contact_name.unwrap_or_else(|| "\u{2014}".to_string())
                                            />
                                            <InfoRow
                                                label="긴급 연락처 (전화)".to_string()
                                                value=p.emergency_contact_phone.unwrap_or_else(|| "\u{2014}".to_string())
                                            />
                                        </div>
                                    }.into_any()
                                }
                                None => view! { <EmptyState message="프로필 정보를 불러올 수 없습니다." /> }.into_any(),
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

/// Emergency contacts with tel: links and health info -- fetches profile for contact details.
#[component]
pub fn EmergencyPage() -> impl IntoView {
    let profile = LocalResource::new(|| {
        crate::api::get::<serde_json::Value>("/api/profile/me")
    });

    let triggering = RwSignal::new(false);
    let triggered = RwSignal::new(false);
    let trigger_error = RwSignal::new(Option::<String>::None);

    let on_notify_family = move |_| {
        triggering.set(true);
        trigger_error.set(None);
        leptos::task::spawn_local(async move {
            let body = serde_json::json!({ "type": "senior_emergency" });
            match crate::api::post::<serde_json::Value, _>("/api/emergency/trigger", &body).await {
                Ok(resp) if resp.success => {
                    triggered.set(true);
                }
                Ok(resp) => trigger_error.set(resp.error),
                Err(e) => trigger_error.set(Some(e)),
            }
            triggering.set(false);
        });
    };

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="긴급 연락" subtitle="긴급 상황 시 사용하세요" />

            // 119 Emergency call
            <a href="tel:119"
               class="block w-full bg-danger text-white text-center text-xl font-bold \
                      rounded-2xl py-6 shadow-lg hover:bg-danger-hover active:scale-[0.98] transition-all">
                "119 응급 전화"
            </a>

            // Notify family button
            <Show when=move || trigger_error.get().is_some()>
                <div class="bg-danger-light rounded-2xl p-4 text-danger text-lg">
                    {move || trigger_error.get().unwrap_or_default()}
                </div>
            </Show>
            {move || if triggered.get() {
                view! {
                    <div class="bg-success-light rounded-2xl p-5">
                        <p class="font-medium text-success">"긴급 알림이 가족에게 전송되었습니다."</p>
                    </div>
                }.into_any()
            } else {
                view! {
                    <button
                        class="w-full bg-warning text-white text-lg font-semibold rounded-xl \
                               py-4 hover:opacity-90 active:scale-[0.98] transition-all disabled:opacity-50"
                        disabled=move || triggering.get()
                        on:click=on_notify_family
                    >
                        {move || if triggering.get() { "전송 중..." } else { "가족에게 알리기" }}
                    </button>
                }.into_any()
            }}

            // Profile emergency contact info
            <Suspense fallback=move || view! { <div class="skeleton h-8 w-20"></div> }>
                {move || Suspend::new(async move {
                    match profile.await {
                        Ok(resp) if resp.success => {
                            match resp.data {
                                Some(data) => {
                                    let name = data.get("korean_name")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("\u{2014}")
                                        .to_string();
                                    let address = data.get("address")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("\u{2014}")
                                        .to_string();
                                    let ec_name = data.get("emergency_contact_name")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("\u{2014}")
                                        .to_string();
                                    let ec_phone = data.get("emergency_contact_phone")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("")
                                        .to_string();
                                    let ec_phone_display = if ec_phone.is_empty() {
                                        "\u{2014}".to_string()
                                    } else {
                                        crate::api::format_phone_kr(&ec_phone)
                                    };
                                    let ec_phone_href = if ec_phone.is_empty() {
                                        None
                                    } else {
                                        Some(format!("tel:{}", ec_phone))
                                    };

                                    view! {
                                        <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-3">
                                            <h2 class="text-lg font-semibold text-txt-primary">"내 긴급 정보"</h2>
                                            <InfoRow label="이름".to_string() value=name />
                                            <InfoRow label="주소".to_string() value=address />
                                            <InfoRow label="긴급 연락처".to_string() value=ec_name />
                                            <div class="flex items-center justify-between py-2 border-b border-surface-subtle">
                                                <span class="text-base text-txt-tertiary">"연락처 전화"</span>
                                                {match ec_phone_href {
                                                    Some(href) => view! {
                                                        <a href=href class="text-lg text-primary font-medium underline">
                                                            {ec_phone_display}
                                                        </a>
                                                    }.into_any(),
                                                    None => view! {
                                                        <span class="text-lg text-txt-primary font-medium">{ec_phone_display}</span>
                                                    }.into_any(),
                                                }}
                                            </div>
                                        </div>
                                    }.into_any()
                                }
                                None => view! {
                                    <EmptyState message="프로필 정보를 불러올 수 없습니다." />
                                }.into_any(),
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

/// Consent toggles for data sharing -- fetches consent records from API.
#[component]
pub fn ConsentPage() -> impl IntoView {
    let consents = LocalResource::new(|| {
        crate::api::get::<Vec<serde_json::Value>>("/api/consent")
    });

    let revoke_error = RwSignal::new(Option::<String>::None);
    let revoked_id = RwSignal::new(Option::<String>::None);

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="동의 관리" subtitle="데이터 공유 및 개인정보 동의" />

            <Show when=move || revoke_error.get().is_some()>
                <div class="bg-danger-light rounded-2xl p-4 text-danger text-lg">
                    {move || revoke_error.get().unwrap_or_default()}
                </div>
            </Show>
            <Show when=move || revoked_id.get().is_some()>
                <div class="bg-success-light rounded-2xl p-4 text-success text-lg">
                    "동의가 철회되었습니다."
                </div>
            </Show>

            <Suspense fallback=move || view! { <div class="skeleton h-8 w-20"></div> }>
                {move || Suspend::new(async move {
                    match consents.await {
                        Ok(resp) if resp.success => {
                            let items = resp.data.unwrap_or_default();
                            if items.is_empty() {
                                view! { <EmptyState message="등록된 동의 내역이 없습니다." /> }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-3">
                                        {items.into_iter().map(|consent| {
                                            let id = consent.get("id")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("")
                                                .to_string();
                                            let purpose = consent.get("purpose")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("알 수 없음")
                                                .to_string();
                                            let is_active = consent.get("is_active")
                                                .and_then(|v| v.as_bool())
                                                .unwrap_or(false);
                                            let granted_at = consent.get("granted_at")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("")
                                                .to_string();
                                            let date_display = if granted_at.is_empty() {
                                                "\u{2014}".to_string()
                                            } else {
                                                granted_at.chars().take(10).collect::<String>()
                                            };
                                            let (badge_label, badge_class) = if is_active {
                                                ("유효", "bg-success-light text-success")
                                            } else {
                                                ("만료", "bg-surface-subtle text-txt-tertiary")
                                            };
                                            let revoke_id = id.clone();

                                            view! {
                                                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                                                    <div class="flex items-center justify-between">
                                                        <p class="text-lg font-medium text-txt-primary">{purpose}</p>
                                                        <span class={format!("text-xs px-2 py-1 rounded-full {badge_class}")}>{badge_label}</span>
                                                    </div>
                                                    <p class="text-base text-txt-secondary mt-1">{format!("동의일: {}", date_display)}</p>
                                                    {if is_active {
                                                        let err_sig = revoke_error;
                                                        let done_sig = revoked_id;
                                                        Some(view! {
                                                            <button
                                                                class="mt-3 text-sm text-danger font-medium underline"
                                                                on:click=move |_| {
                                                                    let rid = revoke_id.clone();
                                                                    leptos::task::spawn_local(async move {
                                                                        err_sig.set(None);
                                                                        match crate::api::delete::<serde_json::Value>(
                                                                            &format!("/api/consent/{}", rid),
                                                                        ).await {
                                                                            Ok(resp) if resp.success => {
                                                                                done_sig.set(Some(rid));
                                                                                if let Some(w) = leptos::web_sys::window() {
                                                                                    let _ = w.location().reload();
                                                                                }
                                                                            }
                                                                            Ok(resp) => err_sig.set(resp.error),
                                                                            Err(e) => err_sig.set(Some(e)),
                                                                        }
                                                                    });
                                                                }
                                                            >"철회"</button>
                                                        })
                                                    } else {
                                                        None
                                                    }}
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

/// Single consent record detail -- fetches consent list and filters by ID.
#[component]
pub fn ConsentDetailPage(
    #[prop(into)] consent_id: Uuid,
) -> impl IntoView {
    let cid = consent_id;
    let consent = LocalResource::new(move || {
        let id = cid;
        async move {
            crate::api::get::<Vec<serde_json::Value>>("/api/consent").await
                .map(|resp| {
                    let target_id = id.to_string();
                    let filtered = resp.data.unwrap_or_default()
                        .into_iter()
                        .find(|c| {
                            c.get("id")
                                .and_then(|v| v.as_str())
                                .map(|s| s == target_id)
                                .unwrap_or(false)
                        });
                    crate::api::ApiResponse {
                        success: resp.success,
                        data: filtered,
                        error: resp.error,
                        meta: resp.meta,
                    }
                })
        }
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/consent" class="text-primary text-lg">"< 동의 관리"</a>

            <Suspense fallback=move || view! { <div class="skeleton h-8 w-20"></div> }>
                {move || Suspend::new(async move {
                    match consent.await {
                        Ok(resp) if resp.success => {
                            match resp.data {
                                Some(data) => {
                                    let purpose = data.get("purpose")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("알 수 없음")
                                        .to_string();
                                    let is_active = data.get("is_active")
                                        .and_then(|v| v.as_bool())
                                        .unwrap_or(false);
                                    let granted_at = data.get("granted_at")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("\u{2014}")
                                        .to_string();
                                    let revoked_at = data.get("revoked_at")
                                        .and_then(|v| v.as_str())
                                        .map(|s| {
                                            let short: String = s.chars().take(10).collect();
                                            short
                                        });
                                    let granted_at_short: String = granted_at.chars().take(10).collect();
                                    let (badge_label, badge_class) = if is_active {
                                        ("유효", "bg-success-light text-success")
                                    } else {
                                        ("만료", "bg-surface-subtle text-txt-tertiary")
                                    };

                                    view! {
                                        <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-3">
                                            <div class="flex items-center justify-between mb-3">
                                                <h1 class="text-xl font-bold text-txt-primary">{purpose}</h1>
                                                <span class={format!("text-xs px-2 py-1 rounded-full {badge_class}")}>{badge_label}</span>
                                            </div>
                                            <InfoRow label="동의일".to_string() value=granted_at_short />
                                            {revoked_at.map(|r| view! {
                                                <InfoRow label="철회일".to_string() value=r />
                                            })}
                                        </div>
                                    }.into_any()
                                }
                                None => view! { <EmptyState message="동의 기록을 찾을 수 없습니다." /> }.into_any(),
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

/// Paginated list of medical history conditions -- fetches from API.
#[component]
pub fn MedicalHistoryPage() -> impl IntoView {
    let history = LocalResource::new(|| {
        crate::api::get::<Vec<serde_json::Value>>("/api/medical-history")
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="병력 기록" subtitle="과거 및 현재 질환 기록" />
            <Suspense fallback=move || view! { <div class="skeleton h-8 w-20"></div> }>
                {move || Suspend::new(async move {
                    match history.await {
                        Ok(resp) if resp.success => {
                            let items = resp.data.unwrap_or_default();
                            if items.is_empty() {
                                view! { <EmptyState message="등록된 병력이 없습니다." /> }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-3">
                                        {items.into_iter().map(|entry| {
                                            let condition = entry.get("condition_name")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("알 수 없음")
                                                .to_string();
                                            let diagnosed = entry.get("diagnosed_date")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("")
                                                .to_string();
                                            let date_display = if diagnosed.is_empty() {
                                                "\u{2014}".to_string()
                                            } else {
                                                diagnosed.chars().take(10).collect::<String>()
                                            };
                                            let status = entry.get("status")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("UNKNOWN")
                                                .to_string();
                                            let notes = entry.get("notes")
                                                .and_then(|v| v.as_str())
                                                .map(|s| s.to_string());
                                            let (badge_label, badge_class) = match status.as_str() {
                                                "ACTIVE" | "Active" => ("진행 중", "bg-warning-light text-warning"),
                                                "RESOLVED" | "Resolved" => ("완치", "bg-success-light text-success"),
                                                "CHRONIC" | "Chronic" => ("만성", "bg-primary-light text-primary"),
                                                _ => ("기타", "bg-surface-subtle text-txt-tertiary"),
                                            };

                                            view! {
                                                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                                                    <div class="flex items-center justify-between">
                                                        <p class="text-lg font-medium text-txt-primary">{condition}</p>
                                                        <span class={format!("text-xs px-2 py-1 rounded-full {badge_class}")}>{badge_label}</span>
                                                    </div>
                                                    <p class="text-base text-txt-secondary mt-1">{format!("진단일: {}", date_display)}</p>
                                                    {notes.map(|n| view! {
                                                        <p class="text-sm text-txt-tertiary mt-2">{n}</p>
                                                    })}
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

/// Single medical history condition detail -- fetches entry from API.
#[component]
pub fn MedicalHistoryDetailPage(
    #[prop(into)] entry_id: Uuid,
) -> impl IntoView {
    let eid = entry_id;
    let entry = LocalResource::new(move || {
        let id = eid;
        async move {
            crate::api::get::<Vec<serde_json::Value>>("/api/medical-history").await
                .map(|resp| {
                    let target_id = id.to_string();
                    let filtered = resp.data.unwrap_or_default()
                        .into_iter()
                        .find(|e| {
                            e.get("id")
                                .and_then(|v| v.as_str())
                                .map(|s| s == target_id)
                                .unwrap_or(false)
                        });
                    crate::api::ApiResponse {
                        success: resp.success,
                        data: filtered,
                        error: resp.error,
                        meta: resp.meta,
                    }
                })
        }
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/medical-history" class="text-primary text-lg">"< 병력 기록"</a>

            <Suspense fallback=move || view! { <div class="skeleton h-8 w-20"></div> }>
                {move || Suspend::new(async move {
                    match entry.await {
                        Ok(resp) if resp.success => {
                            match resp.data {
                                Some(data) => {
                                    let condition = data.get("condition_name")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("알 수 없음")
                                        .to_string();
                                    let diagnosed_raw = data.get("diagnosed_date")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("\u{2014}")
                                        .to_string();
                                    let diagnosed_short: String = diagnosed_raw.chars().take(10).collect();
                                    let status = data.get("status")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("UNKNOWN")
                                        .to_string();
                                    let notes = data.get("notes")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string());
                                    let severity = data.get("severity")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string());
                                    let (badge_label, badge_class) = match status.as_str() {
                                        "ACTIVE" | "Active" => ("진행 중", "bg-warning-light text-warning"),
                                        "RESOLVED" | "Resolved" => ("완치", "bg-success-light text-success"),
                                        "CHRONIC" | "Chronic" => ("만성", "bg-primary-light text-primary"),
                                        _ => ("기타", "bg-surface-subtle text-txt-tertiary"),
                                    };

                                    view! {
                                        <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-3">
                                            <div class="flex items-center justify-between mb-3">
                                                <h1 class="text-xl font-bold text-txt-primary">{condition}</h1>
                                                <span class={format!("text-xs px-2 py-1 rounded-full {badge_class}")}>{badge_label}</span>
                                            </div>
                                            <InfoRow label="진단일".to_string() value=diagnosed_short />
                                            <InfoRow label="상태".to_string() value=badge_label.to_string() />
                                            {severity.map(|s| view! {
                                                <InfoRow label="중증도".to_string() value=s />
                                            })}
                                            {notes.map(|n| view! {
                                                <div class="mt-3 p-3 bg-surface-subtle rounded-xl">
                                                    <p class="text-sm text-txt-tertiary font-medium">"메모"</p>
                                                    <p class="text-base text-txt-primary mt-1">{n}</p>
                                                </div>
                                            })}
                                        </div>
                                    }.into_any()
                                }
                                None => view! { <EmptyState message="기록을 찾을 수 없습니다." /> }.into_any(),
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

/// Care plan overview -- list of care plans from API.
#[component]
pub fn CarePlanPage() -> impl IntoView {
    let plans = LocalResource::new(|| {
        crate::api::get::<Vec<serde_json::Value>>("/api/care-plans?page=1&limit=10")
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="케어 플랜" subtitle="돌봄 계획을 확인하세요" />
            <Suspense fallback=move || view! { <div class="skeleton h-8 w-20"></div> }>
                {move || Suspend::new(async move {
                    match plans.await {
                        Ok(resp) if resp.success => {
                            let items = resp.data.unwrap_or_default();
                            if items.is_empty() {
                                view! { <EmptyState message="등록된 케어 플랜이 없습니다." /> }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-3">
                                        {items.into_iter().map(|plan| {
                                            let title = plan.get("title")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("케어 플랜")
                                                .to_string();
                                            let description = plan.get("description")
                                                .and_then(|v| v.as_str())
                                                .map(|s| s.to_string());
                                            let status = plan.get("status")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("UNKNOWN")
                                                .to_string();
                                            let start_date = plan.get("start_date")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("")
                                                .to_string();
                                            let end_date = plan.get("end_date")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("")
                                                .to_string();
                                            let date_range = if start_date.is_empty() && end_date.is_empty() {
                                                "\u{2014}".to_string()
                                            } else {
                                                let s = start_date.chars().take(10).collect::<String>();
                                                let e = end_date.chars().take(10).collect::<String>();
                                                format!("{} ~ {}", s, e)
                                            };
                                            let (badge_label, badge_class) = match status.as_str() {
                                                "ACTIVE" | "Active" => ("진행 중", "bg-success-light text-success"),
                                                "DRAFT" | "Draft" => ("초안", "bg-surface-subtle text-txt-tertiary"),
                                                "COMPLETED" | "Completed" => ("완료", "bg-primary-light text-primary"),
                                                "CANCELLED" | "Cancelled" => ("취소", "bg-danger-light text-danger"),
                                                _ => ("기타", "bg-surface-subtle text-txt-tertiary"),
                                            };

                                            view! {
                                                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                                                    <div class="flex items-center justify-between">
                                                        <p class="text-lg font-medium text-txt-primary">{title}</p>
                                                        <span class={format!("text-xs px-2 py-1 rounded-full {badge_class}")}>{badge_label}</span>
                                                    </div>
                                                    {description.map(|d| view! {
                                                        <p class="text-base text-txt-secondary mt-1">{d}</p>
                                                    })}
                                                    <p class="text-sm text-txt-tertiary mt-2">{date_range}</p>
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

/// Care plan detail with visits -- fetches individual plan from API.
#[component]
pub fn CarePlanDetailPage(
    #[prop(into)] plan_id: Uuid,
) -> impl IntoView {
    let pid = plan_id;
    let plan = LocalResource::new(move || {
        let id = pid;
        async move {
            crate::api::get::<serde_json::Value>(&format!("/api/care-plans/{}", id)).await
        }
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/care" class="text-primary text-lg">"< 케어 플랜"</a>

            <Suspense fallback=move || view! { <div class="skeleton h-8 w-20"></div> }>
                {move || Suspend::new(async move {
                    match plan.await {
                        Ok(resp) if resp.success => {
                            match resp.data {
                                Some(data) => {
                                    let title = data.get("title")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("케어 플랜")
                                        .to_string();
                                    let description = data.get("description")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string());
                                    let status = data.get("status")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("UNKNOWN")
                                        .to_string();
                                    let start_date_raw = data.get("start_date")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("\u{2014}")
                                        .to_string();
                                    let start_date_short: String = start_date_raw.chars().take(10).collect();
                                    let end_date_raw = data.get("end_date")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("\u{2014}")
                                        .to_string();
                                    let end_date_short: String = end_date_raw.chars().take(10).collect();
                                    let goals = data.get("goals")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string());
                                    let (badge_label, badge_class) = match status.as_str() {
                                        "ACTIVE" | "Active" => ("진행 중", "bg-success-light text-success"),
                                        "DRAFT" | "Draft" => ("초안", "bg-surface-subtle text-txt-tertiary"),
                                        "COMPLETED" | "Completed" => ("완료", "bg-primary-light text-primary"),
                                        "CANCELLED" | "Cancelled" => ("취소", "bg-danger-light text-danger"),
                                        _ => ("기타", "bg-surface-subtle text-txt-tertiary"),
                                    };

                                    view! {
                                        <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-3">
                                            <div class="flex items-center justify-between mb-3">
                                                <h1 class="text-xl font-bold text-txt-primary">{title}</h1>
                                                <span class={format!("text-xs px-2 py-1 rounded-full {badge_class}")}>{badge_label}</span>
                                            </div>
                                            {description.map(|d| view! {
                                                <p class="text-base text-txt-secondary mb-3">{d}</p>
                                            })}
                                            <InfoRow label="시작일".to_string() value=start_date_short />
                                            <InfoRow label="종료일".to_string() value=end_date_short />
                                            <InfoRow label="상태".to_string() value=badge_label.to_string() />
                                            {goals.map(|g| view! {
                                                <div class="mt-3 p-3 bg-surface-subtle rounded-xl">
                                                    <p class="text-sm text-txt-tertiary font-medium">"목표"</p>
                                                    <p class="text-base text-txt-primary mt-1">{g}</p>
                                                </div>
                                            })}
                                        </div>
                                    }.into_any()
                                }
                                None => view! { <EmptyState message="케어 플랜을 찾을 수 없습니다." /> }.into_any(),
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
