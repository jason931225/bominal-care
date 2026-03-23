use leptos::prelude::*;

use crate::components::data_display::EmptyState;
use crate::components::layout::PageHeader;
use crate::i18n::t;

// =============================================================================
// Dispensing Queue — list and confirm pending prescriptions
// =============================================================================

/// Dispensing queue page for pharmacy staff.
///
/// Fetches signed prescriptions awaiting dispensing and provides a confirm
/// button for each row. On confirmation, a POST is sent to the backend and
/// the item is removed from the local list.
#[component]
pub fn QueuePage() -> impl IntoView {
    let queue_data = LocalResource::new(|| {
        crate::api::get::<Vec<serde_json::Value>>("/api/dispensing/queue")
    });

    // Local list signal: populated once the resource resolves.
    let items = RwSignal::new(Vec::<serde_json::Value>::new());
    let loaded = RwSignal::new(false);
    let confirming_id = RwSignal::new(None::<String>);
    let error_msg = RwSignal::new(None::<String>);
    let success_msg = RwSignal::new(None::<String>);

    view! {
        <div class="space-y-8">
            <PageHeader
                title=t("pharmacy.queue.title").to_string()
                subtitle=t("pharmacy.queue.subtitle").to_string()
            />

            // Success / error feedback
            {move || error_msg.get().map(|msg| view! {
                <div class="bg-red-50 border border-red-200 rounded-xl p-4 text-sm text-red-700">{msg}</div>
            })}
            {move || success_msg.get().map(|msg| view! {
                <div class="bg-green-50 border border-green-200 rounded-xl p-4 text-sm text-green-700">{msg}</div>
            })}

            // Queue table
            <Suspense fallback=move || view! {
                <div class="animate-pulse bg-gray-200 rounded-2xl h-40" />
            }>
                {move || Suspend::new(async move {
                    if !loaded.get_untracked() {
                        match queue_data.await {
                            Ok(resp) if resp.success => {
                                items.set(resp.data.unwrap_or_default());
                            }
                            _ => {}
                        }
                        loaded.set(true);
                    }

                    view! {
                        <div>
                            {move || {
                                let current_items = items.get();
                                if current_items.is_empty() {
                                    view! { <EmptyState message=t("pharmacy.queue.empty").to_string() /> }.into_any()
                                } else {
                                    view! {
                                        <div class="overflow-x-auto shadow-sm rounded-2xl overflow-hidden">
                                            <table class="min-w-full divide-y divide-gray-100 text-sm">
                                                <thead class="bg-surface-subtle">
                                                    <tr>
                                                        <th class="px-4 py-3 text-left font-medium text-txt-secondary">{t("pharmacy.queue.senior_name")}</th>
                                                        <th class="px-4 py-3 text-left font-medium text-txt-secondary">{t("pharmacy.queue.medication")}</th>
                                                        <th class="px-4 py-3 text-left font-medium text-txt-secondary">{t("pharmacy.queue.dosage")}</th>
                                                        <th class="px-4 py-3 text-left font-medium text-txt-secondary">{t("pharmacy.queue.frequency")}</th>
                                                        <th class="px-4 py-3 text-left font-medium text-txt-secondary">{t("pharmacy.queue.prescribed_date")}</th>
                                                        <th class="px-4 py-3 text-right font-medium text-txt-secondary">{t("pharmacy.queue.confirm")}</th>
                                                    </tr>
                                                </thead>
                                                <tbody class="bg-white divide-y divide-gray-50">
                                                    {current_items.into_iter().map(|item| {
                                                        let prescription_id = item.get("id")
                                                            .and_then(|v| v.as_str())
                                                            .unwrap_or("")
                                                            .to_string();
                                                        let senior_name = item.get("senior_name")
                                                            .and_then(|v| v.as_str())
                                                            .unwrap_or("-")
                                                            .to_string();
                                                        let medication = item.get("medication_name")
                                                            .and_then(|v| v.as_str())
                                                            .unwrap_or("-")
                                                            .to_string();
                                                        let dosage = item.get("dosage")
                                                            .and_then(|v| v.as_str())
                                                            .unwrap_or("-")
                                                            .to_string();
                                                        let frequency = item.get("frequency")
                                                            .and_then(|v| v.as_str())
                                                            .unwrap_or("-")
                                                            .to_string();
                                                        let created_at = item.get("created_at")
                                                            .and_then(|v| v.as_str())
                                                            .unwrap_or("-")
                                                            .to_string();
                                                        let pid = prescription_id.clone();
                                                        let pid_for_check = prescription_id.clone();

                                                        let on_confirm = move |_| {
                                                            let pid = pid.clone();
                                                            error_msg.set(None);
                                                            success_msg.set(None);
                                                            confirming_id.set(Some(pid.clone()));

                                                            leptos::task::spawn_local(async move {
                                                                let url = format!("/api/dispensing/{}/confirm", pid);
                                                                let body = serde_json::json!({
                                                                    "notes": "",
                                                                    "quantity_dispensed": 1
                                                                });
                                                                match crate::api::post::<serde_json::Value, _>(&url, &body).await {
                                                                    Ok(resp) if resp.success => {
                                                                        success_msg.set(Some(t("pharmacy.queue.confirmed").to_string()));
                                                                        items.update(|list| {
                                                                            list.retain(|item| {
                                                                                item.get("id")
                                                                                    .and_then(|v| v.as_str())
                                                                                    .unwrap_or("")
                                                                                    != pid
                                                                            });
                                                                        });
                                                                    }
                                                                    Ok(resp) => {
                                                                        error_msg.set(
                                                                            resp.error
                                                                                .or_else(|| Some(t("common.error_occurred").to_string())),
                                                                        );
                                                                    }
                                                                    Err(e) => error_msg.set(Some(e)),
                                                                }
                                                                confirming_id.set(None);
                                                            });
                                                        };

                                                        let pid_for_disabled = pid_for_check.clone();

                                                        view! {
                                                            <tr class="hover:bg-gray-50 transition-colors">
                                                                <td class="px-4 py-3 text-txt-primary font-medium">{senior_name}</td>
                                                                <td class="px-4 py-3 text-txt-secondary">{medication}</td>
                                                                <td class="px-4 py-3 text-txt-secondary">{dosage}</td>
                                                                <td class="px-4 py-3 text-txt-secondary">{frequency}</td>
                                                                <td class="px-4 py-3 text-txt-tertiary text-xs">{created_at}</td>
                                                                <td class="px-4 py-3 text-right">
                                                                    <button
                                                                        class="px-3 py-1.5 bg-[var(--portal-accent)] text-white text-xs font-semibold rounded-lg hover:opacity-90 active:scale-[0.98] transition-all disabled:opacity-50 min-h-[36px]"
                                                                        disabled=move || confirming_id.get().as_deref() == Some(pid_for_disabled.as_str())
                                                                        on:click=on_confirm
                                                                    >
                                                                        {move || if confirming_id.get().as_deref() == Some(pid_for_check.as_str()) { t("pharmacy.queue.confirming") } else { t("pharmacy.queue.confirm") }}
                                                                    </button>
                                                                </td>
                                                            </tr>
                                                        }
                                                    }).collect_view()}
                                                </tbody>
                                            </table>
                                        </div>
                                    }.into_any()
                                }
                            }}
                        </div>
                    }.into_any()
                })}
            </Suspense>
        </div>
    }
}
