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

/// Single appointment detail view.
#[component]
pub fn AppointmentDetailPage(
    #[prop(into)] appointment_id: Uuid,
) -> impl IntoView {
    let _ = appointment_id;
    view! {
        <div class="max-w-lg mx-auto px-4 py-6 space-y-4">
            <a href="/appointments" class="text-primary text-lg">"< 예약 목록"</a>
            <EmptyState message="예약을 찾을 수 없습니다." />
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
            error_msg.set(Some("의료기관명과 예약 날짜를 입력해주세요.".to_string()));
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
                    error_msg.set(Some(resp.error.unwrap_or_else(|| "오류가 발생했습니다.".to_string())));
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
                    <input type="text"
                        class="w-full px-4 py-3 border border-gray-300 rounded-xl text-lg \
                               focus:outline-none focus:ring-2 focus:ring-primary"
                        prop:value=move || institution.get()
                        on:input=move |ev| institution.set(event_target_value(&ev))
                        placeholder="병원/의원 이름"
                    />
                </FormRow>
                <FormRow label="예약 날짜">
                    <input type="date"
                        class="w-full px-4 py-3 border border-gray-300 rounded-xl text-lg \
                               focus:outline-none focus:ring-2 focus:ring-primary"
                        prop:value=move || date.get()
                        on:input=move |ev| date.set(event_target_value(&ev))
                    />
                </FormRow>
                <FormRow label="방문 목적">
                    <input type="text"
                        class="w-full px-4 py-3 border border-gray-300 rounded-xl text-lg \
                               focus:outline-none focus:ring-2 focus:ring-primary"
                        prop:value=move || purpose.get()
                        on:input=move |ev| purpose.set(event_target_value(&ev))
                        placeholder="진료, 검사 등"
                    />
                </FormRow>
                <FormRow label="주소">
                    <input type="text"
                        class="w-full px-4 py-3 border border-gray-300 rounded-xl text-lg \
                               focus:outline-none focus:ring-2 focus:ring-primary"
                        prop:value=move || address.get()
                        on:input=move |ev| address.set(event_target_value(&ev))
                        placeholder="병원 주소"
                    />
                </FormRow>
                <FormRow label="메모">
                    <textarea
                        class="w-full px-4 py-3 border border-gray-300 rounded-xl text-lg \
                               focus:outline-none focus:ring-2 focus:ring-primary"
                        rows=3
                        prop:value=move || notes.get()
                        on:input=move |ev| notes.set(event_target_value(&ev))
                        placeholder="추가 메모"
                    />
                </FormRow>
                <button
                    class="w-full bg-primary text-white text-lg font-semibold rounded-xl \
                           py-4 hover:bg-primary-hover active:scale-[0.98] transition-all disabled:opacity-50"
                    disabled=move || submitting.get()
                    on:click=on_submit
                >
                    {move || if submitting.get() { "등록 중..." } else { "예약 등록" }}
                </button>
            </div>
        </div>
    }
}
