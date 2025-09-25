//! Database connection pool management with thread-safe access.

use anyhow::Result;
use once_cell::sync::OnceCell;
use sqlx::PgPool;
use std::sync::{Arc, RwLock};
use crate::stronghold::StrongholdManager;
use crate::config::AppConfig;

/// Global connection pool storage using OnceCell for thread-safe initialization.
static POOL: OnceCell<RwLock<Option<Arc<PgPool>>>> = OnceCell::new();

/// Returns the global pool slot, initializing it if necessary.
fn pool_slot() -> &'static RwLock<Option<Arc<PgPool>>> {
    POOL.get_or_init(|| RwLock::new(None))
}

/// Initializes the database connection using Stronghold for secure credential storage.
/// Currently uses direct config access as a fallback.
pub async fn initialize_database(_stronghold: &mut StrongholdManager) -> Result<()> {
    let config = AppConfig::from_env();
    let db_url = config.database_url.clone();

    let pool = super::create_pool_with_url(&db_url).await?;
    super::test_connection(&pool).await?;

    let arc = Arc::new(pool);
    let mut guard = pool_slot()
        .write()
        .map_err(|_| anyhow::anyhow!("Failed to lock database pool for initialization"))?;
    *guard = Some(arc);

    Ok(())
}

/// Initializes the global connection pool with a pre-created pool.
pub async fn initialize_pool(pool: PgPool) {
    let arc = Arc::new(pool);
    if let Ok(mut guard) = pool_slot().write() {
        *guard = Some(arc);
    }
}

/// Returns the current database connection pool if initialized.
pub fn get_pool() -> Option<Arc<PgPool>> {
    pool_slot()
        .read()
        .ok()
        .and_then(|guard| guard.as_ref().cloned())
}

/// Returns the database connection pool or an error if not initialized.
pub fn get_pool_ref() -> Result<Arc<PgPool>> {
    get_pool().ok_or_else(|| anyhow::anyhow!("Database pool not initialized"))
}

/// Resets the connection pool for testing purposes.
#[cfg(test)]
pub fn reset_pool_for_tests() {
    if let Ok(mut guard) = pool_slot().write() {
        *guard = None;
    }
}
