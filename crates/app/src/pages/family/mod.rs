use leptos::prelude::*;
use crate::server_fns::*;

// =============================================================================
// Family Portal Pages — 23 components
// =============================================================================

/// Family dashboard: linked senior overview, timeline preview, alerts.
#[component]
pub fn DashboardPage() -> impl IntoView {
    let notifications = Resource::new(|| (), |_| notifications::get_unread_count(
        uuid::Uuid::nil(),
    ));

    view! {
        <div class="p-6 space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"가족 케어 대시보드"</h1>
                <p class="text-sm text-gray-600 mt-1">"돌봄 대상자의 현황을 한눈에 확인하세요."</p>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <h3 class="text-sm text-gray-500">"오늘의 일정"</h3>
                    <p class="text-2xl font-bold text-gray-900 mt-1">"2"<span class="text-sm font-normal text-gray-500">" 건"</span></p>
                    <p class="text-sm text-gray-500 mt-1">"방문요양 10:00, 방문목욕 14:00"</p>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <h3 class="text-sm text-gray-500">"복약 상태"</h3>
                    <p class="text-2xl font-bold text-green-600 mt-1">"정상"</p>
                    <p class="text-sm text-gray-500 mt-1">"오전 약 복용 완료"</p>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <h3 class="text-sm text-gray-500">"알림"</h3>
                    <Suspense fallback=move || view! { <p class="text-2xl font-bold text-gray-300">"..."</p> }>
                        {move || notifications.get().map(|res| match res {
                            Ok(count) => view! {
                                <p class="text-2xl font-bold text-orange-600">{count}<span class="text-sm font-normal text-gray-500">" 건"</span></p>
                            }.into_any(),
                            Err(_) => view! {
                                <p class="text-2xl font-bold text-red-400">"오류"</p>
                            }.into_any(),
                        })}
                    </Suspense>
                </div>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h2 class="font-semibold text-gray-900 mb-3">"케어 타임라인 미리보기"</h2>
                <ul class="space-y-2 text-sm text-gray-600">
                    <li class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-blue-500"></span>
                        "10:00 — 방문요양 (김요양 보호사)"
                    </li>
                    <li class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-teal-500"></span>
                        "14:00 — 방문목욕"
                    </li>
                </ul>
                <a href="/family/timeline" class="text-sm text-blue-600 hover:underline mt-3 inline-block">"전체 타임라인 보기 →"</a>
            </div>
        </div>
    }
}

/// 30-day care timeline: visits, meds, appointments.
#[component]
pub fn TimelinePage() -> impl IntoView {
    let visits = Resource::new(|| (), |_| schedule::list_visits(
        None, None, None, 1, 30,
    ));

    view! {
        <div class="p-6 space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"케어 타임라인"</h1>
                <p class="text-sm text-gray-600 mt-1">"최근 30일간의 돌봄 기록입니다."</p>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <Suspense fallback=move || view! { <p class="text-sm text-gray-400">"불러오는 중..."</p> }>
                    {move || visits.get().map(|res| match res {
                        Ok(list) => {
                            if list.data.is_empty() {
                                view! { <p class="text-sm text-gray-500">"기록이 없습니다."</p> }.into_any()
                            } else {
                                view! {
                                    <ul class="divide-y divide-gray-100">
                                        {list.data.into_iter().map(|v| view! {
                                            <li class="py-3 flex justify-between items-center">
                                                <div>
                                                    <p class="text-sm font-medium text-gray-900">{v.tasks.clone().map(|t| t.to_string()).unwrap_or_default()}</p>
                                                    <p class="text-xs text-gray-500">{v.scheduled_start.format("%m/%d %H:%M").to_string()}</p>
                                                </div>
                                                <span class="text-xs px-2 py-1 rounded-full bg-blue-50 text-blue-700">{v.status.to_string()}</span>
                                            </li>
                                        }).collect_view()}
                                    </ul>
                                }.into_any()
                            }
                        }
                        Err(_) => view! { <p class="text-sm text-red-500">"타임라인을 불러올 수 없습니다."</p> }.into_any(),
                    })}
                </Suspense>
            </div>
        </div>
    }
}

