//! Rate-limited wrappers for all Tauri command handlers.

use crate::rate_limiter::RateLimiterConfig;
use crate::handlers::*;
use crate::logging::handlers::{get_log_config, update_log_config, get_log_entries, clear_old_logs, get_log_stats, create_test_log};
use std::sync::Arc;
use tauri::State;

/// Helper macro to create rate-limited wrappers for command handlers.
macro_rules! create_rate_limited_handler {
    ($func_name:ident, $original_func:ident, $($param:ident: $param_type:ty),* $(,)?) => {
        #[tauri::command]
        pub async fn $func_name(
            rate_limiter: State<'_, Arc<RateLimiterConfig>>,
            $($param: $param_type,)*
        ) -> Result<serde_json::Value, String> {
            if let Err(e) = rate_limiter.check_rate_limit(None).await {
                tracing::warn!("Rate limit exceeded: {}", e);
                return Err(format!("Rate limit exceeded: {}", e));
            }

            let result = $original_func($($param,)*).await;
            match result {
                Ok(value) => serde_json::to_value(value).map_err(|e| format!("Serialization error: {}", e)),
                Err(e) => Err(format!("{}", e)),
            }
        }
    };
}

// Create rate-limited wrappers for database commands
create_rate_limited_handler!(
    rl_check_database_connection,
    check_database_connection,
);

create_rate_limited_handler!(
    rl_initialize_database,
    initialize_database,
);

create_rate_limited_handler!(
    rl_run_migrations,
    run_migrations,
);

// Create rate-limited wrappers for user commands
create_rate_limited_handler!(
    rl_get_all_users,
    get_all_users,
);

create_rate_limited_handler!(
    rl_get_user_by_id,
    get_user_by_id,
    user_id: String
);

create_rate_limited_handler!(
    rl_create_user,
    create_user,
    user: crate::models::CreateUser
);

create_rate_limited_handler!(
    rl_update_user,
    update_user,
    user_id: String,
    user: crate::models::UpdateUser
);

create_rate_limited_handler!(
    rl_delete_user,
    delete_user,
    user_id: String
);

create_rate_limited_handler!(
    rl_authenticate_user,
    authenticate_user,
    credentials: crate::models::LoginRequest
);

// Create rate-limited wrappers for log commands
create_rate_limited_handler!(
    rl_create_log,
    create_log,
    log_data: crate::models::CreateAppLog
);

create_rate_limited_handler!(
    rl_get_logs,
    get_logs,
    query: crate::models::logs::LogQuery
);

create_rate_limited_handler!(
    rl_delete_old_logs,
    delete_old_logs,
    days: i32
);

// Create rate-limited wrappers for system commands
create_rate_limited_handler!(
    rl_get_system_info,
    get_system_info,
);

create_rate_limited_handler!(
    rl_send_notification,
    send_notification,
    app: tauri::AppHandle,
    title: String,
    body: String
);

create_rate_limited_handler!(
    rl_get_window_info,
    get_window_info_by_app,
    app: tauri::AppHandle
);

create_rate_limited_handler!(
    rl_toggle_window_maximize,
    toggle_window_maximize_by_app,
    app: tauri::AppHandle
);

create_rate_limited_handler!(
    rl_minimize_window,
    minimize_window_by_app,
    app: tauri::AppHandle
);

create_rate_limited_handler!(
    rl_center_window,
    center_window_by_app,
    app: tauri::AppHandle
);

create_rate_limited_handler!(
    rl_set_window_title,
    set_window_title_by_app,
    app: tauri::AppHandle,
    title: String
);

create_rate_limited_handler!(
    rl_create_new_window,
    create_new_window,
    app: tauri::AppHandle,
    label: String,
    url: String
);

create_rate_limited_handler!(
    rl_execute_command,
    execute_command,
    command: String,
    args: Vec<String>
);

create_rate_limited_handler!(
    rl_get_app_data_dir,
    get_app_data_dir,
    app_handle: tauri::AppHandle
);

create_rate_limited_handler!(
    rl_get_app_log_dir,
    get_app_log_dir,
    app_handle: tauri::AppHandle
);

// Create rate-limited wrappers for filesystem commands
create_rate_limited_handler!(
    rl_read_text_file,
    read_text_file,
    path: String
);

create_rate_limited_handler!(
    rl_write_text_file,
    write_text_file,
    path: String,
    content: String
);

create_rate_limited_handler!(
    rl_append_text_file,
    append_text_file,
    path: String,
    content: String
);

create_rate_limited_handler!(
    rl_delete_file,
    delete_file,
    path: String
);

create_rate_limited_handler!(
    rl_create_directory,
    create_directory,
    path: String
);

