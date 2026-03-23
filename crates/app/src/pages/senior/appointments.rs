use leptos::prelude::*;
use uuid::Uuid;

use bominal_types::Appointment;
use crate::components::data_display::EmptyState;
use crate::components::layout::PageHeader;
use super::FormRow;

/// Upcoming and past appointments with pagination.
#[component]
pub fn AppointmentsListPage() -> impl IntoView {
    let appointments = LocalResource::new(|| {
        crate::api::get::<Vec<Appointment>>("/api/appointments?page=1&limit=50")
    });

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <PageHeader title="진료 예약" subtitle="예약 내역을 관리하세요" />
            <a href="/appointments/new"
               class="block w-full text-center bg-primary text-white text-lg font-semibold \
                      rounded-xl py-4 hover:bg-primary-hover active:scale-[0.98] transition-all">
                "새 예약 등록"
            </a>
            <Suspense fallback=move || view! { <div class="skeleton h-8 w-20"></div> }>
                {move || Suspend::new(async move {
                    match appointments.await {
                        Ok(resp) if resp.success => {
                            let items = resp.data.unwrap_or_default();
                            if items.is_empty() {
                                view! { <EmptyState message="예약 내역이 없습니다." /> }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-3">
                                        {items.into_iter().map(|appt| {
                                            let status_class = match format!("{}", appt.status).as_str() {
                                                "COMPLETED" => "bg-success-light text-success",
                                                "CANCELLED" | "NO_SHOW" => "bg-danger-light text-danger",
                                                _ => "bg-primary-light text-primary",
                                            };
                                            view! {
                                                <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                                                    <div class="flex items-center justify-between">
                                                        <p class="text-lg font-medium text-txt-primary">{appt.institution_name}</p>
                                                        <span class={format!("text-xs px-2 py-1 rounded-full {status_class}")}>
                                                            {format!("{}", appt.status)}
                                                        </span>
                                                    </div>
                                                    <p class="text-base text-txt-secondary mt-1">{format!("{}", appt.appointment_date.format("%Y-%m-%d"))}</p>
                                                    {appt.purpose.map(|p| view! { <p class="text-base text-txt-tertiary mt-1">{p}</p> })}
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_any()
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

// =============================================================================
// Appointment Detail — API-driven
// =============================================================================

/// Map appointment status string to (Korean label, badge CSS classes).
fn status_badge(status: &str) -> (&'static str, &'static str) {
    match status {
        "COMPLETED" => ("진료 완료", "bg-success-light text-success"),
        "CANCELLED" => ("취소됨", "bg-danger-light text-danger"),
        "SCHEDULED" => ("예약됨", "bg-primary-light text-primary"),
        "CONFIRMED" => ("확인됨", "bg-success-light text-success"),
        _ => ("상태 불명", "bg-surface-subtle text-txt-tertiary"),
    }
}

/// Single appointment detail view (API-driven).
#[component]
pub fn AppointmentDetailPage(
    #[prop(into)] appointment_id: Uuid,
) -> impl IntoView {
    let appt_id = appointment_id;
    let appointment = LocalResource::new(move || {
        let id = appt_id;
        async move {
            crate::api::get::<serde_json::Value>(&format!("/api/appointments/{}", id)).await
        }
    });

    let cancel_error = RwSignal::new(Option::<String>::None);
    let cancelling = RwSignal::new(false);

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/appointments" class="text-primary text-lg">"< 예약 목록"</a>

            <Show when=move || cancel_error.get().is_some()>
                <div class="bg-danger-light rounded-2xl p-4 text-danger text-lg">
                    {move || cancel_error.get().unwrap_or_default()}
                </div>
            </Show>

            <Suspense fallback=move || view! { <div class="skeleton h-8 w-20"></div> }>
                {move || {
                    let cancel_id = appt_id;
                    Suspend::new(async move {
                        match appointment.await {
                            Ok(resp) if resp.success => {
                                match resp.data {
                                    Some(appt) => {
                                        let institution = appt
                                            .get("institution_name")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("알 수 없음")
                                            .to_string();
                                        let date = appt
                                            .get("appointment_date")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("")
                                            .to_string();
                                        let purpose = appt
                                            .get("purpose")
                                            .and_then(|v| v.as_str())
                                            .map(|s| s.to_string());
                                        let address = appt
                                            .get("address")
                                            .and_then(|v| v.as_str())
                                            .map(|s| s.to_string());
                                        let notes = appt
                                            .get("notes")
                                            .and_then(|v| v.as_str())
                                            .map(|s| s.to_string());
                                        let status_raw = appt
                                            .get("status")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("SCHEDULED")
                                            .to_string();

                                        let (status_label, status_class) =
                                            status_badge(&status_raw);
                                        let status_label = status_label.to_string();
                                        let status_class = status_class.to_string();

                                        let can_cancel =
                                            status_raw == "SCHEDULED" || status_raw == "CONFIRMED";

                                        let on_cancel = move |_| {
                                            let id = cancel_id;
                                            cancelling.set(true);
                                            cancel_error.set(None);
                                            leptos::task::spawn_local(async move {
                                                match crate::api::delete::<serde_json::Value>(
                                                    &format!("/api/appointments/{}", id),
                                                )
                                                .await
                                                {
                                                    Ok(resp) if resp.success => {
                                                        if let Some(window) =
                                                            leptos::web_sys::window()
                                                        {
                                                            let _ = window
                                                                .location()
                                                                .set_href("/appointments");
                                                        }
                                                    }
                                                    Ok(resp) => {
                                                        cancel_error.set(Some(
                                                            resp.error.unwrap_or_else(|| {
                                                                "취소 중 오류가 발생했습니다."
                                                                    .to_string()
                                                            }),
                                                        ));
                                                        cancelling.set(false);
                                                    }
                                                    Err(e) => {
                                                        cancel_error.set(Some(e));
                                                        cancelling.set(false);
                                                    }
                                                }
                                            });
                                        };

                                        view! {
                                            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-1">
                                                <div class="flex items-center justify-between mb-3">
                                                    <h1 class="text-xl font-bold text-txt-primary">
                                                        {institution}
                                                    </h1>
                                                    <span class={format!(
                                                        "text-xs px-2 py-1 rounded-full {status_class}"
                                                    )}>
                                                        {status_label}
                                                    </span>
                                                </div>

                                                <div class="flex justify-between py-2 border-b border-surface-subtle">
                                                    <span class="text-sm text-txt-tertiary">
                                                        "예약 날짜"
                                                    </span>
                                                    <span class="text-sm font-medium text-txt-primary">
                                                        {date}
                                                    </span>
                                                </div>

                                                {purpose.map(|p| view! {
                                                    <div class="flex justify-between py-2 border-b border-surface-subtle">
                                                        <span class="text-sm text-txt-tertiary">
                                                            "진료 목적"
                                                        </span>
                                                        <span class="text-sm font-medium text-txt-primary">
                                                            {p}
                                                        </span>
                                                    </div>
                                                })}

                                                {address.map(|a| view! {
                                                    <div class="flex justify-between py-2 border-b border-surface-subtle">
                                                        <span class="text-sm text-txt-tertiary">
                                                            "주소"
                                                        </span>
                                                        <span class="text-sm font-medium text-txt-primary">
                                                            {a}
                                                        </span>
                                                    </div>
                                                })}

                                                {notes.map(|n| view! {
                                                    <div class="flex justify-between py-2 border-b border-surface-subtle">
                                                        <span class="text-sm text-txt-tertiary">
                                                            "메모"
                                                        </span>
                                                        <span class="text-sm font-medium text-txt-primary">
                                                            {n}
                                                        </span>
                                                    </div>
                                                })}
                                            </div>

                                            {if can_cancel {
                                                Some(view! {
                                                    <button
                                                        class="w-full bg-danger text-white text-lg font-semibold rounded-xl \
                                                               py-4 hover:bg-danger-hover active:scale-[0.98] transition-all \
                                                               disabled:opacity-50"
                                                        disabled=move || cancelling.get()
                                                        on:click=on_cancel
                                                    >
                                                        {move || {
                                                            if cancelling.get() {
                                                                "취소 중..."
                                                            } else {
                                                                "예약 취소"
                                                            }
                                                        }}
                                                    </button>
                                                })
                                            } else {
                                                None
                                            }}
                                        }
                                        .into_any()
                                    }
                                    None => {
                                        view! {
                                            <EmptyState message="예약을 찾을 수 없습니다." />
                                        }
                                        .into_any()
                                    }
                                }
                            }
                            Ok(resp) => {
                                view! {
                                    <p class="text-danger">
                                        {resp.error.unwrap_or_default()}
                                    </p>
                                }
                                .into_any()
                            }
                            Err(e) => view! { <p class="text-danger">{e}</p> }.into_any(),
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}

/// Request body for creating a new appointment.
#[derive(Debug, Clone, serde::Serialize)]
struct CreateAppointmentRequest {
    pub institution_name: String,
    pub appointment_date: String,
    pub purpose: Option<String>,
    pub address: Option<String>,
    pub notes: Option<String>,
}

/// Form to create a new appointment.
#[component]
pub fn AppointmentNewPage() -> impl IntoView {
    let institution = RwSignal::new(String::new());
    let date = RwSignal::new(String::new());
    let purpose = RwSignal::new(String::new());
    let address = RwSignal::new(String::new());
    let notes = RwSignal::new(String::new());
    let submitted = RwSignal::new(false);
    let submitting = RwSignal::new(false);
    let error_msg = RwSignal::new(Option::<String>::None);

    let on_submit = move |_| {
        let inst = institution.get_untracked();
        let dt = date.get_untracked();
        let purp = purpose.get_untracked();
        let addr = address.get_untracked();
        let nts = notes.get_untracked();

        if inst.is_empty() || dt.is_empty() {
            error_msg.set(Some(
                "의료기관명과 예약 날짜를 입력해주세요.".to_string(),
            ));
            return;
        }

        submitting.set(true);
        error_msg.set(None);

        let body = CreateAppointmentRequest {
            institution_name: inst,
            appointment_date: dt,
            purpose: if purp.is_empty() { None } else { Some(purp) },
            address: if addr.is_empty() { None } else { Some(addr) },
            notes: if nts.is_empty() { None } else { Some(nts) },
        };

        leptos::task::spawn_local(async move {
            match crate::api::post::<Appointment, _>("/api/appointments", &body).await {
                Ok(resp) if resp.success => {
                    submitted.set(true);
                    submitting.set(false);
                }
                Ok(resp) => {
                    error_msg.set(Some(
                        resp.error
                            .unwrap_or_else(|| "오류가 발생했습니다.".to_string()),
                    ));
                    submitting.set(false);
                }
                Err(e) => {
                    error_msg.set(Some(e));
                    submitting.set(false);
                }
            }
        });
    };

    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/appointments" class="text-primary text-lg">"< 예약 목록"</a>
            <PageHeader title="새 예약 등록" />
            <Show when=move || submitted.get()>
                <div class="bg-success-light rounded-2xl p-4 text-success text-lg">
                    "예약이 등록되었습니다!"
                </div>
            </Show>
            <Show when=move || error_msg.get().is_some()>
                <div class="bg-danger-light rounded-2xl p-4 text-danger text-lg">
                    {move || error_msg.get().unwrap_or_default()}
                </div>
            </Show>
            <div class="bg-surface-card rounded-2xl p-5 shadow-sm space-y-4">
                <FormRow label="의료기관명">
                    <input
                        type="text"
                        class="w-full px-4 py-3 border border-surface-subtle rounded-xl text-lg \
                               focus:outline-none focus:ring-2 focus:ring-primary"
                        prop:value=move || institution.get()
                        on:input=move |ev| institution.set(event_target_value(&ev))
                        placeholder="병원/의원 이름"
                    />
                </FormRow>
                <FormRow label="예약 날짜">
                    <input
                        type="date"
                        class="w-full px-4 py-3 border border-surface-subtle rounded-xl text-lg \
                               focus:outline-none focus:ring-2 focus:ring-primary"
                        prop:value=move || date.get()
                        on:input=move |ev| date.set(event_target_value(&ev))
                    />
                </FormRow>
                <FormRow label="방문 목적">
                    <input
                        type="text"
                        class="w-full px-4 py-3 border border-surface-subtle rounded-xl text-lg \
                               focus:outline-none focus:ring-2 focus:ring-primary"
                        prop:value=move || purpose.get()
                        on:input=move |ev| purpose.set(event_target_value(&ev))
                        placeholder="진료, 검사 등"
                    />
                </FormRow>
                <FormRow label="주소">
                    <input
                        type="text"
                        class="w-full px-4 py-3 border border-surface-subtle rounded-xl text-lg \
                               focus:outline-none focus:ring-2 focus:ring-primary"
                        prop:value=move || address.get()
                        on:input=move |ev| address.set(event_target_value(&ev))
                        placeholder="병원 주소"
                    />
                </FormRow>
                <FormRow label="메모">
                    <textarea
                        class="w-full px-4 py-3 border border-surface-subtle rounded-xl text-lg \
                               focus:outline-none focus:ring-2 focus:ring-primary"
                        rows=3
                        prop:value=move || notes.get()
                        on:input=move |ev| notes.set(event_target_value(&ev))
                        placeholder="추가 메모"
                    />
                </FormRow>
                <button
                    class="w-full bg-primary text-white text-lg font-semibold rounded-xl \
                           py-4 hover:bg-primary-hover active:scale-[0.98] transition-all \
                           disabled:opacity-50"
                    disabled=move || submitting.get()
                    on:click=on_submit
                >
                    {move || if submitting.get() { "등록 중..." } else { "예약 등록" }}
                </button>
            </div>
        </div>
    }
}
