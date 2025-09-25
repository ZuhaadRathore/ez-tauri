//! Database connection management and utilities.
//!
//! Provides PostgreSQL connection pooling, migrations, and database utilities
//! with environment-aware configuration.

use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use crate::config::AppConfig;

pub mod connection;
pub mod migrations;
#[cfg(test)]
pub mod test_utils;

pub use connection::*;

/// Creates a database connection pool using configuration from environment.
pub async fn create_pool() -> Result<PgPool> {
    let config = AppConfig::from_env();
    create_pool_with_url(&config.database_url).await
}

/// Creates a database connection pool with a specific database URL.
///
/// # Arguments
/// * `database_url` - PostgreSQL connection string
///
/// # Returns
/// * `Result<PgPool>` - Connection pool or error
pub async fn create_pool_with_url(database_url: &str) -> Result<PgPool> {
    let config = AppConfig::from_env();

    let pool = PgPoolOptions::new()
        .max_connections(if config.is_production() { 50 } else { 20 })
        .acquire_timeout(Duration::from_secs(60))
        .connect(database_url)
        .await?;

    Ok(pool)
}

pub async fn test_connection(pool: &PgPool) -> Result<bool> {
    let row: (i32,) = sqlx::query_as("SELECT 1").fetch_one(pool).await?;

    Ok(row.0 == 1)
}
