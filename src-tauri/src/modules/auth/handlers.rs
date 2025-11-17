//! Authentication command handlers

use super::jwt::{JwtError, JwtService, TokenPair};
use super::session::{SessionError, SessionInfo, SessionManager};
use crate::database::get_pool_ref;
use crate::errors::{AppError, AppResult, ErrorCode, IntoAppError};
use crate::models::{CreateUser, PublicUser, User};
use crate::validation::{validate_email, validate_optional_name, validate_username};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use tauri::State;
use specta::Type;
use std::env;
use uuid::Uuid;

/// Login request structure
#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
#[specta(rename_all = "camelCase")]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub device_info: Option<String>,
}

/// Login response with tokens
#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
#[specta(rename_all = "camelCase")]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: PublicUser,
    pub expires_in: i64,
}

/// Registration request
#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
#[specta(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

/// Token refresh request
#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
#[specta(rename_all = "camelCase")]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

/// Token refresh response
#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
#[specta(rename_all = "camelCase")]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

/// Register a new user account
#[tauri::command]
pub async fn auth_register(
    jwt_service: State<'_, JwtService>,
    request: RegisterRequest,
) -> AppResult<LoginResponse> {
    tracing::info!("Registration attempt for email: {}", request.email);

    let pool = get_pool_ref()
        .into_app_error(ErrorCode::DatabaseConnection)?;

    // Validate input
    let email = validate_email(&request.email)
        .map_err(|e| AppError::validation_error(format!("Invalid email: {}", e)))?;

    let username = validate_username(&request.username)
        .map_err(|e| AppError::validation_error(format!("Invalid username: {}", e)))?;

    let first_name = validate_optional_name(request.first_name.as_deref())
        .map_err(|e| AppError::validation_error(format!("Invalid first name: {}", e)))?;

    let last_name = validate_optional_name(request.last_name.as_deref())
        .map_err(|e| AppError::validation_error(format!("Invalid last name: {}", e)))?;

    // Validate password strength
    if request.password.len() < 8 {
        return Err(AppError::validation_error("Password must be at least 8 characters"));
    }

    // Hash password
    let password_hash = hash(&request.password, DEFAULT_COST)
        .map_err(|e| AppError::internal_error(format!("Password hashing failed: {}", e)))?;

    // Create user in database
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, username, password_hash, first_name, last_name)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, email, username, password_hash, first_name, last_name, is_active, created_at, updated_at
        "#,
    )
    .bind(&email)
    .bind(&username)
    .bind(&password_hash)
    .bind(&first_name)
    .bind(&last_name)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| {
        // Check for unique constraint violation using proper SQLx error types
        if let sqlx::Error::Database(db_err) = &e {
            if let Some(pg_err) = db_err.downcast_ref::<sqlx::postgres::PgDatabaseError>() {
                // PostgreSQL error code 23505 = unique_violation
                if pg_err.code() == "23505" {
                    return AppError::validation_error("Email or username already exists");
                }
            }
        }
        AppError::database_error(format!("Failed to create user: {}", e))
    })?;

    // Generate tokens using managed state
    let token_pair = jwt_service
        .generate_token_pair(user.id, &user.email)
        .map_err(|e| AppError::internal_error(format!("Token generation failed: {}", e)))?;

    // Create session
    SessionManager::create_session(
        pool.as_ref(),
        user.id,
        &token_pair.refresh_token,
        None, // device_info
        None, // ip_address
        30,   // expiry_days
    )
    .await
    .map_err(|e| AppError::database_error(format!("Session creation failed: {}", e)))?;

    tracing::info!("User registered successfully: {}", user.id);

    Ok(LoginResponse {
        access_token: token_pair.access_token,
        refresh_token: token_pair.refresh_token,
        user: PublicUser::from(user),
        expires_in: jwt_service.access_token_expiry_hours() * 3600, // Convert hours to seconds
    })
}

