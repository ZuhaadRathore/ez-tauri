//! Tests for rate limiting functionality.

use crate::rate_limiter::RateLimiterConfig;

#[tokio::test]
async fn test_global_rate_limiting() {
    let limiter = RateLimiterConfig::new_with_limits(2, 1);

    assert!(limiter.check_rate_limit(None).await.is_ok());
    assert!(limiter.check_rate_limit(None).await.is_ok());

    assert!(limiter.check_rate_limit(None).await.is_err());
}

#[tokio::test]
async fn test_user_rate_limiting() {
    let limiter = RateLimiterConfig::new_with_limits(100, 1);

    assert!(limiter.check_rate_limit(Some("user1")).await.is_ok());

    assert!(limiter.check_rate_limit(Some("user1")).await.is_err());

    assert!(limiter.check_rate_limit(Some("user2")).await.is_ok());
}

#[tokio::test]
async fn test_rate_limit_status() {
    let limiter = RateLimiterConfig::new();

    assert!(limiter.check_rate_limit(None).await.is_ok());
}