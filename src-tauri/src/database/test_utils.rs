//! Test utilities for database operations using Docker containers.

#![cfg(test)]

use std::{sync::Arc, time::Duration};

use anyhow::{anyhow, Result};
use sqlx::{postgres::PgConnection, Connection, PgPool};
use testcontainers::core::IntoContainerPort;
use testcontainers::runners::AsyncRunner;
use testcontainers::ContainerAsync;
use testcontainers_modules::postgres::Postgres;
use tokio::sync::OnceCell;
use tokio::time::sleep;

use super::{connection, migrations};

/// Container context for managing test database lifecycle.
struct ContainerContext {
    #[allow(dead_code)]
    container: ContainerAsync<Postgres>,
    connection_string: String,
}

/// Global context for sharing test database container across tests.
static CONTEXT: OnceCell<ContainerContext> = OnceCell::const_new();

/// Gets or initializes the shared test database context.
async fn context() -> Result<&'static ContainerContext> {
    CONTEXT
        .get_or_try_init(|| async {
            let image = Postgres::default()
                .with_user("tauri_user")
                .with_password("tauri_password")
                .with_db_name("tauri_app");

            let container = image.start().await.map_err(|e| anyhow!(e))?;
            let host = container
                .get_host()
                .await
                .map_err(|e| anyhow!(e))?
                .to_string();
            let port = container
                .get_host_port_ipv4(5432_u16.tcp())
                .await
                .map_err(|e| anyhow!(e))?;

            let connection_string =
                format!("postgres://tauri_user:tauri_password@{host}:{port}/tauri_app");

            wait_for_database(&connection_string).await?;

            Ok(ContainerContext {
                container,
                connection_string,
            })
        })
        .await
}

/// Creates a test database pool with migrations applied.
pub async fn pool() -> Result<Arc<PgPool>> {
    let ctx = context().await?;

    connection::reset_pool_for_tests();

    std::env::set_var("DATABASE_URL", &ctx.connection_string);
    connection::initialize_database()
        .await
        .map_err(|e| anyhow!(e))?;

    let pool = connection::get_pool_ref()?;

    migrations::run_migrations(pool.as_ref())
        .await
        .map_err(|e| anyhow!(e))?;

    Ok(pool)
}

/// Resets all tables in the test database for clean test isolation.
pub async fn reset_all_tables(pool: &PgPool) -> Result<()> {
    sqlx::query("TRUNCATE TABLE app_logs RESTART IDENTITY CASCADE")
        .execute(pool)
        .await?;
    sqlx::query("TRUNCATE TABLE user_settings RESTART IDENTITY CASCADE")
        .execute(pool)
        .await?;
    sqlx::query("TRUNCATE TABLE users RESTART IDENTITY CASCADE")
        .execute(pool)
        .await?;

    Ok(())
}

/// Waits for the database container to be ready for connections.
async fn wait_for_database(connection_string: &str) -> Result<()> {
    let mut attempts = 0;
    loop {
        match PgConnection::connect(connection_string).await {
            Ok(conn) => {
                conn.close().await?;
                break;
            }
            Err(error) => {
                if attempts >= 20 {
                    return Err(anyhow!("database never became ready: {error}"));
                }
                attempts += 1;
                sleep(Duration::from_millis(300)).await;
            }
        }
    }
    Ok(())
}
