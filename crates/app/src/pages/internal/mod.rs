use leptos::prelude::*;
use crate::server_fns::*;

// =============================================================================
// Internal (Provider) Portal Pages — 19 components
// =============================================================================

/// Generic placeholder page for routes not yet implemented.
#[component]
pub fn StubPage() -> impl IntoView {
    view! {
        <div class="p-6">
            <p class="text-sm text-gray-500">"준비 중입니다."</p>
        </div>
    }
}

/// Provider dashboard with operational metrics.
#[component]
pub fn DashboardPage() -> impl IntoView {
    let stats = Resource::new(|| (), |_| observability::get_dashboard_stats());

    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"내부 관리 대시보드"</h1>
                <p class="text-sm text-gray-600 mt-1">"기관 운영 현황을 관리하세요."</p>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <p class="text-sm text-gray-500">"이용자 수"</p>
                    <p class="text-2xl font-bold text-gray-900 mt-1">"47"</p>
                    <p class="text-xs text-green-600 mt-1">"+ 3 이번 달"</p>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <p class="text-sm text-gray-500">"요양보호사"</p>
                    <p class="text-2xl font-bold text-gray-900 mt-1">"12"</p>
                    <p class="text-xs text-gray-500 mt-1">"활동 중"</p>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <p class="text-sm text-gray-500">"오늘 방문"</p>
                    <p class="text-2xl font-bold text-gray-900 mt-1">"23"</p>
                    <p class="text-xs text-blue-600 mt-1">"예정: 5건"</p>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <p class="text-sm text-gray-500">"사고 건수"</p>
                    <Suspense fallback=move || view! { <p class="text-2xl font-bold text-gray-300">"..."</p> }>
                        {move || stats.get().map(|res| match res {
                            Ok(s) => view! {
                                <p class="text-2xl font-bold text-gray-900 mt-1">{s.total}</p>
                                <p class="text-xs text-orange-600 mt-1">"미확인: "{s.unacknowledged}</p>
                            }.into_any(),
                            Err(_) => view! { <p class="text-2xl font-bold text-red-400 mt-1">"오류"</p> }.into_any(),
                        })}
                    </Suspense>
                </div>
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <h2 class="font-semibold text-gray-900 mb-3">"일정 충돌"</h2>
                    <p class="text-sm text-gray-500">"현재 일정 충돌이 없습니다."</p>
                    <a href="/internal/schedules/conflicts" class="text-sm text-blue-600 hover:underline mt-2 inline-block">"상세 보기 →"</a>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <h2 class="font-semibold text-gray-900 mb-3">"최근 의뢰"</h2>
                    <p class="text-sm text-gray-500">"새로운 의뢰가 없습니다."</p>
                    <a href="/internal/referrals" class="text-sm text-blue-600 hover:underline mt-2 inline-block">"전체 보기 →"</a>
                </div>
            </div>
        </div>
    }
}

/// Paginated client list.
#[component]
pub fn ClientsListPage() -> impl IntoView {
    let clients = Resource::new(|| (), |_| care_plans::list_care_plans(
        uuid::Uuid::nil(), 1, 20,
    ));

    view! {
        <div class="space-y-6">
            <div class="flex justify-between items-center">
                <div>
                    <h1 class="text-xl font-bold text-gray-900">"이용자 관리"</h1>
                    <p class="text-sm text-gray-600 mt-1">"등록된 이용자 목록입니다."</p>
                </div>
                <div class="flex gap-2">
                    <input type="text" placeholder="이름 검색..." class="border border-gray-300 rounded-lg px-3 py-2 text-sm" />
                </div>
            </div>

            <div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
                <table class="w-full text-sm">
                    <thead class="bg-gray-50 border-b border-gray-100">
                        <tr>
                            <th class="text-left px-4 py-3 font-medium text-gray-500">"이름"</th>
                            <th class="text-left px-4 py-3 font-medium text-gray-500">"등급"</th>
                            <th class="text-left px-4 py-3 font-medium text-gray-500">"상태"</th>
                            <th class="text-left px-4 py-3 font-medium text-gray-500">"담당자"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-gray-100">
                        <Suspense fallback=move || view! {
                            <tr><td colspan="4" class="px-4 py-8 text-center text-gray-400">"불러오는 중..."</td></tr>
                        }>
                            {move || clients.get().map(|res| match res {
                                Ok(list) => {
                                    if list.data.is_empty() {
                                        view! { <tr><td colspan="4" class="px-4 py-8 text-center text-gray-500">"이용자가 없습니다."</td></tr> }.into_any()
                                    } else {
                                        view! {
                                            {list.data.into_iter().map(|p| view! {
                                                <tr class="hover:bg-gray-50 cursor-pointer">
                                                    <td class="px-4 py-3 font-medium text-gray-900">
                                                        <a href={format!("/internal/clients/{}", p.id)} class="hover:text-blue-600">{p.title.clone()}</a>
                                                    </td>
                                                    <td class="px-4 py-3 text-gray-600">"3등급"</td>
                                                    <td class="px-4 py-3"><span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">{p.status.to_string()}</span></td>
                                                    <td class="px-4 py-3 text-gray-600">"김요양"</td>
                                                </tr>
                                            }).collect_view()}
                                        }.into_any()
                                    }
                                }
                                Err(_) => view! { <tr><td colspan="4" class="px-4 py-8 text-center text-red-500">"데이터를 불러올 수 없습니다."</td></tr> }.into_any(),
                            })}
                        </Suspense>
                    </tbody>
                </table>
            </div>
        </div>
    }
}

