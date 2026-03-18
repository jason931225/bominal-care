// =============================================================================
// Audit log server functions — web-government portal
// =============================================================================

use leptos::prelude::*;
use leptos::server;

use bominal_db::queries::audit::{
    self, AuditLogFilters, AuditLogRow, Pagination, PaginatedResult,
};
use bominal_types::AuditAction;

/// Paginated list of audit logs with optional action and entity_type filters.
#[server]
pub async fn list_audit_logs(
    action: Option<String>,
    entity_type: Option<String>,
    page: i64,
    limit: i64,
) -> Result<PaginatedResult<AuditLogRow>, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("Database pool not available"))?;

    let parsed_action = action
        .as_deref()
        .map(|s| {
            s.parse::<AuditAction>()
                .map_err(|_| ServerFnError::new(format!("Invalid action: {s}")))
        })
        .transpose()?;

    let filters = AuditLogFilters {
        action: parsed_action,
        entity_type,
        ..Default::default()
    };

    let pagination = Pagination {
        page: page.max(1),
        limit: limit.clamp(1, 100),
    };

    audit::list_audit_logs(&pool, &filters, &pagination)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
