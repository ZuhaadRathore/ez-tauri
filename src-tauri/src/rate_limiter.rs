//! Rate limiting implementation using the Governor crate.
//!
//! This module provides both global and per-user rate limiting functionality
//! to protect the application from abuse and ensure fair resource usage.

use governor::{Quota, RateLimiter, Jitter};
use governor::state::{InMemoryState, NotKeyed, keyed::DashMapStateStore};
use governor::clock::QuantaClock;
use nonzero_ext::*;
use std::time::Duration;

/// Rate limiter for global application-wide limits.
pub type GlobalRateLimiter = RateLimiter<NotKeyed, InMemoryState, QuantaClock>;

/// Rate limiter for per-user limits, keyed by user ID.
pub type UserRateLimiter = RateLimiter<String, DashMapStateStore<String>, QuantaClock>;

/// Configuration for both global and per-user rate limiting.
///
/// Manages two types of rate limits:
/// - Global: Applies to all requests regardless of user
/// - Per-user: Applies per individual user to prevent single-user abuse
pub struct RateLimiterConfig {
    global_limiter: GlobalRateLimiter,
    user_limiter: UserRateLimiter,
    jitter: Jitter,
}

impl RateLimiterConfig {
    /// Creates a new rate limiter configuration with default limits.
    ///
    /// Default limits:
    /// - Global: 100 requests per minute
    /// - Per-user: 10 requests per minute
    pub fn new() -> Self {
        let global_quota = Quota::per_minute(nonzero!(100u32));
        let global_limiter = RateLimiter::direct(global_quota);

        let user_quota = Quota::per_minute(nonzero!(10u32));
        let user_limiter = RateLimiter::keyed(user_quota);

        let jitter = Jitter::up_to(Duration::from_millis(100));

        Self {
            global_limiter,
            user_limiter,
            jitter,
        }
    }

    /// Creates a new rate limiter configuration with custom limits.
    ///
    /// # Arguments
    /// * `global_per_minute` - Maximum requests per minute globally
    /// * `user_per_minute` - Maximum requests per minute per user
    pub fn new_with_limits(global_per_minute: u32, user_per_minute: u32) -> Self {
        let global_quota = Quota::per_minute(std::num::NonZeroU32::new(global_per_minute).unwrap_or(nonzero!(60u32)));
        let global_limiter = RateLimiter::direct(global_quota);

        let user_quota = Quota::per_minute(std::num::NonZeroU32::new(user_per_minute).unwrap_or(nonzero!(30u32)));
        let user_limiter = RateLimiter::keyed(user_quota);

        let jitter = Jitter::up_to(Duration::from_millis(100));

        Self {
            global_limiter,
            user_limiter,
            jitter,
        }
    }

    /// Checks if a request is within rate limits without blocking.
    ///
    /// # Arguments
    /// * `user_id` - Optional user identifier for per-user rate limiting
    ///
    /// # Returns
    /// * `Ok(())` if within limits
    /// * `Err(RateLimitError)` if limits exceeded
    pub async fn check_rate_limit(&self, user_id: Option<&str>) -> Result<(), RateLimitError> {
        match self.global_limiter.check() {
            Ok(_) => {},
            Err(_) => {
                tracing::warn!("Global rate limit exceeded");
                return Err(RateLimitError::GlobalLimitExceeded);
            }
        }

        if let Some(user_id) = user_id {
            match self.user_limiter.check_key(&user_id.to_string()) {
                Ok(_) => {},
                Err(_) => {
                    tracing::warn!("User rate limit exceeded for user: {}", user_id);
                    return Err(RateLimitError::UserLimitExceeded(user_id.to_string()));
                }
            }
        }

        Ok(())
    }

    /// Waits until the request is within rate limits before proceeding.
    ///
    /// Uses jitter to prevent thundering herd problems when multiple
    /// requests are waiting for rate limits to reset.
    ///
    /// # Arguments
    /// * `user_id` - Optional user identifier for per-user rate limiting
    pub async fn wait_for_rate_limit(&self, user_id: Option<&str>) -> Result<(), RateLimitError> {
        self.global_limiter.until_ready_with_jitter(self.jitter).await;

        if let Some(user_id) = user_id {
            self.user_limiter.until_key_ready_with_jitter(&user_id.to_string(), self.jitter).await;
        }

        Ok(())
    }

    /// Cleanup method for old rate limiter entries.
    ///
    /// Note: DashMapStateStore handles cleanup automatically,
    /// but this method is provided for compatibility.
    pub fn cleanup_old_limiters(&self) {
        tracing::debug!("Rate limiter cleanup called - handled automatically by DashMapStateStore");
    }
}

impl Default for RateLimiterConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors that can occur during rate limiting operations.
#[derive(Debug, Clone)]
pub enum RateLimitError {
    GlobalLimitExceeded,
    UserLimitExceeded(String),
}

impl std::fmt::Display for RateLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RateLimitError::GlobalLimitExceeded => {
                write!(f, "Global rate limit exceeded. Please try again later.")
            }
            RateLimitError::UserLimitExceeded(user_id) => {
                write!(f, "Rate limit exceeded for user {}. Please try again later.", user_id)
            }
        }
    }
}

impl std::error::Error for RateLimitError {}

/// Macro to wrap command handlers with rate limiting that fails fast.
///
/// This macro checks rate limits and immediately returns an error if limits are exceeded.
#[macro_export]
macro_rules! rate_limited_command {
    ($rate_limiter:expr, $user_id:expr, $command:expr) => {{
        match $rate_limiter.check_rate_limit($user_id).await {
            Ok(_) => $command.await,
            Err(e) => Err(format!("Rate limit error: {}", e)),
        }
    }};
}

/// Macro to wrap command handlers with rate limiting that waits for capacity.
///
/// This macro waits until rate limits allow the request to proceed, using jitter
/// to prevent thundering herd problems.
#[macro_export]
macro_rules! rate_limited_command_wait {
    ($rate_limiter:expr, $user_id:expr, $command:expr) => {{
        match $rate_limiter.wait_for_rate_limit($user_id).await {
            Ok(_) => $command.await,
            Err(e) => Err(format!("Rate limit error: {}", e)),
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_global_rate_limiting() {
        let limiter = RateLimiterConfig::new_with_limits(2, 1);

        // First two requests should pass
        assert!(limiter.check_rate_limit(None).await.is_ok());
        assert!(limiter.check_rate_limit(None).await.is_ok());

        // Third request should fail
        assert!(limiter.check_rate_limit(None).await.is_err());
    }

    #[tokio::test]
    async fn test_user_rate_limiting() {
        let limiter = RateLimiterConfig::new_with_limits(100, 1);

        // First request should pass
        assert!(limiter.check_rate_limit(Some("user1")).await.is_ok());

        // Second request from same user should fail
        assert!(limiter.check_rate_limit(Some("user1")).await.is_err());

        // Request from different user should pass
        assert!(limiter.check_rate_limit(Some("user2")).await.is_ok());
    }

    #[tokio::test]
    async fn test_rate_limit_recovery() {
        let limiter = RateLimiterConfig::new_with_limits(60, 60); // 1 per second

        // First request should pass
        assert!(limiter.check_rate_limit(None).await.is_ok());

        // Second request should fail immediately
        assert!(limiter.check_rate_limit(None).await.is_err());

        // Wait for rate limit to reset
        sleep(Duration::from_secs(2)).await;

        // Request should now pass
        assert!(limiter.check_rate_limit(None).await.is_ok());
    }
}