//! Tauri application library with comprehensive feature set including database management,
//! rate limiting, caching, and secure user authentication.

pub mod stronghold;
mod cache;
mod config;
mod database;
mod errors;
mod handlers;
mod logging;
mod models;
mod rate_limiter;
#[cfg(test)]
mod rate_limiter_test;
mod validation;

use config::AppConfig;
use handlers::*;
use rate_limiter::RateLimiterConfig;
use std::sync::Arc;
use tauri::Manager;

/// Basic greeting command for testing Tauri functionality.
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Initializes and runs the Tauri application with all configured plugins and handlers.
///
/// Sets up the application with:
/// - File system, dialog, notification, and shell plugins
/// - Database connection and migrations
/// - Rate limiting for all commands
/// - Comprehensive error handling and logging
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_stronghold::Builder::new(|password| {
            use argon2::{Algorithm, Argon2, Params, Version};
            let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default());
            let salt = &[0; 32];
            let mut output = [0u8; 32];
            argon2.hash_password_into(password.as_bytes(), salt, &mut output)
                .expect("failed to hash password");
            output.to_vec()
        }).build())
        .setup(|app| {
            let config = AppConfig::from_env();
            tracing::info!("App environment: {:?}", config.environment);

            let rate_limiter = Arc::new(RateLimiterConfig::new());
            app.manage(rate_limiter.clone());
            tracing::info!("Rate limiter initialized successfully");

            if let Err(e) = logging::init_logging_from_env() {
                eprintln!("Failed to initialize logging: {}", e);
            } else {
                tracing::info!("Logging system initialized successfully");
            }

            if let Err(e) = cache::initialize_redis() {
                tracing::warn!("Failed to initialize Redis: {}. Continuing without caching.", e);
            }

            tauri::async_runtime::spawn(async move {
                match database::create_pool().await {
                    Ok(pool) => {
                        database::connection::initialize_pool(pool).await;
                        tracing::info!("Database initialized successfully");

                        if let Ok(pool) = database::get_pool_ref() {
                            if let Err(e) = database::migrations::run_migrations(pool.as_ref()).await {
                                tracing::error!("Failed to run migrations: {}", e);
                            } else {
                                tracing::info!("Migrations completed successfully");
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to initialize database: {}", e);
                    }
                }
            });

            let rate_limiter_cleanup = rate_limiter.clone();
            tauri::async_runtime::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(3600));
                loop {
                    interval.tick().await;
                    rate_limiter_cleanup.cleanup_old_limiters();
                    tracing::debug!("Cleaned up old rate limiters");
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            rl_greet,
            rl_check_database_connection,
            rl_initialize_database,
            rl_run_migrations,
            rl_get_all_users,
            rl_get_user_by_id,
            rl_create_user,
            rl_update_user,
            rl_delete_user,
            rl_authenticate_user,
            rl_create_log,
            rl_get_logs,
            rl_delete_old_logs,
            rl_get_system_info,
            rl_send_notification,
            rl_get_window_info,
            rl_toggle_window_maximize,
            rl_minimize_window,
            rl_center_window,
            rl_set_window_title,
            rl_create_new_window,
            rl_execute_command,
            rl_get_app_data_dir,
            rl_get_app_log_dir,
            rl_read_text_file,
            rl_write_text_file,
            rl_append_text_file,
            rl_delete_file,
            rl_create_directory,
            rl_list_directory,
            rl_file_exists,
            rl_get_file_info,
            rl_copy_file,
            rl_move_file,
            rl_get_log_config,
            rl_update_log_config,
            rl_get_log_entries,
            rl_clear_old_logs,
            rl_get_log_stats,
            rl_create_test_log,
            rl_set_cache_value,
            rl_get_cache_value,
            rl_delete_cache_value,
            rl_cache_key_exists,
            rl_is_cache_available,
            get_rate_limiter_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