/// Client detail with care plan summary.
#[component]
pub fn ClientDetailPage() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"이용자 상세"</h1>
                <p class="text-sm text-gray-600 mt-1">"이용자 정보와 케어 플랜을 확인하세요."</p>
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-3">
                    <h2 class="font-semibold text-gray-900">"기본 정보"</h2>
                    <div class="space-y-2 text-sm">
                        <div class="flex justify-between"><span class="text-gray-500">"이름"</span><span class="text-gray-900">"홍길동"</span></div>
                        <div class="flex justify-between"><span class="text-gray-500">"등급"</span><span class="text-gray-900">"3등급"</span></div>
                        <div class="flex justify-between"><span class="text-gray-500">"연락처"</span><span class="text-gray-900">"010-1234-5678"</span></div>
                        <div class="flex justify-between"><span class="text-gray-500">"주소"</span><span class="text-gray-900">"서울시 강남구"</span></div>
                    </div>
                </div>
                <div class="lg:col-span-2 bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-3">
                    <div class="flex justify-between items-center">
                        <h2 class="font-semibold text-gray-900">"케어 플랜"</h2>
                        <a href="/internal/clients/1/care-plan" class="text-sm text-blue-600 hover:underline">"관리 →"</a>
                    </div>
                    <div class="bg-gray-50 rounded-lg p-4">
                        <p class="text-sm font-medium text-gray-900">"방문요양 주 3회"</p>
                        <p class="text-xs text-gray-500 mt-1">"월, 수, 금 10:00 - 12:00"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Care plan management for a client.
#[component]
pub fn ClientCarePlanPage() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div class="flex justify-between items-center">
                <div>
                    <h1 class="text-xl font-bold text-gray-900">"케어 플랜 관리"</h1>
                    <p class="text-sm text-gray-600 mt-1">"이용자의 케어 플랜을 관리하세요."</p>
                </div>
                <a href="/internal/clients/1/care-plan/edit" class="bg-blue-600 text-white rounded-lg px-4 py-2 text-sm font-medium hover:bg-blue-700 transition-colors">"수정"</a>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <div class="flex justify-between items-center">
                    <h2 class="font-semibold text-gray-900">"현재 플랜"</h2>
                    <span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"활성"</span>
                </div>
                <div class="grid grid-cols-2 gap-4 text-sm">
                    <div><p class="text-gray-500">"서비스"</p><p class="font-medium text-gray-900 mt-1">"방문요양"</p></div>
                    <div><p class="text-gray-500">"빈도"</p><p class="font-medium text-gray-900 mt-1">"주 3회"</p></div>
                    <div><p class="text-gray-500">"시작일"</p><p class="font-medium text-gray-900 mt-1">"2026-01-01"</p></div>
                    <div><p class="text-gray-500">"종료일"</p><p class="font-medium text-gray-900 mt-1">"2026-12-31"</p></div>
                </div>
                <div>
                    <p class="text-sm text-gray-500">"목표"</p>
                    <ul class="mt-2 space-y-1 text-sm text-gray-700 list-disc list-inside">
                        <li>"일상생활 지원 (식사, 이동)"</li>
                        <li>"사회 참여 증진"</li>
                        <li>"인지 기능 유지"</li>
                    </ul>
                </div>
            </div>
        </div>
    }
}

