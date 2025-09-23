use anyhow::Result;
use once_cell::sync::OnceCell;
use sqlx::PgPool;
use std::sync::Arc;

static POOL: OnceCell<Arc<PgPool>> = OnceCell::new();

pub async fn initialize_database() -> Result<()> {
    dotenv::dotenv().ok();

    let pool = super::create_pool().await?;

    // Test connection
    super::test_connection(&pool).await?;

    POOL.set(Arc::new(pool))
        .map_err(|_| anyhow::anyhow!("Failed to initialize database pool"))?;

    Ok(())
}

pub fn get_pool() -> Option<Arc<PgPool>> {
    POOL.get().cloned()
}

pub fn get_pool_ref() -> Result<&'static PgPool> {
    POOL.get()
        .map(|arc| arc.as_ref())
        .ok_or_else(|| anyhow::anyhow!("Database pool not initialized"))
}
