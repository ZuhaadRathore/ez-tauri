//! Application configuration management with environment-based settings.

use std::env;
use serde::{Deserialize, Serialize};

/// Application deployment environments with different configuration defaults.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AppEnvironment {
    Development,
    Staging,
    Production,
}

impl Default for AppEnvironment {
    fn default() -> Self {
        Self::Development
    }
}

impl From<String> for AppEnvironment {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "prod" | "production" => Self::Production,
            "stage" | "staging" => Self::Staging,
            _ => Self::Development,
        }
    }
}

impl From<&str> for AppEnvironment {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

/// Main application configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub environment: AppEnvironment,
    pub database_url: String,
    pub redis_url: Option<String>,
}

impl AppConfig {
    /// Creates configuration from environment variables with sensible defaults.
    pub fn from_env() -> Self {
        let environment = env::var("APP_ENV")
            .unwrap_or_else(|_| "development".to_string())
            .into();

        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            match environment {
                AppEnvironment::Production => {
                    panic!("DATABASE_URL must be set in production environment")
                }
                _ => "postgresql://tauri_user:tauri_password@localhost:5432/tauri_app".to_string(),
            }
        });

        let redis_url = env::var("REDIS_URL").ok();

        Self {
            environment,
            database_url,
            redis_url,
        }
    }

    /// Returns true if running in development environment.
    pub fn is_development(&self) -> bool {
        matches!(self.environment, AppEnvironment::Development)
    }

    /// Returns true if running in staging environment.
    pub fn is_staging(&self) -> bool {
        matches!(self.environment, AppEnvironment::Staging)
    }

    /// Returns true if running in production environment.
    pub fn is_production(&self) -> bool {
        matches!(self.environment, AppEnvironment::Production)
    }
}