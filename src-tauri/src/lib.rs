mod database;
mod handlers;
mod logging;
mod models;

use handlers::*;
use logging::handlers::*;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

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
        .setup(|_app| {
            if let Err(e) = logging::init_logging_from_env() {
                eprintln!("Failed to initialize logging: {}", e);
            } else {
                tracing::info!("Logging system initialized successfully");
            }

            tauri::async_runtime::spawn(async {
                if let Err(e) = database::initialize_database().await {
                    tracing::error!("Failed to initialize database: {}", e);
                } else {
                    tracing::info!("Database initialized successfully");

                    if let Ok(pool) = database::get_pool_ref() {
                        if let Err(e) = database::migrations::run_migrations(pool.as_ref()).await {
                            tracing::error!("Failed to run migrations: {}", e);
                        } else {
                            tracing::info!("Migrations completed successfully");
                        }
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            // Database commands
            check_database_connection,
            initialize_database,
            run_migrations,
            // User commands
            get_all_users,
            get_user_by_id,
            create_user,
            update_user,
            delete_user,
            authenticate_user,
            // Log commands
            create_log,
            get_logs,
            delete_old_logs,
            // System commands
            get_system_info,
            send_notification,
            get_window_info,
            toggle_window_maximize,
            minimize_window,
            center_window,
            set_window_title,
            create_new_window,
            execute_command,
            get_app_data_dir,
            get_app_log_dir,
            // File system commands
            read_text_file,
            write_text_file,
            append_text_file,
            delete_file,
            create_directory,
            list_directory,
            file_exists,
            get_file_info,
            copy_file,
            move_file,
            // Logging commands
            get_log_config,
            update_log_config,
            get_log_entries,
            clear_old_logs,
            get_log_stats,
            create_test_log
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
