use leptos::prelude::*;
use uuid::Uuid;

use bominal_types::PersonProfile;
use crate::components::data_display::EmptyState;
use crate::components::layout::PageHeader;
use super::InfoRow;

/// Profile API response — server returns PersonProfile directly.
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
                                                value=p.korean_name.unwrap_or_else(|| "—".to_string())
                                            />
                                            <InfoRow
                                                label="전화번호".to_string()
                                                value={
                                                    let raw = p.phone.unwrap_or_else(|| "—".to_string());
                                                    if raw != "—" { crate::api::format_phone_kr(&raw) } else { raw }
                                                }
                                            />
                                            <InfoRow
                                                label="주소".to_string()
                                                value=p.address.unwrap_or_else(|| "—".to_string())
                                            />
                                            <InfoRow
                                                label="긴급 연락처 (이름)".to_string()
                                                value=p.emergency_contact_name.unwrap_or_else(|| "—".to_string())
                                            />
                                            <InfoRow
                                                label="긴급 연락처 (전화)".to_string()
                                                value=p.emergency_contact_phone.unwrap_or_else(|| "—".to_string())
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

/// Emergency contacts with tel: links and health info.
#[component]
pub fn EmergencyPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="긴급 연락" subtitle="긴급 상황 시 사용하세요" />

            // 119 Emergency call
            <a href="tel:119"
               class="block w-full bg-danger text-white text-center text-xl font-bold \
                      rounded-2xl py-6 shadow-lg hover:bg-danger-hover active:scale-[0.98] transition-all">
                "119 응급 전화"
            </a>

            <div class="bg-danger-light rounded-2xl p-5 shadow-sm">
                <p class="text-lg text-danger">"프로필 정보를 불러올 수 없습니다."</p>
            </div>
        </div>
    }
}

/// Consent toggles for data sharing.
#[component]
pub fn ConsentPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="동의 관리" subtitle="데이터 공유 및 개인정보 동의" />
            <EmptyState message="등록된 동의 내역이 없습니다." />
        </div>
    }
}

/// Single consent record detail.
#[component]
pub fn ConsentDetailPage(
    #[prop(into)] consent_id: Uuid,
) -> impl IntoView {
    let _ = consent_id;
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/consent" class="text-primary text-lg">"< 동의 관리"</a>
            <EmptyState message="동의 기록을 찾을 수 없습니다." />
        </div>
    }
}

/// Paginated list of medical history conditions.
#[component]
pub fn MedicalHistoryPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="병력 기록" subtitle="과거 및 현재 질환 기록" />
            <EmptyState message="병력 기록이 없습니다." />
        </div>
    }
}

/// Single medical history condition detail.
#[component]
pub fn MedicalHistoryDetailPage(
    #[prop(into)] entry_id: Uuid,
) -> impl IntoView {
    let _ = entry_id;
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/medical-history" class="text-primary text-lg">"< 병력 기록"</a>
            <EmptyState message="기록을 찾을 수 없습니다." />
        </div>
    }
}

/// Care plan overview -- list of care plans.
#[component]
pub fn CarePlanPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="케어 플랜" subtitle="돌봄 계획을 확인하세요" />
            <EmptyState message="등록된 케어 플랜이 없습니다." />
        </div>
    }
}

/// Care plan detail with visits.
#[component]
pub fn CarePlanDetailPage(
    #[prop(into)] plan_id: Uuid,
) -> impl IntoView {
    let _ = plan_id;
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/care" class="text-primary text-lg">"< 케어 플랜"</a>
            <EmptyState message="케어 플랜을 찾을 수 없습니다." />
        </div>
    }
}
