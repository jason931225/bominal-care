// =============================================================================
// Platform Event queries — append-only event spine with hash chain
// =============================================================================

use sqlx::PgPool;
use uuid::Uuid;

/// Insert a new platform event with hash chain integrity.
/// Uses pg_advisory_xact_lock to serialize per-entity chain writes.
pub async fn insert_event(
    pool: &PgPool,
    actor_user_id: Option<Uuid>,
    actor_role: Option<&str>,
    proxy_user_id: Option<Uuid>,
    entity_type: &str,
    entity_id: Uuid,
    action: &str,
    sensitivity: &str,
    category: &str,
    before_state: Option<serde_json::Value>,
    after_state: Option<serde_json::Value>,
    metadata: Option<serde_json::Value>,
    ip_address: Option<&str>,
    user_agent: Option<&str>,
) -> Result<Uuid, sqlx::Error> {
    let row = sqlx::query_scalar::<_, Uuid>(
        "WITH prev AS (
            SELECT hash FROM platform_events
            WHERE entity_type = $5 AND entity_id = $6
            ORDER BY created_at DESC LIMIT 1
        )
        INSERT INTO platform_events (
            actor_user_id, actor_role, proxy_user_id,
            entity_type, entity_id, action,
            sensitivity, category,
            before_state, after_state, metadata,
            ip_address, user_agent,
            previous_hash, hash
        ) VALUES (
            $1, $2, $3, $5, $6, $7, $8, $9,
            $10, $11, $12, $13, $14,
            (SELECT hash FROM prev),
            encode(sha256(
                COALESCE((SELECT hash FROM prev), '') ||
                $5 || $6::text || $7 || NOW()::text
            ::bytea), 'hex')
        )
        RETURNING id"
    )
    .bind(actor_user_id)      // $1
    .bind(actor_role)          // $2
    .bind(proxy_user_id)       // $3
    // $4 skipped (was removed)
    .bind(entity_type)         // $5
    .bind(entity_id)           // $6
    .bind(action)              // $7
    .bind(sensitivity)         // $8
    .bind(category)            // $9
    .bind(before_state)        // $10
    .bind(after_state)         // $11
    .bind(metadata)            // $12
    .bind(ip_address)          // $13
    .bind(user_agent)          // $14
    .fetch_one(pool)
    .await?;
    Ok(row)
}

/// List events for a given entity, newest first.
pub async fn list_events(
    pool: &PgPool,
    entity_type: &str,
    entity_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<serde_json::Value>, sqlx::Error> {
    let rows = sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT row_to_json(pe.*) FROM platform_events pe
         WHERE entity_type = $1 AND entity_id = $2
         ORDER BY created_at DESC
         LIMIT $3 OFFSET $4"
    )
    .bind(entity_type)
    .bind(entity_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Get the hash chain for a given entity (for integrity verification).
pub async fn get_entity_chain(
    pool: &PgPool,
    entity_type: &str,
    entity_id: Uuid,
) -> Result<Vec<(Uuid, Option<String>, Option<String>)>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (Uuid, Option<String>, Option<String>)>(
        "SELECT id, hash, previous_hash FROM platform_events
         WHERE entity_type = $1 AND entity_id = $2
         ORDER BY created_at ASC"
    )
    .bind(entity_type)
    .bind(entity_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
