use leptos::prelude::*;
use chrono::Datelike;
use bominal_types::PersonProfile;

use super::demo_client_id;

// =============================================================================
// 6. ClientsListPage — client list with care level badges
// =============================================================================

#[component]
pub fn ClientsListPage() -> impl IntoView {
    let search = RwSignal::new(String::new());
    let clients = LocalResource::new(|| crate::api::get::<Vec<PersonProfile>>("/api/profile/seniors?caregiver=me"));

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <h1 class="text-xl font-bold text-gray-900">"담당 고객"</h1>

            // Search
            <div class="relative">
                <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-4.35-4.35M11 19a8 8 0 100-16 8 8 0 000 16z" />
                </svg>
                <input
                    type="search"
                    class="w-full pl-10 pr-4 py-2.5 border border-gray-300 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-teal-500"
                    placeholder="고객 검색..."
                    prop:value=move || search.get()
                    on:input=move |ev| search.set(event_target_value(&ev))
                />
            </div>

            // Client list
            <Suspense fallback=move || view! {
                <div class="animate-pulse bg-gray-200 rounded-xl h-20" />
            }>
                {move || Suspend::new(async move {
                    let query = search.get();
                    match clients.await {
                        Ok(resp) if resp.success => {
                            let items = resp.data.unwrap_or_default();
                            let filtered: Vec<PersonProfile> = if query.is_empty() {
                                items
                            } else {
                                items.into_iter().filter(|p| {
                                    p.korean_name.as_deref().unwrap_or("").contains(&query)
                                }).collect()
                            };
                            if filtered.is_empty() {
                                view! {
                                    <p class="text-center text-gray-500 py-8">"담당 고객이 없습니다."</p>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-3">
                                        {filtered.into_iter().map(|profile| {
                                            let id = profile.id.to_string();
                                            let name = profile.korean_name.clone().unwrap_or_else(|| "이름 없음".to_string());
                                            let age: u32 = profile.date_of_birth.map(|dob| {
                                                let now = chrono::Utc::now();
                                                (now.year() - dob.year()) as u32
                                            }).unwrap_or(0);
                                            view! {
                                                <ClientCard
                                                    id=id
                                                    name=name
                                                    age=age
                                                    care_level="미확인".to_string()
                                                    services="방문요양".to_string()
                                                    next_visit="일정 없음".to_string()
                                                />
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            }
                        }
                        _ => view! {
                            <p class="text-center text-gray-500 py-8">"데이터를 불러올 수 없습니다."</p>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

#[component]
fn ClientCard(
    #[prop(into)] id: String,
    #[prop(into)] name: String,
    age: u32,
    #[prop(into)] care_level: String,
    #[prop(into)] services: String,
    #[prop(into)] next_visit: String,
) -> impl IntoView {
    let initial = name.chars().next().unwrap_or('?').to_string();

    let level_cls = match care_level.as_str() {
        "1등급" => "bg-red-100 text-red-700",
        "2등급" => "bg-orange-100 text-orange-700",
        "3등급" => "bg-yellow-100 text-yellow-700",
        "4등급" => "bg-green-100 text-green-700",
        _ => "bg-gray-100 text-gray-700",
    };
    let href = format!("/caregiver/clients/{id}");

    view! {
        <a href=href class="block bg-white rounded-xl p-4 shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
            <div class="flex items-center gap-3">
                <div class="w-11 h-11 bg-teal-100 rounded-full flex items-center justify-center shrink-0">
                    <span class="text-lg font-bold text-teal-700">{initial}</span>
                </div>
                <div class="flex-1 min-w-0">
                    <div class="flex items-center gap-2">
                        <p class="font-semibold text-gray-900">{name}"님"</p>
                        <span class={format!("text-xs font-medium px-2 py-0.5 rounded-full {level_cls}")}>{care_level}</span>
                    </div>
                    <p class="text-sm text-gray-500">{age}"세 · "{services}</p>
                    <p class="text-xs text-teal-600 mt-0.5">"다음 방문: "{next_visit}</p>
                </div>
                <svg class="w-5 h-5 text-gray-400 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
                </svg>
            </div>
        </a>
    }
}

// =============================================================================
// 7. ClientDetailPage — client profile with care plan link
// =============================================================================

#[component]
pub fn ClientDetailPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <div class="flex items-center gap-3">
                <a href="/caregiver/clients" class="p-2 rounded-lg hover:bg-gray-100">
                    <svg class="w-5 h-5 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
                    </svg>
                </a>
                <h1 class="text-xl font-bold text-gray-900">"고객 상세"</h1>
            </div>

            // Profile
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 text-center">
                <div class="w-16 h-16 bg-teal-100 rounded-full flex items-center justify-center mx-auto mb-3">
                    <span class="text-2xl font-bold text-teal-700">"김"</span>
                </div>
                <h2 class="text-lg font-bold text-gray-900">"김복순님"</h2>
                <p class="text-sm text-gray-500">"82세 · 여성"</p>
                <span class="inline-block mt-2 px-3 py-1 bg-yellow-100 text-yellow-700 text-xs font-medium rounded-full">"장기요양 3등급"</span>
            </div>

            // Details
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h3 class="font-semibold text-gray-900 mb-3">"기본 정보"</h3>
                <dl class="space-y-2 text-sm">
                    <div class="flex justify-between"><dt class="text-gray-500">"주소"</dt><dd class="font-medium text-gray-900">"서울시 강남구 역삼동"</dd></div>
                    <div class="flex justify-between"><dt class="text-gray-500">"연락처"</dt><dd class="font-medium text-gray-900">"010-1234-5678"</dd></div>
                    <div class="flex justify-between"><dt class="text-gray-500">"보호자"</dt><dd class="font-medium text-gray-900">"김철수 (아들)"</dd></div>
                    <div class="flex justify-between"><dt class="text-gray-500">"보호자 연락처"</dt><dd class="font-medium text-gray-900">"010-9876-5432"</dd></div>
                    <div class="flex justify-between"><dt class="text-gray-500">"특이사항"</dt><dd class="font-medium text-gray-900">"치매 초기, 당뇨"</dd></div>
                </dl>
            </div>

            // Quick links
            <div class="grid grid-cols-2 gap-3">
                <a href={format!("/caregiver/clients/{}/care-plan", demo_client_id())} class="bg-white rounded-xl p-4 shadow-sm border border-gray-100 text-center hover:shadow-md">
                    <svg class="w-6 h-6 text-teal-600 mx-auto mb-1" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                    </svg>
                    <p class="text-sm font-medium text-gray-900">"케어플랜"</p>
                </a>
                <a href={format!("/caregiver/clients/{}/medications", demo_client_id())} class="bg-white rounded-xl p-4 shadow-sm border border-gray-100 text-center hover:shadow-md">
                    <svg class="w-6 h-6 text-blue-600 mx-auto mb-1" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z" />
                    </svg>
                    <p class="text-sm font-medium text-gray-900">"투약 정보"</p>
                </a>
            </div>
        </div>
    }
}

// =============================================================================
// 8. ClientCarePlanPage — client's care plan detail
// =============================================================================

#[component]
pub fn ClientCarePlanPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <div class="flex items-center gap-3">
                <a href={format!("/caregiver/clients/{}", demo_client_id())} class="p-2 rounded-lg hover:bg-gray-100">
                    <svg class="w-5 h-5 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
                    </svg>
                </a>
                <h1 class="text-xl font-bold text-gray-900">"케어플랜"</h1>
            </div>

            // Plan header
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <div class="flex items-center justify-between mb-2">
                    <h2 class="font-semibold text-gray-900">"김복순님 케어플랜"</h2>
                    <span class="text-xs font-medium px-2 py-0.5 rounded-full bg-green-100 text-green-700">"활성"</span>
                </div>
                <p class="text-sm text-gray-500">"2026.01.15 ~ 2026.07.14"</p>
            </div>

            // Goals
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h3 class="font-semibold text-gray-900 mb-3">"케어 목표"</h3>
                <ul class="space-y-3">
                    <CarePlanGoal title="일상생활 자립 지원" desc="식사, 세면, 착탈의 보조를 통한 자립 유지" progress=70 />
                    <CarePlanGoal title="인지기능 유지" desc="인지 자극 활동 및 대화를 통한 기능 유지" progress=55 />
                    <CarePlanGoal title="안전한 환경 관리" desc="낙상 예방 및 가정 내 안전 확인" progress=85 />
                </ul>
            </div>

            // Schedule
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h3 class="font-semibold text-gray-900 mb-3">"서비스 일정"</h3>
                <div class="space-y-2 text-sm">
                    <div class="flex justify-between items-center">
                        <span class="text-gray-700">"월·수·금"</span>
                        <span class="text-gray-500">"14:00 - 16:00 방문요양"</span>
                    </div>
                    <div class="flex justify-between items-center">
                        <span class="text-gray-700">"화·목"</span>
                        <span class="text-gray-500">"10:00 - 11:00 방문간호"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn CarePlanGoal(
    #[prop(into)] title: String,
    #[prop(into)] desc: String,
    progress: u32,
) -> impl IntoView {
    let width = format!("width: {}%", progress);
    let bar_color = if progress >= 70 { "bg-teal-500" } else { "bg-yellow-500" };

    view! {
        <li>
            <div class="flex items-center justify-between mb-1">
                <p class="text-sm font-medium text-gray-900">{title}</p>
                <span class="text-xs text-gray-500">{progress}"%"</span>
            </div>
            <p class="text-xs text-gray-500 mb-2">{desc}</p>
            <div class="w-full bg-gray-200 rounded-full h-1.5">
                <div class={format!("{bar_color} h-1.5 rounded-full")} style=width />
            </div>
        </li>
    }
}

// =============================================================================
// 9. ClientMedicationsPage — client's medications list
// =============================================================================

#[component]
pub fn ClientMedicationsPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <div class="flex items-center gap-3">
                <a href={format!("/caregiver/clients/{}", demo_client_id())} class="p-2 rounded-lg hover:bg-gray-100">
                    <svg class="w-5 h-5 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
                    </svg>
                </a>
                <h1 class="text-xl font-bold text-gray-900">"투약 정보"</h1>
                <span class="text-sm text-gray-500">"김복순님"</span>
            </div>

            <div class="space-y-3">
                <MedicationCard name="메트포르민 500mg" schedule="하루 2회 (아침, 저녁 식후)" purpose="당뇨" status="복용중" />
                <MedicationCard name="도네페질 5mg" schedule="하루 1회 (저녁 식후)" purpose="치매" status="복용중" />
                <MedicationCard name="아스피린 100mg" schedule="하루 1회 (아침 식후)" purpose="혈액순환" status="복용중" />
                <MedicationCard name="칼슘+비타민D" schedule="하루 1회 (점심 식후)" purpose="골다공증" status="복용중" />
            </div>

            // Notes
            <div class="bg-orange-50 border border-orange-200 rounded-xl p-4">
                <p class="text-sm font-medium text-orange-800 mb-1">"투약 주의사항"</p>
                <p class="text-sm text-orange-700">"메트포르민과 도네페질 복용 간격을 최소 2시간 유지해주세요."</p>
            </div>
        </div>
    }
}

#[component]
fn MedicationCard(
    #[prop(into)] name: String,
    #[prop(into)] schedule: String,
    #[prop(into)] purpose: String,
    #[prop(into)] status: String,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
            <div class="flex items-center justify-between mb-2">
                <h3 class="font-medium text-gray-900">{name}</h3>
                <span class="text-xs font-medium px-2 py-0.5 rounded-full bg-green-100 text-green-700">{status}</span>
            </div>
            <p class="text-sm text-gray-500">{schedule}</p>
            <p class="text-xs text-gray-400 mt-1">"용도: "{purpose}</p>
        </div>
    }
}

// =============================================================================
// 10. MedicationsPage — all assigned client medications overview
// =============================================================================

#[component]
pub fn MedicationsPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <h1 class="text-xl font-bold text-gray-900">"투약 관리"</h1>
            <p class="text-sm text-gray-600">"담당 고객의 투약 현황을 확인하세요."</p>

            // Summary
            <div class="grid grid-cols-3 gap-3">
                <div class="bg-white rounded-xl p-3 shadow-sm border border-gray-100 text-center">
                    <p class="text-lg font-bold text-green-600">"12"</p>
                    <p class="text-xs text-gray-500">"복용 완료"</p>
                </div>
                <div class="bg-white rounded-xl p-3 shadow-sm border border-gray-100 text-center">
                    <p class="text-lg font-bold text-yellow-600">"3"</p>
                    <p class="text-xs text-gray-500">"예정"</p>
                </div>
                <div class="bg-white rounded-xl p-3 shadow-sm border border-gray-100 text-center">
                    <p class="text-lg font-bold text-red-600">"1"</p>
                    <p class="text-xs text-gray-500">"미복용"</p>
                </div>
            </div>

            // Per-client medication list
            <div class="space-y-4">
                <ClientMedSummary client="김복순님" count=4 next_time="12:00 점심 식후" alert=false />
                <ClientMedSummary client="이순자님" count=3 next_time="14:00 오후" alert=false />
                <ClientMedSummary client="박영자님" count=5 next_time="아침 미복용" alert=true />
                <ClientMedSummary client="최영희님" count=2 next_time="18:00 저녁 식후" alert=false />
            </div>
        </div>
    }
}

#[component]
fn ClientMedSummary(
    #[prop(into)] client: String,
    count: u32,
    #[prop(into)] next_time: String,
    alert: bool,
) -> impl IntoView {
    let border = if alert { "border-red-200 bg-red-50" } else { "border-gray-100 bg-white" };

    view! {
        <a href={format!("/caregiver/clients/{}/medications", demo_client_id())} class={format!("block rounded-xl p-4 shadow-sm border {border} hover:shadow-md transition-shadow")}>
            <div class="flex items-center justify-between">
                <div>
                    <p class="font-medium text-gray-900">{client}</p>
                    <p class="text-sm text-gray-500">{count}"개 약물"</p>
                </div>
                <div class="text-right">
                    <p class="text-sm" class=("text-red-600", alert) class=("font-medium", alert) class=("text-gray-500", !alert)>{next_time}</p>
                    {if alert {
                        Some(view! { <span class="text-xs text-red-600">"확인 필요"</span> })
                    } else {
                        None
                    }}
                </div>
            </div>
        </a>
    }
}
