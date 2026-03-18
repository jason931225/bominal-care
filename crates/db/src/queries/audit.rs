// =============================================================================
// Audit queries — AuditLog management
// Ported from packages/db/src/services/audit.service.ts
// =============================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use bominal_types::{AuditAction, DateRange};

// -----------------------------------------------------------------------------
// Input types
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditData {
    pub user_id: Option<Uuid>,
    pub action: AuditAction,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub old_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditLogFilters {
    pub user_id: Option<Uuid>,
    pub action: Option<AuditAction>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub date_range: Option<DateRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResult<T> {
    pub data: Vec<T>,
    pub total: i64,
}

// -----------------------------------------------------------------------------
// Row type for audit log queries
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuditLogRow {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: AuditAction,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub old_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

// Row type for COUNT(*) result
#[derive(Debug, sqlx::FromRow)]
struct CountRow {
    count: i64,
}

// -----------------------------------------------------------------------------
// Service functions
// -----------------------------------------------------------------------------

pub async fn log_audit(
    pool: &PgPool,
    data: &LogAuditData,
) -> Result<AuditLogRow, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now();

    sqlx::query_as::<_, AuditLogRow>(
        "INSERT INTO audit_logs (
           id, user_id, action, entity_type, entity_id,
           old_value, new_value, ip_address, user_agent, created_at
         ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
         RETURNING *",
    )
    .bind(id)
    .bind(data.user_id)
    .bind(data.action)
    .bind(&data.entity_type)
    .bind(&data.entity_id)
    .bind(&data.old_value)
    .bind(&data.new_value)
    .bind(&data.ip_address)
    .bind(&data.user_agent)
    .bind(now)
    .fetch_one(pool)
    .await
}

pub async fn list_audit_logs(
    pool: &PgPool,
    filters: &AuditLogFilters,
    pagination: &Pagination,
) -> Result<PaginatedResult<AuditLogRow>, sqlx::Error> {
    // Build dynamic WHERE clauses
    let mut conditions: Vec<String> = Vec::new();
    let mut param_idx: i32 = 1;

    if filters.user_id.is_some() {
        conditions.push(format!("user_id = ${}", param_idx));
        param_idx += 1;
    }
    if filters.action.is_some() {
        conditions.push(format!("action = ${}", param_idx));
        param_idx += 1;
    }
    if filters.entity_type.is_some() {
        conditions.push(format!("entity_type = ${}", param_idx));
        param_idx += 1;
    }
    if filters.entity_id.is_some() {
        conditions.push(format!("entity_id = ${}", param_idx));
        param_idx += 1;
    }
    if filters.date_range.is_some() {
        conditions.push(format!(
            "created_at >= ${} AND created_at <= ${}",
            param_idx,
            param_idx + 1
        ));
        param_idx += 2;
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    let offset = (pagination.page - 1) * pagination.limit;

    let data_sql = format!(
        "SELECT * FROM audit_logs {} ORDER BY created_at DESC LIMIT ${} OFFSET ${}",
        where_clause, param_idx, param_idx + 1
    );

    let count_sql = format!("SELECT COUNT(*)::bigint AS count FROM audit_logs {}", where_clause);

    // Build and execute both queries with matching binds
    let mut data_query = sqlx::query_as::<_, AuditLogRow>(&data_sql);
    let mut count_query = sqlx::query_as::<_, CountRow>(&count_sql);

    // Bind filter values in order
    if let Some(user_id) = filters.user_id {
        data_query = data_query.bind(user_id);
        count_query = count_query.bind(user_id);
    }
    if let Some(action) = filters.action {
        data_query = data_query.bind(action);
        count_query = count_query.bind(action);
    }
    if let Some(ref entity_type) = filters.entity_type {
        data_query = data_query.bind(entity_type.clone());
        count_query = count_query.bind(entity_type.clone());
    }
    if let Some(ref entity_id) = filters.entity_id {
        data_query = data_query.bind(entity_id.clone());
        count_query = count_query.bind(entity_id.clone());
    }
    if let Some(ref date_range) = filters.date_range {
        data_query = data_query.bind(date_range.from).bind(date_range.to);
        count_query = count_query.bind(date_range.from).bind(date_range.to);
    }

    // Bind pagination params (data query only)
    data_query = data_query.bind(pagination.limit).bind(offset);

    // Execute both queries concurrently
    let (data_result, count_result) =
        tokio::try_join!(data_query.fetch_all(pool), count_query.fetch_one(pool))?;

    Ok(PaginatedResult {
        data: data_result,
        total: count_result.count,
    })
}
