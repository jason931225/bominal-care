use leptos::prelude::*;

// =============================================================================
// Matching pages — search, results, detail
// =============================================================================

/// Search form for caregiver matching with region and service type filters.
#[component]
pub fn MatchingSearchPage() -> impl IntoView {
    let (region, set_region) = signal(String::new());
    let (service, set_service) = signal(String::new());
    let submitting = RwSignal::new(false);
    let error_msg = RwSignal::new(None::<String>);

    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"요양보호사 매칭"</h1>
                <p class="text-sm text-txt-secondary mt-1">"조건을 입력하고 매칭을 시작하세요."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <div>
                    <label class="block text-sm font-medium text-txt-secondary mb-1">"지역"</label>
                    <input
                        type="text"
                        class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-2 focus:ring-[var(--portal-accent)]/30 focus:border-[var(--portal-accent)]"
                        placeholder="예: 서울시 강남구"
                        prop:value=move || region.get()
                        on:input=move |ev| set_region.set(event_target_value(&ev))
                    />
                </div>
                <div>
                    <label class="block text-sm font-medium text-txt-secondary mb-1">"서비스 유형"</label>
                    <select
                        class="w-full border border-gray-200 rounded-xl px-3 py-2 text-sm focus:ring-2 focus:ring-[var(--portal-accent)]/30 focus:border-[var(--portal-accent)]"
                        on:change=move |ev| set_service.set(event_target_value(&ev))
                    >
                        <option value="">"선택하세요"</option>
                        <option value="home_care">"방문요양"</option>
                        <option value="home_bath">"방문목욕"</option>
                        <option value="home_nursing">"방문간호"</option>
                        <option value="day_care">"주야간보호"</option>
                    </select>
                </div>
                {move || error_msg.get().map(|msg| view! {
                    <p class="text-sm text-danger">{msg}</p>
                })}
                <button
                    class="w-full bg-[var(--portal-accent)] text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all disabled:opacity-50"
                    prop:disabled=move || submitting.get()
                    on:click=move |_| {
                        let region_val = region.get();
                        let service_val = service.get();
                        leptos::task::spawn_local(async move {
                            submitting.set(true);
                            error_msg.set(None);
                            let body = serde_json::json!({
                                "region": region_val,
                                "service_category": service_val,
                            });
                            match crate::api::post::<serde_json::Value, _>("/api/match-requests", &body).await {
                                Ok(resp) if resp.success => {
                                    if let Some(window) = leptos::web_sys::window() {
                                        let _ = window.location().set_href("/family/matching/results");
                                    }
                                }
                                Ok(resp) => error_msg.set(resp.error),
                                Err(e) => error_msg.set(Some(e)),
                            }
                            submitting.set(false);
                        });
                    }
                >
                    {move || if submitting.get() { "검색 중..." } else { "매칭 검색" }}
                </button>
            </div>
        </div>
    }
}

