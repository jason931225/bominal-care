use leptos::prelude::*;

// =============================================================================
// Payment pages — list and detail
// =============================================================================

/// Displays payment history for care services with status badges.
#[component]
pub fn PaymentsListPage() -> impl IntoView {
    let data = LocalResource::new(|| {
        crate::api::get::<serde_json::Value>("/api/benefits/utilization")
    });

    view! {
        <div class="p-6 space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"결제 내역"</h1>
                <p class="text-sm text-txt-secondary mt-1">"돌봄 서비스 결제 기록입니다."</p>
            </div>

            <div class="space-y-3">
                <Suspense fallback=move || view! { <div class="animate-pulse bg-gray-200 rounded-xl h-20" /> }>
                    {move || Suspend::new(async move {
                        match data.await {
                            Ok(resp) if resp.success => {
                                let payload = resp.data.unwrap_or(serde_json::Value::Null);
                                // The utilization endpoint may return an object or array.
                                // Normalise to a list of items for rendering.
                                let items: Vec<serde_json::Value> = if let Some(arr) = payload.as_array() {
                                    arr.clone()
                                } else if payload.is_object() {
                                    vec![payload]
                                } else {
                                    vec![]
                                };

                                if items.is_empty() {
                                    view! {
                                        <p class="text-center text-txt-secondary py-8">"결제 내역이 없습니다."</p>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="space-y-3">
                                            {items.into_iter().enumerate().map(|(i, item)| {
                                                let id_str = item.get("id")
                                                    .and_then(|v| v.as_str())
                                                    .map(|s| s.to_string())
                                                    .unwrap_or_else(|| i.to_string());
                                                let href = format!("/family/payments/{}", id_str);
                                                let label = item.get("service_name")
                                                    .or_else(|| item.get("program_name"))
                                                    .or_else(|| item.get("label"))
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("결제 항목")
                                                    .to_string();
                                                let period = item.get("period")
                                                    .or_else(|| item.get("date_range"))
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("-")
                                                    .to_string();
                                                let amount = item.get("amount")
                                                    .or_else(|| item.get("total"))
                                                    .and_then(|v| v.as_str().map(|s| s.to_string())
                                                        .or_else(|| v.as_f64().map(|n| format!("₩{:.0}", n))))
                                                    .unwrap_or_else(|| "-".to_string());
                                                let status = item.get("status")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("결제완료")
                                                    .to_string();
                                                view! {
                                                    <a href=href class="block bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200">
                                                        <div class="flex justify-between items-center">
                                                            <div>
                                                                <p class="font-medium text-txt-primary">{label}</p>
                                                                <p class="text-sm text-txt-tertiary">{period}</p>
                                                            </div>
                                                            <div class="text-right">
                                                                <p class="font-bold text-txt-primary">{amount}</p>
                                                                <span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">{status}</span>
                                                            </div>
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

/// Shows a single payment detail view with breakdown and status.
#[component]
pub fn PaymentDetailPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"결제 상세"</h1>
                <p class="text-sm text-txt-secondary mt-1">"결제 건의 상세 정보입니다."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <div class="flex justify-between">
                    <p class="text-sm text-txt-tertiary">"서비스"</p>
                    <p class="font-medium text-txt-primary">"방문요양"</p>
                </div>
                <div class="flex justify-between">
                    <p class="text-sm text-txt-tertiary">"기간"</p>
                    <p class="text-sm text-txt-secondary">"2026-03-01 ~ 2026-03-15"</p>
                </div>
                <div class="flex justify-between">
                    <p class="text-sm text-txt-tertiary">"총 금액"</p>
                    <p class="font-bold text-txt-primary">"₩320,000"</p>
                </div>
                <div class="flex justify-between">
                    <p class="text-sm text-txt-tertiary">"본인부담금 (15%)"</p>
                    <p class="font-bold text-[var(--portal-accent)]">"₩48,000"</p>
                </div>
                <div class="flex justify-between">
                    <p class="text-sm text-txt-tertiary">"결제 상태"</p>
                    <span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"결제완료"</span>
                </div>
                <div class="flex justify-between">
                    <p class="text-sm text-txt-tertiary">"결제일"</p>
                    <p class="text-sm text-txt-secondary">"2026-03-16"</p>
                </div>
            </div>
        </div>
    }
}
