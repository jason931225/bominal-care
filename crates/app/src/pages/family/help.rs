use leptos::prelude::*;

// =============================================================================
// Help pages — hub, booking, emergency, report
// =============================================================================

/// Presents help options menu for the senior including booking, emergency, and reporting.
#[component]
pub fn HelpSeniorPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"도움이 필요하신가요?"</h1>
                <p class="text-sm text-txt-secondary mt-1">"어르신을 위한 도움 옵션입니다."</p>
            </div>

            <div class="space-y-3">
                <a href="/family/help/book" class="block bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200">
                    <p class="font-medium text-txt-primary">"서비스 예약"</p>
                    <p class="text-sm text-txt-tertiary mt-1">"방문요양, 방문목욕 등 서비스를 예약합니다."</p>
                </a>
                <a href="/family/help/emergency" class="block bg-danger-light rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200">
                    <p class="font-medium text-danger">"긴급 연락처"</p>
                    <p class="text-sm text-txt-tertiary mt-1">"응급 상황 시 즉시 연락할 수 있습니다."</p>
                </a>
                <a href="/family/help/report" class="block bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200">
                    <p class="font-medium text-txt-primary">"우려사항 신고"</p>
                    <p class="text-sm text-txt-tertiary mt-1">"돌봄 관련 우려사항을 신고합니다."</p>
                </a>
            </div>
        </div>
    }
}

/// Service booking form for scheduling visits on behalf of the senior.
#[component]
pub fn HelpBookPage() -> impl IntoView {
    let (service_type, set_service_type) = signal(String::new());
    let (preferred_date, set_preferred_date) = signal(String::new());
    let submitting = RwSignal::new(false);
    let error_msg = RwSignal::new(None::<String>);

    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"서비스 예약"</h1>
                <p class="text-sm text-txt-secondary mt-1">"어르신을 위한 서비스를 예약하세요."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <div>
                    <label class="block text-sm font-medium text-txt-secondary mb-1">"서비스 유형"</label>
                    <select
                        class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-2 focus:ring-[var(--portal-accent)]/30 focus:border-[var(--portal-accent)]"
                        on:change=move |ev| set_service_type.set(event_target_value(&ev))
                    >
                        <option value="">"선택하세요"</option>
                        <option value="home_care">"방문요양"</option>
                        <option value="home_bath">"방문목욕"</option>
                        <option value="home_nursing">"방문간호"</option>
                    </select>
                </div>
                <div>
                    <label class="block text-sm font-medium text-txt-secondary mb-1">"희망 날짜"</label>
                    <input
                        type="date"
                        class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-2 focus:ring-[var(--portal-accent)]/30 focus:border-[var(--portal-accent)]"
                        prop:value=move || preferred_date.get()
                        on:input=move |ev| set_preferred_date.set(event_target_value(&ev))
                    />
                </div>
                {move || error_msg.get().map(|msg| view! {
                    <p class="text-sm text-danger">{msg}</p>
                })}
                <button
                    class="w-full bg-[var(--portal-accent)] text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all disabled:opacity-50"
                    prop:disabled=move || submitting.get()
                    on:click=move |_| {
                        let svc = service_type.get();
                        let date = preferred_date.get();
                        leptos::task::spawn_local(async move {
                            submitting.set(true);
                            error_msg.set(None);
                            let body = serde_json::json!({
                                "service_category": svc,
                                "preferred_date": date,
                            });
                            match crate::api::post::<serde_json::Value, _>("/api/match-requests", &body).await {
                                Ok(resp) if resp.success => {
                                    if let Some(window) = leptos::web_sys::window() {
                                        let _ = window.location().set_href("/family/help");
                                    }
                                }
                                Ok(resp) => error_msg.set(resp.error),
                                Err(e) => error_msg.set(Some(e)),
                            }
                            submitting.set(false);
                        });
                    }
                >
                    {move || if submitting.get() { "처리 중..." } else { "예약 요청" }}
                </button>
            </div>
        </div>
    }
}