/// Edit care plan form.
#[component]
pub fn ClientCarePlanEditPage() -> impl IntoView {
    let (title, set_title) = signal(String::from("방문요양 주 3회"));
    let (description, set_description) = signal(String::new());
    let (start_date, set_start_date) = signal(String::from("2026-01-01"));
    let (end_date, set_end_date) = signal(String::from("2026-12-31"));

    view! {
        <div class="space-y-6 max-w-2xl">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"케어 플랜 수정"</h1>
                <p class="text-sm text-gray-600 mt-1">"케어 플랜 내용을 수정하세요."</p>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">"플랜 제목"</label>
                    <input type="text" class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm" prop:value=move || title.get() on:input=move |ev| set_title.set(event_target_value(&ev)) />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">"설명"</label>
                    <textarea rows=3 class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm" prop:value=move || description.get() on:input=move |ev| set_description.set(event_target_value(&ev))></textarea>
                </div>
                <div class="grid grid-cols-2 gap-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">"시작일"</label>
                        <input type="date" class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm" prop:value=move || start_date.get() on:input=move |ev| set_start_date.set(event_target_value(&ev)) />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">"종료일"</label>
                        <input type="date" class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm" prop:value=move || end_date.get() on:input=move |ev| set_end_date.set(event_target_value(&ev)) />
                    </div>
                </div>
                <div class="flex gap-3 pt-2">
                    <button class="flex-1 bg-blue-600 text-white rounded-lg px-4 py-2.5 text-sm font-medium hover:bg-blue-700 transition-colors">"저장"</button>
                    <a href="/internal/clients/1/care-plan" class="flex-1 text-center border border-gray-300 text-gray-700 rounded-lg px-4 py-2.5 text-sm font-medium hover:bg-gray-50 transition-colors">"취소"</a>
                </div>
            </div>
        </div>
    }
}

/// Approved caregivers list.
#[component]
pub fn CaregiversListPage() -> impl IntoView {
    let caregivers = Resource::new(|| (), |_| caregivers::list_caregivers(
        Some("approved".to_string()), 1, 20,
    ));

    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"요양보호사 관리"</h1>
                <p class="text-sm text-gray-600 mt-1">"승인된 요양보호사 목록입니다."</p>
            </div>

            <Suspense fallback=move || view! { <p class="text-sm text-gray-400">"불러오는 중..."</p> }>
                {move || caregivers.get().map(|res| match res {
                    Ok(list) => view! {
                        <div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
                            <table class="w-full text-sm">
                                <thead class="bg-gray-50 border-b border-gray-100">
                                    <tr>
                                        <th class="text-left px-4 py-3 font-medium text-gray-500">"이름"</th>
                                        <th class="text-left px-4 py-3 font-medium text-gray-500">"경력"</th>
                                        <th class="text-left px-4 py-3 font-medium text-gray-500">"전문분야"</th>
                                        <th class="text-left px-4 py-3 font-medium text-gray-500">"상태"</th>
                                    </tr>
                                </thead>
                                <tbody class="divide-y divide-gray-100">
                                    {list.data.into_iter().map(|c| view! {
                                        <tr class="hover:bg-gray-50 cursor-pointer">
                                            <td class="px-4 py-3">
                                                <a href={format!("/internal/caregivers/{}", c.id)} class="font-medium text-gray-900 hover:text-blue-600">{c.bio.clone().unwrap_or_else(|| "요양보호사".into())}</a>
                                            </td>
                                            <td class="px-4 py-3 text-gray-600">{c.experience_years}"년"</td>
                                            <td class="px-4 py-3 text-gray-600">{c.specializations.clone().unwrap_or_default()}</td>
                                            <td class="px-4 py-3"><span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">{c.status.to_string()}</span></td>
                                        </tr>
                                    }).collect_view()}
                                </tbody>
                            </table>
                            <div class="px-4 py-3 bg-gray-50 border-t border-gray-100 text-sm text-gray-500">"총 "{list.total}" 명"</div>
                        </div>
                    }.into_any(),
                    Err(_) => view! { <p class="text-sm text-red-500">"목록을 불러올 수 없습니다."</p> }.into_any(),
                })}
            </Suspense>
        </div>
    }
}

