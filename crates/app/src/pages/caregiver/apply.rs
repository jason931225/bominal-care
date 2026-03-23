use leptos::prelude::*;
use bominal_types::CaregiverApplication;

// ---------------------------------------------------------------------------
// localStorage helpers for multi-step form persistence
// ---------------------------------------------------------------------------

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = localStorage, js_name = setItem)]
    fn ls_set(key: &str, value: &str);

    #[wasm_bindgen(js_namespace = localStorage, js_name = getItem)]
    fn ls_get(key: &str) -> Option<String>;

    #[wasm_bindgen(js_namespace = localStorage, js_name = removeItem)]
    fn ls_remove(key: &str);
}

fn save_field(key: &str, value: &str) {
    ls_set(&format!("apply_{key}"), value);
}

fn load_field(key: &str) -> String {
    ls_get(&format!("apply_{key}")).unwrap_or_default()
}

fn clear_form() {
    let keys = [
        "name", "phone", "experience", "regions", "schedule_days",
        "schedule_start", "schedule_end", "overnight", "weekend",
        "emergency_call", "services", "specializations",
        "ref1_name", "ref1_rel", "ref1_phone",
        "ref2_name", "ref2_rel", "ref2_phone",
    ];
    for k in keys {
        ls_remove(&format!("apply_{k}"));
    }
}

// =============================================================================
// 17. ApplyOverviewPage — application landing / CTA page
// =============================================================================

#[component]
pub fn ApplyOverviewPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-6">
            <div class="text-center pt-4">
                <div class="w-16 h-16 bg-teal-100 rounded-2xl flex items-center justify-center mx-auto mb-4">
                    <svg class="w-8 h-8 text-teal-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                    </svg>
                </div>
                <h1 class="text-2xl font-bold text-gray-900">"요양보호사 지원"</h1>
                <p class="text-sm text-gray-600 mt-2">"전문 요양보호사로 활동하세요. 간단한 지원 절차를 통해 시작할 수 있습니다."</p>
            </div>

            // Steps overview
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h2 class="font-semibold text-gray-900 mb-4">"지원 절차"</h2>
                <div class="space-y-4">
                    <ApplyStepPreview step=1 title="본인 인증" desc="신분증 확인 및 본인 인증" />
                    <ApplyStepPreview step=2 title="자격 등록" desc="요양보호사 자격증 및 관련 서류 업로드" />
                    <ApplyStepPreview step=3 title="서비스 지역" desc="활동 가능한 지역 선택" />
                    <ApplyStepPreview step=4 title="근무 일정" desc="가능한 근무 시간 설정" />
                    <ApplyStepPreview step=5 title="서비스 유형" desc="제공 가능한 서비스 선택" />
                    <ApplyStepPreview step=6 title="추천인" desc="추천인 정보 입력" />
                    <ApplyStepPreview step=7 title="검토 및 제출" desc="지원서 확인 및 제출" />
                </div>
            </div>

            <a href="/caregiver/apply/identity" class="block w-full py-4 bg-teal-600 text-white font-semibold rounded-xl hover:bg-teal-700 text-center">"지원 시작하기"</a>
        </div>
    }
}

#[component]
fn ApplyStepPreview(
    step: u32,
    #[prop(into)] title: String,
    #[prop(into)] desc: String,
) -> impl IntoView {
    view! {
        <div class="flex items-start gap-3">
            <div class="w-7 h-7 bg-teal-100 rounded-full flex items-center justify-center shrink-0">
                <span class="text-xs font-bold text-teal-700">{step.to_string()}</span>
            </div>
            <div>
                <p class="text-sm font-medium text-gray-900">{title}</p>
                <p class="text-xs text-gray-500">{desc}</p>
            </div>
        </div>
    }
}

// =============================================================================
// 18. ApplyIdentityPage — identity verification step
// =============================================================================

