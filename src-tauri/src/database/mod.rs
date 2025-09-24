use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, time::Duration};

pub mod connection;
pub mod migrations;
#[cfg(test)]
pub mod test_utils;

pub use connection::*;

pub async fn create_pool() -> Result<PgPool> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://tauri_user:tauri_password@localhost:5432/tauri_app".to_string()
    });

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(60))
        .connect(&database_url)
        .await?;

    Ok(pool)
}

pub async fn test_connection(pool: &PgPool) -> Result<bool> {
    let row: (i32,) = sqlx::query_as("SELECT 1").fetch_one(pool).await?;

    Ok(row.0 == 1)
}