/// Caregiver profile detail.
#[component]
pub fn CaregiverDetailPage() -> impl IntoView {
    view! {
        <div class="space-y-6 max-w-2xl">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"요양보호사 상세"</h1>
                <p class="text-sm text-gray-600 mt-1">"요양보호사 프로필 정보입니다."</p>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <div class="flex items-center gap-4">
                    <div class="w-14 h-14 bg-teal-100 rounded-full flex items-center justify-center">
                        <span class="text-xl font-bold text-teal-600">"김"</span>
                    </div>
                    <div>
                        <p class="font-semibold text-gray-900">"김요양"</p>
                        <p class="text-sm text-gray-500">"경력 8년 · 치매 전문"</p>
                    </div>
                </div>
                <div class="grid grid-cols-2 gap-4 text-sm">
                    <div><p class="text-gray-500">"상태"</p><span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"승인됨"</span></div>
                    <div><p class="text-gray-500">"야간 가능"</p><p class="font-medium text-gray-900 mt-1">"가능"</p></div>
                    <div><p class="text-gray-500">"담당 이용자"</p><p class="font-medium text-gray-900 mt-1">"5명"</p></div>
                    <div><p class="text-gray-500">"이번 달 방문"</p><p class="font-medium text-gray-900 mt-1">"18건"</p></div>
                </div>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h2 class="font-semibold text-gray-900 mb-3">"자격증"</h2>
                <div class="space-y-2">
                    <div class="flex justify-between text-sm">
                        <span class="text-gray-700">"요양보호사 자격증"</span>
                        <span class="text-gray-500">"2018-06-15 취득"</span>
                    </div>
                    <div class="flex justify-between text-sm">
                        <span class="text-gray-700">"치매 전문 교육 수료"</span>
                        <span class="text-gray-500">"2023-03-20 취득"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Pending caregiver applications list.
#[component]
pub fn ApplicationsListPage() -> impl IntoView {
    let apps = Resource::new(|| (), |_| caregivers::list_caregivers(
        Some("submitted".to_string()), 1, 20,
    ));

    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"지원서 검토"</h1>
                <p class="text-sm text-gray-600 mt-1">"대기 중인 요양보호사 지원서입니다."</p>
            </div>

            <Suspense fallback=move || view! { <p class="text-sm text-gray-400">"불러오는 중..."</p> }>
                {move || apps.get().map(|res| match res {
                    Ok(list) => {
                        if list.data.is_empty() {
                            view! { <p class="text-sm text-gray-500">"대기 중인 지원서가 없습니다."</p> }.into_any()
                        } else {
                            view! {
                                <div class="space-y-3">
                                    {list.data.into_iter().map(|a| view! {
                                        <a href={format!("/internal/applications/{}", a.id)} class="block bg-white rounded-xl p-5 shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
                                            <div class="flex justify-between items-center">
                                                <div>
                                                    <p class="font-medium text-gray-900">{a.bio.clone().unwrap_or_else(|| "지원자".into())}</p>
                                                    <p class="text-sm text-gray-500">"경력 "{a.experience_years}"년"</p>
                                                </div>
                                                <span class="text-xs px-2 py-1 rounded-full bg-yellow-50 text-yellow-700">{a.status.to_string()}</span>
                                            </div>
                                        </a>
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    }
                    Err(_) => view! { <p class="text-sm text-red-500">"지원서를 불러올 수 없습니다."</p> }.into_any(),
                })}
            </Suspense>
        </div>
    }
}

/// Review a single application.
#[component]
pub fn ApplicationDetailPage() -> impl IntoView {
    view! {
        <div class="space-y-6 max-w-2xl">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"지원서 상세"</h1>
                <p class="text-sm text-gray-600 mt-1">"지원서를 검토하고 승인/반려하세요."</p>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <div class="grid grid-cols-2 gap-4 text-sm">
                    <div><p class="text-gray-500">"경력"</p><p class="font-medium text-gray-900 mt-1">"5년"</p></div>
                    <div><p class="text-gray-500">"전문분야"</p><p class="font-medium text-gray-900 mt-1">"방문요양, 방문목욕"</p></div>
                    <div><p class="text-gray-500">"치매 경험"</p><p class="font-medium text-gray-900 mt-1">"있음"</p></div>
                    <div><p class="text-gray-500">"야간 가능"</p><p class="font-medium text-gray-900 mt-1">"가능"</p></div>
                </div>
                <div>
                    <p class="text-sm text-gray-500">"자기소개"</p>
                    <p class="text-sm text-gray-700 mt-1">"5년간 방문요양 서비스를 제공하며 어르신들의 일상생활을 도왔습니다."</p>
                </div>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h2 class="font-semibold text-gray-900 mb-3">"자격증"</h2>
                <div class="space-y-2 text-sm">
                    <div class="flex justify-between"><span class="text-gray-700">"요양보호사 자격증"</span><span class="text-green-600">"확인됨"</span></div>
                </div>
            </div>

            <div class="flex gap-3">
                <button class="flex-1 bg-green-600 text-white rounded-lg px-4 py-2.5 text-sm font-medium hover:bg-green-700 transition-colors">"승인"</button>
                <button class="flex-1 border border-red-300 text-red-600 rounded-lg px-4 py-2.5 text-sm font-medium hover:bg-red-50 transition-colors">"반려"</button>
            </div>
        </div>
    }
}

/// Compliance dashboard.
#[component]
pub fn CompliancePage() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"컴플라이언스"</h1>
                <p class="text-sm text-gray-600 mt-1">"규정 준수 현황을 확인하세요."</p>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <p class="text-sm text-gray-500">"인력 기준 충족"</p>
                    <p class="text-2xl font-bold text-green-600 mt-1">"충족"</p>
                    <p class="text-xs text-gray-500 mt-1">"12/10 (최소 기준 대비)"</p>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <p class="text-sm text-gray-500">"교육 이수율"</p>
                    <p class="text-2xl font-bold text-blue-600 mt-1">"95%"</p>
                    <p class="text-xs text-gray-500 mt-1">"법정 의무교육"</p>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <p class="text-sm text-gray-500">"서류 완비율"</p>
                    <p class="text-2xl font-bold text-gray-900 mt-1">"100%"</p>
                    <p class="text-xs text-green-600 mt-1">"모든 필수 서류 제출됨"</p>
                </div>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h2 class="font-semibold text-gray-900 mb-3">"주요 점검 항목"</h2>
                <ul class="space-y-2 text-sm">
                    <li class="flex items-center gap-2"><span class="w-2 h-2 rounded-full bg-green-500"></span>"인력 배치 기준 — 충족"</li>
                    <li class="flex items-center gap-2"><span class="w-2 h-2 rounded-full bg-green-500"></span>"시설 안전 점검 — 완료"</li>
                    <li class="flex items-center gap-2"><span class="w-2 h-2 rounded-full bg-green-500"></span>"감염 예방 관리 — 정상"</li>
                    <li class="flex items-center gap-2"><span class="w-2 h-2 rounded-full bg-yellow-500"></span>"직원 건강검진 — 1명 미완료"</li>
                </ul>
            </div>
        </div>
    }
}

/// Quality metrics dashboard.
#[component]
pub fn QualityPage() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"품질 관리"</h1>
                <p class="text-sm text-gray-600 mt-1">"서비스 품질 지표를 모니터링하세요."</p>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <p class="text-sm text-gray-500">"종합 품질 점수"</p>
                    <p class="text-2xl font-bold text-green-600 mt-1">"92"<span class="text-sm font-normal text-gray-500">" / 100"</span></p>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <p class="text-sm text-gray-500">"이용자 만족도"</p>
                    <p class="text-2xl font-bold text-blue-600 mt-1">"4.6"<span class="text-sm font-normal text-gray-500">" / 5"</span></p>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <p class="text-sm text-gray-500">"방문 정시율"</p>
                    <p class="text-2xl font-bold text-gray-900 mt-1">"96%"</p>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                    <p class="text-sm text-gray-500">"사고 발생률"</p>
                    <p class="text-2xl font-bold text-gray-900 mt-1">"0.3%"</p>
                </div>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h2 class="font-semibold text-gray-900 mb-3">"월별 품질 추이"</h2>
                <div class="h-40 flex items-end justify-around gap-2">
                    {["85","88","90","89","92","94","92"].into_iter().enumerate().map(|(i, val)| {
                        let months = ["9월","10월","11월","12월","1월","2월","3월"];
                        let height = format!("{}%", val.parse::<f64>().unwrap_or(0.0));
                        view! {
                            <div class="flex-1 flex flex-col items-center gap-1">
                                <div class="w-full bg-blue-200 rounded-t-sm" style={format!("height: {height}")}></div>
                                <span class="text-xs text-gray-500">{months[i]}</span>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}

/// Incidents list.
#[component]
pub fn IncidentsListPage() -> impl IntoView {
    let incidents = Resource::new(|| (), |_| observability::list_signals(
        Some("incident".to_string()), None, None, None, 1, 20,
    ));

    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"사고 관리"</h1>
                <p class="text-sm text-gray-600 mt-1">"발생한 사고를 관리하세요."</p>
            </div>

            <Suspense fallback=move || view! { <p class="text-sm text-gray-400">"불러오는 중..."</p> }>
                {move || incidents.get().map(|res| match res {
                    Ok(list) => {
                        if list.data.is_empty() {
                            view! { <p class="text-sm text-gray-500">"보고된 사고가 없습니다."</p> }.into_any()
                        } else {
                            view! {
                                <div class="space-y-3">
                                    {list.data.into_iter().map(|s| view! {
                                        <a href={format!("/internal/incidents/{}", s.id)} class="block bg-white rounded-xl p-5 shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
                                            <div class="flex justify-between items-start">
                                                <div>
                                                    <p class="font-medium text-gray-900">{s.message.clone()}</p>
                                                    <p class="text-xs text-gray-500 mt-1">{s.created_at.format("%Y-%m-%d %H:%M").to_string()}</p>
                                                </div>
                                                <span class={format!("text-xs px-2 py-1 rounded-full {}",
                                                    if s.acknowledged_at.is_some() { "bg-green-50 text-green-700" } else { "bg-red-50 text-red-700" }
                                                )}>{if s.acknowledged_at.is_some() { "처리됨" } else { "미처리" }}</span>
                                            </div>
                                        </a>
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    }
                    Err(_) => view! { <p class="text-sm text-red-500">"사고 목록을 불러올 수 없습니다."</p> }.into_any(),
                })}
            </Suspense>
        </div>
    }
}

/// Incident detail view.
#[component]
pub fn IncidentDetailPage() -> impl IntoView {
    view! {
        <div class="space-y-6 max-w-2xl">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"사고 상세"</h1>
                <p class="text-sm text-gray-600 mt-1">"사고 내용과 조치 사항을 확인하세요."</p>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <div class="flex justify-between items-center">
                    <span class="text-xs px-2 py-1 rounded-full bg-red-50 text-red-700">"미처리"</span>
                    <p class="text-xs text-gray-500">"2026-03-15 14:30"</p>
                </div>
                <div class="text-sm space-y-3">
                    <div><p class="text-gray-500">"사고 유형"</p><p class="font-medium text-gray-900 mt-1">"낙상"</p></div>
                    <div><p class="text-gray-500">"발생 장소"</p><p class="font-medium text-gray-900 mt-1">"이용자 자택 거실"</p></div>
                    <div><p class="text-gray-500">"상세 내용"</p><p class="text-gray-700 mt-1">"방문요양 중 이용자가 거실에서 미끄러져 경미한 타박상을 입었습니다."</p></div>
                    <div><p class="text-gray-500">"조치 사항"</p><p class="text-gray-700 mt-1">"즉시 응급처치 실시, 보호자 연락 완료"</p></div>
                </div>
            </div>

            <div class="flex gap-3">
                <button class="flex-1 bg-green-600 text-white rounded-lg px-4 py-2.5 text-sm font-medium hover:bg-green-700 transition-colors">"처리 완료"</button>
                <button class="flex-1 border border-gray-300 text-gray-700 rounded-lg px-4 py-2.5 text-sm font-medium hover:bg-gray-50 transition-colors">"추가 조치 필요"</button>
            </div>
        </div>
    }
}

/// Referrals list.
#[component]
pub fn ReferralsListPage() -> impl IntoView {
    let refs = Resource::new(|| (), |_| referrals::list_referrals(
        None, None, 1, 20,
    ));

    view! {
        <div class="space-y-6">
            <div class="flex justify-between items-center">
                <div>
                    <h1 class="text-xl font-bold text-gray-900">"의뢰 관리"</h1>
                    <p class="text-sm text-gray-600 mt-1">"기관 간 의뢰 현황입니다."</p>
                </div>
                <a href="/internal/referrals/new" class="bg-blue-600 text-white rounded-lg px-4 py-2 text-sm font-medium hover:bg-blue-700 transition-colors">"새 의뢰"</a>
            </div>

            <Suspense fallback=move || view! { <p class="text-sm text-gray-400">"불러오는 중..."</p> }>
                {move || refs.get().map(|res| match res {
                    Ok(list) => {
                        if list.data.is_empty() {
                            view! { <p class="text-sm text-gray-500">"의뢰 내역이 없습니다."</p> }.into_any()
                        } else {
                            view! {
                                <div class="space-y-3">
                                    {list.data.into_iter().map(|r| view! {
                                        <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                            <div class="flex justify-between items-center">
                                                <div>
                                                    <p class="font-medium text-gray-900">{r.reason.clone().unwrap_or_else(|| "의뢰".into())}</p>
                                                    <p class="text-xs text-gray-500 mt-1">{r.created_at.format("%Y-%m-%d").to_string()}</p>
                                                </div>
                                                <span class="text-xs px-2 py-1 rounded-full bg-blue-50 text-blue-700">{r.status.to_string()}</span>
                                            </div>
                                        </div>
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    }
                    Err(_) => view! { <p class="text-sm text-red-500">"의뢰 목록을 불러올 수 없습니다."</p> }.into_any(),
                })}
            </Suspense>
        </div>
    }
}

/// Create new referral form.
#[component]
pub fn ReferralNewPage() -> impl IntoView {
    let (reason, set_reason) = signal(String::new());
    let (notes, set_notes) = signal(String::new());

    view! {
        <div class="space-y-6 max-w-2xl">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"새 의뢰 작성"</h1>
                <p class="text-sm text-gray-600 mt-1">"다른 기관으로 의뢰서를 작성하세요."</p>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">"의뢰 대상 기관"</label>
                    <select class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm">
                        <option value="">"기관을 선택하세요"</option>
                        <option value="1">"행복요양원"</option>
                        <option value="2">"사랑재가센터"</option>
                    </select>
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">"의뢰 대상 이용자"</label>
                    <select class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm">
                        <option value="">"이용자를 선택하세요"</option>
                    </select>
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">"의뢰 사유"</label>
                    <input type="text" class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm" placeholder="의뢰 사유를 입력하세요" prop:value=move || reason.get() on:input=move |ev| set_reason.set(event_target_value(&ev)) />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">"비고"</label>
                    <textarea rows=3 class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm" placeholder="추가 사항을 적어주세요" prop:value=move || notes.get() on:input=move |ev| set_notes.set(event_target_value(&ev))></textarea>
                </div>
                <div class="flex gap-3 pt-2">
                    <button class="flex-1 bg-blue-600 text-white rounded-lg px-4 py-2.5 text-sm font-medium hover:bg-blue-700 transition-colors">"의뢰 전송"</button>
                    <a href="/internal/referrals" class="flex-1 text-center border border-gray-300 text-gray-700 rounded-lg px-4 py-2.5 text-sm font-medium hover:bg-gray-50 transition-colors">"취소"</a>
                </div>
            </div>
        </div>
    }
}

/// Reports dashboard.
#[component]
pub fn ReportsPage() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"보고서"</h1>
                <p class="text-sm text-gray-600 mt-1">"운영 보고서를 생성하고 조회하세요."</p>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 hover:shadow-md transition-shadow cursor-pointer">
                    <h3 class="font-semibold text-gray-900">"월간 운영 보고서"</h3>
                    <p class="text-sm text-gray-500 mt-1">"이용자 현황, 방문 실적, 매출 요약"</p>
                    <button class="mt-3 text-sm text-blue-600 hover:underline">"생성하기 →"</button>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 hover:shadow-md transition-shadow cursor-pointer">
                    <h3 class="font-semibold text-gray-900">"품질 평가 보고서"</h3>
                    <p class="text-sm text-gray-500 mt-1">"서비스 품질, 만족도, 사고 현황"</p>
                    <button class="mt-3 text-sm text-blue-600 hover:underline">"생성하기 →"</button>
                </div>
                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 hover:shadow-md transition-shadow cursor-pointer">
                    <h3 class="font-semibold text-gray-900">"인력 관리 보고서"</h3>
                    <p class="text-sm text-gray-500 mt-1">"요양보호사 근무 현황, 자격 관리"</p>
                    <button class="mt-3 text-sm text-blue-600 hover:underline">"생성하기 →"</button>
                </div>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h2 class="font-semibold text-gray-900 mb-3">"최근 생성 보고서"</h2>
                <p class="text-sm text-gray-500">"생성된 보고서가 없습니다."</p>
            </div>
        </div>
    }
}

/// Visit schedules view.
#[component]
pub fn SchedulesPage() -> impl IntoView {
    let visits = Resource::new(|| (), |_| schedule::list_visits(
        None, None, None, 1, 30,
    ));

    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"방문 일정"</h1>
                <p class="text-sm text-gray-600 mt-1">"요양보호사 방문 일정을 관리하세요."</p>
            </div>

            <Suspense fallback=move || view! { <p class="text-sm text-gray-400">"불러오는 중..."</p> }>
                {move || visits.get().map(|res| match res {
                    Ok(list) => {
                        if list.data.is_empty() {
                            view! { <p class="text-sm text-gray-500">"예정된 방문이 없습니다."</p> }.into_any()
                        } else {
                            view! {
                                <div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
                                    <table class="w-full text-sm">
                                        <thead class="bg-gray-50 border-b border-gray-100">
                                            <tr>
                                                <th class="text-left px-4 py-3 font-medium text-gray-500">"날짜/시간"</th>
                                                <th class="text-left px-4 py-3 font-medium text-gray-500">"업무"</th>
                                                <th class="text-left px-4 py-3 font-medium text-gray-500">"상태"</th>
                                            </tr>
                                        </thead>
                                        <tbody class="divide-y divide-gray-100">
                                            {list.data.into_iter().map(|v| view! {
                                                <tr class="hover:bg-gray-50">
                                                    <td class="px-4 py-3 text-gray-900">{v.scheduled_start.format("%m/%d %H:%M").to_string()}</td>
                                                    <td class="px-4 py-3 text-gray-600">{v.tasks.clone().map(|t| t.to_string()).unwrap_or_else(|| "미정".into())}</td>
                                                    <td class="px-4 py-3"><span class="text-xs px-2 py-1 rounded-full bg-blue-50 text-blue-700">{v.status.to_string()}</span></td>
                                                </tr>
                                            }).collect_view()}
                                        </tbody>
                                    </table>
                                </div>
                            }.into_any()
                        }
                    }
                    Err(_) => view! { <p class="text-sm text-red-500">"일정을 불러올 수 없습니다."</p> }.into_any(),
                })}
            </Suspense>
        </div>
    }
}

/// Schedule conflicts view.
#[component]
pub fn ScheduleConflictsPage() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"일정 충돌"</h1>
                <p class="text-sm text-gray-600 mt-1">"겹치는 방문 일정을 확인하고 해결하세요."</p>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <div class="flex items-center gap-3 text-sm text-green-700 bg-green-50 rounded-lg p-4">
                    <span class="w-2 h-2 rounded-full bg-green-500"></span>
                    "현재 일정 충돌이 없습니다."
                </div>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h2 class="font-semibold text-gray-900 mb-3">"충돌 해결 가이드"</h2>
                <ul class="space-y-2 text-sm text-gray-600">
                    <li>"1. 충돌이 발생하면 관련 방문 일정이 표시됩니다."</li>
                    <li>"2. 담당 요양보호사를 변경하거나 시간을 조정하세요."</li>
                    <li>"3. 변경 사항은 관련자에게 자동 통보됩니다."</li>
                </ul>
            </div>
        </div>
    }
}

/// Provider settings page.
#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <div class="space-y-6 max-w-2xl">
            <div>
                <h1 class="text-xl font-bold text-gray-900">"기관 설정"</h1>
                <p class="text-sm text-gray-600 mt-1">"기관 정보와 운영 설정을 관리하세요."</p>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <h2 class="font-semibold text-gray-900">"기관 정보"</h2>
                <div class="space-y-3 text-sm">
                    <div class="flex justify-between"><span class="text-gray-500">"기관명"</span><span class="text-gray-900">"행복재가센터"</span></div>
                    <div class="flex justify-between"><span class="text-gray-500">"기관 유형"</span><span class="text-gray-900">"재가장기요양기관"</span></div>
                    <div class="flex justify-between"><span class="text-gray-500">"기관 번호"</span><span class="text-gray-900">"12345678"</span></div>
                    <div class="flex justify-between"><span class="text-gray-500">"주소"</span><span class="text-gray-900">"서울시 강남구 역삼동"</span></div>
                    <div class="flex justify-between"><span class="text-gray-500">"대표 연락처"</span><span class="text-gray-900">"02-1234-5678"</span></div>
                </div>
                <button class="w-full border border-gray-300 text-gray-700 rounded-lg px-4 py-2.5 text-sm font-medium hover:bg-gray-50 transition-colors">"정보 수정"</button>
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <h2 class="font-semibold text-gray-900">"운영 설정"</h2>
                <div class="space-y-3 text-sm">
                    <div class="flex justify-between items-center">
                        <span class="text-gray-700">"자동 일정 알림"</span>
                        <span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"활성"</span>
                    </div>
                    <div class="flex justify-between items-center">
                        <span class="text-gray-700">"사고 즉시 알림"</span>
                        <span class="text-xs px-2 py-1 rounded-full bg-green-50 text-green-700">"활성"</span>
                    </div>
                    <div class="flex justify-between items-center">
                        <span class="text-gray-700">"월간 보고서 자동 생성"</span>
                        <span class="text-xs px-2 py-1 rounded-full bg-gray-100 text-gray-600">"비활성"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