/// Displays match recommendation results with compatibility scores.
#[component]
pub fn MatchingResultsPage() -> impl IntoView {
    let data = LocalResource::new(|| {
        crate::api::get::<Vec<bominal_types::MatchRecommendation>>("/api/match-requests")
    });

    view! {
        <div class="p-6 space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"매칭 결과"</h1>
                <p class="text-sm text-txt-secondary mt-1">"추천 요양보호사 목록입니다."</p>
            </div>

            <Suspense fallback=move || view! { <div class="animate-pulse bg-gray-200 rounded-xl h-20" /> }>
                {move || Suspend::new(async move {
                    match data.await {
                        Ok(resp) if resp.success => {
                            let items = resp.data.unwrap_or_default();
                            if items.is_empty() {
                                view! {
                                    <p class="text-center text-txt-secondary py-8">"매칭 결과가 없습니다."</p>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-3">
                                        {items.into_iter().map(|rec| {
                                            let href = format!("/family/matching/{}", rec.id);
                                            let score = rec.score;
                                            let rank = rec.rank;
                                            view! {
                                                <a href=href class="block bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200">
                                                    <div class="flex justify-between items-center">
                                                        <div>
                                                            <p class="font-medium text-txt-primary">{format!("추천 #{}", rank)}</p>
                                                            <p class="text-sm text-txt-tertiary">{format!("매칭 점수: {:.0}", score)}</p>
                                                        </div>
                                                        <span class="text-xs px-2 py-1 rounded-full bg-[var(--portal-accent-light)] text-[var(--portal-accent)]">{format!("{:.0}점", score)}</span>
                                                    </div>
                                                </a>
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
    }
}

/// Shows a caregiver profile from a match recommendation with rating and score.
/// Fetches the recommendation list from the API and finds the matching record by id.
#[component]
pub fn MatchingDetailPage() -> impl IntoView {
    let loading = RwSignal::new(false);
    let error_msg = RwSignal::new(None::<String>);
    let success_msg = RwSignal::new(None::<String>);

    // Extract the recommendation id from the URL path (last segment)
    let id = {
        let path = leptos::web_sys::window()
            .and_then(|w| w.location().pathname().ok())
            .unwrap_or_default();
        path.rsplit('/').next().unwrap_or("").to_string()
    };
    let id_for_fetch = id.clone();
    let id_for_click = id.clone();

    let data = LocalResource::new(move || {
        let target_id = id_for_fetch.clone();
        async move {
            let resp = crate::api::get::<Vec<bominal_types::MatchRecommendation>>(
                "/api/match-requests",
            ).await;
            match resp {
                Ok(api_resp) if api_resp.success => {
                    let items = api_resp.data.unwrap_or_default();
                    items.into_iter().find(|r| r.id.to_string() == target_id)
                }
                _ => None,
            }
        }
    });

    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"요양보호사 프로필"</h1>
                <p class="text-sm text-txt-secondary mt-1">"매칭된 요양보호사의 상세 정보입니다."</p>
            </div>

            <Suspense fallback=move || view! { <div class="animate-pulse bg-gray-200 rounded-xl h-40" /> }>
                {move || Suspend::new(async move {
                    match data.await {
                        Some(rec) => {
                            let score = format!("{:.0}", rec.score);
                            let rank = format!("추천 #{}", rec.rank);
                            // Extract extra info from score_breakdown if available
                            let breakdown = rec.score_breakdown.unwrap_or(serde_json::Value::Null);
                            let rating = breakdown.get("rating")
                                .and_then(|v: &serde_json::Value| v.as_f64())
                                .map(|r| format!("{:.1}", r))
                                .unwrap_or_else(|| "-".to_string());
                            let experience = breakdown.get("experience_years")
                                .and_then(|v: &serde_json::Value| v.as_i64())
                                .map(|y| format!("경력 {}년", y))
                                .unwrap_or_else(|| "경력 정보 없음".to_string());
                            let specialty = breakdown.get("specialty")
                                .and_then(|v: &serde_json::Value| v.as_str())
                                .unwrap_or("전문 분야 정보 없음")
                                .to_string();
                            view! {
                                <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                                    <div class="flex items-center gap-4">
                                        <div class="w-14 h-14 bg-[var(--portal-accent-light)] rounded-full flex items-center justify-center">
                                            <span class="text-xl font-bold text-[var(--portal-accent)]">{rank.clone()}</span>
                                        </div>
                                        <div>
                                            <p class="font-semibold text-txt-primary">{rank}</p>
                                            <p class="text-sm text-txt-tertiary">{format!("{} · {}", experience, specialty)}</p>
                                        </div>
                                    </div>
                                    <div class="grid grid-cols-2 gap-3">
                                        <div class="bg-surface-page rounded-xl p-3">
                                            <p class="text-xs text-txt-tertiary">"매칭 점수"</p>
                                            <p class="text-lg font-bold text-[var(--portal-accent)]">
                                                {score.clone()}<span class="text-xs text-txt-disabled">"/100"</span>
                                            </p>
                                        </div>
                                        <div class="bg-surface-page rounded-xl p-3">
                                            <p class="text-xs text-txt-tertiary">"평점"</p>
                                            <p class="text-lg font-bold text-yellow-500">
                                                {rating}<span class="text-xs text-txt-disabled">"/5"</span>
                                            </p>
                                        </div>
                                    </div>
                                </div>
                            }.into_any()
                        }
                        _ => view! {
                            <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                                <p class="text-sm text-txt-tertiary">"추천 정보를 불러올 수 없습니다."</p>
                            </div>
                        }.into_any(),
                    }
                })}
            </Suspense>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                {move || error_msg.get().map(|msg| view! {
                    <p class="text-sm text-danger">{msg}</p>
                })}
                {move || success_msg.get().map(|msg| view! {
                    <div class="bg-success-light rounded-xl p-3">
                        <p class="text-sm font-medium text-success">{msg}</p>
                    </div>
                })}

                <button
                    class="w-full bg-[var(--portal-accent)] text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all disabled:opacity-50"
                    prop:disabled=move || loading.get()
                    on:click={
                        let rec_id = id_for_click.clone();
                        move |_| {
                            let rec_id = rec_id.clone();
                            leptos::task::spawn_local(async move {
                                loading.set(true);
                                error_msg.set(None);
                                success_msg.set(None);
                                let body = serde_json::json!({
                                    "recommendation_id": rec_id,
                                });
                                let url = format!("/api/match-requests/{}/select", rec_id);
                                match crate::api::post::<serde_json::Value, _>(&url, &body).await {
                                    Ok(resp) if resp.success => {
                                        success_msg.set(Some("매칭 요청이 완료되었습니다".to_string()));
                                    }
                                    Ok(resp) => error_msg.set(resp.error),
                                    Err(e) => error_msg.set(Some(e)),
                                }
                                loading.set(false);
                            });
                        }
                    }
                >
                    {move || if loading.get() { "처리 중..." } else { "매칭 요청" }}
                </button>
            </div>
        </div>
    }
}
