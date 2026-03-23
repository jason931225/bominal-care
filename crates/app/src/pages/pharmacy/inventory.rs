use leptos::prelude::*;

use crate::i18n::t;

// =============================================================================
// Inventory — placeholder page for pharmacy stock management
// =============================================================================

/// Placeholder page for pharmacy inventory management.
///
/// Will be expanded to list current stock levels, expiry dates, and
/// reorder thresholds once the inventory API is implemented.
#[component]
pub fn InventoryPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-gray-900">{t("pharmacy.inventory.title")}</h1>
                <p class="text-sm text-gray-500 mt-1">{t("pharmacy.inventory.subtitle")}</p>
            </div>
            <div class="bg-surface-card rounded-2xl p-8 shadow-sm text-center">
                <p class="text-gray-400 text-lg">{t("common.preparing")}</p>
            </div>
        </div>
    }
}
