use anyhow::Result;
use once_cell::sync::OnceCell;
use sqlx::PgPool;
use std::sync::{Arc, RwLock};

static POOL: OnceCell<RwLock<Option<Arc<PgPool>>>> = OnceCell::new();

fn pool_slot() -> &'static RwLock<Option<Arc<PgPool>>> {
    POOL.get_or_init(|| RwLock::new(None))
}

pub async fn initialize_database() -> Result<()> {
    dotenv::dotenv().ok();

    let pool = super::create_pool().await?;

    super::test_connection(&pool).await?;

    let arc = Arc::new(pool);
    let mut guard = pool_slot()
        .write()
        .map_err(|_| anyhow::anyhow!("Failed to lock database pool for initialization"))?;
    *guard = Some(arc);

    Ok(())
}

pub fn get_pool() -> Option<Arc<PgPool>> {
    pool_slot()
        .read()
        .ok()
        .and_then(|guard| guard.as_ref().cloned())
}

pub fn get_pool_ref() -> Result<Arc<PgPool>> {
    get_pool().ok_or_else(|| anyhow::anyhow!("Database pool not initialized"))
}

#[cfg(test)]
pub fn reset_pool_for_tests() {
    if let Ok(mut guard) = pool_slot().write() {
        *guard = None;
    }
}
