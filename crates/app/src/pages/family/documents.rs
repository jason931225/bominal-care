use leptos::prelude::*;
use uuid::Uuid;

// =============================================================================
// Document pages — list and detail
// =============================================================================

/// Lists care-related documents with validity status badges.
#[component]
pub fn DocumentsPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-8">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"서류 관리"</h1>
                <p class="text-sm text-txt-secondary mt-1">"돌봄 관련 서류를 확인하세요."</p>
            </div>

            <div class="space-y-3">
                {
                    let id1 = Uuid::new_v5(&Uuid::NAMESPACE_OID, b"doc-ltci-certificate").to_string();
                    let id2 = Uuid::new_v5(&Uuid::NAMESPACE_OID, b"doc-ltci-care-plan").to_string();
                    let id3 = Uuid::new_v5(&Uuid::NAMESPACE_OID, b"doc-service-contract").to_string();
                    let href1 = format!("/family/documents/{id1}");
                    let href2 = format!("/family/documents/{id2}");
                    let href3 = format!("/family/documents/{id3}");
                    view! {
                        <a href=href1 class="block bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200">
                            <div class="flex justify-between items-center">
                                <div>
                                    <p class="font-medium text-txt-primary">"장기요양 인정서"</p>
                                    <p class="text-sm text-txt-tertiary">"2025-12-15 발급"</p>
                                </div>
                                <span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"유효"</span>
                            </div>
                        </a>
                        <a href=href2 class="block bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200">
                            <div class="flex justify-between items-center">
                                <div>
                                    <p class="font-medium text-txt-primary">"표준 장기요양 이용계획서"</p>
                                    <p class="text-sm text-txt-tertiary">"2025-12-20 발급"</p>
                                </div>
                                <span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"유효"</span>
                            </div>
                        </a>
                        <a href=href3 class="block bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200">
                            <div class="flex justify-between items-center">
                                <div>
                                    <p class="font-medium text-txt-primary">"서비스 이용 계약서"</p>
                                    <p class="text-sm text-txt-tertiary">"2026-01-05 발급"</p>
                                </div>
                                <span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"유효"</span>
                            </div>
                        </a>
                    }
                }
            </div>
        </div>
    }
}

/// Shows a single document with preview area and download option.
#[component]
pub fn DocumentDetailPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-8 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-txt-primary">"서류 상세"</h1>
                <p class="text-sm text-txt-secondary mt-1">"서류 내용을 확인하세요."</p>
            </div>

            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <div class="flex justify-between">
                    <p class="text-sm text-txt-tertiary">"서류명"</p>
                    <p class="font-medium text-txt-primary">"장기요양 인정서"</p>
                </div>
                <div class="flex justify-between">
                    <p class="text-sm text-txt-tertiary">"발급일"</p>
                    <p class="text-sm text-txt-secondary">"2025-12-15"</p>
                </div>
                <div class="flex justify-between">
                    <p class="text-sm text-txt-tertiary">"상태"</p>
                    <span class="text-xs px-2 py-1 rounded-full bg-success-light text-success">"유효"</span>
                </div>
                <div class="bg-surface-page rounded-xl p-8 text-center">
                    <p class="text-sm text-txt-disabled">"서류 미리보기"</p>
                </div>
                <button class="w-full border border-gray-200 text-txt-secondary rounded-xl px-4 py-2.5 text-sm font-medium hover:bg-surface-page active:scale-[0.98] transition-all">
                    "다운로드"
                </button>
            </div>
        </div>
    }
}