#[component]
pub fn ApplyIdentityPage() -> impl IntoView {
    let name = RwSignal::new(load_field("name"));
    let phone = RwSignal::new(load_field("phone"));
    let verified = RwSignal::new(false);

    // Save on every input change
    Effect::new(move || { save_field("name", &name.get()); });
    Effect::new(move || { save_field("phone", &phone.get()); });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <ApplyStepHeader step=1 total=7 title="본인 인증" />

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"이름"<span class="text-red-500 ml-0.5">"*"</span></label>
                    <input
                        type="text"
                        class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500"
                        placeholder="실명을 입력하세요"
                        prop:value=move || name.get()
                        on:input=move |ev| name.set(event_target_value(&ev))
                    />
                </div>
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"휴대전화"<span class="text-red-500 ml-0.5">"*"</span></label>
                    <input
                        type="tel"
                        class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500"
                        placeholder="010-0000-0000"
                        prop:value=move || phone.get()
                        on:input=move |ev| phone.set(event_target_value(&ev))
                    />
                </div>
                // PASS 본인인증 — 주민등록번호 수집 대신 통신사 인증 사용 (개인정보보호법 준수)
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"본인 인증"<span class="text-red-500 ml-0.5">"*"</span></label>
                    <button
                        type="button"
                        class="w-full py-3 border border-teal-600 text-teal-700 font-medium rounded-lg hover:bg-teal-50 flex items-center justify-center gap-2"
                        on:click=move |_| {
                            if let Some(window) = leptos::web_sys::window() {
                                let _ = window.alert_with_message("본인인증 서비스는 준비 중입니다");
                            }
                        }
                    >
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
                        </svg>
                        "PASS 본인인증"
                    </button>
                    {move || if verified.get() {
                        Some(view! { <p class="text-xs text-green-600 mt-1">"인증이 완료되었습니다."</p> })
                    } else {
                        None
                    }}
                </div>
            </div>

            <ApplyNavButtons prev_href="" next_href="/caregiver/apply/credentials" />
        </div>
    }
}

#[component]
fn ApplyStepHeader(
    step: u32,
    total: u32,
    #[prop(into)] title: String,
) -> impl IntoView {
    view! {
        <div>
            <p class="text-sm text-teal-600 font-medium">"단계 "{step.to_string()}" / "{total.to_string()}</p>
            <h1 class="text-xl font-bold text-gray-900 mt-1">{title}</h1>
            <div class="mt-3 w-full bg-gray-200 rounded-full h-1.5">
                <div
                    class="bg-teal-600 h-1.5 rounded-full transition-all"
                    style=format!("width: {}%", (step as f64 / total as f64 * 100.0) as u32)
                />
            </div>
        </div>
    }
}

#[component]
fn ApplyNavButtons(
    #[prop(into)] prev_href: String,
    #[prop(into)] next_href: String,
) -> impl IntoView {
    view! {
        <div class="flex gap-3">
            {if !prev_href.is_empty() {
                Some(view! {
                    <a href=prev_href class="flex-1 text-center py-3 border border-gray-300 text-gray-700 font-medium rounded-xl hover:bg-gray-50">"이전"</a>
                })
            } else {
                None
            }}
            <a href=next_href class="flex-1 text-center py-3 bg-teal-600 text-white font-medium rounded-xl hover:bg-teal-700">"다음"</a>
        </div>
    }
}

// =============================================================================
// 19. ApplyCredentialsPage — credentials upload step
// =============================================================================

#[component]
pub fn ApplyCredentialsPage() -> impl IntoView {
    let experience = RwSignal::new(load_field("experience"));

    Effect::new(move || { save_field("experience", &experience.get()); });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <ApplyStepHeader step=2 total=7 title="자격 등록" />

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                // Caregiver certificate
                <div class="space-y-2">
                    <label class="text-sm font-medium text-gray-700">"요양보호사 자격증"<span class="text-red-500 ml-0.5">"*"</span></label>
                    <div class="border-2 border-dashed border-gray-300 rounded-lg p-6 text-center cursor-pointer hover:border-teal-400">
                        <svg class="w-8 h-8 text-gray-400 mx-auto mb-2" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
                        </svg>
                        <p class="text-sm text-gray-600">"클릭하여 파일 업로드"</p>
                        <p class="text-xs text-gray-400 mt-1">"JPG, PNG, PDF (최대 10MB)"</p>
                    </div>
                </div>

                // Additional certifications
                <div class="space-y-2">
                    <label class="text-sm font-medium text-gray-700">"추가 자격증 (선택)"</label>
                    <div class="border-2 border-dashed border-gray-300 rounded-lg p-4 text-center cursor-pointer hover:border-teal-400">
                        <p class="text-sm text-gray-500">"치매전문교육, 응급처치 등"</p>
                    </div>
                </div>

                // Experience
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"경력 기간"</label>
                    <select
                        class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500"
                        on:change=move |ev| experience.set(event_target_value(&ev))
                    >
                        <option value="" selected=move || experience.get().is_empty()>"경력을 선택하세요"</option>
                        <option value="0" selected=move || experience.get() == "0">"신입"</option>
                        <option value="1" selected=move || experience.get() == "1">"1년 미만"</option>
                        <option value="3" selected=move || experience.get() == "3">"1~3년"</option>
                        <option value="5" selected=move || experience.get() == "5">"3~5년"</option>
                        <option value="10" selected=move || experience.get() == "10">"5년 이상"</option>
                    </select>
                </div>
            </div>

            <ApplyNavButtons prev_href="/caregiver/apply/identity" next_href="/caregiver/apply/service-region" />
        </div>
    }
}

