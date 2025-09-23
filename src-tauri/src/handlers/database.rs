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
            match test_connection(pool).await {
                Ok(_) => {
                    // Get database name and version
                    let db_info: Result<(String, String), sqlx::Error> =
                        sqlx::query_as("SELECT current_database(), version()")
                            .fetch_one(pool)
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
        Ok(pool) => match crate::database::migrations::run_migrations(pool).await {
            Ok(_) => Ok("Migrations completed successfully".to_string()),
            Err(e) => Err(format!("Migration failed: {}", e)),
        },
        Err(e) => Err(format!("Database not available: {}", e)),
    }
}
