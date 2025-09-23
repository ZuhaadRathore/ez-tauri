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
    .fetch_one(pool)
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
        .fetch_all(pool)
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
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to delete old logs: {}", e))?;

    Ok(format!(
        "Deleted {} old log entries",
        result.rows_affected()
    ))
}