// =============================================================================
// 20. ApplyServiceRegionPage — service region selection
// =============================================================================

#[component]
pub fn ApplyServiceRegionPage() -> impl IntoView {
    let saved_regions = load_field("regions");
    let districts_list: &[&str] = &["강남구", "서초구", "송파구", "마포구", "영등포구", "강서구", "용산구", "종로구"];

    // Track checked state per district, pre-populated from localStorage
    let checked_signals: Vec<(&str, RwSignal<bool>)> = districts_list.iter().map(|d| {
        let is_checked = saved_regions.contains(d);
        (*d, RwSignal::new(is_checked))
    }).collect();

    // Save whenever any checkbox changes
    let signals_for_effect = checked_signals.clone();
    Effect::new(move || {
        let selected: Vec<&str> = signals_for_effect.iter()
            .filter(|(_, sig)| sig.get())
            .map(|(name, _)| *name)
            .collect();
        save_field("regions", &selected.join(", "));
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <ApplyStepHeader step=3 total=7 title="서비스 지역" />

            <p class="text-sm text-gray-600">"활동 가능한 지역을 선택해주세요. (복수 선택 가능)"</p>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-4">
                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"시/도"</label>
                    <select class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-teal-500">
                        <option value="">"시/도를 선택하세요"</option>
                        <option value="seoul">"서울특별시"</option>
                        <option value="gyeonggi">"경기도"</option>
                        <option value="incheon">"인천광역시"</option>
                        <option value="busan">"부산광역시"</option>
                    </select>
                </div>

                <div class="space-y-1">
                    <label class="text-sm font-medium text-gray-700">"구/군"</label>
                    <div class="grid grid-cols-2 gap-2">
                        {checked_signals.into_iter().map(|(district, checked)| {
                            view! {
                                <label class="flex items-center gap-2 p-2 rounded-lg border border-gray-200 cursor-pointer hover:bg-gray-50">
                                    <input
                                        type="checkbox"
                                        class="w-4 h-4 text-teal-600 rounded border-gray-300 focus:ring-teal-500"
                                        prop:checked=move || checked.get()
                                        on:change=move |_| checked.update(|v| *v = !*v)
                                    />
                                    <span class="text-sm text-gray-700">{district}</span>
                                </label>
                            }
                        }).collect_view()}
                    </div>
                </div>
            </div>

            <ApplyNavButtons prev_href="/caregiver/apply/credentials" next_href="/caregiver/apply/schedule" />
        </div>
    }
}

// =============================================================================
// 21. ApplySchedulePage — availability schedule step
// =============================================================================

#[component]
pub fn ApplySchedulePage() -> impl IntoView {
    let saved_days = load_field("schedule_days");
    let days_list: &[&str] = &["월요일", "화요일", "수요일", "목요일", "금요일", "토요일", "일요일"];

    let day_signals: Vec<(&str, RwSignal<bool>)> = days_list.iter().map(|d| {
        (*d, RwSignal::new(saved_days.contains(d)))
    }).collect();

    let overnight = RwSignal::new(load_field("overnight") == "true");
    let weekend = RwSignal::new(load_field("weekend") == "true");
    let emergency_call = RwSignal::new(load_field("emergency_call") == "true");

    let days_for_effect = day_signals.clone();
    Effect::new(move || {
        let selected: Vec<&str> = days_for_effect.iter()
            .filter(|(_, sig)| sig.get())
            .map(|(name, _)| *name)
            .collect();
        save_field("schedule_days", &selected.join(", "));
    });
    Effect::new(move || { save_field("overnight", if overnight.get() { "true" } else { "false" }); });
    Effect::new(move || { save_field("weekend", if weekend.get() { "true" } else { "false" }); });
    Effect::new(move || { save_field("emergency_call", if emergency_call.get() { "true" } else { "false" }); });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <ApplyStepHeader step=4 total=7 title="근무 일정" />

            <p class="text-sm text-gray-600">"근무 가능한 요일과 시간대를 선택해주세요."</p>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-3">
                {day_signals.into_iter().map(|(day, enabled)| {
                    view! {
                        <div class="flex items-center justify-between py-2 border-b border-gray-50 last:border-0">
                            <label class="flex items-center gap-3">
                                <input
                                    type="checkbox"
                                    class="w-4 h-4 text-teal-600 rounded border-gray-300 focus:ring-teal-500"
                                    prop:checked=move || enabled.get()
                                    on:change=move |_| enabled.update(|v| *v = !*v)
                                />
                                <span class="text-sm font-medium text-gray-700">{day}</span>
                            </label>
                            <Show when=move || enabled.get()>
                                <div class="flex items-center gap-1 text-sm text-gray-500">
                                    <select class="px-2 py-1 border border-gray-200 rounded text-xs">
                                        <option>"09:00"</option>
                                        <option>"10:00"</option>
                                        <option>"11:00"</option>
                                    </select>
                                    <span>"~"</span>
                                    <select class="px-2 py-1 border border-gray-200 rounded text-xs">
                                        <option>"17:00"</option>
                                        <option>"18:00"</option>
                                        <option>"19:00"</option>
                                    </select>
                                </div>
                            </Show>
                        </div>
                    }
                }).collect_view()}
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-3">
                <h3 class="font-semibold text-gray-900">"추가 설정"</h3>
                <label class="flex items-center gap-3">
                    <input type="checkbox" class="w-4 h-4 text-teal-600 rounded border-gray-300 focus:ring-teal-500"
                        prop:checked=move || overnight.get()
                        on:change=move |_| overnight.update(|v| *v = !*v)
                    />
                    <span class="text-sm text-gray-700">"야간 근무 가능"</span>
                </label>
                <label class="flex items-center gap-3">
                    <input type="checkbox" class="w-4 h-4 text-teal-600 rounded border-gray-300 focus:ring-teal-500"
                        prop:checked=move || weekend.get()
                        on:change=move |_| weekend.update(|v| *v = !*v)
                    />
                    <span class="text-sm text-gray-700">"주말 근무 가능"</span>
                </label>
                <label class="flex items-center gap-3">
                    <input type="checkbox" class="w-4 h-4 text-teal-600 rounded border-gray-300 focus:ring-teal-500"
                        prop:checked=move || emergency_call.get()
                        on:change=move |_| emergency_call.update(|v| *v = !*v)
                    />
                    <span class="text-sm text-gray-700">"긴급 호출 가능"</span>
                </label>
            </div>

            <ApplyNavButtons prev_href="/caregiver/apply/service-region" next_href="/caregiver/apply/services" />
        </div>
    }
}

