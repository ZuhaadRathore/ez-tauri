//! Cache management command handlers.

use crate::cache;
use serde_json::Value;

/// Sets a value in the cache with optional time-to-live.
#[tauri::command]
pub async fn set_cache_value(key: String, value: Value, ttl_seconds: Option<u64>) -> Result<(), String> {
    cache::set_cache(&key, &value, ttl_seconds)
        .map_err(|e| format!("Failed to set cache: {}", e))
}

/// Retrieves a value from the cache by key.
#[tauri::command]
pub async fn get_cache_value(key: String) -> Result<Option<Value>, String> {
    cache::get_cache::<Value>(&key)
        .map_err(|e| format!("Failed to get cache: {}", e))
}

/// Deletes a value from the cache.
#[tauri::command]
pub async fn delete_cache_value(key: String) -> Result<(), String> {
    cache::delete_cache(&key)
        .map_err(|e| format!("Failed to delete cache: {}", e))
}

/// Checks if a key exists in the cache.
#[tauri::command]
pub async fn cache_key_exists(key: String) -> Result<bool, String> {
    cache::cache_exists(&key)
        .map_err(|e| format!("Failed to check cache: {}", e))
}

/// Returns whether the cache system is available.
#[tauri::command]
pub async fn is_cache_available() -> Result<bool, String> {
    Ok(cache::is_redis_available())
}