/// Displays emergency contact numbers for urgent situations.
/// Real emergency numbers (119, 1577-1000) are kept. Caregiver and institution
/// phone numbers are fetched from the API; if unavailable, shows a fallback message.
#[component]
pub fn HelpEmergencyPage() -> impl IntoView {
    let triggering = RwSignal::new(false);
    let triggered = RwSignal::new(false);
    let error_msg = RwSignal::new(None::<String>);

    // Fetch profile to get any linked caregiver / institution contacts
    let contacts = LocalResource::new(|| {
        crate::api::get::<serde_json::Value>("/api/profile/me")
    });

    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-danger">"긴급 연락처"</h1>
                <p class="text-sm text-txt-secondary mt-1">"응급 상황 시 아래 번호로 연락하세요."</p>
            </div>

            <div class="space-y-3">
                <div class="bg-danger-light rounded-2xl p-5">
                    <p class="font-semibold text-danger">"119 응급전화"</p>
                    <a href="tel:119" class="text-lg font-bold text-danger mt-1 block">"119"</a>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <p class="font-medium text-txt-primary">"담당 요양보호사"</p>
                    <Suspense fallback=move || view! { <p class="text-sm text-txt-tertiary mt-1">"불러오는 중..."</p> }>
                        {move || Suspend::new(async move {
                            match contacts.await {
                                Ok(resp) if resp.success => {
                                    let data = resp.data.unwrap_or(serde_json::Value::Null);
                                    let phone = data.get("caregiver_phone")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string());
                                    match phone {
                                        Some(p) => {
                                            let tel = format!("tel:{}", p);
                                            view! {
                                                <a href=tel class="text-sm text-[var(--portal-accent)] hover:underline mt-1 block">{p}</a>
                                            }.into_any()
                                        }
                                        None => view! {
                                            <p class="text-sm text-txt-tertiary mt-1">"등록된 번호 없음"</p>
                                        }.into_any(),
                                    }
                                }
                                _ => view! {
                                    <p class="text-sm text-txt-tertiary mt-1">"등록된 번호 없음"</p>
                                }.into_any(),
                            }
                        })}
                    </Suspense>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <p class="font-medium text-txt-primary">"담당 기관"</p>
                    <Suspense fallback=move || view! { <p class="text-sm text-txt-tertiary mt-1">"불러오는 중..."</p> }>
                        {move || Suspend::new(async move {
                            match contacts.await {
                                Ok(resp) if resp.success => {
                                    let data = resp.data.unwrap_or(serde_json::Value::Null);
                                    let phone = data.get("institution_phone")
                                        .or_else(|| data.get("provider_phone"))
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string());
                                    match phone {
                                        Some(p) => {
                                            let tel = format!("tel:{}", p);
                                            view! {
                                                <a href=tel class="text-sm text-[var(--portal-accent)] hover:underline mt-1 block">{p}</a>
                                            }.into_any()
                                        }
                                        None => view! {
                                            <p class="text-sm text-txt-tertiary mt-1">"등록된 번호 없음"</p>
                                        }.into_any(),
                                    }
                                }
                                _ => view! {
                                    <p class="text-sm text-txt-tertiary mt-1">"등록된 번호 없음"</p>
                                }.into_any(),
                            }
                        })}
                    </Suspense>
                </div>
                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                    <p class="font-medium text-txt-primary">"국민건강보험공단"</p>
                    <a href="tel:1577-1000" class="text-sm text-[var(--portal-accent)] hover:underline mt-1 block">"1577-1000"</a>
                </div>
                {move || error_msg.get().map(|msg| view! {
                    <p class="text-sm text-danger">{msg}</p>
                })}
                {move || if triggered.get() {
                    view! {
                        <div class="bg-success-light rounded-2xl p-5">
                            <p class="font-medium text-success">"긴급 알림이 전송되었습니다."</p>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <button
                            class="w-full bg-danger text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all disabled:opacity-50"
                            prop:disabled=move || triggering.get()
                            on:click=move |_| {
                                leptos::task::spawn_local(async move {
                                    triggering.set(true);
                                    error_msg.set(None);
                                    let body = serde_json::json!({
                                        "type": "family_emergency",
                                    });
                                    match crate::api::post::<serde_json::Value, _>("/api/emergency/trigger", &body).await {
                                        Ok(resp) if resp.success => {
                                            triggered.set(true);
                                        }
                                        Ok(resp) => error_msg.set(resp.error),
                                        Err(e) => error_msg.set(Some(e)),
                                    }
                                    triggering.set(false);
                                });
                            }
                        >
                            {move || if triggering.get() { "전송 중..." } else { "긴급 알림 보내기" }}
                        </button>
                    }.into_any()
                }}
            </div>
        </div>
    }
}

