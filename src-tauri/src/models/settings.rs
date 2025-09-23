use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserSettings {
    pub id: Uuid,
    pub user_id: Uuid,
    pub theme: String,
    pub language: String,
    pub notifications_enabled: bool,
    pub settings_data: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserSettings {
    pub user_id: Uuid,
    pub theme: Option<String>,
    pub language: Option<String>,
    pub notifications_enabled: Option<bool>,
    pub settings_data: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserSettings {
    pub theme: Option<String>,
    pub language: Option<String>,
    pub notifications_enabled: Option<bool>,
    pub settings_data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub sidebar_collapsed: Option<bool>,
    pub auto_save: Option<bool>,
    pub notifications: Option<bool>,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            sidebar_collapsed: Some(false),
            auto_save: Some(true),
            notifications: Some(true),
        }
    }
}