// =============================================================================
// 22. ApplyServicesPage — service types selection
// =============================================================================

#[component]
pub fn ApplyServicesPage() -> impl IntoView {
    let saved_services = load_field("services");
    let saved_specs = load_field("specializations");

    let service_items: &[(&str, &str)] = &[
        ("방문요양", "식사, 세면, 배설 등 일상생활 지원"),
        ("방문목욕", "이동식 욕조를 이용한 목욕 서비스"),
        ("방문간호", "간호, 진료보조 등 의료 서비스"),
        ("주야간보호", "주간/야간 시설 보호 서비스"),
        ("인지활동", "치매 예방 및 인지 자극 프로그램"),
        ("정서지원", "말벗, 외출 동행 등 정서적 지원"),
    ];
    let svc_signals: Vec<(&str, RwSignal<bool>)> = service_items.iter().map(|(name, _)| {
        (*name, RwSignal::new(saved_services.contains(name)))
    }).collect();

    let spec_items: &[&str] = &["치매 케어 경험", "와상 환자 케어", "재활 보조"];
    let spec_signals: Vec<(&str, RwSignal<bool>)> = spec_items.iter().map(|name| {
        (*name, RwSignal::new(saved_specs.contains(name)))
    }).collect();

    let svcs_for_effect = svc_signals.clone();
    let specs_for_effect = spec_signals.clone();
    Effect::new(move || {
        let selected: Vec<&str> = svcs_for_effect.iter().filter(|(_, s)| s.get()).map(|(n, _)| *n).collect();
        save_field("services", &selected.join(", "));
    });
    Effect::new(move || {
        let selected: Vec<&str> = specs_for_effect.iter().filter(|(_, s)| s.get()).map(|(n, _)| *n).collect();
        save_field("specializations", &selected.join(", "));
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <ApplyStepHeader step=5 total=7 title="서비스 유형" />

            <p class="text-sm text-gray-600">"제공 가능한 서비스를 모두 선택해주세요."</p>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-3">
                {service_items.iter().zip(svc_signals.into_iter()).map(|((title, desc), (_, selected))| {
                    let t = title.to_string();
                    let d = desc.to_string();
                    view! {
                        <button
                            type="button"
                            class="w-full flex items-start gap-3 p-3 rounded-lg border text-left transition-colors"
                            class=("border-teal-500", move || selected.get())
                            class=("bg-teal-50", move || selected.get())
                            class=("border-gray-200", move || !selected.get())
                            class=("hover:bg-gray-50", move || !selected.get())
                            on:click=move |_| selected.update(|v| *v = !*v)
                        >
                            <div
                                class="w-5 h-5 rounded border-2 flex items-center justify-center shrink-0 mt-0.5"
                                class=("border-teal-600", move || selected.get())
                                class=("bg-teal-600", move || selected.get())
                                class=("border-gray-300", move || !selected.get())
                            >
                                <Show when=move || selected.get()>
                                    <svg class="w-3 h-3 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                                    </svg>
                                </Show>
                            </div>
                            <div>
                                <p class="text-sm font-medium text-gray-900">{t}</p>
                                <p class="text-xs text-gray-500">{d}</p>
                            </div>
                        </button>
                    }
                }).collect_view()}
            </div>

            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-3">
                <h3 class="font-semibold text-gray-900">"전문 분야"</h3>
                {spec_signals.into_iter().map(|(name, checked)| {
                    view! {
                        <label class="flex items-center gap-3">
                            <input type="checkbox" class="w-4 h-4 text-teal-600 rounded border-gray-300 focus:ring-teal-500"
                                prop:checked=move || checked.get()
                                on:change=move |_| checked.update(|v| *v = !*v)
                            />
                            <span class="text-sm text-gray-700">{name}</span>
                        </label>
                    }
                }).collect_view()}
            </div>

            <ApplyNavButtons prev_href="/caregiver/apply/schedule" next_href="/caregiver/apply/references" />
        </div>
    }
}

// =============================================================================
// 23. ApplyReferencesPage — references form
// =============================================================================

#[component]
pub fn ApplyReferencesPage() -> impl IntoView {
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <ApplyStepHeader step=6 total=7 title="추천인" />

            <p class="text-sm text-gray-600">"이전 근무지의 추천인 정보를 입력해주세요. (선택사항)"</p>

            <div class="space-y-4">
                <ReferenceForm index=1 />
                <ReferenceForm index=2 />
            </div>

            <button type="button" class="w-full py-3 border-2 border-dashed border-gray-300 text-gray-500 text-sm rounded-xl hover:border-gray-400">"+ 추천인 추가"</button>

            <ApplyNavButtons prev_href="/caregiver/apply/services" next_href="/caregiver/apply/review" />
        </div>
    }
}

