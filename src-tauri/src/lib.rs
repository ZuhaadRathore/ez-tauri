//! Tauri application library with comprehensive feature set including database management,
//! rate limiting, caching, and secure user authentication.

pub mod stronghold;
mod cache;
mod config;
mod database;
mod errors;
pub mod handlers;
mod logging;
mod models;
mod rate_limiter;
#[cfg(test)]
mod rate_limiter_test;
mod validation;

pub mod modules;
use config::AppConfig;
use handlers::*;
use modules::auth::jwt::JwtService;
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
            use sha2::{Sha256, Digest};

            let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default());

            // Derive a unique salt from the password using SHA-256
            // This ensures different passwords produce different salts
            let mut hasher = Sha256::new();
            hasher.update(password.as_bytes());
            hasher.update(b"ez-tauri-stronghold-salt-v1"); // Application-specific domain separator
            let salt = hasher.finalize();

            let mut output = [0u8; 32];
            argon2.hash_password_into(password.as_bytes(), &salt, &mut output)
                .expect("failed to hash password");
            output.to_vec()
        }).build())
        .setup(|app| {
            let config = AppConfig::from_env();
            tracing::info!("App environment: {:?}", config.environment);

            // Validate production configuration
            if let Err(e) = AppConfig::validate_production_config() {
                panic!("Configuration validation failed: {}", e);
            }

            // Initialize JWT service once from environment variables
            let jwt_secret = std::env::var("JWT_SECRET")
                .expect("JWT_SECRET environment variable must be set. Generate a secure secret with: openssl rand -base64 32");

            // Validate secret length for security (already validated in config, but double-check)
            if jwt_secret.len() < 32 {
                panic!("JWT_SECRET must be at least 32 characters long for security. Current length: {}", jwt_secret.len());
            }

            let access_hours = std::env::var("JWT_ACCESS_TOKEN_HOURS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1); // Default to 1 hour for better security

            let refresh_days = std::env::var("JWT_REFRESH_TOKEN_DAYS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(30);

            let jwt_service = JwtService::with_expiry(jwt_secret, access_hours, refresh_days);
            app.manage(jwt_service);
            tracing::info!("JWT service initialized successfully with access_hours={}, refresh_days={}", access_hours, refresh_days);

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

            // Initialize database synchronously to prevent race conditions with command handlers
            // This ensures the database pool is ready before any commands can be invoked
            match database::create_pool().await {
                Ok(pool) => {
                    database::connection::initialize_pool(pool).await;
                    tracing::info!("Database initialized successfully");

                    if let Ok(pool_ref) = database::get_pool_ref() {
                        if let Err(e) = database::migrations::run_migrations(pool_ref.as_ref()).await {
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

            let rate_limiter_cleanup = rate_limiter.clone();
            tauri::async_runtime::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(3600));
                loop {
                    interval.tick().await;
                    rate_limiter_cleanup.cleanup_old_limiters();
                    tracing::debug!("Cleaned up old rate limiters");
                }
            });

            // Spawn background task for periodic session cleanup
            tauri::async_runtime::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(86400)); // Run daily
                loop {
                    interval.tick().await;
                    if let Ok(pool) = database::get_pool_ref() {
                        match modules::auth::session::SessionManager::cleanup_expired_sessions(pool.as_ref()).await {
                            Ok(count) => {
                                tracing::info!("Session cleanup: removed {} expired sessions", count);
                            }
                            Err(e) => {
                                tracing::error!("Failed to cleanup expired sessions: {}", e);
                            }
                        }
                    } else {
                        tracing::warn!("Database not initialized, skipping session cleanup");
                    }
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
            get_rate_limiter_status,
            // Authentication commands
            modules::auth::auth_register,
            modules::auth::auth_login,
            modules::auth::auth_logout,
            modules::auth::auth_logout_all,
            modules::auth::auth_refresh_token,
            modules::auth::auth_verify_token,
            modules::auth::auth_get_sessions,
            modules::auth::auth_revoke_session,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
