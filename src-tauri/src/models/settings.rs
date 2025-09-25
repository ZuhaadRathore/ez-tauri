//! User settings and application configuration models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// User-specific settings stored in the database.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[allow(dead_code)]
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

/// Request payload for creating new user settings.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CreateUserSettings {
    pub user_id: Uuid,
    pub theme: Option<String>,
    pub language: Option<String>,
    pub notifications_enabled: Option<bool>,
    pub settings_data: Option<serde_json::Value>,
}

/// Request payload for updating existing user settings.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct UpdateUserSettings {
    pub theme: Option<String>,
    pub language: Option<String>,
    pub notifications_enabled: Option<bool>,
    pub settings_data: Option<serde_json::Value>,
}

/// General application settings with common UI preferences.
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct AppSettings {
    pub sidebar_collapsed: Option<bool>,
    pub auto_save: Option<bool>,
    pub notifications: Option<bool>,
}

impl Default for AppSettings {
    /// Creates default application settings with sensible defaults.
    fn default() -> Self {
        AppSettings {
            sidebar_collapsed: Some(false),
            auto_save: Some(true),
            notifications: Some(true),
        }
    }
}