/// Authenticate a user and return tokens
#[tauri::command]
pub async fn auth_login(
    jwt_service: State<'_, JwtService>,
    request: LoginRequest,
) -> AppResult<LoginResponse> {
    tracing::info!("Login attempt for user: {}", request.email);

    let pool = get_pool_ref()
        .into_app_error(ErrorCode::DatabaseConnection)?;

    // Validate email format
    let email = validate_email(&request.email)
        .map_err(|e| AppError::validation_error(format!("Invalid email: {}", e)))?;

    // Find user by email
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, username, password_hash, first_name, last_name, is_active, created_at, updated_at
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(&email)
    .fetch_optional(pool.as_ref())
    .await
    .into_app_error(ErrorCode::DatabaseQuery)?
    .ok_or_else(|| AppError::unauthorized("Invalid email or password"))?;

    // Check if user is active
    if !user.is_active {
        return Err(AppError::forbidden("Account is inactive"));
    }

    // Verify password
    let password_valid = verify(&request.password, &user.password_hash)
        .map_err(|e| AppError::internal_error(format!("Password verification failed: {}", e)))?;

    if !password_valid {
        tracing::warn!("Failed login attempt for user: {}", user.email);
        return Err(AppError::unauthorized("Invalid email or password"));
    }

    // Generate tokens using managed state
    let token_pair = jwt_service
        .generate_token_pair(user.id, &user.email)
        .map_err(|e| AppError::internal_error(format!("Token generation failed: {}", e)))?;

    // Create session with device info
    SessionManager::create_session(
        pool.as_ref(),
        user.id,
        &token_pair.refresh_token,
        request.device_info,
        None, // ip_address - could be extracted from request context
        30,   // expiry_days
    )
    .await
    .map_err(|e| AppError::database_error(format!("Session creation failed: {}", e)))?;

    tracing::info!("User logged in successfully: {}", user.id);

    Ok(LoginResponse {
        access_token: token_pair.access_token,
        refresh_token: token_pair.refresh_token,
        user: PublicUser::from(user),
        expires_in: jwt_service.access_token_expiry_hours() * 3600, // Convert hours to seconds
    })
}

/// Refresh access token using refresh token
#[tauri::command]
pub async fn auth_refresh_token(
    jwt_service: State<'_, JwtService>,
    request: RefreshTokenRequest,
) -> AppResult<RefreshTokenResponse> {
    tracing::debug!("Token refresh attempt");

    let pool = get_pool_ref()
        .into_app_error(ErrorCode::DatabaseConnection)?;

    // Validate refresh token using managed state
    let claims = jwt_service
        .validate_refresh_token(&request.refresh_token)
        .map_err(|e| match e {
            JwtError::TokenExpired => AppError::new(ErrorCode::TokenExpired, "Refresh token expired"),
            JwtError::InvalidToken | JwtError::InvalidSignature => {
                AppError::unauthorized("Invalid refresh token")
            }
            _ => AppError::internal_error(format!("Token validation failed: {}", e)),
        })?;

    // Find and validate session
    let session = SessionManager::find_by_refresh_token(pool.as_ref(), &request.refresh_token)
        .await
        .map_err(|e| match e {
            SessionError::NotFound => AppError::unauthorized("Session not found"),
            _ => AppError::database_error(format!("Session lookup failed: {}", e)),
        })?
        .ok_or_else(|| AppError::unauthorized("Invalid or expired session"))?;

    // Verify session is still valid
    if !SessionManager::is_session_valid(&session) {
        return Err(AppError::unauthorized("Session is no longer valid"));
    }

    // Update session last used time
    SessionManager::update_last_used(pool.as_ref(), session.id)
        .await
        .map_err(|e| AppError::database_error(format!("Failed to update session: {}", e)))?;

    // Parse user ID from claims
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|e| AppError::internal_error(format!("Invalid user ID in token: {}", e)))?;

    // Generate new access token (refresh token stays the same)
    let access_token = jwt_service
        .generate_access_token(user_id, &claims.email)
        .map_err(|e| AppError::internal_error(format!("Token generation failed: {}", e)))?;

    tracing::debug!("Token refreshed successfully for user: {}", user_id);

    Ok(RefreshTokenResponse {
        access_token,
        refresh_token: request.refresh_token, // Return same refresh token
        expires_in: jwt_service.access_token_expiry_hours() * 3600, // Convert hours to seconds
    })
}

/// Logout and revoke current session
#[tauri::command]
pub async fn auth_logout(refresh_token: String) -> AppResult<()> {
    tracing::info!("Logout attempt");

    let pool = get_pool_ref()
        .into_app_error(ErrorCode::DatabaseConnection)?;

    // Revoke the session
    SessionManager::revoke_by_token(pool.as_ref(), &refresh_token)
        .await
        .map_err(|e| AppError::database_error(format!("Failed to revoke session: {}", e)))?;

    tracing::info!("User logged out successfully");
    Ok(())
}

