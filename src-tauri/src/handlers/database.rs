use crate::database::{get_pool_ref, test_connection};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseStatus {
    pub connected: bool,
    pub database_name: Option<String>,
    pub version: Option<String>,
    pub error: Option<String>,
}

#[tauri::command]
pub async fn check_database_connection() -> Result<DatabaseStatus, String> {
    match get_pool_ref() {
        Ok(pool) => {
            match test_connection(pool.as_ref()).await {
                Ok(_) => {
                    // Get database name and version
                    let db_info: Result<(String, String), sqlx::Error> =
                        sqlx::query_as("SELECT current_database(), version()")
                            .fetch_one(pool.as_ref())
                            .await;

                    match db_info {
                        Ok((db_name, version)) => Ok(DatabaseStatus {
                            connected: true,
                            database_name: Some(db_name),
                            version: Some(version),
                            error: None,
                        }),
                        Err(e) => Ok(DatabaseStatus {
                            connected: true,
                            database_name: None,
                            version: None,
                            error: Some(format!("Failed to get database info: {}", e)),
                        }),
                    }
                }
                Err(e) => Ok(DatabaseStatus {
                    connected: false,
                    database_name: None,
                    version: None,
                    error: Some(e.to_string()),
                }),
            }
        }
        Err(e) => Ok(DatabaseStatus {
            connected: false,
            database_name: None,
            version: None,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn initialize_database() -> Result<String, String> {
    match crate::database::initialize_database().await {
        Ok(_) => Ok("Database initialized successfully".to_string()),
        Err(e) => Err(format!("Failed to initialize database: {}", e)),
    }
}

#[tauri::command]
pub async fn run_migrations() -> Result<String, String> {
    match get_pool_ref() {
        Ok(pool) => match crate::database::migrations::run_migrations(pool.as_ref()).await {
            Ok(_) => Ok("Migrations completed successfully".to_string()),
            Err(e) => Err(format!("Migration failed: {}", e)),
        },
        Err(e) => Err(format!("Database not available: {}", e)),
    }
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