/// Search form for caregiver matching.
#[component]
pub fn MatchingSearchPage() -> impl IntoView {
    let (region, set_region) = signal(String::new());
    let (service, set_service) = signal(String::new());

    view! {
        <div class="p-6 space-y-6 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"요양보호사 매칭"</h1>
                <p class="text-sm text-gray-600 mt-1">"조건을 입력하고 매칭을 시작하세요."</p>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">"지역"</label>
                    <input
                        type="text"
                        class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                        placeholder="예: 서울시 강남구"
                        prop:value=move || region.get()
                        on:input=move |ev| set_region.set(event_target_value(&ev))
                    />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">"서비스 유형"</label>
                    <select
                        class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                        on:change=move |ev| set_service.set(event_target_value(&ev))
                    >
                        <option value="">"선택하세요"</option>
                        <option value="home_care">"방문요양"</option>
                        <option value="home_bath">"방문목욕"</option>
                        <option value="home_nursing">"방문간호"</option>
                        <option value="day_care">"주야간보호"</option>
                    </select>
                </div>
                <a
                    href="/family/matching/results"
                    class="block w-full text-center bg-blue-600 text-white rounded-lg px-4 py-2.5 text-sm font-medium hover:bg-blue-700 transition-colors"
                >
                    "매칭 검색"
                </a>
            </div>
        </div>
    }
}

/// Match recommendation results with scores.
#[component]
pub fn MatchingResultsPage() -> impl IntoView {
    let matches = Resource::new(|| (), |_| matching::list_match_requests(
        None, 1, 10,
    ));

    view! {
        <div class="p-6 space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"매칭 결과"</h1>
                <p class="text-sm text-gray-600 mt-1">"추천 요양보호사 목록입니다."</p>
            </div>

            <Suspense fallback=move || view! { <p class="text-sm text-gray-400">"검색 중..."</p> }>
                {move || matches.get().map(|res| match res {
                    Ok(list) => view! {
                        <div class="space-y-3">
                            {list.data.into_iter().map(|m| view! {
                                <a href={format!("/family/matching/{}", m.id)} class="block bg-white rounded-xl p-5 shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
                                    <div class="flex justify-between items-center">
                                        <div>
                                            <p class="font-medium text-gray-900">{m.service_category.to_string()}</p>
                                            <p class="text-sm text-gray-500">{m.region_city.clone()}" "{m.region_district.clone()}</p>
                                        </div>
                                        <span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">{m.status.to_string()}</span>
                                    </div>
                                </a>
                            }).collect_view()}
                        </div>
                    }.into_any(),
                    Err(_) => view! { <p class="text-sm text-red-500">"결과를 불러올 수 없습니다."</p> }.into_any(),
                })}
            </Suspense>
        </div>
    }
}

/// Caregiver profile from a match recommendation.
#[component]
pub fn MatchingDetailPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-6 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"요양보호사 프로필"</h1>
                <p class="text-sm text-gray-600 mt-1">"매칭된 요양보호사의 상세 정보입니다."</p>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <div class="flex items-center gap-4">
                    <div class="w-14 h-14 bg-blue-100 rounded-full flex items-center justify-center">
                        <span class="text-xl font-bold text-blue-600">"김"</span>
                    </div>
                    <div>
                        <p class="font-semibold text-gray-900">"김요양"</p>
                        <p class="text-sm text-gray-500">"경력 8년 · 치매 전문"</p>
                    </div>
                </div>
                <div class="grid grid-cols-2 gap-3">
                    <div class="bg-gray-50 rounded-lg p-3">
                        <p class="text-xs text-gray-500">"매칭 점수"</p>
                        <p class="text-lg font-bold text-blue-600">"92"<span class="text-xs text-gray-400">"/100"</span></p>
                    </div>
                    <div class="bg-gray-50 rounded-lg p-3">
                        <p class="text-xs text-gray-500">"평점"</p>
                        <p class="text-lg font-bold text-yellow-500">"4.8"<span class="text-xs text-gray-400">"/5"</span></p>
                    </div>
                </div>
                <button class="w-full bg-blue-600 text-white rounded-lg px-4 py-2.5 text-sm font-medium hover:bg-blue-700 transition-colors">
                    "매칭 요청"
                </button>
            </div>
        </div>
    }
}

