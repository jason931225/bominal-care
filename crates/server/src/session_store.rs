// =============================================================================
// Custom PostgreSQL session store for tower-sessions 0.15
//
// Bypasses the tower-sessions-sqlx-store crate which has a
// tower-sessions-core 0.14 vs 0.15 version mismatch.
// Uses migration 0003_session_store.sql table: tower_sessions(id, data, expiry_date)
// =============================================================================

use async_trait::async_trait;
use sqlx::PgPool;
use time::OffsetDateTime;
use tower_sessions::session::{Id, Record};
use tower_sessions::session_store;

#[derive(Debug, Clone)]
pub struct PgSessionStore {
    pool: PgPool,
}

impl PgSessionStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl session_store::ExpiredDeletion for PgSessionStore {
    async fn delete_expired(&self) -> session_store::Result<()> {
        sqlx::query("DELETE FROM tower_sessions WHERE expiry_date < NOW()")
            .execute(&self.pool)
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl tower_sessions::SessionStore for PgSessionStore {
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        let id = record.id.to_string();
        let data = serde_json::to_vec(&record.data)
            .map_err(|e| session_store::Error::Encode(e.to_string()))?;

        sqlx::query(
            "INSERT INTO tower_sessions (id, data, expiry_date) VALUES ($1, $2, $3)
             ON CONFLICT (id) DO UPDATE SET data = EXCLUDED.data, expiry_date = EXCLUDED.expiry_date",
        )
        .bind(&id)
        .bind(&data)
        .bind(record.expiry_date)
        .execute(&self.pool)
        .await
        .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        Ok(())
    }

    async fn save(&self, record: &Record) -> session_store::Result<()> {
        let id = record.id.to_string();
        let data = serde_json::to_vec(&record.data)
            .map_err(|e| session_store::Error::Encode(e.to_string()))?;

        sqlx::query(
            "INSERT INTO tower_sessions (id, data, expiry_date) VALUES ($1, $2, $3)
             ON CONFLICT (id) DO UPDATE SET data = EXCLUDED.data, expiry_date = EXCLUDED.expiry_date",
        )
        .bind(&id)
        .bind(&data)
        .bind(record.expiry_date)
        .execute(&self.pool)
        .await
        .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let id = session_id.to_string();

        let row: Option<(Vec<u8>, OffsetDateTime)> = sqlx::query_as(
            "SELECT data, expiry_date FROM tower_sessions WHERE id = $1 AND expiry_date > NOW()",
        )
        .bind(&id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        match row {
            Some((data_bytes, expiry)) => {
                let data = serde_json::from_slice(&data_bytes)
                    .map_err(|e| session_store::Error::Decode(e.to_string()))?;
                Ok(Some(Record {
                    id: *session_id,
                    data,
                    expiry_date: expiry,
                }))
            }
            None => Ok(None),
        }
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        let id = session_id.to_string();

        sqlx::query("DELETE FROM tower_sessions WHERE id = $1")
            .bind(&id)
            .execute(&self.pool)
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        Ok(())
    }
}
