//! Comprehensive logging system with file rotation, structured logging, and configuration management.

use anyhow::Result;
use chrono::{DateTime, Utc};
use directories::ProjectDirs;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use tracing::{error, info, warn};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Layer,
};

pub mod config;
pub mod handlers;

/// Ensures logging system is initialized only once.
static LOG_INITIALIZED: Lazy<std::sync::Mutex<bool>> = Lazy::new(|| std::sync::Mutex::new(false));

/// Log levels supported by the application.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for tracing::Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Error => tracing::Level::ERROR,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Trace => tracing::Level::TRACE,
        }
    }
}

impl From<&str> for LogLevel {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "error" => LogLevel::Error,
            "warn" | "warning" => LogLevel::Warn,
            "info" => LogLevel::Info,
            "debug" => LogLevel::Debug,
            "trace" => LogLevel::Trace,
            _ => LogLevel::Info,
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level = match self {
            LogLevel::Error => "error",
            LogLevel::Warn => "warn",
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
            LogLevel::Trace => "trace",
        };

        f.write_str(level)
    }
}

/// Structured log entry with metadata and context information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub target: String,
    pub message: String,
    pub fields: HashMap<String, serde_json::Value>,
    pub span: Option<String>,
    pub thread_name: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
}

/// Configuration for the logging system with file and console output options.
#[derive(Debug, Clone)]
pub struct LogConfig {
    pub level: LogLevel,
    pub console_enabled: bool,
    pub file_enabled: bool,
    pub json_format: bool,
    pub log_dir: PathBuf,
    pub file_prefix: String,
    pub rotation: Rotation,
    pub max_log_files: usize,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            console_enabled: true,
            file_enabled: true,
            json_format: false,
            log_dir: default_log_dir(),
            file_prefix: "ez-tauri".to_string(),
            rotation: Rotation::DAILY,
            max_log_files: 30,
        }
    }
}

/// Initializes the logging system with the given configuration.
///
/// Sets up both console and file logging with the specified format and rotation.
/// This function is idempotent - calling it multiple times has no additional effect.
pub fn init_logging(config: LogConfig) -> Result<()> {
    let mut guard = LOG_INITIALIZED.lock().unwrap();
    if *guard {
        warn!("Logging system already initialized");
        return Ok(());
    }

    if config.file_enabled {
        fs::create_dir_all(&config.log_dir)?;
    }

    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(config.level.to_string()))
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let mut layers = Vec::new();

    if config.console_enabled {
        let console_layer = fmt::layer()
            .with_target(true)
            .with_thread_names(true)
            .with_file(true)
            .with_line_number(true)
            .with_span_events(FmtSpan::CLOSE)
            .with_writer(std::io::stderr);

        if config.json_format {
            layers.push(console_layer.json().boxed());
        } else {
            layers.push(console_layer.pretty().boxed());
        }
    }

    if config.file_enabled {
        let file_appender = RollingFileAppender::new(
            config.rotation.clone(),
            &config.log_dir,
            &format!("{}.log", config.file_prefix),
        );

        let file_layer = fmt::layer()
            .with_target(true)
            .with_thread_names(true)
            .with_file(true)
            .with_line_number(true)
            .with_span_events(FmtSpan::CLOSE)
            .with_writer(file_appender);

        if config.json_format {
            layers.push(file_layer.json().boxed());
        } else {
            layers.push(file_layer.boxed());
        }
    }

    tracing_subscriber::registry()
        .with(env_filter)
        .with(layers)
        .init();

    *guard = true;

    info!(
        "Logging system initialized - Level: {:?}, Console: {}, File: {}, JSON: {}",
        config.level, config.console_enabled, config.file_enabled, config.json_format
    );

    if config.file_enabled {
        cleanup_old_logs(&config)?;
    }

    Ok(())
}

/// Returns the default log directory for the application.
pub(crate) fn default_log_dir() -> PathBuf {
    ProjectDirs::from("com", "tavuc", "eztauri")
        .map(|dirs| dirs.data_dir().join("logs"))
        .unwrap_or_else(|| {
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("logs")
        })
}

/// Returns the default path for logging configuration file.
pub(crate) fn default_log_config_path() -> PathBuf {
    ProjectDirs::from("com", "tavuc", "eztauri")
        .map(|dirs| dirs.config_dir().join("logging.json"))
        .unwrap_or_else(|| {
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("logging.json")
        })
}

/// Cleans up old log files based on retention policy.
fn cleanup_old_logs(config: &LogConfig) -> Result<()> {
    let log_dir = &config.log_dir;

    if !log_dir.exists() {
        return Ok(());
    }

    let mut log_files: Vec<_> = fs::read_dir(log_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .starts_with(&config.file_prefix)
        })
        .collect();

    log_files.sort_by(|a, b| {
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

    for old_file in log_files.iter().skip(config.max_log_files) {
        if let Err(e) = fs::remove_file(old_file.path()) {
            error!("Failed to remove old log file {:?}: {}", old_file.path(), e);
        } else {
            info!("Removed old log file: {:?}", old_file.path());
        }
    }

    Ok(())
}

/// Creates a structured log entry with additional context fields.
#[macro_export]
macro_rules! log_with_context {
    ($level:expr, $message:expr, $($key:expr => $value:expr),*) => {
        match $level {
            $crate::logging::LogLevel::Error => {
                tracing::error!($($key = ?$value,)* $message);
            }
            $crate::logging::LogLevel::Warn => {
                tracing::warn!($($key = ?$value,)* $message);
            }
            $crate::logging::LogLevel::Info => {
                tracing::info!($($key = ?$value,)* $message);
            }
            $crate::logging::LogLevel::Debug => {
                tracing::debug!($($key = ?$value,)* $message);
            }
            $crate::logging::LogLevel::Trace => {
                tracing::trace!($($key = ?$value,)* $message);
            }
        }
    };
}

/// Initializes logging system using environment variables and configuration files.
pub fn init_logging_from_env() -> Result<()> {
    let env_config = config::load_config_from_env();

    let json_format_override = env::var("LOG_JSON")
        .ok()
        .and_then(|value| value.parse::<bool>().ok());
    let json_format = json_format_override
        .unwrap_or(matches!(env_config.console.format, config::LogFormat::Json));

    let log_directory_override = env::var("LOG_DIRECTORY")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map(|value| value.trim().to_string())
        .or_else(|| {
            let candidate = env_config.file.directory.trim();
            if candidate.is_empty() {
                None
            } else {
                Some(candidate.to_string())
            }
        });

    let log_dir = log_directory_override
        .map(PathBuf::from)
        .unwrap_or_else(default_log_dir);

    let file_prefix = env::var("LOG_FILE_PREFIX")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map(|value| value.trim().to_string())
        .unwrap_or_else(|| env_config.file.filename_prefix.clone());

    let config = LogConfig {
        level: env_config.level.clone(),
        console_enabled: env_config.enabled && env_config.console.enabled,
        file_enabled: env_config.enabled && env_config.file.enabled,
        json_format,
        log_dir,
        file_prefix,
        rotation: env_config.file.rotation.clone().into(),
        max_log_files: env_config.file.max_files,
    };

    init_logging(config)
}
