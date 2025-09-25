//! User management command handlers.

use crate::database::get_pool_ref;
use crate::models::{CreateUser, LoginRequest, PublicUser, UpdateUser, User};
use crate::validation::{validate_email, validate_username, validate_optional_name};
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

/// Retrieves all users from the database (excluding password hashes).
#[tauri::command]
pub async fn get_all_users() -> Result<Vec<PublicUser>, String> {
    let pool = get_pool_ref().map_err(|e| e.to_string())?;

    let users: Vec<User> = sqlx::query_as::<_, User>(
        r#"
        SELECT id,
               email,
               username,
               password_hash,
               first_name,
               last_name,
               is_active,
               created_at,
               updated_at
        FROM users
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| format!("Failed to fetch users: {}", e))?;

    Ok(users.into_iter().map(PublicUser::from).collect())
}

/// Retrieves a specific user by their UUID.
#[tauri::command]
pub async fn get_user_by_id(user_id: String) -> Result<Option<PublicUser>, String> {
    let pool = get_pool_ref().map_err(|e| e.to_string())?;
    let uuid = Uuid::parse_str(&user_id).map_err(|e| format!("Invalid UUID: {}", e))?;

    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id,
               email,
               username,
               password_hash,
               first_name,
               last_name,
               is_active,
               created_at,
               updated_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(uuid)
    .fetch_optional(pool.as_ref())
    .await
    .map_err(|e| format!("Failed to fetch user: {}", e))?;

    Ok(user.map(PublicUser::from))
}

/// Creates a new user account with validation and password hashing.
#[tauri::command]
pub async fn create_user(user_data: CreateUser) -> Result<PublicUser, String> {
    let pool = get_pool_ref().map_err(|e| e.to_string())?;
    let CreateUser {
        email,
        username,
        password,
        first_name,
        last_name,
    } = user_data;

    let email = validate_email(&email).map_err(|e| format!("Invalid email: {}", e))?;
    let username = validate_username(&username).map_err(|e| format!("Invalid username: {}", e))?;
    let first_name = validate_optional_name(first_name.as_deref()).map_err(|e| format!("Invalid first name: {}", e))?;
    let last_name = validate_optional_name(last_name.as_deref()).map_err(|e| format!("Invalid last name: {}", e))?;

    let password_hash = hash(password.as_str(), DEFAULT_COST)
        .map_err(|e| format!("Failed to hash password: {}", e))?;

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, username, password_hash, first_name, last_name)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id,
                  email,
                  username,
                  password_hash,
                  first_name,
                  last_name,
                  is_active,
                  created_at,
                  updated_at
        "#,
    )
    .bind(email)
    .bind(username)
    .bind(password_hash)
    .bind(first_name)
    .bind(last_name)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| format!("Failed to create user: {}", e))?;

    Ok(PublicUser::from(user))
}

#[tauri::command]
pub async fn update_user(user_id: String, user_data: UpdateUser) -> Result<PublicUser, String> {
    let pool = get_pool_ref().map_err(|e| e.to_string())?;
    let uuid = Uuid::parse_str(&user_id).map_err(|e| format!("Invalid UUID: {}", e))?;
    let UpdateUser {
        email,
        username,
        first_name,
        last_name,
        is_active,
    } = user_data;

    // Validate and sanitize inputs
    let email = match email.as_deref() {
        Some(e) => Some(validate_email(e).map_err(|e| format!("Invalid email: {}", e))?),
        None => None,
    };
    let username = match username.as_deref() {
        Some(u) => Some(validate_username(u).map_err(|e| format!("Invalid username: {}", e))?),
        None => None,
    };
    let first_name = validate_optional_name(first_name.as_deref()).map_err(|e| format!("Invalid first name: {}", e))?;
    let last_name = validate_optional_name(last_name.as_deref()).map_err(|e| format!("Invalid last name: {}", e))?;

    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET email = COALESCE($2, email),
            username = COALESCE($3, username),
            first_name = COALESCE($4, first_name),
            last_name = COALESCE($5, last_name),
            is_active = COALESCE($6, is_active),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING id,
                  email,
                  username,
                  password_hash,
                  first_name,
                  last_name,
                  is_active,
                  created_at,
                  updated_at
        "#,
    )
    .bind(uuid)
    .bind(email)
    .bind(username)
    .bind(first_name)
    .bind(last_name)
    .bind(is_active)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| format!("Failed to update user: {}", e))?;

    Ok(PublicUser::from(user))
}

