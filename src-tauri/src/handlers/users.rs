use crate::database::get_pool_ref;
use crate::models::{CreateUser, LoginRequest, PublicUser, UpdateUser, User};
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

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
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Failed to fetch users: {}", e))?;

    Ok(users.into_iter().map(PublicUser::from).collect())
}

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
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Failed to fetch user: {}", e))?;

    Ok(user.map(PublicUser::from))
}

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
    .fetch_one(pool)
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
    .fetch_one(pool)
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
        .execute(pool)
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
    .fetch_optional(pool)
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
