use leptos::prelude::*;

use crate::components::data_display::EmptyState;
use crate::components::layout::PageHeader;
use crate::i18n::t;

// =============================================================================
// Patients — lookup and session views
// =============================================================================

/// Patient lookup page with search field.
///
/// Allows medical staff to search for patients by name or ID to initiate
/// or resume a handoff session from the senior portal.
#[component]
pub fn PatientsPage() -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());
    let (submitted_query, set_submitted_query) = signal(String::new());

    let search_results =
        LocalResource::new(move || {
            let query = submitted_query.get();
            async move {
                if query.is_empty() {
                    return Ok(crate::api::ApiResponse::<Vec<bominal_types::PersonProfile>> {
                        success: true,
                        data: Some(vec![]),
                        error: None,
                        meta: None,
                    });
                }
                crate::api::get::<Vec<bominal_types::PersonProfile>>(&format!(
                    "/api/clinical?search={}",
                    query
                ))
                .await
            }
        });

    let on_search = move |_| {
        set_submitted_query.set(search_query.get());
    };

    let on_keydown = move |ev: leptos::web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            set_submitted_query.set(search_query.get());
        }
    };

    view! {
        <div class="space-y-8">
            <PageHeader
                title=t("medical.patients.title").to_string()
                subtitle=t("medical.patients.subtitle").to_string()
            />

            // Search bar
            <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                <div class="flex gap-2">
                    <div class="relative flex-1">
                        <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-txt-tertiary" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                        </svg>
                        <input
                            type="text"
                            class="w-full pl-10 pr-4 py-3 border border-gray-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-[var(--portal-accent)]/30 min-h-[44px]"
                            placeholder=t("medical.patients.search_placeholder")
                            prop:value=move || search_query.get()
                            on:input=move |ev| set_search_query.set(event_target_value(&ev))
                            on:keydown=on_keydown
                        />
                    </div>
                    <button
                        class="bg-[var(--portal-accent)] text-white text-sm font-semibold rounded-xl px-5 hover:opacity-90 active:scale-[0.98] transition-all min-h-[44px]"
                        on:click=on_search
                    >
                        {t("common.search")}
                    </button>
                </div>
            </div>

            // Results area
            <Suspense fallback=move || view! {
                <div class="space-y-3">
                    <div class="animate-pulse bg-gray-200 rounded-2xl h-20" />
                    <div class="animate-pulse bg-gray-200 rounded-2xl h-20" />
                </div>
            }>
                {move || Suspend::new(async move {
                    let query_used = submitted_query.get();
                    if query_used.is_empty() {
                        return view! {
                            <EmptyState message=t("medical.patients.search_hint").to_string() />
                        }.into_any();
                    }
                    match search_results.await {
                        Ok(resp) if resp.success => {
                            let patients = resp.data.unwrap_or_default();
                            if patients.is_empty() {
                                view! {
                                    <EmptyState message=t("medical.patients.no_results").to_string() />
                                }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-3">
                                        {patients.into_iter().map(|p| {
                                            let name = p.korean_name
                                                .clone()
                                                .or(p.english_name.clone())
                                                .unwrap_or_else(|| "-".to_string());
                                            let phone = p.phone.clone().unwrap_or_else(|| "-".to_string());
                                            let address = p.address.clone().unwrap_or_else(|| "-".to_string());
                                            view! {
                                                <div class="bg-surface-card rounded-2xl p-5 shadow-sm flex items-center gap-4">
                                                    <div class="w-12 h-12 bg-[var(--portal-accent-light)] rounded-full flex items-center justify-center flex-shrink-0">
                                                        <svg class="w-6 h-6 text-[var(--portal-accent)]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                            <path stroke-linecap="round" stroke-linejoin="round" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                                                        </svg>
                                                    </div>
                                                    <div class="flex-1 min-w-0">
                                                        <p class="font-semibold text-txt-primary">{name}</p>
                                                        <p class="text-sm text-txt-tertiary">{phone}</p>
                                                        <p class="text-xs text-txt-tertiary truncate">{address}</p>
                                                    </div>
                                                    <a
                                                        href="/medical/session"
                                                        class="text-sm text-[var(--portal-accent)] font-medium hover:underline min-h-[44px] flex items-center"
                                                    >
                                                        {t("medical.patients.view_session")}
                                                    </a>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            }
                        }
                        _ => view! {
                            <p class="text-center text-gray-500 py-8">{t("medical.patients.search_error")}</p>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

/// Patient handoff session page.
///
/// Shows the current session with patient details handed off from the
/// senior portal, including demographics, care grade, and active care plan.
#[component]
pub fn PatientSessionPage() -> impl IntoView {
    let session = LocalResource::new(|| {
        crate::api::get::<serde_json::Value>("/api/handoff/active")
    });

    view! {
        <div class="space-y-8">
            <PageHeader
                title=t("medical.session.title").to_string()
                subtitle=t("medical.session.subtitle").to_string()
            />

            <Suspense fallback=move || view! {
                <div class="animate-pulse bg-gray-200 rounded-2xl h-24" />
            }>
                {move || Suspend::new(async move {
                    match session.await {
                        Ok(resp) if resp.success => {
                            let data = resp.data.unwrap_or_default();
                            let name = data.get("patient_name")
                                .and_then(|v| v.as_str())
                                .unwrap_or("-")
                                .to_string();
                            let care_grade = data.get("care_grade")
                                .and_then(|v| v.as_str())
                                .unwrap_or("-")
                                .to_string();
                            let phone = data.get("phone")
                                .and_then(|v| v.as_str())
                                .unwrap_or("-")
                                .to_string();
                            let address = data.get("address")
                                .and_then(|v| v.as_str())
                                .unwrap_or("-")
                                .to_string();
                            let has_session = data.get("is_active")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false);
                            let care_plan = data.get("care_plan_title")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string());

                            let session_indicator = if has_session {
                                view! {
                                    <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                                        <div class="flex items-center gap-4">
                                            <div class="w-14 h-14 bg-[var(--portal-accent-light)] rounded-full flex items-center justify-center">
                                                <svg class="w-7 h-7 text-[var(--portal-accent)]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                                                </svg>
                                            </div>
                                            <div>
                                                <p class="font-semibold text-txt-primary">{name.clone()}</p>
                                                <p class="text-sm text-txt-tertiary">{t("medical.session.active_session")}</p>
                                            </div>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                                        <div class="flex items-center gap-4">
                                            <div class="w-14 h-14 bg-[var(--portal-accent-light)] rounded-full flex items-center justify-center">
                                                <svg class="w-7 h-7 text-[var(--portal-accent)]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                                                </svg>
                                            </div>
                                            <div>
                                                <p class="font-semibold text-txt-primary">{t("medical.session.no_active")}</p>
                                                <p class="text-sm text-txt-tertiary">{t("medical.session.no_active_sub")}</p>
                                            </div>
                                        </div>
                                    </div>
                                }.into_any()
                            };

                            let care_plan_view = if let Some(title) = care_plan {
                                view! {
                                    <p class="text-sm text-txt-secondary">{title}</p>
                                }.into_any()
                            } else {
                                view! {
                                    <EmptyState message=t("medical.session.no_care_plan").to_string() />
                                }.into_any()
                            };

                            view! {
                                <div class="space-y-6">
                                    {session_indicator}
                                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                                        <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-3">
                                            <h2 class="font-semibold text-txt-primary">{t("medical.session.demographics")}</h2>
                                            <div class="space-y-2 text-sm">
                                                <div class="flex justify-between">
                                                    <span class="text-txt-tertiary">{t("common.name")}</span>
                                                    <span class="text-txt-secondary">{name}</span>
                                                </div>
                                                <div class="flex justify-between">
                                                    <span class="text-txt-tertiary">{t("medical.session.care_grade")}</span>
                                                    <span class="text-txt-secondary">{care_grade}</span>
                                                </div>
                                                <div class="flex justify-between">
                                                    <span class="text-txt-tertiary">{t("common.phone")}</span>
                                                    <span class="text-txt-secondary">{phone}</span>
                                                </div>
                                                <div class="flex justify-between">
                                                    <span class="text-txt-tertiary">{t("common.address")}</span>
                                                    <span class="text-txt-secondary">{address}</span>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-3">
                                            <h2 class="font-semibold text-txt-primary">{t("medical.session.care_plan")}</h2>
                                            {care_plan_view}
                                        </div>
                                    </div>
                                </div>
                            }.into_any()
                        }
                        _ => view! {
                            <div class="space-y-6">
                                <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                                    <div class="flex items-center gap-4">
                                        <div class="w-14 h-14 bg-[var(--portal-accent-light)] rounded-full flex items-center justify-center">
                                            <svg class="w-7 h-7 text-[var(--portal-accent)]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                                            </svg>
                                        </div>
                                        <div>
                                            <p class="font-semibold text-txt-primary">{t("medical.session.no_active")}</p>
                                            <p class="text-sm text-txt-tertiary">{t("medical.session.no_active_sub")}</p>
                                        </div>
                                    </div>
                                </div>
                                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                                    <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-3">
                                        <h2 class="font-semibold text-txt-primary">{t("medical.session.demographics")}</h2>
                                        <div class="space-y-2 text-sm">
                                            <div class="flex justify-between">
                                                <span class="text-txt-tertiary">{t("common.name")}</span>
                                                <span class="text-txt-secondary">"-"</span>
                                            </div>
                                            <div class="flex justify-between">
                                                <span class="text-txt-tertiary">{t("medical.session.care_grade")}</span>
                                                <span class="text-txt-secondary">"-"</span>
                                            </div>
                                            <div class="flex justify-between">
                                                <span class="text-txt-tertiary">{t("common.phone")}</span>
                                                <span class="text-txt-secondary">"-"</span>
                                            </div>
                                            <div class="flex justify-between">
                                                <span class="text-txt-tertiary">{t("common.address")}</span>
                                                <span class="text-txt-secondary">"-"</span>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-3">
                                        <h2 class="font-semibold text-txt-primary">{t("medical.session.care_plan")}</h2>
                                        <EmptyState message=t("medical.session.no_care_plan").to_string() />
                                    </div>
                                </div>
                            </div>
                        }.into_any(),
                    }
                })}
            </Suspense>

            // Quick actions for session
            <div class="flex gap-3">
                <a href="/medical/prescriptions" class="flex-1 text-center bg-[var(--portal-accent)] text-white rounded-xl px-4 py-2.5 text-sm font-medium hover:opacity-90 active:scale-[0.98] transition-all min-h-[44px] flex items-center justify-center">
                    {t("medical.session.write_prescription")}
                </a>
                <a href="/medical/appointments" class="flex-1 text-center border border-gray-200 text-txt-secondary rounded-xl px-4 py-2.5 text-sm font-medium hover:bg-surface-subtle active:scale-[0.98] transition-all min-h-[44px] flex items-center justify-center">
                    {t("medical.session.book_appointment")}
                </a>
            </div>
        </div>
    }
}
