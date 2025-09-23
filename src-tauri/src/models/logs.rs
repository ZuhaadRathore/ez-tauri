use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct AppLog {
    pub id: Uuid,
    pub level: String,
    pub message: String,
    pub metadata: serde_json::Value,
    pub user_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAppLog {
    pub level: String,
    pub message: String,
    pub metadata: Option<serde_json::Value>,
    pub user_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Error => "error".to_string(),
            LogLevel::Warn => "warn".to_string(),
            LogLevel::Info => "info".to_string(),
            LogLevel::Debug => "debug".to_string(),
            LogLevel::Trace => "trace".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogQuery {
    pub level: Option<String>,
    pub user_id: Option<Uuid>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