/// Pending approvals list.
#[component]
pub fn ApprovalsListPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"승인 대기 목록"</h1>
                <p class="text-sm text-gray-600 mt-1">"결정이 필요한 항목들입니다."</p>
            </div>

            <div class="space-y-3">
                <a href="/family/approvals/1" class="block bg-white rounded-xl p-5 shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
                    <div class="flex justify-between items-center">
                        <div>
                            <p class="font-medium text-gray-900">"케어 플랜 변경 승인"</p>
                            <p class="text-sm text-gray-500">"방문 시간 변경 요청"</p>
                        </div>
                        <span class="text-xs px-2 py-1 rounded-full bg-orange-50 text-orange-700">"대기 중"</span>
                    </div>
                </a>
                <a href="/family/approvals/2" class="block bg-white rounded-xl p-5 shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
                    <div class="flex justify-between items-center">
                        <div>
                            <p class="font-medium text-gray-900">"약물 변경 승인"</p>
                            <p class="text-sm text-gray-500">"처방 변경에 대한 동의 필요"</p>
                        </div>
                        <span class="text-xs px-2 py-1 rounded-full bg-orange-50 text-orange-700">"대기 중"</span>
                    </div>
                </a>
            </div>
        </div>
    }
}

/// Individual approval detail with approve/reject actions.
#[component]
pub fn ApprovalDetailPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-6 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"승인 상세"</h1>
                <p class="text-sm text-gray-600 mt-1">"승인 요청의 상세 내용입니다."</p>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <div>
                    <p class="text-sm text-gray-500">"요청 유형"</p>
                    <p class="font-medium text-gray-900">"케어 플랜 변경"</p>
                </div>
                <div>
                    <p class="text-sm text-gray-500">"요청 내용"</p>
                    <p class="text-sm text-gray-700">"방문 시간을 오전 10시에서 오후 2시로 변경 요청합니다."</p>
                </div>
                <div>
                    <p class="text-sm text-gray-500">"요청일"</p>
                    <p class="text-sm text-gray-700">"2026-03-15"</p>
                </div>
                <div class="flex gap-3">
                    <button class="flex-1 bg-blue-600 text-white rounded-lg px-4 py-2.5 text-sm font-medium hover:bg-blue-700">"승인"</button>
                    <button class="flex-1 border border-red-300 text-red-600 rounded-lg px-4 py-2.5 text-sm font-medium hover:bg-red-50">"거부"</button>
                </div>
            </div>
        </div>
    }
}

/// Payment history list.
#[component]
pub fn PaymentsListPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"결제 내역"</h1>
                <p class="text-sm text-gray-600 mt-1">"돌봄 서비스 결제 기록입니다."</p>
            </div>

            <div class="space-y-3">
                <a href="/family/payments/1" class="block bg-white rounded-xl p-5 shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
                    <div class="flex justify-between items-center">
                        <div>
                            <p class="font-medium text-gray-900">"3월 방문요양 서비스"</p>
                            <p class="text-sm text-gray-500">"2026-03-01 ~ 2026-03-15"</p>
                        </div>
                        <div class="text-right">
                            <p class="font-bold text-gray-900">"₩320,000"</p>
                            <span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"결제완료"</span>
                        </div>
                    </div>
                </a>
                <a href="/family/payments/2" class="block bg-white rounded-xl p-5 shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
                    <div class="flex justify-between items-center">
                        <div>
                            <p class="font-medium text-gray-900">"2월 방문요양 서비스"</p>
                            <p class="text-sm text-gray-500">"2026-02-01 ~ 2026-02-28"</p>
                        </div>
                        <div class="text-right">
                            <p class="font-bold text-gray-900">"₩640,000"</p>
                            <span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"결제완료"</span>
                        </div>
                    </div>
                </a>
            </div>
        </div>
    }
}

/// Single payment detail view.
#[component]
pub fn PaymentDetailPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-6 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"결제 상세"</h1>
                <p class="text-sm text-gray-600 mt-1">"결제 건의 상세 정보입니다."</p>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <div class="flex justify-between">
                    <p class="text-sm text-gray-500">"서비스"</p>
                    <p class="font-medium text-gray-900">"방문요양"</p>
                </div>
                <div class="flex justify-between">
                    <p class="text-sm text-gray-500">"기간"</p>
                    <p class="text-sm text-gray-700">"2026-03-01 ~ 2026-03-15"</p>
                </div>
                <div class="flex justify-between">
                    <p class="text-sm text-gray-500">"총 금액"</p>
                    <p class="font-bold text-gray-900">"₩320,000"</p>
                </div>
                <div class="flex justify-between">
                    <p class="text-sm text-gray-500">"본인부담금 (15%)"</p>
                    <p class="font-bold text-blue-600">"₩48,000"</p>
                </div>
                <div class="flex justify-between">
                    <p class="text-sm text-gray-500">"결제 상태"</p>
                    <span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"결제완료"</span>
                </div>
                <div class="flex justify-between">
                    <p class="text-sm text-gray-500">"결제일"</p>
                    <p class="text-sm text-gray-700">"2026-03-16"</p>
                </div>
            </div>
        </div>
    }
}

