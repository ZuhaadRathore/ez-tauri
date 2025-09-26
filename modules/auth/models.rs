//! Authentication models

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub email_verified: bool,
}

impl User {
    pub fn new(email: String, password_hash: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            email,
            password_hash,
            created_at: now,
            updated_at: now,
            email_verified: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthSession {
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}