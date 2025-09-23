use anyhow::Result;
use sqlx::{PgPool, Pool, Postgres};
use std::env;

pub mod connection;
pub mod migrations;

pub use connection::*;

pub type DbPool = Pool<Postgres>;

pub async fn create_pool() -> Result<PgPool> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://tauri_user:tauri_password@localhost:5432/tauri_app".to_string()
    });

    let pool = PgPool::connect(&database_url).await?;

    Ok(pool)
}

pub async fn test_connection(pool: &PgPool) -> Result<bool> {
    let row: (i64,) = sqlx::query_as("SELECT 1").fetch_one(pool).await?;

    Ok(row.0 == 1)
}
