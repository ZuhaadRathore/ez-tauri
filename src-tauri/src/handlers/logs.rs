use crate::database::get_pool_ref;
use crate::models::{AppLog, CreateAppLog, LogQuery};
use sqlx::QueryBuilder;

#[tauri::command]
pub async fn create_log(log_data: CreateAppLog) -> Result<AppLog, String> {
    let pool = get_pool_ref().map_err(|e| e.to_string())?;

    let metadata = log_data.metadata.unwrap_or_else(|| serde_json::json!({}));

    let log = sqlx::query_as::<_, AppLog>(
        r#"
        INSERT INTO app_logs (level, message, metadata, user_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id,
                  level,
                  message,
                  metadata,
                  user_id,
                  created_at
        "#,
    )
    .bind(log_data.level)
    .bind(log_data.message)
    .bind(metadata)
    .bind(log_data.user_id)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| format!("Failed to create log: {}", e))?;

    Ok(log)
}

#[tauri::command]
pub async fn get_logs(query: LogQuery) -> Result<Vec<AppLog>, String> {
    let pool = get_pool_ref().map_err(|e| e.to_string())?;

    let LogQuery {
        level,
        user_id,
        limit,
        offset,
    } = query;

    let limit = limit.unwrap_or(100).clamp(1, 1_000);
    let offset = offset.unwrap_or(0).max(0);

    let mut builder = QueryBuilder::new(
        "SELECT id,
                level,
                message,
                metadata,
                user_id,
                created_at
         FROM app_logs",
    );

    let mut has_condition = false;

    if let Some(level) = level {
        builder.push(" WHERE level = ");
        builder.push_bind(level);
        has_condition = true;
    }

    if let Some(user_id) = user_id {
        builder.push(if has_condition {
            " AND user_id = "
        } else {
            " WHERE user_id = "
        });
        builder.push_bind(user_id);
    }

    builder.push(" ORDER BY created_at DESC LIMIT ");
    builder.push_bind(limit);
    builder.push(" OFFSET ");
    builder.push_bind(offset);

    let logs = builder
        .build_query_as::<AppLog>()
        .fetch_all(pool.as_ref())
        .await
        .map_err(|e| format!("Failed to fetch logs: {}", e))?;

    Ok(logs)
}

#[tauri::command]
pub async fn delete_old_logs(days_old: i32) -> Result<String, String> {
    let pool = get_pool_ref().map_err(|e| e.to_string())?;

    let result = sqlx::query(
        r#"
        DELETE FROM app_logs
        WHERE created_at < NOW() - ($1::INT * INTERVAL '1 day')
        "#,
    )
    .bind(days_old)
    .execute(pool.as_ref())
    .await
    .map_err(|e| format!("Failed to delete old logs: {}", e))?;

    Ok(format!(
        "Deleted {} old log entries",
        result.rows_affected()
    ))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::test_utils::{pool, reset_all_tables};
    use crate::handlers::users::create_user;
    use crate::models::{CreateAppLog, CreateUser, LogQuery};
    use anyhow::Result as AnyResult;
    use serde_json::json;
    use serial_test::serial;
    use uuid::Uuid;

    fn sample_user() -> CreateUser {
        let suffix = Uuid::new_v4();
        CreateUser {
            email: format!("logger+{}@example.com", suffix),
            username: format!("logger_{}", suffix.simple()),
            password: "Sup3r$ecret".to_string(),
            first_name: Some("Log".to_string()),
            last_name: Some("Tester".to_string()),
        }
    }

    #[tokio::test]
    #[serial]
    async fn create_and_query_logs_flow() -> AnyResult<()> {
        let pool = pool().await?;
        reset_all_tables(pool.as_ref()).await?;

        let user = create_user(sample_user())
            .await
            .expect("user creation must succeed for log tests");

        let created_log = create_log(CreateAppLog {
            level: "info".to_string(),
            message: "Test log entry".to_string(),
            metadata: Some(json!({"component": "log_test"})),
            user_id: Some(user.id),
        })
        .await
        .expect("log creation should succeed");

        assert_eq!(created_log.level, "info");
        assert_eq!(created_log.message, "Test log entry");
        assert_eq!(created_log.user_id, Some(user.id));

        let logs = get_logs(LogQuery {
            level: Some("info".to_string()),
            user_id: Some(user.id),
            limit: Some(10),
            offset: Some(0),
        })
        .await
        .expect("fetching logs should succeed");

        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].id, created_log.id);
        assert_eq!(logs[0].metadata["component"], json!("log_test"));

        let deletion_message = delete_old_logs(0)
            .await
            .expect("deleting old logs should succeed");
        assert!(deletion_message.starts_with("Deleted 1"));

        let remaining_logs = get_logs(LogQuery {
            level: None,
            user_id: None,
            limit: Some(10_000),
            offset: Some(-5),
        })
        .await
        .expect("fetch after deletion should succeed");
        assert!(remaining_logs.is_empty());

        Ok(())
    }
}
