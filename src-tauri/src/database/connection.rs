//! Database connection pool management using Tauri State for dependency injection.

use anyhow::Result;
use sqlx::PgPool;
use std::sync::Arc;
use crate::stronghold::StrongholdManager;
use crate::config::AppConfig;

/// Database connection pool wrapper for Tauri State management.
///
/// This struct provides thread-safe access to the database connection pool
/// through Tauri's dependency injection system.
#[derive(Clone)]
pub struct Database {
    pool: Arc<PgPool>,
}

impl Database {
    /// Creates a new Database instance from a PgPool.
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }

    /// Returns a reference to the underlying connection pool.
    pub fn pool(&self) -> &PgPool {
        self.pool.as_ref()
    }

    /// Returns an Arc clone of the connection pool.
    pub fn pool_arc(&self) -> Arc<PgPool> {
        Arc::clone(&self.pool)
    }
}

/// Initializes the database connection using Stronghold for secure credential storage.
/// Currently uses direct config access as a fallback.
pub async fn initialize_database(_stronghold: &mut StrongholdManager) -> Result<Database> {
    let config = AppConfig::from_env();
    let db_url = config.database_url.clone();

    let pool = super::create_pool_with_url(&db_url).await?;
    super::test_connection(&pool).await?;

    Ok(Database::new(pool))
}