/// Form for reporting care-related concerns or safety issues.
#[component]
pub fn HelpReportPage() -> impl IntoView {
    let (category, set_category) = signal(String::new());
    let (description, set_description) = signal(String::new());
    let submitting = RwSignal::new(false);
    let error_msg = RwSignal::new(None::<String>);
    let success_msg = RwSignal::new(None::<String>);

    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"우려사항 신고"</h1>
                <p class="text-sm text-txt-secondary mt-1">"돌봄 관련 문제를 신고해 주세요."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <div>
                    <label class="block text-sm font-medium text-txt-secondary mb-1">"신고 유형"</label>
                    <select
                        class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-2 focus:ring-[var(--portal-accent)]/30 focus:border-[var(--portal-accent)]"
                        on:change=move |ev| set_category.set(event_target_value(&ev))
                    >
                        <option value="">"선택하세요"</option>
                        <option value="quality">"서비스 품질"</option>
                        <option value="safety">"안전 문제"</option>
                        <option value="abuse">"학대 의심"</option>
                        <option value="other">"기타"</option>
                    </select>
                </div>
                <div>
                    <label class="block text-sm font-medium text-txt-secondary mb-1">"상세 설명"</label>
                    <textarea
                        rows=4
                        class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-2 focus:ring-[var(--portal-accent)]/30 focus:border-[var(--portal-accent)]"
                        placeholder="구체적인 내용을 적어주세요."
                        prop:value=move || description.get()
                        on:input=move |ev| set_description.set(event_target_value(&ev))
                    ></textarea>
                </div>
                {move || error_msg.get().map(|msg| view! {
                    <p class="text-sm text-danger">{msg}</p>
                })}
                {move || success_msg.get().map(|msg| view! {
                    <div class="bg-success-light rounded-xl p-3">
                        <p class="text-sm font-medium text-success">{msg}</p>
                    </div>
                })}
                <button
                    class="w-full bg-danger text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all disabled:opacity-50"
                    prop:disabled=move || submitting.get()
                    on:click=move |_| {
                        let cat = category.get();
                        let desc = description.get();
                        leptos::task::spawn_local(async move {
                            submitting.set(true);
                            error_msg.set(None);
                            success_msg.set(None);
                            let body = serde_json::json!({
                                "type": cat,
                                "description": desc,
                                "severity": "moderate",
                            });
                            match crate::api::post::<serde_json::Value, _>("/api/incidents", &body).await {
                                Ok(resp) if resp.success => {
                                    success_msg.set(Some("신고가 접수되었습니다".to_string()));
                                    set_category.set(String::new());
                                    set_description.set(String::new());
                                }
                                Ok(resp) => error_msg.set(resp.error),
                                Err(e) => error_msg.set(Some(e)),
                            }
                            submitting.set(false);
                        });
                    }
                >
                    {move || if submitting.get() { "제출 중..." } else { "신고 제출" }}
                </button>
            </div>
        </div>
    }
}