create_rate_limited_handler!(
    rl_list_directory,
    list_directory,
    path: String
);

create_rate_limited_handler!(
    rl_file_exists,
    file_exists,
    path: String
);

create_rate_limited_handler!(
    rl_get_file_info,
    get_file_info,
    path: String
);

create_rate_limited_handler!(
    rl_copy_file,
    copy_file,
    src: String,
    dst: String
);

create_rate_limited_handler!(
    rl_move_file,
    move_file,
    src: String,
    dst: String
);

// Create rate-limited wrappers for logging commands
// Logging commands with correct parameter types
#[tauri::command]
pub async fn rl_get_log_config(
    rate_limiter: State<'_, Arc<RateLimiterConfig>>,
) -> Result<crate::logging::config::AppLogConfig, String> {
    if let Err(e) = rate_limiter.check_rate_limit(None).await {
        tracing::warn!("Rate limit exceeded: {}", e);
        return Err(format!("Rate limit exceeded: {}", e));
    }

    get_log_config().await
}

#[tauri::command]
pub async fn rl_update_log_config(
    rate_limiter: State<'_, Arc<RateLimiterConfig>>,
    config: crate::logging::config::AppLogConfig,
) -> Result<String, String> {
    if let Err(e) = rate_limiter.check_rate_limit(None).await {
        tracing::warn!("Rate limit exceeded: {}", e);
        return Err(format!("Rate limit exceeded: {}", e));
    }

    update_log_config(config).await
}

#[tauri::command]
pub async fn rl_get_log_entries(
    rate_limiter: State<'_, Arc<RateLimiterConfig>>,
    params: crate::logging::handlers::LogQueryParams,
) -> Result<crate::logging::handlers::LogResponse, String> {
    if let Err(e) = rate_limiter.check_rate_limit(None).await {
        tracing::warn!("Rate limit exceeded: {}", e);
        return Err(format!("Rate limit exceeded: {}", e));
    }

    get_log_entries(params).await
}

#[tauri::command]
pub async fn rl_clear_old_logs(
    rate_limiter: State<'_, Arc<RateLimiterConfig>>,
    days_to_keep: u32,
) -> Result<String, String> {
    if let Err(e) = rate_limiter.check_rate_limit(None).await {
        tracing::warn!("Rate limit exceeded: {}", e);
        return Err(format!("Rate limit exceeded: {}", e));
    }

    clear_old_logs(days_to_keep).await
}

#[tauri::command]
pub async fn rl_get_log_stats(
    rate_limiter: State<'_, Arc<RateLimiterConfig>>,
) -> Result<std::collections::HashMap<String, serde_json::Value>, String> {
    if let Err(e) = rate_limiter.check_rate_limit(None).await {
        tracing::warn!("Rate limit exceeded: {}", e);
        return Err(format!("Rate limit exceeded: {}", e));
    }

    get_log_stats().await
}

#[tauri::command]
pub async fn rl_create_test_log(
    rate_limiter: State<'_, Arc<RateLimiterConfig>>,
    level: String,
    message: String,
) -> Result<String, String> {
    if let Err(e) = rate_limiter.check_rate_limit(None).await {
        tracing::warn!("Rate limit exceeded: {}", e);
        return Err(format!("Rate limit exceeded: {}", e));
    }

    create_test_log(level, message).await
}

// Create rate-limited wrappers for cache commands
create_rate_limited_handler!(
    rl_set_cache_value,
    set_cache_value,
    key: String,
    value: serde_json::Value,
    ttl_seconds: Option<u64>
);

create_rate_limited_handler!(
    rl_get_cache_value,
    get_cache_value,
    key: String
);

create_rate_limited_handler!(
    rl_delete_cache_value,
    delete_cache_value,
    key: String
);

create_rate_limited_handler!(
    rl_cache_key_exists,
    cache_key_exists,
    key: String
);

create_rate_limited_handler!(
    rl_is_cache_available,
    is_cache_available,
);

// Special handler for greet function
#[tauri::command]
pub async fn rl_greet(
    rate_limiter: State<'_, Arc<RateLimiterConfig>>,
    name: String,
) -> Result<String, String> {
    if let Err(e) = rate_limiter.check_rate_limit(None).await {
        tracing::warn!("Rate limit exceeded for greet: {}", e);
        return Err(format!("Rate limit exceeded: {}", e));
    }

    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

// Rate limiter status command for monitoring
#[tauri::command]
pub async fn get_rate_limiter_status(
    _rate_limiter: State<'_, Arc<RateLimiterConfig>>,
) -> Result<String, String> {
    // This command itself doesn't need rate limiting as it's for monitoring
    Ok("Rate limiter is active and protecting all commands".to_string())
}