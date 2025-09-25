//! Redis caching functionality with graceful fallback when unavailable.

use anyhow::Result;
use once_cell::sync::OnceCell;
use redis::{Client, Connection};
use std::sync::Mutex;
use crate::config::AppConfig;

/// Global Redis client instance.
static REDIS_CLIENT: OnceCell<Option<Client>> = OnceCell::new();

/// Global Redis connection wrapped in a mutex for thread safety.
static REDIS_CONNECTION: OnceCell<Mutex<Option<Connection>>> = OnceCell::new();

/// Initializes Redis connection if configured, otherwise runs without caching.
pub fn initialize_redis() -> Result<()> {
    let config = AppConfig::from_env();

    if let Some(redis_url) = &config.redis_url {
        let client = Client::open(redis_url.as_str())?;
        let connection = client.get_connection()?;

        REDIS_CLIENT.set(Some(client)).map_err(|_| anyhow::anyhow!("Failed to set Redis client"))?;
        REDIS_CONNECTION.set(Mutex::new(Some(connection))).map_err(|_| anyhow::anyhow!("Failed to set Redis connection"))?;

        tracing::info!("Redis initialized successfully");
    } else {
        REDIS_CLIENT.set(None).map_err(|_| anyhow::anyhow!("Failed to set Redis client"))?;
        REDIS_CONNECTION.set(Mutex::new(None)).map_err(|_| anyhow::anyhow!("Failed to set Redis connection"))?;

        tracing::info!("Redis not configured - running without caching");
    }

    Ok(())
}

/// Checks if Redis is available for caching operations.
pub fn is_redis_available() -> bool {
    REDIS_CLIENT.get().map_or(false, |client| client.is_some())
}

/// Sets a value in the cache with optional TTL (time-to-live).
///
/// Silently succeeds if Redis is unavailable, allowing the application
/// to continue functioning without caching.
pub fn set_cache<T: serde::Serialize>(key: &str, value: &T, ttl_seconds: Option<u64>) -> Result<()> {
    if !is_redis_available() {
        return Ok(());
    }

    let connection_guard = REDIS_CONNECTION.get()
        .ok_or_else(|| anyhow::anyhow!("Redis not initialized"))?;

    let mut connection = connection_guard.lock().unwrap();

    if let Some(ref mut conn) = *connection {
        let serialized = serde_json::to_string(value)?;

        if let Some(ttl) = ttl_seconds {
            redis::cmd("SETEX")
                .arg(key)
                .arg(ttl)
                .arg(serialized)
                .execute(conn);
        } else {
            redis::cmd("SET")
                .arg(key)
                .arg(serialized)
                .execute(conn);
        }
    }

    Ok(())
}

/// Retrieves a value from the cache, returning None if not found or Redis unavailable.
pub fn get_cache<T: for<'de> serde::Deserialize<'de>>(key: &str) -> Result<Option<T>> {
    if !is_redis_available() {
        return Ok(None);
    }

    let connection_guard = REDIS_CONNECTION.get()
        .ok_or_else(|| anyhow::anyhow!("Redis not initialized"))?;

    let mut connection = connection_guard.lock().unwrap();

    if let Some(ref mut conn) = *connection {
        let result: Option<String> = redis::cmd("GET")
            .arg(key)
            .query(conn)?;

        if let Some(serialized) = result {
            let deserialized: T = serde_json::from_str(&serialized)?;
            return Ok(Some(deserialized));
        }
    }

    Ok(None)
}

/// Deletes a key from the cache.
pub fn delete_cache(key: &str) -> Result<()> {
    if !is_redis_available() {
        return Ok(());
    }

    let connection_guard = REDIS_CONNECTION.get()
        .ok_or_else(|| anyhow::anyhow!("Redis not initialized"))?;

    let mut connection = connection_guard.lock().unwrap();

    if let Some(ref mut conn) = *connection {
        redis::cmd("DEL")
            .arg(key)
            .execute(conn);
    }

    Ok(())
}

/// Checks if a key exists in the cache.
pub fn cache_exists(key: &str) -> Result<bool> {
    if !is_redis_available() {
        return Ok(false);
    }

    let connection_guard = REDIS_CONNECTION.get()
        .ok_or_else(|| anyhow::anyhow!("Redis not initialized"))?;

    let mut connection = connection_guard.lock().unwrap();

    if let Some(ref mut conn) = *connection {
        let result: bool = redis::cmd("EXISTS")
            .arg(key)
            .query(conn)?;
        return Ok(result);
    }

    Ok(false)
}