//! Session management for authentication

use crate::database::get_pool_ref;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;
use sqlx::PgPool;
use uuid::Uuid;

/// Session stored in database
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub refresh_token: String,
    pub device_info: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_used_at: DateTime<Utc>,
    pub revoked: bool,
}

impl Session {
    /// Create a new session
    pub fn new(
        user_id: Uuid,
        refresh_token: String,
        device_info: Option<String>,
        ip_address: Option<String>,
        expiry_days: i64,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            refresh_token,
            device_info,
            ip_address,
            created_at: now,
            expires_at: now + Duration::days(expiry_days),
            last_used_at: now,
            revoked: false,
        }
    }
}

/// Session manager for database operations
pub struct SessionManager;

impl SessionManager {
    /// Create a new session in the database
    pub async fn create_session(
        pool: &PgPool,
        user_id: Uuid,
        refresh_token: &str,
        device_info: Option<String>,
        ip_address: Option<String>,
        expiry_days: i64,
    ) -> Result<Session, SessionError> {
        let session = Session::new(
            user_id,
            refresh_token.to_string(),
            device_info,
            ip_address,
            expiry_days,
        );

        sqlx::query_as::<_, Session>(
            r#"
            INSERT INTO sessions (id, user_id, refresh_token, device_info, ip_address, created_at, expires_at, last_used_at, revoked)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, user_id, refresh_token, device_info, ip_address, created_at, expires_at, last_used_at, revoked
            "#,
        )
        .bind(session.id)
        .bind(session.user_id)
        .bind(&session.refresh_token)
        .bind(&session.device_info)
        .bind(&session.ip_address)
        .bind(session.created_at)
        .bind(session.expires_at)
        .bind(session.last_used_at)
        .bind(session.revoked)
        .fetch_one(pool)
        .await
        .map_err(|e| SessionError::DatabaseError(e.to_string()))
    }

    /// Find a session by refresh token
    pub async fn find_by_refresh_token(
        pool: &PgPool,
        refresh_token: &str,
    ) -> Result<Option<Session>, SessionError> {
        sqlx::query_as::<_, Session>(
            r#"
            SELECT id, user_id, refresh_token, device_info, ip_address, created_at, expires_at, last_used_at, revoked
            FROM sessions
            WHERE refresh_token = $1 AND revoked = false AND expires_at > NOW()
            "#,
        )
        .bind(refresh_token)
        .fetch_optional(pool)
        .await
        .map_err(|e| SessionError::DatabaseError(e.to_string()))
    }

    /// Update session last used time
    pub async fn update_last_used(
        pool: &PgPool,
        session_id: Uuid,
    ) -> Result<(), SessionError> {
        sqlx::query(
            r#"
            UPDATE sessions
            SET last_used_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(session_id)
        .execute(pool)
        .await
        .map_err(|e| SessionError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Revoke a specific session
    pub async fn revoke_session(
        pool: &PgPool,
        session_id: Uuid,
    ) -> Result<(), SessionError> {
        sqlx::query(
            r#"
            UPDATE sessions
            SET revoked = true
            WHERE id = $1
            "#,
        )
        .bind(session_id)
        .execute(pool)
        .await
        .map_err(|e| SessionError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Revoke a session by refresh token
    pub async fn revoke_by_token(
        pool: &PgPool,
        refresh_token: &str,
    ) -> Result<(), SessionError> {
        sqlx::query(
            r#"
            UPDATE sessions
            SET revoked = true
            WHERE refresh_token = $1
            "#,
        )
        .bind(refresh_token)
        .execute(pool)
        .await
        .map_err(|e| SessionError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Revoke all sessions for a user
    pub async fn revoke_all_user_sessions(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<(), SessionError> {
        sqlx::query(
            r#"
            UPDATE sessions
            SET revoked = true
            WHERE user_id = $1 AND revoked = false
            "#,
        )
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| SessionError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Get all active sessions for a user
    pub async fn get_user_sessions(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<Session>, SessionError> {
        sqlx::query_as::<_, Session>(
            r#"
            SELECT id, user_id, refresh_token, device_info, ip_address, created_at, expires_at, last_used_at, revoked
            FROM sessions
            WHERE user_id = $1 AND revoked = false AND expires_at > NOW()
            ORDER BY last_used_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| SessionError::DatabaseError(e.to_string()))
    }

    /// Clean up expired sessions (should be run periodically)
    pub async fn cleanup_expired_sessions(pool: &PgPool) -> Result<u64, SessionError> {
        let result = sqlx::query(
            r#"
            DELETE FROM sessions
            WHERE expires_at < NOW() OR (revoked = true AND last_used_at < NOW() - INTERVAL '30 days')
            "#,
        )
        .execute(pool)
        .await
        .map_err(|e| SessionError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected())
    }

    /// Validate that a session is still valid
    pub fn is_session_valid(session: &Session) -> bool {
        !session.revoked && session.expires_at > Utc::now()
    }
}

/// Session-related errors
#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Session not found")]
    NotFound,

    #[error("Session expired")]
    Expired,

    #[error("Session revoked")]
    Revoked,
}

/// Public session info (without sensitive data)
#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
#[specta(rename_all = "camelCase")]
pub struct SessionInfo {
    pub id: String,
    pub device_info: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: DateTime<Utc>,
}

impl From<Session> for SessionInfo {
    fn from(session: Session) -> Self {
        Self {
            id: session.id.to_string(),
            device_info: session.device_info,
            created_at: session.created_at,
            last_used_at: session.last_used_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_session() {
        let user_id = Uuid::new_v4();
        let session = Session::new(
            user_id,
            "refresh_token".to_string(),
            Some("Desktop App".to_string()),
            Some("127.0.0.1".to_string()),
            30,
        );

        assert_eq!(session.user_id, user_id);
        assert!(!session.revoked);
        assert!(session.expires_at > Utc::now());
    }

    #[test]
    fn test_session_validity() {
        let user_id = Uuid::new_v4();
        let session = Session::new(
            user_id,
            "refresh_token".to_string(),
            None,
            None,
            30,
        );

        assert!(SessionManager::is_session_valid(&session));

        let mut revoked_session = session.clone();
        revoked_session.revoked = true;
        assert!(!SessionManager::is_session_valid(&revoked_session));
    }
}
