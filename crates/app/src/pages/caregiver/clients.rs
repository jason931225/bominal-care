use bominal_types::PersonProfile;
use chrono::Datelike;
use leptos::prelude::*;

// =============================================================================
// Helpers
// =============================================================================

/// Extract a path segment by index from the current URL.
fn path_segment(index: usize) -> String {
    leptos::web_sys::window()
        .and_then(|w| w.location().pathname().ok())
        .unwrap_or_default()
        .split('/')
        .nth(index)
        .unwrap_or("")
        .to_string()
}

// =============================================================================
// 6. ClientsListPage — client list with care level badges
// =============================================================================

#[component]
pub fn ClientsListPage() -> impl IntoView {
    let search = RwSignal::new(String::new());
    let clients = LocalResource::new(|| {
        crate::api::get::<Vec<PersonProfile>>("/api/profile/seniors?caregiver=me")
    });

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
// 7. ClientDetailPage — client profile fetched from API
// =============================================================================

#[component]
pub fn ClientDetailPage() -> impl IntoView {
    // Extract the client id from the URL path: /caregiver/clients/{id}
    let client_id = path_segment(3);
    let client_id_for_links = client_id.clone();
    let client_id_for_links2 = client_id.clone();

    let profile_url = format!("/api/profile/{}", client_id);
    let wellness_url = format!("/api/wellness/history/{}", client_id);

    let profile = LocalResource::new(move || {
        let url = profile_url.clone();
        async move { crate::api::get::<PersonProfile>(&url).await }
    });
    let wellness = LocalResource::new(move || {
        let url = wellness_url.clone();
        async move { crate::api::get::<serde_json::Value>(&url).await }
    });

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

            // Profile — from API
            <Suspense fallback=move || view! {
                <div class="animate-pulse space-y-4">
                    <div class="bg-gray-200 rounded-xl h-40" />
                    <div class="bg-gray-200 rounded-xl h-32" />
                </div>
            }>
                {move || Suspend::new(async move {
                    match profile.await {
                        Ok(resp) if resp.success => {
                            match resp.data {
                                Some(p) => {
                                    let name = p.korean_name.clone().unwrap_or_else(|| "이름 없음".to_string());
                                    let initial = name.chars().next().unwrap_or('?').to_string();
                                    let age: u32 = p.date_of_birth.map(|dob| {
                                        let now = chrono::Utc::now();
                                        (now.year() - dob.year()) as u32
                                    }).unwrap_or(0);
                                    let gender_str = p.gender.map(|g| format!("{}", g)).unwrap_or_default();
                                    let age_gender = if gender_str.is_empty() {
                                        format!("{}세", age)
                                    } else {
                                        format!("{}세 · {}", age, gender_str)
                                    };
                                    let address = build_address(&p);
                                    let phone = p.phone.as_deref().map(crate::api::format_phone_kr).unwrap_or_else(|| "등록되지 않음".to_string());
                                    let ec_name = p.emergency_contact_name.clone().unwrap_or_else(|| "등록되지 않음".to_string());
                                    let ec_phone = p.emergency_contact_phone.as_deref().map(crate::api::format_phone_kr).unwrap_or_else(|| "등록되지 않음".to_string());

                                    view! {
                                        <div class="space-y-5">
                                            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100 text-center">
                                                <div class="w-16 h-16 bg-teal-100 rounded-full flex items-center justify-center mx-auto mb-3">
                                                    <span class="text-2xl font-bold text-teal-700">{initial}</span>
                                                </div>
                                                <h2 class="text-lg font-bold text-gray-900">{name.clone()}"님"</h2>
                                                <p class="text-sm text-gray-500">{age_gender}</p>
                                            </div>

                                            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                                <h3 class="font-semibold text-gray-900 mb-3">"기본 정보"</h3>
                                                <dl class="space-y-2 text-sm">
                                                    <div class="flex justify-between"><dt class="text-gray-500">"주소"</dt><dd class="font-medium text-gray-900">{address}</dd></div>
                                                    <div class="flex justify-between"><dt class="text-gray-500">"연락처"</dt><dd class="font-medium text-gray-900">{phone}</dd></div>
                                                    <div class="flex justify-between"><dt class="text-gray-500">"보호자"</dt><dd class="font-medium text-gray-900">{ec_name}</dd></div>
                                                    <div class="flex justify-between"><dt class="text-gray-500">"보호자 연락처"</dt><dd class="font-medium text-gray-900">{ec_phone}</dd></div>
                                                </dl>
                                            </div>
                                        </div>
                                    }.into_any()
                                }
                                None => view! {
                                    <p class="text-center text-gray-500 py-8">"고객 정보를 찾을 수 없습니다."</p>
                                }.into_any(),
                            }
                        }
                        _ => view! {
                            <p class="text-center text-gray-500 py-8">"데이터를 불러올 수 없습니다."</p>
                        }.into_any(),
                    }
                })}
            </Suspense>

            // Wellness section
            <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                <h3 class="font-semibold text-gray-900 mb-3">"최근 건강 상태"</h3>
                <Suspense fallback=move || view! {
                    <div class="animate-pulse bg-gray-200 rounded h-8 w-full" />
                }>
                    {move || Suspend::new(async move {
                        match wellness.await {
                            Ok(resp) if resp.success => {
                                let payload = resp.data.unwrap_or(serde_json::Value::Null);
                                let entries: Vec<serde_json::Value> = if let Some(arr) = payload.as_array() {
                                    arr.clone()
                                } else {
                                    vec![]
                                };
                                let recent: Vec<serde_json::Value> = entries.into_iter().take(7).collect();
                                if recent.is_empty() {
                                    view! {
                                        <p class="text-sm text-gray-500">"건강 기록이 없습니다"</p>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="flex gap-2 flex-wrap">
                                            {recent.into_iter().map(|entry| {
                                                let mood = entry.get("mood")
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("unknown")
                                                    .to_string();
                                                let (emoji, bg_cls) = match mood.as_str() {
                                                    "good" => ("\u{1F60A}", "bg-gray-50"),
                                                    "okay" => ("\u{1F642}", "bg-gray-50"),
                                                    "not_great" => ("\u{1F610}", "bg-gray-50"),
                                                    "need_help" => ("\u{1F198}", "bg-red-50"),
                                                    _ => ("\u{2753}", "bg-gray-50"),
                                                };
                                                view! {
                                                    <div class={format!("w-10 h-10 rounded-lg flex items-center justify-center text-lg {}", bg_cls)}>
                                                        {emoji}
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    }.into_any()
                                }
                            }
                            _ => view! {
                                <p class="text-sm text-gray-500">"건강 기록이 없습니다"</p>
                            }.into_any(),
                        }
                    })}
                </Suspense>
            </div>

            // Quick links — use the actual client_id from URL
            <div class="grid grid-cols-2 gap-3">
                <a href={format!("/caregiver/clients/{}/care-plan", client_id_for_links)} class="bg-white rounded-xl p-4 shadow-sm border border-gray-100 text-center hover:shadow-md">
                    <svg class="w-6 h-6 text-teal-600 mx-auto mb-1" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                    </svg>
                    <p class="text-sm font-medium text-gray-900">"케어플랜"</p>
                </a>
                <a href={format!("/caregiver/clients/{}/medications", client_id_for_links2)} class="bg-white rounded-xl p-4 shadow-sm border border-gray-100 text-center hover:shadow-md">
                    <svg class="w-6 h-6 text-blue-600 mx-auto mb-1" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z" />
                    </svg>
                    <p class="text-sm font-medium text-gray-900">"투약 정보"</p>
                </a>
            </div>
        </div>
    }
}

/// Build a display address from PersonProfile fields.
fn build_address(p: &PersonProfile) -> String {
    let parts: Vec<&str> = [
        p.city.as_deref(),
        p.district.as_deref(),
        p.address.as_deref(),
    ]
    .iter()
    .filter_map(|&s| s)
    .collect();
    if parts.is_empty() {
        "주소 미등록".to_string()
    } else {
        parts.join(" ")
    }
}

// =============================================================================
// 8. ClientCarePlanPage — client's care plan detail (from API)
// =============================================================================

#[component]
pub fn ClientCarePlanPage() -> impl IntoView {
    let client_id = path_segment(3);
    let back_href = format!("/caregiver/clients/{}", client_id);

    let data = LocalResource::new(|| {
        crate::api::get::<Vec<serde_json::Value>>("/api/care-plans?page=1&limit=5")
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <div class="flex items-center gap-3">
                <a href=back_href class="p-2 rounded-lg hover:bg-gray-100">
                    <svg class="w-5 h-5 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
                    </svg>
                </a>
                <h1 class="text-xl font-bold text-gray-900">"케어플랜"</h1>
            </div>

            <Suspense fallback=move || view! {
                <div class="animate-pulse bg-gray-200 rounded-xl h-20" />
            }>
                {move || Suspend::new(async move {
                    match data.await {
                        Ok(resp) if resp.success => {
                            let items = resp.data.unwrap_or_default();
                            if items.is_empty() {
                                view! {
                                    <p class="text-center text-gray-500 py-8">"등록된 케어 플랜이 없습니다"</p>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-3">
                                        {items.into_iter().map(|plan| {
                                            let title = plan.get("title")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("케어플랜")
                                                .to_string();
                                            let status = plan.get("status")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("unknown")
                                                .to_string();
                                            let description = plan.get("description")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("")
                                                .to_string();
                                            let (badge_bg, badge_label) = match status.as_str() {
                                                "active" | "Active" => ("bg-green-100 text-green-700", "활성"),
                                                "draft" | "Draft" => ("bg-yellow-100 text-yellow-700", "초안"),
                                                "completed" | "Completed" => ("bg-gray-100 text-gray-600", "완료"),
                                                _ => ("bg-gray-100 text-gray-600", "기타"),
                                            };
                                            view! {
                                                <div class="bg-white rounded-xl p-5 shadow-sm border border-gray-100">
                                                    <div class="flex items-center justify-between mb-2">
                                                        <h2 class="font-semibold text-gray-900">{title}</h2>
                                                        <span class={format!("text-xs font-medium px-2 py-0.5 rounded-full {}", badge_bg)}>{badge_label}</span>
                                                    </div>
                                                    {if !description.is_empty() {
                                                        view! { <p class="text-sm text-gray-500">{description}</p> }.into_any()
                                                    } else {
                                                        view! { <></> }.into_any()
                                                    }}
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
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

// =============================================================================
// 9. ClientMedicationsPage — client's medications list (from API)
// =============================================================================

#[component]
pub fn ClientMedicationsPage() -> impl IntoView {
    let client_id = path_segment(3);
    let back_href = format!("/caregiver/clients/{}", client_id);

    let data = LocalResource::new(|| {
        crate::api::get::<Vec<serde_json::Value>>("/api/medications")
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <div class="flex items-center gap-3">
                <a href=back_href class="p-2 rounded-lg hover:bg-gray-100">
                    <svg class="w-5 h-5 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
                    </svg>
                </a>
                <h1 class="text-xl font-bold text-gray-900">"투약 정보"</h1>
            </div>

            <Suspense fallback=move || view! {
                <div class="animate-pulse bg-gray-200 rounded-xl h-20" />
            }>
                {move || Suspend::new(async move {
                    match data.await {
                        Ok(resp) if resp.success => {
                            let items = resp.data.unwrap_or_default();
                            if items.is_empty() {
                                view! {
                                    <p class="text-center text-gray-500 py-8">"등록된 약물이 없습니다"</p>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-3">
                                        {items.into_iter().map(|med| {
                                            let name = med.get("name")
                                                .or_else(|| med.get("medication_name"))
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("약물명 없음")
                                                .to_string();
                                            let dosage = med.get("dosage")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("-")
                                                .to_string();
                                            let frequency = med.get("frequency")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("-")
                                                .to_string();
                                            let is_active = med.get("is_active")
                                                .and_then(|v| v.as_bool())
                                                .unwrap_or(true);
                                            let (badge_bg, badge_label) = if is_active {
                                                ("bg-green-100 text-green-700", "복용중")
                                            } else {
                                                ("bg-gray-100 text-gray-600", "중단")
                                            };
                                            view! {
                                                <div class="bg-white rounded-xl p-4 shadow-sm border border-gray-100">
                                                    <div class="flex items-center justify-between mb-2">
                                                        <h3 class="font-medium text-gray-900">{name}</h3>
                                                        <span class={format!("text-xs font-medium px-2 py-0.5 rounded-full {}", badge_bg)}>{badge_label}</span>
                                                    </div>
                                                    <p class="text-sm text-gray-500">{dosage}</p>
                                                    <p class="text-xs text-gray-400 mt-1">"복용 주기: "{frequency}</p>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
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

// =============================================================================
// 10. MedicationsPage — all assigned client medications overview (from API)
// =============================================================================

#[component]
pub fn MedicationsPage() -> impl IntoView {
    let meds = LocalResource::new(|| {
        crate::api::get::<Vec<serde_json::Value>>("/api/medications")
    });
    let clients = LocalResource::new(|| {
        crate::api::get::<Vec<PersonProfile>>("/api/profile/seniors?caregiver=me")
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-5">
            <h1 class="text-xl font-bold text-gray-900">"투약 관리"</h1>
            <p class="text-sm text-gray-600">"담당 고객의 투약 현황을 확인하세요."</p>

            // Summary — from API medication data
            <Suspense fallback=move || view! {
                <div class="animate-pulse grid grid-cols-3 gap-3">
                    <div class="bg-gray-200 rounded-xl h-16" />
                    <div class="bg-gray-200 rounded-xl h-16" />
                    <div class="bg-gray-200 rounded-xl h-16" />
                </div>
            }>
                {move || Suspend::new(async move {
                    match meds.await {
                        Ok(resp) if resp.success => {
                            let items = resp.data.unwrap_or_default();
                            let active_count = items.iter().filter(|m| {
                                m.get("is_active").and_then(|v| v.as_bool()).unwrap_or(true)
                            }).count();
                            let inactive_count = items.len().saturating_sub(active_count);
                            let total = items.len();
                            view! {
                                <div class="grid grid-cols-3 gap-3">
                                    <div class="bg-white rounded-xl p-3 shadow-sm border border-gray-100 text-center">
                                        <p class="text-lg font-bold text-green-600">{total.to_string()}</p>
                                        <p class="text-xs text-gray-500">"전체 약물"</p>
                                    </div>
                                    <div class="bg-white rounded-xl p-3 shadow-sm border border-gray-100 text-center">
                                        <p class="text-lg font-bold text-teal-600">{active_count.to_string()}</p>
                                        <p class="text-xs text-gray-500">"복용중"</p>
                                    </div>
                                    <div class="bg-white rounded-xl p-3 shadow-sm border border-gray-100 text-center">
                                        <p class="text-lg font-bold text-gray-600">{inactive_count.to_string()}</p>
                                        <p class="text-xs text-gray-500">"중단"</p>
                                    </div>
                                </div>
                            }.into_any()
                        }
                        _ => view! {
                            <div class="grid grid-cols-3 gap-3">
                                <div class="bg-white rounded-xl p-3 shadow-sm border border-gray-100 text-center">
                                    <p class="text-lg font-bold text-gray-400">"—"</p>
                                    <p class="text-xs text-gray-500">"전체 약물"</p>
                                </div>
                                <div class="bg-white rounded-xl p-3 shadow-sm border border-gray-100 text-center">
                                    <p class="text-lg font-bold text-gray-400">"—"</p>
                                    <p class="text-xs text-gray-500">"복용중"</p>
                                </div>
                                <div class="bg-white rounded-xl p-3 shadow-sm border border-gray-100 text-center">
                                    <p class="text-lg font-bold text-gray-400">"—"</p>
                                    <p class="text-xs text-gray-500">"중단"</p>
                                </div>
                            </div>
                        }.into_any(),
                    }
                })}
            </Suspense>

            // Per-client list — from API
            <Suspense fallback=move || view! {
                <div class="animate-pulse bg-gray-200 rounded-xl h-20" />
            }>
                {move || Suspend::new(async move {
                    match clients.await {
                        Ok(resp) if resp.success => {
                            let items = resp.data.unwrap_or_default();
                            if items.is_empty() {
                                view! {
                                    <p class="text-center text-gray-500 py-4">"담당 고객이 없습니다."</p>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-4">
                                        {items.into_iter().map(|profile| {
                                            let name = format!("{}님", profile.korean_name.clone().unwrap_or_else(|| "이름 없음".to_string()));
                                            let id = profile.id.to_string();
                                            view! {
                                                <a href={format!("/caregiver/clients/{}/medications", id)} class="block rounded-xl p-4 shadow-sm border border-gray-100 bg-white hover:shadow-md transition-shadow">
                                                    <div class="flex items-center justify-between">
                                                        <div>
                                                            <p class="font-medium text-gray-900">{name}</p>
                                                            <p class="text-sm text-gray-500">"투약 정보 보기"</p>
                                                        </div>
                                                        <svg class="w-5 h-5 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
                                                        </svg>
                                                    </div>
                                                </a>
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            }
                        }
                        _ => view! {
                            <p class="text-center text-gray-500 py-4">"데이터를 불러올 수 없습니다."</p>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
