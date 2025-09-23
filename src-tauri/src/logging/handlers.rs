use crate::logging::{config::AppLogConfig, LogEntry, LogLevel};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tracing::{debug, error, info};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogQueryParams {
    pub level: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub target: Option<String>,
    pub message_contains: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogResponse {
    pub logs: Vec<LogEntry>,
    pub total_count: usize,
    pub has_more: bool,
}

/// Get the current logging configuration
#[tauri::command]
pub async fn get_log_config() -> Result<AppLogConfig, String> {
    debug!("Getting current log configuration");

    // Try to load from config file first, fallback to environment
    let config_path = get_log_config_path();
    let config = if config_path.exists() {
        crate::logging::config::load_config_from_file(&config_path)
            .unwrap_or_else(|_| crate::logging::config::load_config_from_env())
    } else {
        crate::logging::config::load_config_from_env()
    };

    Ok(config)
}

/// Update the logging configuration
#[tauri::command]
pub async fn update_log_config(config: AppLogConfig) -> Result<String, String> {
    info!("Updating log configuration: {:?}", config);

    // Save the configuration to file
    let config_path = get_log_config_path();
    if let Some(parent) = config_path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            error!("Failed to create config directory: {}", e);
            return Err(format!("Failed to create config directory: {}", e));
        }
    }

    if let Err(e) = crate::logging::config::save_config_to_file(&config, &config_path) {
        error!("Failed to save log configuration: {}", e);
        return Err(format!("Failed to save configuration: {}", e));
    }

    // Note: In a real application, you might want to reinitialize the logging system here
    // For now, we'll just save the config and return success
    info!("Log configuration updated successfully");
    Ok(
        "Configuration updated successfully. Restart the application for changes to take effect."
            .to_string(),
    )
}

/// Get log entries based on query parameters
#[tauri::command]
pub async fn get_log_entries(params: LogQueryParams) -> Result<LogResponse, String> {
    debug!("Getting log entries with params: {:?}", params);

    let log_dir = get_log_directory();
    if !log_dir.exists() {
        return Ok(LogResponse {
            logs: vec![],
            total_count: 0,
            has_more: false,
        });
    }

    // Get all log files
    let mut log_files = get_log_files(&log_dir)?;
    log_files.sort_by(|a, b| {
        // Sort by modification time, newest first
        let a_metadata = a.metadata().ok();
        let b_metadata = b.metadata().ok();

        match (a_metadata, b_metadata) {
            (Some(a_meta), Some(b_meta)) => b_meta
                .modified()
                .unwrap_or(std::time::UNIX_EPOCH)
                .cmp(&a_meta.modified().unwrap_or(std::time::UNIX_EPOCH)),
            _ => std::cmp::Ordering::Equal,
        }
    });

    let mut all_logs = Vec::new();

    // Read logs from files (limit to recent files for performance)
    for log_file in log_files.iter().take(5) {
        if let Ok(content) = fs::read_to_string(log_file) {
            let file_logs = parse_log_content(&content, &params);
            all_logs.extend(file_logs);
        }
    }

    // Apply filters and sorting
    all_logs = filter_logs(all_logs, &params);
    all_logs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp)); // Newest first

    // Apply pagination
    let total_count = all_logs.len();
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(100).min(1000); // Max 1000 entries per request

    let end_index = (offset + limit).min(total_count);
    let paginated_logs = if offset < total_count {
        all_logs[offset..end_index].to_vec()
    } else {
        vec![]
    };

    let has_more = end_index < total_count;

    Ok(LogResponse {
        logs: paginated_logs,
        total_count,
        has_more,
    })
}