/// Logout from all devices (revoke all sessions)
#[tauri::command]
pub async fn auth_logout_all(
    jwt_service: State<'_, JwtService>,
    access_token: String,
) -> AppResult<()> {
    tracing::info!("Logout all devices attempt");

    let pool = get_pool_ref()
        .into_app_error(ErrorCode::DatabaseConnection)?;

    // Validate access token and get user ID using managed state
    let claims = jwt_service
        .validate_access_token(&access_token)
        .map_err(|_| AppError::unauthorized("Invalid access token"))?;

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|e| AppError::internal_error(format!("Invalid user ID: {}", e)))?;

    // Revoke all user sessions
    SessionManager::revoke_all_user_sessions(pool.as_ref(), user_id)
        .await
        .map_err(|e| AppError::database_error(format!("Failed to revoke sessions: {}", e)))?;

    tracing::info!("All sessions revoked for user: {}", user_id);
    Ok(())
}

/// Check if access token is valid
#[tauri::command]
pub async fn auth_verify_token(
    jwt_service: State<'_, JwtService>,
    access_token: String,
) -> AppResult<PublicUser> {
    let pool = get_pool_ref()
        .into_app_error(ErrorCode::DatabaseConnection)?;

    // Validate token using managed state
    let claims = jwt_service
        .validate_access_token(&access_token)
        .map_err(|e| match e {
            JwtError::TokenExpired => AppError::new(ErrorCode::TokenExpired, "Token expired"),
            _ => AppError::unauthorized("Invalid token"),
        })?;

    // Get user from database
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|e| AppError::internal_error(format!("Invalid user ID: {}", e)))?;

    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, username, password_hash, first_name, last_name, is_active, created_at, updated_at
        FROM users
        WHERE id = $1 AND is_active = true
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool.as_ref())
    .await
    .into_app_error(ErrorCode::DatabaseQuery)?
    .ok_or_else(|| AppError::unauthorized("User not found or inactive"))?;

    Ok(PublicUser::from(user))
}

/// Get all active sessions for the current user
#[tauri::command]
pub async fn auth_get_sessions(
    jwt_service: State<'_, JwtService>,
    access_token: String,
) -> AppResult<Vec<SessionInfo>> {
    let pool = get_pool_ref()
        .into_app_error(ErrorCode::DatabaseConnection)?;

    // Validate token and get user ID using managed state
    let claims = jwt_service
        .validate_access_token(&access_token)
        .map_err(|_| AppError::unauthorized("Invalid access token"))?;

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|e| AppError::internal_error(format!("Invalid user ID: {}", e)))?;

    // Get user sessions
    let sessions = SessionManager::get_user_sessions(pool.as_ref(), user_id)
        .await
        .map_err(|e| AppError::database_error(format!("Failed to get sessions: {}", e)))?;

    Ok(sessions.into_iter().map(SessionInfo::from).collect())
}

/// Revoke a specific session by ID
#[tauri::command]
pub async fn auth_revoke_session(
    jwt_service: State<'_, JwtService>,
    access_token: String,
    session_id: String,
) -> AppResult<()> {
    let pool = get_pool_ref()
        .into_app_error(ErrorCode::DatabaseConnection)?;

    // Validate token and get user ID using managed state
    let claims = jwt_service
        .validate_access_token(&access_token)
        .map_err(|_| AppError::unauthorized("Invalid access token"))?;

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|e| AppError::internal_error(format!("Invalid user ID: {}", e)))?;

    let session_uuid = Uuid::parse_str(&session_id)
        .map_err(|e| AppError::validation_error(format!("Invalid session ID: {}", e)))?;

    // Verify session belongs to user before revoking
    let user_sessions = SessionManager::get_user_sessions(pool.as_ref(), user_id)
        .await
        .map_err(|e| AppError::database_error(format!("Failed to verify session ownership: {}", e)))?;

    // Check if the session ID belongs to this user
    if !user_sessions.iter().any(|s| s.id.to_string() == session_id) {
        return Err(AppError::forbidden("Session does not belong to user"));
    }

    // Revoke the session
    SessionManager::revoke_session(pool.as_ref(), session_uuid)
        .await
        .map_err(|e| AppError::database_error(format!("Failed to revoke session: {}", e)))?;

    tracing::info!("Session revoked: {} for user: {}", session_id, user_id);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_service_creation() {
        let service = JwtService::new("test-secret-key-min-32-chars!!".to_string());
        let user_id = Uuid::new_v4();
        let email = "test@example.com";

        let result = service.generate_token_pair(user_id, email);
        assert!(result.is_ok());
    }
}
