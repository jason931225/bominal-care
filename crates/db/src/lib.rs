pub mod queries;

use sqlx::PgPool;

/// Creates a database connection pool.
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(database_url).await
}

/// Runs all pending migrations.
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}

/// Sets RLS context variables for a transaction.
/// Must be called within a transaction before executing queries.
pub async fn set_rls_context(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: &str,
    role: &str,
    tenant_id: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT set_config('app.current_user_id', $1, true)")
        .bind(user_id)
        .execute(&mut **tx)
        .await?;
    sqlx::query("SELECT set_config('app.current_role', $1, true)")
        .bind(role)
        .execute(&mut **tx)
        .await?;
    if let Some(tid) = tenant_id {
        sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
            .bind(tid)
            .execute(&mut **tx)
            .await?;
    }
    Ok(())
}
