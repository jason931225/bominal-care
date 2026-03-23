use leptos::prelude::*;

// =============================================================================
// Document pages — list and detail
// =============================================================================

/// Lists care-related documents with validity status badges.
#[component]
pub fn DocumentsPage() -> impl IntoView {
    let data = LocalResource::new(|| {
        crate::api::get::<Vec<bominal_types::ConsentRecord>>("/api/consent")
    });

    view! {
        <div class="p-6 space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"서류 관리"</h1>
                <p class="text-sm text-txt-secondary mt-1">"돌봄 관련 서류를 확인하세요."</p>
            </div>

            <div class="space-y-3">
                <Suspense fallback=move || view! { <div class="animate-pulse bg-gray-200 rounded-xl h-20" /> }>
                    {move || Suspend::new(async move {
                        match data.await {
                            Ok(resp) if resp.success => {
                                let items = resp.data.unwrap_or_default();
                                if items.is_empty() {
                                    view! {
                                        <p class="text-center text-txt-secondary py-8">"데이터가 없습니다."</p>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="space-y-3">
                                            {items.into_iter().map(|record| {
                                                let href = format!("/family/documents/{}", record.id);
                                                let purpose = format!("{}", record.purpose);
                                                let granted_at = record.granted_at.format("%Y-%m-%d").to_string();
                                                let (badge_class, badge_text) = if record.is_active {
                                                    ("text-xs px-2 py-1 rounded-full bg-success-light text-success", "유효")
                                                } else {
                                                    ("text-xs px-2 py-1 rounded-full bg-gray-100 text-txt-disabled", "만료")
                                                };
                                                view! {
                                                    <a href=href class="block bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200">
                                                        <div class="flex justify-between items-center">
                                                            <div>
                                                                <p class="font-medium text-txt-primary">{purpose}</p>
                                                                <p class="text-sm text-txt-tertiary">{granted_at}" 발급"</p>
                                                            </div>
                                                            <span class=badge_class>{badge_text}</span>
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
        </div>
    }
}

/// Shows a single document detail, fetched from consent records API.
#[component]
pub fn DocumentDetailPage() -> impl IntoView {
    // Extract the id from the current URL path (last segment)
    let id = {
        let path = leptos::web_sys::window()
            .and_then(|w| w.location().pathname().ok())
            .unwrap_or_default();
        path.rsplit('/').next().unwrap_or("").to_string()
    };

    let data = LocalResource::new(move || {
        let target_id = id.clone();
        async move {
            let resp = crate::api::get::<Vec<bominal_types::ConsentRecord>>("/api/consent").await;
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
                <h1 class="text-xl font-bold text-txt-primary">"서류 상세"</h1>
                <p class="text-sm text-txt-secondary mt-1">"서류 내용을 확인하세요."</p>
            </div>

            <Suspense fallback=move || view! { <div class="animate-pulse bg-gray-200 rounded-xl h-40" /> }>
                {move || Suspend::new(async move {
                    match data.await {
                        Some(record) => {
                            let purpose = format!("{}", record.purpose);
                            let granted_at = record.granted_at.format("%Y-%m-%d").to_string();
                            let expires_at = record.expires_at
                                .map(|d: chrono::DateTime<chrono::Utc>| d.format("%Y-%m-%d").to_string())
                                .unwrap_or_else(|| "없음".to_string());
                            let (badge_class, badge_text) = if record.is_active {
                                ("text-xs px-2 py-1 rounded-full bg-success-light text-success", "유효")
                            } else {
                                ("text-xs px-2 py-1 rounded-full bg-gray-100 text-txt-disabled", "만료")
                            };
                            view! {
                                <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                                    <div class="flex justify-between">
                                        <p class="text-sm text-txt-tertiary">"서류명"</p>
                                        <p class="font-medium text-txt-primary">{purpose}</p>
                                    </div>
                                    <div class="flex justify-between">
                                        <p class="text-sm text-txt-tertiary">"발급일"</p>
                                        <p class="text-sm text-txt-secondary">{granted_at}</p>
                                    </div>
                                    <div class="flex justify-between">
                                        <p class="text-sm text-txt-tertiary">"만료일"</p>
                                        <p class="text-sm text-txt-secondary">{expires_at}</p>
                                    </div>
                                    <div class="flex justify-between">
                                        <p class="text-sm text-txt-tertiary">"상태"</p>
                                        <span class=badge_class>{badge_text}</span>
                                    </div>
                                    <div class="bg-surface-page rounded-xl p-8 text-center">
                                        <p class="text-sm text-txt-disabled">"동의서 상세 내용"</p>
                                    </div>
                                </div>
                            }.into_any()
                        }
                        _ => view! {
                            <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                                <p class="text-sm text-txt-tertiary">"서류 정보를 불러올 수 없습니다."</p>
                            </div>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