#[component]
fn ReferenceForm(index: u32) -> impl IntoView {
    let name_key = format!("ref{index}_name");
    let rel_key = format!("ref{index}_rel");
    let phone_key = format!("ref{index}_phone");

    let name = RwSignal::new(load_field(&name_key));
    let rel = RwSignal::new(load_field(&rel_key));
    let phone = RwSignal::new(load_field(&phone_key));

    let nk = name_key.clone();
    let rk = rel_key.clone();
    let pk = phone_key.clone();
    Effect::new(move || { save_field(&nk, &name.get()); });
    Effect::new(move || { save_field(&rk, &rel.get()); });
    Effect::new(move || { save_field(&pk, &phone.get()); });

    view! {
        <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-3">
            <h3 class="font-semibold text-gray-900">"추천인 "{index.to_string()}</h3>
            <div class="space-y-1">
                <label class="text-sm font-medium text-gray-700">"이름"</label>
                <input type="text" class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-teal-500" placeholder="추천인 이름"
                    prop:value=move || name.get()
                    on:input=move |ev| name.set(event_target_value(&ev))
                />
            </div>
            <div class="space-y-1">
                <label class="text-sm font-medium text-gray-700">"관계"</label>
                <input type="text" class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-teal-500" placeholder="예: 이전 기관 관리자"
                    prop:value=move || rel.get()
                    on:input=move |ev| rel.set(event_target_value(&ev))
                />
            </div>
            <div class="space-y-1">
                <label class="text-sm font-medium text-gray-700">"연락처"</label>
                <input type="tel" class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-teal-500" placeholder="010-0000-0000"
                    prop:value=move || phone.get()
                    on:input=move |ev| phone.set(event_target_value(&ev))
                />
            </div>
        </div>
    }
}