#[tauri::command]
pub async fn delete_user(user_id: String) -> Result<String, String> {
    let pool = get_pool_ref().map_err(|e| e.to_string())?;
    let uuid = Uuid::parse_str(&user_id).map_err(|e| format!("Invalid UUID: {}", e))?;

    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(uuid)
        .execute(pool.as_ref())
        .await
        .map_err(|e| format!("Failed to delete user: {}", e))?;

    if result.rows_affected() > 0 {
        Ok("User deleted successfully".to_string())
    } else {
        Err("User not found".to_string())
    }
}

#[tauri::command]
pub async fn authenticate_user(login_data: LoginRequest) -> Result<Option<PublicUser>, String> {
    let pool = get_pool_ref().map_err(|e| e.to_string())?;
    let LoginRequest { email, password } = login_data;

    // Validate email input
    let email = validate_email(&email).map_err(|e| format!("Invalid email: {}", e))?;

    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id,
               email,
               username,
               password_hash,
               first_name,
               last_name,
               is_active,
               created_at,
               updated_at
        FROM users
        WHERE email = $1
          AND is_active = TRUE
        LIMIT 1
        "#,
    )
    .bind(&email)
    .fetch_optional(pool.as_ref())
    .await
    .map_err(|e| format!("Failed to authenticate user: {}", e))?;

    if let Some(user) = user {
        match verify(password.as_str(), &user.password_hash) {
            Ok(true) => Ok(Some(PublicUser::from(user))),
            Ok(false) => Ok(None),
            Err(e) => Err(format!("Failed to verify password: {}", e)),
        }
    } else {
        Ok(None)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::test_utils::{pool, reset_all_tables};
    use crate::models::{CreateUser, LoginRequest, UpdateUser};
    use anyhow::Result as AnyResult;
    use serial_test::serial;
    use uuid::Uuid;

    fn sample_user_payload() -> CreateUser {
        let unique_suffix = Uuid::new_v4();
        CreateUser {
            email: format!("user+{}@example.com", unique_suffix),
            username: format!("user_{}", unique_suffix.simple()),
            password: "Sup3r$ecret".to_string(),
            first_name: Some("Test".to_string()),
            last_name: Some("User".to_string()),
        }
    }

    #[tokio::test]
    #[serial]
    async fn full_user_lifecycle_and_authentication() -> AnyResult<()> {
        let pool = pool().await?;
        reset_all_tables(pool.as_ref()).await?;

        let payload = sample_user_payload();
        let email = payload.email.clone();
        let password = payload.password.clone();

        let created = create_user(payload)
            .await
            .expect("user creation should succeed");
        assert_eq!(created.email, email);
        assert_eq!(created.first_name.as_deref(), Some("Test"));

        let listed = get_all_users().await.expect("listing users should succeed");
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].email, email);

        let fetched = get_user_by_id(created.id.to_string())
            .await
            .expect("fetching user should succeed")
            .expect("user should exist");
        assert_eq!(fetched.username, listed[0].username);

        let updated = update_user(
            created.id.to_string(),
            UpdateUser {
                email: None,
                username: Some("updated_user".to_string()),
                first_name: Some("Updated".to_string()),
                last_name: None,
                is_active: Some(true),
            },
        )
        .await
        .expect("updating user should succeed");

        assert_eq!(updated.first_name.as_deref(), Some("Updated"));
        assert_eq!(updated.username, "updated_user");

        let authenticated = authenticate_user(LoginRequest {
            email: email.clone(),
            password,
        })
        .await
        .expect("authentication should succeed")
        .expect("credentials should match");
        assert_eq!(authenticated.id, created.id);

        let wrong_password = authenticate_user(LoginRequest {
            email: email.clone(),
            password: "badpassword".to_string(),
        })
        .await
        .expect("authentication should return Ok")
        .is_none();
        assert!(wrong_password);

        let deletion = delete_user(created.id.to_string())
            .await
            .expect("deleting user should succeed");
        assert_eq!(deletion, "User deleted successfully");

        let missing = get_user_by_id(created.id.to_string())
            .await
            .expect("fetch should succeed")
            .is_none();
        assert!(missing);

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn delete_user_reports_when_missing() -> AnyResult<()> {
        let pool = pool().await?;
        reset_all_tables(pool.as_ref()).await?;

        let response = delete_user(Uuid::new_v4().to_string()).await;
        assert!(matches!(response, Err(message) if message == "User not found"));
        Ok(())
    }
}
