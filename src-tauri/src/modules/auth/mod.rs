//! Authentication module
//!
//! Provides user authentication, registration, and session management.

pub mod handlers;
pub mod models;

pub use handlers::*;
pub use models::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiry_hours: u64,
    pub password_min_length: u8,
    pub enable_registration: bool,
    pub require_email_verification: bool,
    pub hash_algorithm: String,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "your-secret-key-here".to_string(),
            jwt_expiry_hours: 24,
            password_min_length: 8,
            enable_registration: true,
            require_email_verification: false,
            hash_algorithm: "argon2".to_string(),
        }
    }
}

/// Initialize the auth module
pub fn init() {
    tracing::info!("Authentication module initialized");
}