/// Clear old log files
#[tauri::command]
pub async fn clear_old_logs(days_to_keep: u32) -> Result<String, String> {
    info!("Clearing logs older than {} days", days_to_keep);

    let log_dir = get_log_directory();
    if !log_dir.exists() {
        return Ok("No log directory found".to_string());
    }

    let cutoff_time = Utc::now() - chrono::Duration::days(days_to_keep as i64);
    let mut removed_count = 0;

    match fs::read_dir(&log_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        if let Ok(modified) = metadata.modified() {
                            let modified_datetime: DateTime<Utc> = modified.into();
                            if modified_datetime < cutoff_time {
                                if let Err(e) = fs::remove_file(entry.path()) {
                                    error!("Failed to remove log file {:?}: {}", entry.path(), e);
                                } else {
                                    removed_count += 1;
                                    info!("Removed old log file: {:?}", entry.path());
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            error!("Failed to read log directory: {}", e);
            return Err(format!("Failed to read log directory: {}", e));
        }
    }

    let message = format!("Removed {} old log files", removed_count);
    info!("{}", message);
    Ok(message)
}

/// Get log statistics
#[tauri::command]
pub async fn get_log_stats() -> Result<HashMap<String, serde_json::Value>, String> {
    debug!("Getting log statistics");

    let log_dir = get_log_directory();
    let mut stats = HashMap::new();

    if !log_dir.exists() {
        stats.insert("total_files".to_string(), serde_json::Value::from(0));
        stats.insert("total_size_bytes".to_string(), serde_json::Value::from(0));
        stats.insert("oldest_log".to_string(), serde_json::Value::Null);
        stats.insert("newest_log".to_string(), serde_json::Value::Null);
        return Ok(stats);
    }

    let log_files = get_log_files(&log_dir)?;
    let mut total_size = 0u64;
    let mut oldest_time: Option<DateTime<Utc>> = None;
    let mut newest_time: Option<DateTime<Utc>> = None;

    for file in &log_files {
        if let Ok(metadata) = file.metadata() {
            total_size += metadata.len();

            if let Ok(modified) = metadata.modified() {
                let modified_datetime: DateTime<Utc> = modified.into();

                if oldest_time.is_none() || Some(modified_datetime) < oldest_time {
                    oldest_time = Some(modified_datetime);
                }

                if newest_time.is_none() || Some(modified_datetime) > newest_time {
                    newest_time = Some(modified_datetime);
                }
            }
        }
    }

    stats.insert(
        "total_files".to_string(),
        serde_json::Value::from(log_files.len()),
    );
    stats.insert(
        "total_size_bytes".to_string(),
        serde_json::Value::from(total_size),
    );
    stats.insert(
        "total_size_mb".to_string(),
        serde_json::Value::from((total_size as f64) / 1_048_576.0),
    );
    stats.insert(
        "log_directory".to_string(),
        serde_json::Value::from(log_dir.to_string_lossy().to_string()),
    );

    if let Some(oldest) = oldest_time {
        stats.insert(
            "oldest_log".to_string(),
            serde_json::Value::from(oldest.to_rfc3339()),
        );
    } else {
        stats.insert("oldest_log".to_string(), serde_json::Value::Null);
    }

    if let Some(newest) = newest_time {
        stats.insert(
            "newest_log".to_string(),
            serde_json::Value::from(newest.to_rfc3339()),
        );
    } else {
        stats.insert("newest_log".to_string(), serde_json::Value::Null);
    }

    Ok(stats)
}

/// Create a test log entry
#[tauri::command]
pub async fn create_test_log(level: String, message: String) -> Result<String, String> {
    let log_level: LogLevel = level.as_str().into();

    match log_level {
        LogLevel::Error => error!(test_log = true, "{}", message),
        LogLevel::Warn => tracing::warn!(test_log = true, "{}", message),
        LogLevel::Info => info!(test_log = true, "{}", message),
        LogLevel::Debug => debug!(test_log = true, "{}", message),
        LogLevel::Trace => tracing::trace!(test_log = true, "{}", message),
    }

    Ok(format!("Test log created: {} - {}", level, message))
}

// Helper functions

fn get_log_directory() -> PathBuf {
    crate::logging::default_log_dir()
}

fn get_log_config_path() -> PathBuf {
    crate::logging::default_log_config_path()
}

fn get_log_files(log_dir: &PathBuf) -> Result<Vec<PathBuf>, String> {
    let mut log_files = Vec::new();

    match fs::read_dir(log_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("log") {
                        log_files.push(path);
                    }
                }
            }
        }
        Err(e) => return Err(format!("Failed to read log directory: {}", e)),
    }

    Ok(log_files)
}

fn parse_log_content(content: &str, _params: &LogQueryParams) -> Vec<LogEntry> {
    let mut logs = Vec::new();

    for line in content.lines() {
        // Try to parse as JSON first (structured logs)
        if let Ok(entry) = serde_json::from_str::<LogEntry>(line) {
            logs.push(entry);
        } else {
            // Try to parse plain text logs (fallback)
            if let Some(entry) = parse_plain_text_log(line) {
                logs.push(entry);
            }
        }
    }

    logs
}

fn parse_plain_text_log(line: &str) -> Option<LogEntry> {
    // Simple parser for plain text logs
    // Expected format: TIMESTAMP LEVEL TARGET: MESSAGE
    let parts: Vec<&str> = line.splitn(4, ' ').collect();
    if parts.len() < 4 {
        return None;
    }

    let timestamp_str = format!("{} {}", parts[0], parts[1]);
    let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
        .or_else(|_| DateTime::parse_from_str(&timestamp_str, "%Y-%m-%d %H:%M:%S%.3f"))
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());

    let level = parts[2].to_string();
    let target_and_message = parts[3];

    if let Some(colon_pos) = target_and_message.find(':') {
        let target = target_and_message[..colon_pos].to_string();
        let message = target_and_message[colon_pos + 1..].trim().to_string();

        Some(LogEntry {
            timestamp,
            level,
            target,
            message,
            fields: HashMap::new(),
            span: None,
            thread_name: None,
            file: None,
            line: None,
        })
    } else {
        Some(LogEntry {
            timestamp,
            level,
            target: "unknown".to_string(),
            message: target_and_message.to_string(),
            fields: HashMap::new(),
            span: None,
            thread_name: None,
            file: None,
            line: None,
        })
    }
}

fn filter_logs(mut logs: Vec<LogEntry>, params: &LogQueryParams) -> Vec<LogEntry> {
    if let Some(ref level_filter) = params.level {
        logs.retain(|log| log.level.to_lowercase() == level_filter.to_lowercase());
    }

    if let Some(start_time) = params.start_time {
        logs.retain(|log| log.timestamp >= start_time);
    }

    if let Some(end_time) = params.end_time {
        logs.retain(|log| log.timestamp <= end_time);
    }

    if let Some(ref target_filter) = params.target {
        logs.retain(|log| log.target.contains(target_filter));
    }

    if let Some(ref message_filter) = params.message_contains {
        logs.retain(|log| {
            log.message
                .to_lowercase()
                .contains(&message_filter.to_lowercase())
        });
    }

    logs
}
