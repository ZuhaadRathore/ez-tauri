//! Redis caching functionality with graceful fallback when unavailable.

use anyhow::Result;
use redis::{Client, Connection};
use std::sync::Mutex;

/// Redis cache wrapper for Tauri State management.
///
/// This struct provides thread-safe access to Redis caching with graceful
/// degradation when Redis is unavailable.
pub struct RedisCache {
    client: Option<Client>,
    connection: Mutex<Option<Connection>>,
}

impl RedisCache {
    /// Creates a new RedisCache instance.
    ///
    /// If redis_url is None, the cache will operate in disabled mode,
    /// allowing the application to function without caching.
    pub fn new(redis_url: Option<String>) -> Self {
        if let Some(url) = redis_url {
            match Client::open(url.as_str()) {
                Ok(client) => {
                    match client.get_connection() {
                        Ok(connection) => {
                            tracing::info!("Redis initialized successfully");
                            return Self {
                                client: Some(client),
                                connection: Mutex::new(Some(connection)),
                            };
                        }
                        Err(e) => {
                            tracing::warn!("Failed to connect to Redis: {}. Continuing without caching.", e);
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to create Redis client: {}. Continuing without caching.", e);
                }
            }
        } else {
            tracing::info!("Redis not configured - running without caching");
        }

        Self {
            client: None,
            connection: Mutex::new(None),
        }
    }

    /// Checks if Redis is available for caching operations.
    pub fn is_available(&self) -> bool {
        self.client.is_some()
    }

    /// Sets a value in the cache with optional TTL (time-to-live).
    ///
    /// Silently succeeds if Redis is unavailable, allowing the application
    /// to continue functioning without caching.
    pub fn set<T: serde::Serialize>(&self, key: &str, value: &T, ttl_seconds: Option<u64>) -> Result<()> {
        if !self.is_available() {
            return Ok(());
        }

        let mut connection = self.connection.lock().unwrap();

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
    pub fn get<T: for<'de> serde::Deserialize<'de>>(&self, key: &str) -> Result<Option<T>> {
        if !self.is_available() {
            return Ok(None);
        }

        let mut connection = self.connection.lock().unwrap();

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
    pub fn delete(&self, key: &str) -> Result<()> {
        if !self.is_available() {
            return Ok(());
        }

        let mut connection = self.connection.lock().unwrap();

        if let Some(ref mut conn) = *connection {
            redis::cmd("DEL")
                .arg(key)
                .execute(conn);
        }

        Ok(())
    }

    /// Checks if a key exists in the cache.
    pub fn exists(&self, key: &str) -> Result<bool> {
        if !self.is_available() {
            return Ok(false);
        }

        let mut connection = self.connection.lock().unwrap();

        if let Some(ref mut conn) = *connection {
            let result: bool = redis::cmd("EXISTS")
                .arg(key)
                .query(conn)?;
            return Ok(result);
        }

        Ok(false)
    }
}