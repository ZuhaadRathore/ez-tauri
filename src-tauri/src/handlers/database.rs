//! Database connection and health check handlers.

use crate::database::{Database, test_connection};
use crate::errors::{AppError, AppResult, ErrorCode};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::State;

/// Database connection status information.
#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
#[specta(rename_all = "camelCase")]
pub struct DatabaseStatus {
    pub connected: bool,
    pub database_name: Option<String>,
    pub version: Option<String>,
    pub error: Option<String>,
}

/// Checks database connectivity and returns connection status information.
#[tauri::command]
pub async fn check_database_connection(db: State<'_, Database>) -> Result<DatabaseStatus, AppError> {
    tracing::info!("Checking database connection");

    let pool = db.pool();

    match test_connection(pool).await {
        Ok(_) => {
            let db_info_result = sqlx::query_as::<_, (String, String)>(
                "SELECT current_database(), version()"
            )
            .fetch_one(pool)
            .await;

            match db_info_result {
                Ok((db_name, version)) => {
                    tracing::info!("Database connection successful: {} ({})", db_name, version);
                    Ok(DatabaseStatus {
                        connected: true,
                        database_name: Some(db_name),
                        version: Some(version),
                        error: None,
                    })
                }
                Err(e) => {
                    tracing::warn!("Connected to database but failed to get info: {}", e);
                    Ok(DatabaseStatus {
                        connected: true,
                        database_name: None,
                        version: None,
                        error: Some(format!("Failed to get database info: {}", e)),
                    })
                }
            }
        }
        Err(e) => {
            tracing::error!("Database connection test failed: {}", e);
            Ok(DatabaseStatus {
                connected: false,
                database_name: None,
                version: None,
                error: Some(e.to_string()),
            })
        }
    }
}

#[tauri::command]
pub async fn initialize_database() -> AppResult<String> {
    tracing::info!("Initializing database");

    // Note: This would need access to stronghold, but we can't easily pass it through
    // For now, we'll return an error indicating it needs to be called differently
    Err(AppError::new(
        ErrorCode::InvalidInput,
        "Database initialization requires stronghold access".to_string()
    ))
}

#[tauri::command]
pub async fn run_migrations(db: State<'_, Database>) -> AppResult<String> {
    tracing::info!("Running database migrations");

    let pool = db.pool();

    crate::database::migrations::run_migrations(pool)
        .await
        .map_err(|e| AppError::new(ErrorCode::DatabaseMigration, e.to_string()))
        .map(|_| {
            tracing::info!("Migrations completed successfully");
            "Migrations completed successfully".to_string()
        })
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::test_utils::{pool, reset_all_tables};
    use anyhow::Result as AnyResult;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn check_connection_reports_connected_when_pool_ready() -> AnyResult<()> {
        let pool = pool().await?;
        reset_all_tables(pool.as_ref()).await?;

        let status = check_database_connection()
            .await
            .expect("expected database status");

        assert!(status.connected);
        assert_eq!(status.database_name.as_deref(), Some("tauri_app"));
        assert!(status.version.is_some());
        assert!(status.error.is_none());
        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn run_migrations_command_is_idempotent() -> AnyResult<()> {
        let pool = pool().await?;
        reset_all_tables(pool.as_ref()).await?;

        run_migrations()
            .await
            .expect("first migration run should succeed");
        run_migrations()
            .await
            .expect("second migration run should be idempotent");
        Ok(())
    }
}
