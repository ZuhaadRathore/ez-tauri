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

    /// Validates production configuration to ensure security requirements are met.
    ///
    /// This should be called during application startup to fail fast if critical
    /// configuration is missing or insecure.
    pub fn validate_production_config() -> Result<(), String> {
        let environment = env::var("APP_ENV")
            .unwrap_or_else(|_| "development".to_string())
            .into();

        // Only validate in production/staging environments
        if !matches!(environment, AppEnvironment::Production | AppEnvironment::Staging) {
            return Ok(());
        }

        let env_name = match environment {
            AppEnvironment::Production => "production",
            AppEnvironment::Staging => "staging",
            _ => "unknown",
        };

        tracing::info!("Validating configuration for {} environment", env_name);

        // Validate JWT_SECRET
        match env::var("JWT_SECRET") {
            Ok(secret) => {
                if secret.len() < 32 {
                    return Err(format!(
                        "JWT_SECRET must be at least 32 characters in {} (current: {} chars). Generate with: openssl rand -base64 32",
                        env_name, secret.len()
                    ));
                }
                // Check if using the placeholder value
                if secret.contains("your-secret-key-here") || secret.contains("change-in-production") {
                    return Err(format!(
                        "JWT_SECRET appears to use a placeholder value in {}. Generate a proper secret with: openssl rand -base64 32",
                        env_name
                    ));
                }
            }
            Err(_) => {
                return Err(format!(
                    "JWT_SECRET environment variable must be set in {}. Generate with: openssl rand -base64 32",
                    env_name
                ));
            }
        }

        // Validate DATABASE_URL
        if env::var("DATABASE_URL").is_err() {
            return Err(format!("DATABASE_URL must be set in {}", env_name));
        }

        // Warn about missing REDIS_URL but don't fail (it's optional)
        if env::var("REDIS_URL").is_err() {
            tracing::warn!("REDIS_URL not set in {} - caching will be disabled", env_name);
        }

        tracing::info!("Configuration validation passed for {} environment", env_name);
        Ok(())
    }
}