/// Senior's medications overview.
#[component]
pub fn MedicationsPage() -> impl IntoView {
    let meds = Resource::new(|| (), |_| medications::list_medications(
        uuid::Uuid::nil(),
    ));

    view! {
        <div class="p-6 space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"복약 관리"</h1>
                <p class="text-sm text-gray-600 mt-1">"어르신의 복약 현황입니다."</p>
            </div>

            <Suspense fallback=move || view! { <p class="text-sm text-gray-400">"불러오는 중..."</p> }>
                {move || meds.get().map(|res| match res {
                    Ok(list) => {
                        if list.is_empty() {
                            view! { <p class="text-sm text-gray-500">"등록된 약물이 없습니다."</p> }.into_any()
                        } else {
                            view! {
                                <div class="space-y-3">
                                    {list.into_iter().map(|m| view! {
                                        <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                            <div class="flex justify-between items-start">
                                                <div>
                                                    <p class="font-medium text-gray-900">{m.medication.name.clone()}</p>
                                                    <p class="text-sm text-gray-500">{m.medication.dosage.clone()}" · "{m.medication.frequency.to_string()}</p>
                                                </div>
                                                <span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"복용 중"</span>
                                            </div>
                                        </div>
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    }
                    Err(_) => view! { <p class="text-sm text-red-500">"복약 정보를 불러올 수 없습니다."</p> }.into_any(),
                })}
            </Suspense>
        </div>
    }
}

/// Senior's care plan view.
#[component]
pub fn CarePlanPage() -> impl IntoView {
    let plans = Resource::new(|| (), |_| care_plans::list_care_plans(
        uuid::Uuid::nil(), 1, 10,
    ));

    view! {
        <div class="p-6 space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"케어 플랜"</h1>
                <p class="text-sm text-gray-600 mt-1">"어르신의 돌봄 계획입니다."</p>
            </div>

            <Suspense fallback=move || view! { <p class="text-sm text-gray-400">"불러오는 중..."</p> }>
                {move || plans.get().map(|res| match res {
                    Ok(list) => {
                        if list.data.is_empty() {
                            view! { <p class="text-sm text-gray-500">"등록된 케어 플랜이 없습니다."</p> }.into_any()
                        } else {
                            view! {
                                <div class="space-y-3">
                                    {list.data.into_iter().map(|p| view! {
                                        <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                            <p class="font-medium text-gray-900">{p.title.clone()}</p>
                                            <p class="text-sm text-gray-500 mt-1">{p.description.clone().unwrap_or_default()}</p>
                                            <p class="text-xs text-gray-400 mt-2">{p.status.to_string()}</p>
                                        </div>
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    }
                    Err(_) => view! { <p class="text-sm text-red-500">"케어 플랜을 불러올 수 없습니다."</p> }.into_any(),
                })}
            </Suspense>
        </div>
    }
}

/// Help options menu for the senior.
#[component]
pub fn HelpSeniorPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-6 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"도움이 필요하신가요?"</h1>
                <p class="text-sm text-gray-600 mt-1">"어르신을 위한 도움 옵션입니다."</p>
            </div>

            <div class="space-y-3">
                <a href="/family/help/book" class="block bg-white rounded-xl p-5 shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
                    <p class="font-medium text-gray-900">"서비스 예약"</p>
                    <p class="text-sm text-gray-500 mt-1">"방문요양, 방문목욕 등 서비스를 예약합니다."</p>
                </a>
                <a href="/family/help/emergency" class="block bg-white rounded-xl p-5 shadow-sm border border-red-100 hover:shadow-md transition-shadow">
                    <p class="font-medium text-red-700">"긴급 연락처"</p>
                    <p class="text-sm text-gray-500 mt-1">"응급 상황 시 즉시 연락할 수 있습니다."</p>
                </a>
                <a href="/family/help/report" class="block bg-white rounded-xl p-5 shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
                    <p class="font-medium text-gray-900">"우려사항 신고"</p>
                    <p class="text-sm text-gray-500 mt-1">"돌봄 관련 우려사항을 신고합니다."</p>
                </a>
            </div>
        </div>
    }
}

/// Book a service for the senior.
#[component]
pub fn HelpBookPage() -> impl IntoView {
    let (service_type, set_service_type) = signal(String::new());
    let (preferred_date, set_preferred_date) = signal(String::new());

    view! {
        <div class="p-6 space-y-6 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"서비스 예약"</h1>
                <p class="text-sm text-gray-600 mt-1">"어르신을 위한 서비스를 예약하세요."</p>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">"서비스 유형"</label>
                    <select
                        class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm"
                        on:change=move |ev| set_service_type.set(event_target_value(&ev))
                    >
                        <option value="">"선택하세요"</option>
                        <option value="home_care">"방문요양"</option>
                        <option value="home_bath">"방문목욕"</option>
                        <option value="home_nursing">"방문간호"</option>
                    </select>
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">"희망 날짜"</label>
                    <input
                        type="date"
                        class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm"
                        prop:value=move || preferred_date.get()
                        on:input=move |ev| set_preferred_date.set(event_target_value(&ev))
                    />
                </div>
                <button class="w-full bg-blue-600 text-white rounded-lg px-4 py-2.5 text-sm font-medium hover:bg-blue-700 transition-colors">
                    "예약 요청"
                </button>
            </div>
        </div>
    }
}

/// Emergency contacts for the senior.
#[component]
pub fn HelpEmergencyPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-6 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-red-700">"긴급 연락처"</h1>
                <p class="text-sm text-gray-600 mt-1">"응급 상황 시 아래 번호로 연락하세요."</p>
            </div>

            <div class="space-y-3">
                <div class="bg-red-50 rounded-xl p-5 border border-red-100">
                    <p class="font-semibold text-red-800">"119 응급전화"</p>
                    <a href="tel:119" class="text-lg font-bold text-red-700 mt-1 block">"119"</a>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <p class="font-medium text-gray-900">"담당 요양보호사"</p>
                    <a href="tel:010-1234-5678" class="text-sm text-blue-600 mt-1 block">"010-1234-5678"</a>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <p class="font-medium text-gray-900">"담당 기관"</p>
                    <a href="tel:02-1234-5678" class="text-sm text-blue-600 mt-1 block">"02-1234-5678"</a>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <p class="font-medium text-gray-900">"국민건강보험공단"</p>
                    <a href="tel:1577-1000" class="text-sm text-blue-600 mt-1 block">"1577-1000"</a>
                </div>
            </div>
        </div>
    }
}

/// Report a concern form.
#[component]
pub fn HelpReportPage() -> impl IntoView {
    let (category, set_category) = signal(String::new());
    let (description, set_description) = signal(String::new());

    view! {
        <div class="p-6 space-y-6 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"우려사항 신고"</h1>
                <p class="text-sm text-gray-600 mt-1">"돌봄 관련 문제를 신고해 주세요."</p>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">"신고 유형"</label>
                    <select
                        class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm"
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
                    <label class="block text-sm font-medium text-gray-700 mb-1">"상세 설명"</label>
                    <textarea
                        rows=4
                        class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm"
                        placeholder="구체적인 내용을 적어주세요."
                        prop:value=move || description.get()
                        on:input=move |ev| set_description.set(event_target_value(&ev))
                    ></textarea>
                </div>
                <button class="w-full bg-red-600 text-white rounded-lg px-4 py-2.5 text-sm font-medium hover:bg-red-700 transition-colors">
                    "신고 제출"
                </button>
            </div>
        </div>
    }
}

/// Care quality observability signals.
#[component]
pub fn ObservabilityPage() -> impl IntoView {
    let signals = Resource::new(|| (), |_| observability::list_signals(
        None, None, None, None, 1, 20,
    ));

    view! {
        <div class="p-6 space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"케어 품질 신호"</h1>
                <p class="text-sm text-gray-600 mt-1">"돌봄 품질 관련 신호를 확인하세요."</p>
            </div>

            <Suspense fallback=move || view! { <p class="text-sm text-gray-400">"불러오는 중..."</p> }>
                {move || signals.get().map(|res| match res {
                    Ok(list) => {
                        if list.data.is_empty() {
                            view! { <p class="text-sm text-gray-500">"신호가 없습니다."</p> }.into_any()
                        } else {
                            view! {
                                <div class="space-y-3">
                                    {list.data.into_iter().map(|s| view! {
                                        <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
                                            <div class="flex justify-between items-start">
                                                <div>
                                                    <p class="font-medium text-gray-900">{s.message.clone()}</p>
                                                    <p class="text-xs text-gray-500 mt-1">{s.event_type.to_string()}</p>
                                                </div>
                                                <span class="text-xs px-2 py-1 rounded-full bg-yellow-50 text-yellow-700">{s.severity.to_string()}</span>
                                            </div>
                                        </div>
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    }
                    Err(_) => view! { <p class="text-sm text-red-500">"신호를 불러올 수 없습니다."</p> }.into_any(),
                })}
            </Suspense>
        </div>
    }
}

/// LTCI eligibility information.
#[component]
pub fn EligibilityPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-6 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"장기요양 등급 안내"</h1>
                <p class="text-sm text-gray-600 mt-1">"장기요양보험 등급 판정 정보입니다."</p>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <div>
                    <p class="text-sm text-gray-500">"현재 등급"</p>
                    <p class="text-lg font-bold text-blue-600">"3등급"</p>
                </div>
                <div>
                    <p class="text-sm text-gray-500">"판정일"</p>
                    <p class="text-sm text-gray-700">"2025-12-15"</p>
                </div>
                <div>
                    <p class="text-sm text-gray-500">"유효기간"</p>
                    <p class="text-sm text-gray-700">"2025-12-15 ~ 2027-12-14"</p>
                </div>
                <div class="bg-blue-50 rounded-lg p-4">
                    <p class="text-sm text-blue-800">"등급 갱신이 필요하시면 아래 버튼을 눌러 신청하세요."</p>
                </div>
                <a href="/family/eligibility/apply" class="block text-center bg-blue-600 text-white rounded-lg px-4 py-2.5 text-sm font-medium hover:bg-blue-700 transition-colors">
                    "등급 신청 / 갱신"
                </a>
            </div>
        </div>
    }
}

/// Apply for LTCI eligibility.
#[component]
pub fn EligibilityApplyPage() -> impl IntoView {
    let (applicant_name, set_applicant_name) = signal(String::new());
    let (reason, set_reason) = signal(String::new());

    view! {
        <div class="p-6 space-y-6 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"장기요양 등급 신청"</h1>
                <p class="text-sm text-gray-600 mt-1">"등급 판정을 위한 신청서를 작성하세요."</p>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">"신청인 이름"</label>
                    <input
                        type="text"
                        class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm"
                        placeholder="이름을 입력하세요"
                        prop:value=move || applicant_name.get()
                        on:input=move |ev| set_applicant_name.set(event_target_value(&ev))
                    />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">"신청 사유"</label>
                    <textarea
                        rows=3
                        class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm"
                        placeholder="신청 사유를 적어주세요"
                        prop:value=move || reason.get()
                        on:input=move |ev| set_reason.set(event_target_value(&ev))
                    ></textarea>
                </div>
                <div class="bg-yellow-50 rounded-lg p-4">
                    <p class="text-sm text-yellow-800">"신청 후 국민건강보험공단에서 방문 조사가 진행됩니다."</p>
                </div>
                <button class="w-full bg-blue-600 text-white rounded-lg px-4 py-2.5 text-sm font-medium hover:bg-blue-700 transition-colors">
                    "신청서 제출"
                </button>
            </div>
        </div>
    }
}

/// Documents list.
#[component]
pub fn DocumentsPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"서류 관리"</h1>
                <p class="text-sm text-gray-600 mt-1">"돌봄 관련 서류를 확인하세요."</p>
            </div>

            <div class="space-y-3">
                <a href="/family/documents/1" class="block bg-white rounded-xl p-5 shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
                    <div class="flex justify-between items-center">
                        <div>
                            <p class="font-medium text-gray-900">"장기요양 인정서"</p>
                            <p class="text-sm text-gray-500">"2025-12-15 발급"</p>
                        </div>
                        <span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"유효"</span>
                    </div>
                </a>
                <a href="/family/documents/2" class="block bg-white rounded-xl p-5 shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
                    <div class="flex justify-between items-center">
                        <div>
                            <p class="font-medium text-gray-900">"표준 장기요양 이용계획서"</p>
                            <p class="text-sm text-gray-500">"2025-12-20 발급"</p>
                        </div>
                        <span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"유효"</span>
                    </div>
                </a>
                <a href="/family/documents/3" class="block bg-white rounded-xl p-5 shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
                    <div class="flex justify-between items-center">
                        <div>
                            <p class="font-medium text-gray-900">"서비스 이용 계약서"</p>
                            <p class="text-sm text-gray-500">"2026-01-05 발급"</p>
                        </div>
                        <span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"유효"</span>
                    </div>
                </a>
            </div>
        </div>
    }
}

/// Single document viewer.
#[component]
pub fn DocumentDetailPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-6 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"서류 상세"</h1>
                <p class="text-sm text-gray-600 mt-1">"서류 내용을 확인하세요."</p>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <div class="flex justify-between">
                    <p class="text-sm text-gray-500">"서류명"</p>
                    <p class="font-medium text-gray-900">"장기요양 인정서"</p>
                </div>
                <div class="flex justify-between">
                    <p class="text-sm text-gray-500">"발급일"</p>
                    <p class="text-sm text-gray-700">"2025-12-15"</p>
                </div>
                <div class="flex justify-between">
                    <p class="text-sm text-gray-500">"상태"</p>
                    <span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"유효"</span>
                </div>
                <div class="bg-gray-50 rounded-lg p-8 text-center">
                    <p class="text-sm text-gray-400">"서류 미리보기"</p>
                </div>
                <button class="w-full border border-gray-300 text-gray-700 rounded-lg px-4 py-2.5 text-sm font-medium hover:bg-gray-50 transition-colors">
                    "다운로드"
                </button>
            </div>
        </div>
    }
}

/// Notifications list.
#[component]
pub fn NotificationsPage() -> impl IntoView {
    let notifs = Resource::new(|| (), |_| notifications::list_notifications(
        uuid::Uuid::nil(), 1, 20,
    ));

    view! {
        <div class="p-6 space-y-6">
            <div class="flex justify-between items-center">
                <div>
                    <h1 class="text-xl font-bold text-gray-900">"알림"</h1>
                    <p class="text-sm text-gray-600 mt-1">"최근 알림 내역입니다."</p>
                </div>
                <button class="text-sm text-blue-600 hover:underline">"모두 읽음"</button>
            </div>

            <Suspense fallback=move || view! { <p class="text-sm text-gray-400">"불러오는 중..."</p> }>
                {move || notifs.get().map(|res| match res {
                    Ok(list) => {
                        if list.data.is_empty() {
                            view! { <p class="text-sm text-gray-500">"알림이 없습니다."</p> }.into_any()
                        } else {
                            view! {
                                <div class="space-y-2">
                                    {list.data.into_iter().map(|n| view! {
                                        <div class={format!("bg-white rounded-xl p-4 shadow-sm border {} transition-colors",
                                            if n.read_at.is_none() { "border-blue-200 bg-blue-50/30" } else { "border-gray-100" }
                                        )}>
                                            <p class="text-sm font-medium text-gray-900">{n.title.clone()}</p>
                                            <p class="text-xs text-gray-500 mt-1">{n.message.clone()}</p>
                                            <p class="text-xs text-gray-400 mt-2">{n.created_at.format("%m/%d %H:%M").to_string()}</p>
                                        </div>
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    }
                    Err(_) => view! { <p class="text-sm text-red-500">"알림을 불러올 수 없습니다."</p> }.into_any(),
                })}
            </Suspense>
        </div>
    }
}

/// Family member profile page.
#[component]
pub fn ProfilePage() -> impl IntoView {
    let profile = Resource::new(|| (), |_| profile::get_profile(uuid::Uuid::nil()));

    view! {
        <div class="p-6 space-y-6 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"내 프로필"</h1>
                <p class="text-sm text-gray-600 mt-1">"가족 구성원 정보를 관리하세요."</p>
            </div>

            <Suspense fallback=move || view! { <p class="text-sm text-gray-400">"불러오는 중..."</p> }>
                {move || profile.get().map(|res| match res {
                    Ok(Some(p)) => view! {
                        <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                            <div class="flex items-center gap-4">
                                <div class="w-14 h-14 bg-blue-100 rounded-full flex items-center justify-center">
                                    <span class="text-xl font-bold text-blue-600">{p.last_name.chars().next().unwrap_or(' ').to_string()}</span>
                                </div>
                                <div>
                                    <p class="font-semibold text-gray-900">{p.last_name.clone()}{p.first_name.clone()}</p>
                                    <p class="text-sm text-gray-500">{p.user.email.clone()}</p>
                                </div>
                            </div>
                            <div class="space-y-3 divide-y divide-gray-100">
                                <div class="flex justify-between pt-3">
                                    <p class="text-sm text-gray-500">"연락처"</p>
                                    <p class="text-sm text-gray-900">{p.phone.clone().unwrap_or_else(|| "미등록".into())}</p>
                                </div>
                                <div class="flex justify-between pt-3">
                                    <p class="text-sm text-gray-500">"주소"</p>
                                    <p class="text-sm text-gray-900">{p.city.clone().unwrap_or_default()}" "{p.district.clone().unwrap_or_default()}</p>
                                </div>
                            </div>
                            <button class="w-full border border-gray-300 text-gray-700 rounded-lg px-4 py-2.5 text-sm font-medium hover:bg-gray-50 transition-colors">"프로필 수정"</button>
                        </div>
                    }.into_any(),
                    Ok(None) => view! { <p class="text-sm text-gray-500">"프로필 정보를 찾을 수 없습니다."</p> }.into_any(),
                    Err(_) => view! { <p class="text-sm text-red-500">"프로필을 불러올 수 없습니다."</p> }.into_any(),
                })}
            </Suspense>
        </div>
    }
}

/// Generic placeholder page for routes not yet implemented.
#[component]
pub fn StubPage() -> impl IntoView {
    view! {
        <div class="p-6">
            <p class="text-sm text-gray-500">"준비 중입니다."</p>
        </div>
    }
}

/// Family portal settings.
#[component]
pub fn SettingsPage() -> impl IntoView {
    let (push_enabled, set_push_enabled) = signal(true);
    let (email_enabled, set_email_enabled) = signal(true);

    view! {
        <div class="p-6 space-y-6 max-w-lg">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"설정"</h1>
                <p class="text-sm text-gray-600 mt-1">"가족 포털 설정을 관리하세요."</p>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <h2 class="font-semibold text-gray-900">"알림 설정"</h2>
                <div class="flex justify-between items-center">
                    <div>
                        <p class="text-sm font-medium text-gray-900">"푸시 알림"</p>
                        <p class="text-xs text-gray-500">"앱 푸시 알림을 받습니다."</p>
                    </div>
                    <button
                        class={move || format!("w-11 h-6 rounded-full transition-colors {}",
                            if push_enabled.get() { "bg-blue-600" } else { "bg-gray-300" }
                        )}
                        on:click=move |_| set_push_enabled.update(|v| *v = !*v)
                    >
                        <span class={move || format!("block w-5 h-5 bg-white rounded-full shadow transform transition-transform {}",
                            if push_enabled.get() { "translate-x-5" } else { "translate-x-0.5" }
                        )}></span>
                    </button>
                </div>
                <div class="flex justify-between items-center">
                    <div>
                        <p class="text-sm font-medium text-gray-900">"이메일 알림"</p>
                        <p class="text-xs text-gray-500">"이메일로 알림을 받습니다."</p>
                    </div>
                    <button
                        class={move || format!("w-11 h-6 rounded-full transition-colors {}",
                            if email_enabled.get() { "bg-blue-600" } else { "bg-gray-300" }
                        )}
                        on:click=move |_| set_email_enabled.update(|v| *v = !*v)
                    >
                        <span class={move || format!("block w-5 h-5 bg-white rounded-full shadow transform transition-transform {}",
                            if email_enabled.get() { "translate-x-5" } else { "translate-x-0.5" }
                        )}></span>
                    </button>
                </div>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <h2 class="font-semibold text-gray-900">"계정"</h2>
                <a href="/family/profile" class="block text-sm text-blue-600 hover:underline">"프로필 관리 →"</a>
                <button class="text-sm text-red-600 hover:underline">"로그아웃"</button>
            </div>
        </div>
    }
}
