//! Database migration management for creating and maintaining schema.

use anyhow::Result;
use sqlx::PgPool;

/// Runs all database migrations to set up the application schema.
///
/// Creates tables for users, user settings, and application logs along with
/// necessary indexes for performance. In production, consider using sqlx-cli
/// for more sophisticated migration management.
pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    let migrations = [
        r#"CREATE EXTENSION IF NOT EXISTS "uuid-ossp""#,

        r#"CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            email VARCHAR(255) UNIQUE NOT NULL,
            username VARCHAR(100) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            first_name VARCHAR(100),
            last_name VARCHAR(100),
            is_active BOOLEAN DEFAULT true,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )"#,

        r#"CREATE TABLE IF NOT EXISTS user_settings (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            theme VARCHAR(20) DEFAULT 'light',
            language VARCHAR(10) DEFAULT 'en',
            notifications_enabled BOOLEAN DEFAULT true,
            settings_data JSONB DEFAULT '{}',
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(user_id)
        )"#,

        r#"CREATE TABLE IF NOT EXISTS app_logs (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            level VARCHAR(20) NOT NULL,
            message TEXT NOT NULL,
            metadata JSONB DEFAULT '{}',
            user_id UUID REFERENCES users(id) ON DELETE SET NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )"#,

        r#"CREATE INDEX IF NOT EXISTS idx_users_email ON users(email)"#,
        r#"CREATE INDEX IF NOT EXISTS idx_users_username ON users(username)"#,
        r#"CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at)"#,
        r#"CREATE INDEX IF NOT EXISTS idx_user_settings_user_id ON user_settings(user_id)"#,
        r#"CREATE INDEX IF NOT EXISTS idx_app_logs_level ON app_logs(level)"#,
        r#"CREATE INDEX IF NOT EXISTS idx_app_logs_created_at ON app_logs(created_at)"#,
        r#"CREATE INDEX IF NOT EXISTS idx_app_logs_user_id ON app_logs(user_id)"#,
    ];

    for migration in migrations {
        sqlx::query(migration).execute(pool).await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::test_utils::pool;
    use anyhow::Result as AnyResult;
    use serial_test::serial;
    use sqlx::Row;

    #[tokio::test]
    #[serial]
    async fn migrations_create_all_required_tables() -> AnyResult<()> {
        let pool = pool().await?;
        // Start with a clean slate
        sqlx::query("DROP SCHEMA public CASCADE")
            .execute(pool.as_ref())
            .await?;
        sqlx::query("CREATE SCHEMA public")
            .execute(pool.as_ref())
            .await?;

        // Run migrations
        run_migrations(pool.as_ref()).await?;

        // Check that all expected tables exist
        let tables: Vec<String> = sqlx::query(
            "SELECT table_name FROM information_schema.tables
             WHERE table_schema = 'public' AND table_type = 'BASE TABLE'
             ORDER BY table_name"
        )
        .fetch_all(pool.as_ref())
        .await?
        .into_iter()
        .map(|row| row.get::<String, _>(0))
        .collect();

        let expected_tables = vec!["app_logs", "user_settings", "users"];
        assert_eq!(tables, expected_tables);

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn migrations_create_required_indexes() -> AnyResult<()> {
        let pool = pool().await?;
        sqlx::query("DROP SCHEMA public CASCADE")
            .execute(pool.as_ref())
            .await?;
        sqlx::query("CREATE SCHEMA public")
            .execute(pool.as_ref())
            .await?;

        run_migrations(pool.as_ref()).await?;

        // Check that expected indexes exist (filter out auto-created constraint indexes)
        let indexes: Vec<String> = sqlx::query(
            "SELECT indexname FROM pg_indexes
             WHERE schemaname = 'public'
               AND indexname NOT LIKE '%_pkey'
               AND indexname NOT LIKE '%_key'
             ORDER BY indexname"
        )
        .fetch_all(pool.as_ref())
        .await?
        .into_iter()
        .map(|row| row.get::<String, _>(0))
        .collect();

        let expected_indexes = vec![
            "idx_app_logs_created_at",
            "idx_app_logs_level",
            "idx_app_logs_user_id",
            "idx_user_settings_user_id",
            "idx_users_created_at",
            "idx_users_email",
            "idx_users_username",
        ];

        assert_eq!(indexes, expected_indexes);

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn migrations_are_idempotent() -> AnyResult<()> {
        let pool = pool().await?;
        sqlx::query("DROP SCHEMA public CASCADE")
            .execute(pool.as_ref())
            .await?;
        sqlx::query("CREATE SCHEMA public")
            .execute(pool.as_ref())
            .await?;

        // Run migrations multiple times
        run_migrations(pool.as_ref()).await?;
        run_migrations(pool.as_ref()).await?;
        run_migrations(pool.as_ref()).await?;

        // Verify tables still exist and structure is correct
        let table_count: i64 = sqlx::query(
            "SELECT COUNT(*) FROM information_schema.tables
             WHERE table_schema = 'public' AND table_type = 'BASE TABLE'"
        )
        .fetch_one(pool.as_ref())
        .await?
        .get(0);

        assert_eq!(table_count, 3);

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn users_table_has_correct_structure() -> AnyResult<()> {
        let pool = pool().await?;
        sqlx::query("DROP SCHEMA public CASCADE")
            .execute(pool.as_ref())
            .await?;
        sqlx::query("CREATE SCHEMA public")
            .execute(pool.as_ref())
            .await?;

        run_migrations(pool.as_ref()).await?;

        // Check users table structure
        let columns: Vec<(String, String, String)> = sqlx::query(
            "SELECT column_name, data_type, is_nullable
             FROM information_schema.columns
             WHERE table_name = 'users' AND table_schema = 'public'
             ORDER BY ordinal_position"
        )
        .fetch_all(pool.as_ref())
        .await?
        .into_iter()
        .map(|row| (
            row.get::<String, _>(0),
            row.get::<String, _>(1),
            row.get::<String, _>(2)
        ))
        .collect();

        let expected_structure = vec![
            ("id".to_string(), "uuid".to_string(), "NO".to_string()),
            ("email".to_string(), "character varying".to_string(), "NO".to_string()),
            ("username".to_string(), "character varying".to_string(), "NO".to_string()),
            ("password_hash".to_string(), "character varying".to_string(), "NO".to_string()),
            ("first_name".to_string(), "character varying".to_string(), "YES".to_string()),
            ("last_name".to_string(), "character varying".to_string(), "YES".to_string()),
            ("is_active".to_string(), "boolean".to_string(), "YES".to_string()),
            ("created_at".to_string(), "timestamp with time zone".to_string(), "YES".to_string()),
            ("updated_at".to_string(), "timestamp with time zone".to_string(), "YES".to_string()),
        ];

        assert_eq!(columns, expected_structure);

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn user_settings_table_has_foreign_key_constraint() -> AnyResult<()> {
        let pool = pool().await?;
        sqlx::query("DROP SCHEMA public CASCADE")
            .execute(pool.as_ref())
            .await?;
        sqlx::query("CREATE SCHEMA public")
            .execute(pool.as_ref())
            .await?;

        run_migrations(pool.as_ref()).await?;

        // Test foreign key constraint by trying to insert invalid user_id
        let invalid_uuid = uuid::Uuid::new_v4();
        let result = sqlx::query(
            "INSERT INTO user_settings (user_id, theme) VALUES ($1, 'dark')"
        )
        .bind(invalid_uuid)
        .execute(pool.as_ref())
        .await;

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("foreign key constraint") || error_msg.contains("violates"));

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn app_logs_table_allows_null_user_id() -> AnyResult<()> {
        let pool = pool().await?;
        sqlx::query("DROP SCHEMA public CASCADE")
            .execute(pool.as_ref())
            .await?;
        sqlx::query("CREATE SCHEMA public")
            .execute(pool.as_ref())
            .await?;

        run_migrations(pool.as_ref()).await?;

        // Insert log entry without user_id (should succeed)
        let result = sqlx::query(
            "INSERT INTO app_logs (level, message) VALUES ('info', 'test message')"
        )
        .execute(pool.as_ref())
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().rows_affected(), 1);

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn uuid_extension_is_available() -> AnyResult<()> {
        let pool = pool().await?;
        sqlx::query("DROP SCHEMA public CASCADE")
            .execute(pool.as_ref())
            .await?;
        sqlx::query("CREATE SCHEMA public")
            .execute(pool.as_ref())
            .await?;

        run_migrations(pool.as_ref()).await?;

        // Test that uuid_generate_v4() function is available
        let uuid_result: uuid::Uuid = sqlx::query("SELECT uuid_generate_v4() as id")
            .fetch_one(pool.as_ref())
            .await?
            .get("id");

        assert!(!uuid_result.is_nil());

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn unique_constraints_are_enforced() -> AnyResult<()> {
        let pool = pool().await?;
        sqlx::query("DROP SCHEMA public CASCADE")
            .execute(pool.as_ref())
            .await?;
        sqlx::query("CREATE SCHEMA public")
            .execute(pool.as_ref())
            .await?;

        run_migrations(pool.as_ref()).await?;

        // Insert first user
        let user_id: uuid::Uuid = sqlx::query(
            "INSERT INTO users (email, username, password_hash)
             VALUES ('test@example.com', 'testuser', 'hashedpass')
             RETURNING id"
        )
        .fetch_one(pool.as_ref())
        .await?
        .get("id");

        // Try to insert user with duplicate email (should fail)
        let duplicate_email_result = sqlx::query(
            "INSERT INTO users (email, username, password_hash)
             VALUES ('test@example.com', 'different_user', 'hashedpass')"
        )
        .execute(pool.as_ref())
        .await;

        assert!(duplicate_email_result.is_err());

        // Try to insert user with duplicate username (should fail)
        let duplicate_username_result = sqlx::query(
            "INSERT INTO users (email, username, password_hash)
             VALUES ('different@example.com', 'testuser', 'hashedpass')"
        )
        .execute(pool.as_ref())
        .await;

        assert!(duplicate_username_result.is_err());

        // User_settings should also enforce unique user_id constraint
        sqlx::query("INSERT INTO user_settings (user_id) VALUES ($1)")
            .bind(user_id)
            .execute(pool.as_ref())
            .await?;

        let duplicate_user_settings_result = sqlx::query(
            "INSERT INTO user_settings (user_id) VALUES ($1)"
        )
        .bind(user_id)
        .execute(pool.as_ref())
        .await;

        assert!(duplicate_user_settings_result.is_err());

        Ok(())
    }
}
