use leptos::prelude::*;

use crate::i18n::t;

// =============================================================================
// Fulfillment — placeholder page for prescription fulfillment tracking
// =============================================================================

/// Placeholder page for prescription fulfillment tracking.
///
/// Will be expanded to show completed dispensing records, delivery status,
/// and fulfillment history once the fulfillment API is implemented.
#[component]
pub fn FulfillmentPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-gray-900">{t("pharmacy.fulfillment.title")}</h1>
                <p class="text-sm text-gray-500 mt-1">{t("pharmacy.fulfillment.subtitle")}</p>
            </div>
            <div class="bg-surface-card rounded-2xl p-8 shadow-sm text-center">
                <p class="text-gray-400 text-lg">{t("common.preparing")}</p>
            </div>
        </div>
    }
}
