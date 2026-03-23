// =============================================================================
// Pharmacy Portal Pages
// =============================================================================
//
// Bominal Pharmacy portal for pharmacy staff. Handles dispensing queue
// management, inventory tracking, and prescription fulfillment.
//
// Submodules:
//   - queue: QueuePage (dispensing queue)
//   - inventory: InventoryPage (placeholder)
//   - fulfillment: FulfillmentPage (placeholder)
// =============================================================================

mod fulfillment;
mod inventory;
mod queue;

pub use fulfillment::*;
pub use inventory::*;
pub use queue::*;

use leptos::prelude::*;

use crate::components::layout::PageHeader;
use crate::i18n::t;

// =============================================================================
// Dashboard
// =============================================================================

/// Pharmacy portal dashboard showing pending prescription count, quick action
/// cards, and navigation to dispensing queue, inventory, and fulfillment.
#[component]
pub fn DashboardPage() -> impl IntoView {
    let queue_data = LocalResource::new(|| {
        crate::api::get::<Vec<serde_json::Value>>("/api/dispensing/queue")
    });

    view! {
        <div class="space-y-8">
            <PageHeader
                title=t("pharmacy.dashboard.title").to_string()
                subtitle=t("pharmacy.dashboard.subtitle").to_string()
            />

            // KPI cards
            <Suspense fallback=move || view! {
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                    <div class="animate-pulse bg-gray-200 rounded-2xl h-24" />
                    <div class="animate-pulse bg-gray-200 rounded-2xl h-24" />
                    <div class="animate-pulse bg-gray-200 rounded-2xl h-24" />
                </div>
            }>
                {move || Suspend::new(async move {
                    let pending_count = match queue_data.await {
                        Ok(resp) if resp.success => {
                            resp.data
                                .as_ref()
                                .map(|v| v.len())
                                .unwrap_or(0)
                                .to_string()
                        }
                        _ => "0".to_string(),
                    };

                    view! {
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                            <div class="bg-surface-card rounded-2xl p-5 shadow-sm">
                                <p class="text-sm text-txt-tertiary">{t("pharmacy.dashboard.pending")}</p>
                                <p class="text-2xl font-bold text-txt-primary mt-1">{pending_count}</p>
                                <p class="text-xs text-[var(--portal-accent)] mt-1">{t("pharmacy.dashboard.pending_sub")}</p>
                            </div>
                        </div>
                    }.into_any()
                })}
            </Suspense>

            // Quick action cards
            <div>
                <h2 class="font-semibold text-txt-primary mb-3">{t("pharmacy.dashboard.quick_actions")}</h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                    <a href="/pharmacy/queue" class="bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200 cursor-pointer min-h-[44px]">
                        <div class="flex items-center gap-3">
                            <div class="w-10 h-10 bg-[var(--portal-accent-light)] rounded-lg flex items-center justify-center">
                                <svg class="w-5 h-5 text-[var(--portal-accent)]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01" />
                                </svg>
                            </div>
                            <div>
                                <p class="text-sm font-medium text-txt-primary">{t("pharmacy.dashboard.action_queue")}</p>
                                <p class="text-xs text-txt-tertiary">{t("pharmacy.dashboard.action_queue_sub")}</p>
                            </div>
                        </div>
                    </a>
                    <a href="/pharmacy/inventory" class="bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200 cursor-pointer min-h-[44px]">
                        <div class="flex items-center gap-3">
                            <div class="w-10 h-10 bg-[var(--portal-accent-light)] rounded-lg flex items-center justify-center">
                                <svg class="w-5 h-5 text-[var(--portal-accent)]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
                                </svg>
                            </div>
                            <div>
                                <p class="text-sm font-medium text-txt-primary">{t("pharmacy.dashboard.action_inventory")}</p>
                                <p class="text-xs text-txt-tertiary">{t("pharmacy.dashboard.action_inventory_sub")}</p>
                            </div>
                        </div>
                    </a>
                    <a href="/pharmacy/fulfillment" class="bg-surface-card rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow duration-200 cursor-pointer min-h-[44px]">
                        <div class="flex items-center gap-3">
                            <div class="w-10 h-10 bg-[var(--portal-accent-light)] rounded-lg flex items-center justify-center">
                                <svg class="w-5 h-5 text-[var(--portal-accent)]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                </svg>
                            </div>
                            <div>
                                <p class="text-sm font-medium text-txt-primary">{t("pharmacy.dashboard.action_fulfillment")}</p>
                                <p class="text-xs text-txt-tertiary">{t("pharmacy.dashboard.action_fulfillment_sub")}</p>
                            </div>
                        </div>
                    </a>
                </div>
            </div>
        </div>
    }
}
