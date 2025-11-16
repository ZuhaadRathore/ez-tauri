//! Authentication module
//!
//! Provides comprehensive user authentication with JWT tokens, session management,
//! and secure password handling.
//!
//! ## Features
//! - User registration and login
//! - JWT-based access and refresh tokens
//! - Session management with database persistence
//! - Token refresh without re-authentication
//! - Multi-device session support
//! - Session revocation (logout from single or all devices)

pub mod handlers;
pub mod jwt;
pub mod models;
pub mod session;

pub use handlers::*;
pub use jwt::{Claims, JwtError, JwtService, TokenPair, TokenType};
pub use models::*;
pub use session::{Session, SessionError, SessionInfo, SessionManager};

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
            jwt_secret: "your-secret-key-here-change-in-production-min-32-chars!".to_string(),
            jwt_expiry_hours: 24,
            password_min_length: 8,
            enable_registration: true,
            require_email_verification: false,
            hash_algorithm: "bcrypt".to_string(),
        }
    }
}

/// Initialize the auth module
pub fn init() {
    tracing::info!("Authentication module initialized");
}
