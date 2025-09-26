//! Cache module
//!
//! Provides Redis-based caching with graceful fallbacks.

pub mod handlers;

pub use handlers::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheConfig {
    pub redis_url: String,
    pub default_ttl: u64,
    pub max_memory_mb: u32,
    pub enable_fallback: bool,
    pub compression_enabled: bool,
    pub key_prefix: String,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            redis_url: "redis://localhost:6379".to_string(),
            default_ttl: 3600,
            max_memory_mb: 128,
            enable_fallback: true,
            compression_enabled: false,
            key_prefix: "ez-tauri:".to_string(),
        }
    }
}

/// Initialize the cache module
pub fn init() {
    tracing::info!("Cache module initialized");
}