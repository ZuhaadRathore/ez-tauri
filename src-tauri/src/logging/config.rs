//! Logging configuration structures and management.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing_appender::rolling::Rotation;

use super::LogLevel;

/// Main logging configuration structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppLogConfig {
    pub enabled: bool,
    pub level: LogLevel,
    pub console: ConsoleLogConfig,
    pub file: FileLogConfig,
    pub structured: StructuredLogConfig,
}

/// Configuration for console logging output.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsoleLogConfig {
    pub enabled: bool,
    pub format: LogFormat,
    pub colors: bool,
}

/// Configuration for file logging with rotation settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileLogConfig {
    pub enabled: bool,
    pub directory: String,
    pub filename_prefix: String,
    pub rotation: LogRotation,
    pub max_files: usize,
    pub max_size_mb: Option<u64>,
}

/// Configuration for structured logging features.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructuredLogConfig {
    pub enabled: bool,
    pub include_spans: bool,
    pub include_targets: bool,
    pub include_thread_names: bool,
    pub include_file_info: bool,
}

/// Available log output formats.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LogFormat {
    Pretty,
    Compact,
    Json,
    Full,
}

/// Log file rotation intervals.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LogRotation {
    Never,
    Minutely,
    Hourly,
    Daily,
    Weekly,
}

impl From<LogRotation> for Rotation {
    fn from(rotation: LogRotation) -> Self {
        match rotation {
            LogRotation::Never => Rotation::NEVER,
            LogRotation::Minutely => Rotation::MINUTELY,
            LogRotation::Hourly => Rotation::HOURLY,
            LogRotation::Daily => Rotation::DAILY,
            LogRotation::Weekly => Rotation::DAILY, // Tracing doesn't have weekly, use daily
        }
    }
}

impl Default for AppLogConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level: LogLevel::Info,
            console: ConsoleLogConfig::default(),
            file: FileLogConfig::default(),
            structured: StructuredLogConfig::default(),
        }
    }
}

impl Default for ConsoleLogConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            format: LogFormat::Pretty,
            colors: true,
        }
    }
}

impl Default for FileLogConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            directory: "logs".to_string(),
            filename_prefix: "ez-tauri".to_string(),
            rotation: LogRotation::Daily,
            max_files: 30,
            max_size_mb: Some(100),
        }
    }
}

impl Default for StructuredLogConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            include_spans: true,
            include_targets: true,
            include_thread_names: true,
            include_file_info: true,
        }
    }
}

/// Loads logging configuration from environment variables with fallback defaults.
pub fn load_config_from_env() -> AppLogConfig {
    use std::env;

    let mut config = AppLogConfig::default();

    if let Ok(enabled) = env::var("LOG_ENABLED") {
        config.enabled = enabled.parse().unwrap_or(true);
    }

    if let Ok(level) = env::var("LOG_LEVEL") {
        config.level = level.as_str().into();
    }

    if let Ok(console_enabled) = env::var("LOG_CONSOLE_ENABLED") {
        config.console.enabled = console_enabled.parse().unwrap_or(true);
    }

    if let Ok(console_format) = env::var("LOG_CONSOLE_FORMAT") {
        config.console.format = match console_format.to_lowercase().as_str() {
            "json" => LogFormat::Json,
            "compact" => LogFormat::Compact,
            "full" => LogFormat::Full,
            _ => LogFormat::Pretty,
        };
    }

    if let Ok(colors) = env::var("LOG_CONSOLE_COLORS") {
        config.console.colors = colors.parse().unwrap_or(true);
    }

    if let Ok(file_enabled) = env::var("LOG_FILE_ENABLED") {
        config.file.enabled = file_enabled.parse().unwrap_or(true);
    }

    if let Ok(log_dir) = env::var("LOG_DIRECTORY") {
        config.file.directory = log_dir;
    }

    if let Ok(prefix) = env::var("LOG_FILE_PREFIX") {
        config.file.filename_prefix = prefix;
    }

    if let Ok(rotation) = env::var("LOG_ROTATION") {
        config.file.rotation = match rotation.to_lowercase().as_str() {
            "never" => LogRotation::Never,
            "minutely" => LogRotation::Minutely,
            "hourly" => LogRotation::Hourly,
            "weekly" => LogRotation::Weekly,
            _ => LogRotation::Daily,
        };
    }

    if let Ok(max_files) = env::var("LOG_MAX_FILES") {
        if let Ok(num) = max_files.parse() {
            config.file.max_files = num;
        }
    }

    if let Ok(max_size) = env::var("LOG_MAX_SIZE_MB") {
        if let Ok(size) = max_size.parse() {
            config.file.max_size_mb = Some(size);
        }
    }

    config
}

/// Saves logging configuration to a JSON file.
pub fn save_config_to_file(config: &AppLogConfig, path: &PathBuf) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(config)?;
    std::fs::write(path, json)?;
    Ok(())
}

/// Loads logging configuration from a JSON file.
pub fn load_config_from_file(path: &PathBuf) -> anyhow::Result<AppLogConfig> {
    let content = std::fs::read_to_string(path)?;
    let config: AppLogConfig = serde_json::from_str(&content)?;
    Ok(config)
}