// =============================================================================
// 24. ApplyReviewPage — review & submit application
// =============================================================================

#[component]
pub fn ApplyReviewPage() -> impl IntoView {
    let agreed = RwSignal::new(false);
    let submitting = RwSignal::new(false);
    let error_msg = RwSignal::new(None::<String>);

    // Load all form data from localStorage
    let name = load_field("name");
    let phone = load_field("phone");
    let experience_raw = load_field("experience");
    let regions = load_field("regions");
    let schedule_days = load_field("schedule_days");
    let schedule_start = load_field("schedule_start");
    let schedule_end = load_field("schedule_end");
    let overnight = load_field("overnight");
    let weekend = load_field("weekend");
    let services = load_field("services");
    let specializations = load_field("specializations");

    let experience_label = match experience_raw.as_str() {
        "0" => "신입".to_string(),
        "1" => "1년 미만".to_string(),
        "3" => "1~3년".to_string(),
        "5" => "3~5년".to_string(),
        "10" => "5년 이상".to_string(),
        _ => "미입력".to_string(),
    };

    let or_empty = |s: &str| if s.is_empty() { "미입력".to_string() } else { s.to_string() };

    let name_display = or_empty(&name);
    let phone_display = or_empty(&phone);
    let regions_display = or_empty(&regions);
    let schedule_display = if schedule_days.is_empty() {
        "미입력".to_string()
    } else {
        let time_range = if !schedule_start.is_empty() && !schedule_end.is_empty() {
            format!(" ({} ~ {})", schedule_start, schedule_end)
        } else {
            String::new()
        };
        format!("{}{}", schedule_days, time_range)
    };
    let services_display = or_empty(&services);
    let specs_display = or_empty(&specializations);

    let mut schedule_extras = Vec::new();
    if overnight == "true" { schedule_extras.push("야간 근무 가능"); }
    if weekend == "true" { schedule_extras.push("주말 근무 가능"); }
    let extras_display = if schedule_extras.is_empty() {
        String::new()
    } else {
        schedule_extras.join(", ")
    };

    let has_dementia = specializations.contains("치매");
    let has_overnight = overnight == "true";
    let exp_years: Option<i32> = experience_raw.parse().ok();

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <ApplyStepHeader step=7 total=7 title="검토 및 제출" />

            <p class="text-sm text-gray-600">"입력하신 정보를 확인해주세요."</p>

            // Review sections — now showing real form data
            <ReviewSection title="본인 정보" href="/caregiver/apply/identity">
                <ReviewItem label="이름" value=name_display.clone() />
                <ReviewItem label="휴대전화" value=phone_display />
            </ReviewSection>

            <ReviewSection title="자격 정보" href="/caregiver/apply/credentials">
                <ReviewItem label="자격증" value="요양보호사 자격증" />
                <ReviewItem label="경력" value=experience_label />
            </ReviewSection>

            <ReviewSection title="서비스 지역" href="/caregiver/apply/service-region">
                <ReviewItem label="지역" value=regions_display />
            </ReviewSection>

            <ReviewSection title="근무 일정" href="/caregiver/apply/schedule">
                <ReviewItem label="근무 일정" value=schedule_display />
                {if !extras_display.is_empty() {
                    Some(view! { <ReviewItem label="추가 설정" value=extras_display /> })
                } else {
                    None
                }}
            </ReviewSection>

            <ReviewSection title="서비스 유형" href="/caregiver/apply/services">
                <ReviewItem label="서비스" value=services_display />
                <ReviewItem label="전문 분야" value=specs_display />
            </ReviewSection>

            // Agreement
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 space-y-3">
                <label class="flex items-start gap-3">
                    <input
                        type="checkbox"
                        class="w-4 h-4 text-teal-600 rounded border-gray-300 focus:ring-teal-500 mt-0.5"
                        prop:checked=move || agreed.get()
                        on:change=move |_| agreed.update(|v| *v = !*v)
                    />
                    <span class="text-sm text-gray-700">"입력한 정보가 사실과 다름없음을 확인하며, 개인정보 처리방침에 동의합니다."</span>
                </label>
            </div>

            // Error message
            {move || error_msg.get().map(|msg| view! {
                <p class="text-sm text-red-600 text-center">{msg}</p>
            })}

            <div class="flex gap-3">
                <a href="/caregiver/apply/references" class="flex-1 text-center py-3 border border-gray-300 text-gray-700 font-medium rounded-xl hover:bg-gray-50">"이전"</a>
                <button
                    class="flex-1 py-3 bg-teal-600 text-white font-semibold rounded-xl hover:bg-teal-700 disabled:opacity-50"
                    disabled=move || !agreed.get() || submitting.get()
                    on:click=move |_| {
                        let name_val = name.clone();
                        leptos::task::spawn_local(async move {
                            submitting.set(true);
                            error_msg.set(None);

                            // Build body from localStorage data
                            let body = serde_json::json!({
                                "languages_spoken": "ko",
                                "experience_years": exp_years,
                                "has_dementia_experience": has_dementia,
                                "has_overnight_availability": has_overnight,
                                "smoking_status": false,
                                "pet_friendly": true,
                                "bio": format!("{} 요양보호사 지원", name_val),
                            });

                            // Step 1: Create DRAFT application
                            let created = match crate::api::post::<CaregiverApplication, _>(
                                "/api/caregiver-applications", &body,
                            ).await {
                                Ok(resp) if resp.success => resp.data,
                                Ok(resp) => {
                                    error_msg.set(resp.error);
                                    submitting.set(false);
                                    return;
                                }
                                Err(e) => {
                                    error_msg.set(Some(e));
                                    submitting.set(false);
                                    return;
                                }
                            };

                            // Step 2: Submit DRAFT → SUBMITTED
                            if let Some(app) = created {
                                let submit_url = format!("/api/caregiver-applications/{}/submit", app.id);
                                match crate::api::post_no_body(&submit_url).await {
                                    Ok(()) => {
                                        clear_form();
                                        if let Some(window) = leptos::web_sys::window() {
                                            let _ = window.location().set_href("/caregiver/apply/status");
                                        }
                                    }
                                    Err(e) => error_msg.set(Some(e)),
                                }
                            } else {
                                error_msg.set(Some("지원서 생성에 실패했습니다".to_string()));
                            }
                            submitting.set(false);
                        });
                    }
                >
                    {move || if submitting.get() { "처리 중..." } else { "지원서 제출" }}
                </button>
            </div>
        </div>
    }
}

#[component]
fn ReviewSection(
    #[prop(into)] title: String,
    #[prop(into)] href: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
            <div class="flex items-center justify-between mb-3">
                <h3 class="font-semibold text-gray-900">{title}</h3>
                <a href=href class="text-xs text-teal-600 font-medium hover:text-teal-700">"수정"</a>
            </div>
            <dl class="space-y-2 text-sm">
                {children()}
            </dl>
        </div>
    }
}

#[component]
fn ReviewItem(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
) -> impl IntoView {
    view! {
        <div class="flex justify-between">
            <dt class="text-gray-500">{label}</dt>
            <dd class="font-medium text-gray-900">{value}</dd>
        </div>
    }
}

// =============================================================================
// 25. ApplyStatusPage — application status tracker
// =============================================================================

#[component]
pub fn ApplyStatusPage() -> impl IntoView {
    let applications = LocalResource::new(|| {
        crate::api::get::<Vec<CaregiverApplication>>("/api/caregiver-applications?status=mine")
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-6">
            <h1 class="text-xl font-bold text-gray-900">"지원 현황"</h1>

            <Suspense fallback=move || view! {
                <div class="animate-pulse bg-gray-200 rounded-xl h-32" />
            }>
                {move || Suspend::new(async move {
                    match applications.await {
                        Ok(resp) if resp.success => {
                            let apps = resp.data.unwrap_or_default();
                            if apps.is_empty() {
                                view! {
                                    <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 text-center">
                                        <p class="text-gray-500">"제출된 지원서가 없습니다."</p>
                                        <a href="/caregiver/apply" class="inline-block mt-3 px-4 py-2 bg-teal-600 text-white text-sm font-medium rounded-lg hover:bg-teal-700">"지원하기"</a>
                                    </div>
                                }.into_any()
                            } else {
                                let app = apps.into_iter().next().unwrap();
                                let status_label = format!("{:?}", app.status);
                                let submitted = app.submitted_at
                                    .map(|d| d.format("%Y.%m.%d").to_string())
                                    .unwrap_or_else(|| "-".to_string());
                                let is_approved = status_label == "Approved";
                                let is_rejected = status_label == "Rejected";
                                let is_pending = !is_approved && !is_rejected;
                                view! {
                                    <div class="space-y-5">
                                        // Status card
                                        <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 text-center">
                                            <div class="w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-3"
                                                class=("bg-green-100", is_approved)
                                                class=("bg-red-100", is_rejected)
                                                class=("bg-yellow-100", is_pending)
                                            >
                                                <svg class="w-8 h-8" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"
                                                    class=("text-green-600", is_approved)
                                                    class=("text-red-600", is_rejected)
                                                    class=("text-yellow-600", is_pending)
                                                >
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                                                </svg>
                                            </div>
                                            <h2 class="text-lg font-bold text-gray-900">
                                                {if is_approved { "승인 완료" } else if is_rejected { "반려" } else { "심사 중" }}
                                            </h2>
                                            <p class="text-sm text-gray-500 mt-1">
                                                {if is_approved { "지원서가 승인되었습니다." } else if is_rejected { "지원서가 반려되었습니다." } else { "지원서가 검토되고 있습니다." }}
                                            </p>
                                            <p class="text-xs text-gray-400 mt-2">"제출일: "{submitted}</p>
                                        </div>

                                        // Progress steps
                                        <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                            <h3 class="font-semibold text-gray-900 mb-4">"진행 상태"</h3>
                                            <div class="space-y-4">
                                                <StatusStep label="지원서 제출" date="" done=true />
                                                <StatusStep label="서류 검토" date="진행 중" done=is_approved || is_rejected />
                                                <StatusStep label="자격 확인" date="" done=is_approved />
                                                <StatusStep label="최종 승인" date="" done=is_approved />
                                            </div>
                                        </div>
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

            // Contact
            <div class="bg-blue-50 border border-blue-200 rounded-xl p-4">
                <p class="text-sm text-blue-800">"문의사항은 고객센터(1588-9191)로 연락해주세요."</p>
            </div>
        </div>
    }
}

#[component]
fn StatusStep(
    #[prop(into)] label: String,
    #[prop(into)] date: String,
    done: bool,
) -> impl IntoView {
    view! {
        <div class="flex items-center gap-3">
            <div
                class="w-8 h-8 rounded-full flex items-center justify-center shrink-0"
                class=("bg-teal-600", done)
                class=("bg-gray-200", !done)
            >
                {if done {
                    view! { <svg class="w-4 h-4 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3"><path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" /></svg> }.into_any()
                } else {
                    view! { <div class="w-2 h-2 bg-gray-400 rounded-full" /> }.into_any()
                }}
            </div>
            <div class="flex-1">
                <p class="text-sm font-medium" class=("text-gray-900", done) class=("text-gray-500", !done)>{label}</p>
                {if !date.is_empty() {
                    Some(view! { <p class="text-xs text-gray-400">{date}</p> })
                } else {
                    None
                }}
            </div>
        </div>
    }
}
