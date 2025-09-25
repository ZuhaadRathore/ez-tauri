use regex::Regex;
use std::sync::LazyLock;

/// Input validation utilities for preventing security vulnerabilities.
///
/// This module provides server-side validation to complement frontend sanitization,
/// protecting against XSS, injection attacks, and malformed input data.
/// Email validation regex pattern compiled once for performance.
static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
});

/// Username validation regex pattern (3-50 alphanumeric chars and underscores).
static USERNAME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z0-9_]{3,50}$").unwrap()
});

/// Name validation regex pattern (letters, spaces, apostrophes, hyphens).
static NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z\s'-]{1,100}$").unwrap()
});

/// Dangerous patterns that indicate potential XSS or injection attacks.
static DANGEROUS_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        Regex::new(r"(?i)<script").unwrap(),
        Regex::new(r"(?i)javascript:").unwrap(),
        Regex::new(r"(?i)on\w+=").unwrap(),
        Regex::new(r"(?i)<iframe").unwrap(),
        Regex::new(r"(?i)<object").unwrap(),
        Regex::new(r"(?i)<embed").unwrap(),
        Regex::new(r"(?i)data:text/html").unwrap(),
    ]
});

/// Validation errors that can occur during input validation.
#[derive(Debug)]
pub enum ValidationError {
    InvalidEmail,
    InvalidUsername,
    InvalidName,
    TooLong(usize),
    ContainsDangerousContent,
    Empty,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidEmail => write!(f, "Invalid email format"),
            ValidationError::InvalidUsername => write!(f, "Username must be 3-50 chars, alphanumeric and underscores only"),
            ValidationError::InvalidName => write!(f, "Name contains invalid characters"),
            ValidationError::TooLong(max) => write!(f, "Input exceeds maximum length of {}", max),
            ValidationError::ContainsDangerousContent => write!(f, "Input contains potentially dangerous content"),
            ValidationError::Empty => write!(f, "Required field cannot be empty"),
        }
    }
}

impl std::error::Error for ValidationError {}

/// Validate and sanitize email addresses
pub fn validate_email(email: &str) -> Result<String, ValidationError> {
    if email.trim().is_empty() {
        return Err(ValidationError::Empty);
    }

    let email = email.trim().to_lowercase();

    if email.len() > 254 {
        return Err(ValidationError::TooLong(254));
    }

    if !EMAIL_REGEX.is_match(&email) {
        return Err(ValidationError::InvalidEmail);
    }

    check_dangerous_content(&email)?;

    Ok(email)
}

/// Validate and sanitize usernames
pub fn validate_username(username: &str) -> Result<String, ValidationError> {
    if username.trim().is_empty() {
        return Err(ValidationError::Empty);
    }

    let username = username.trim();

    if !USERNAME_REGEX.is_match(username) {
        return Err(ValidationError::InvalidUsername);
    }

    check_dangerous_content(username)?;

    Ok(username.to_string())
}

/// Validate and sanitize name fields
pub fn validate_name(name: &str) -> Result<String, ValidationError> {
    if name.trim().is_empty() {
        return Err(ValidationError::Empty);
    }

    let name = name.trim();

    if name.len() > 100 {
        return Err(ValidationError::TooLong(100));
    }

    if !NAME_REGEX.is_match(name) {
        return Err(ValidationError::InvalidName);
    }

    check_dangerous_content(name)?;

    Ok(name.to_string())
}

/// Validate optional name fields
pub fn validate_optional_name(name: Option<&str>) -> Result<Option<String>, ValidationError> {
    match name {
        Some(n) if !n.trim().is_empty() => Ok(Some(validate_name(n)?)),
        _ => Ok(None),
    }
}

/// Validate log levels
pub fn validate_log_level(level: &str) -> Result<String, ValidationError> {
    let level = level.trim().to_lowercase();

    match level.as_str() {
        "error" | "warn" | "info" | "debug" | "trace" => Ok(level),
        _ => Ok("info".to_string()), // Default to info for invalid levels
    }
}

/// Validate and limit log messages
pub fn validate_log_message(message: &str) -> Result<String, ValidationError> {
    if message.trim().is_empty() {
        return Err(ValidationError::Empty);
    }

    let message = message.trim();

    if message.len() > 1000 {
        // Truncate long messages rather than reject them
        Ok(message.chars().take(1000).collect())
    } else {
        // Still check for dangerous content in log messages
        check_dangerous_content(message)?;
        Ok(message.to_string())
    }
}

/// Checks if input contains potentially dangerous content patterns.
///
/// Scans for common XSS and injection patterns including script tags,
/// javascript URLs, and event handlers.
fn check_dangerous_content(input: &str) -> Result<(), ValidationError> {
    for pattern in DANGEROUS_PATTERNS.iter() {
        if pattern.is_match(input) {
            return Err(ValidationError::ContainsDangerousContent);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validation() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("  USER@EXAMPLE.COM  ").unwrap() == "user@example.com");
        assert!(validate_email("invalid-email").is_err());
        assert!(validate_email("<script>alert('xss')</script>@example.com").is_err());
        assert!(validate_email("").is_err());
    }

    #[test]
    fn test_username_validation() {
        assert!(validate_username("valid_user123").is_ok());
        assert!(validate_username("ab").is_err()); // Too short
        assert!(validate_username("user-with-dashes").is_err()); // Invalid chars
        assert!(validate_username("<script>alert('xss')</script>").is_err());
        assert!(validate_username("").is_err());
    }

    #[test]
    fn test_name_validation() {
        assert!(validate_name("John Doe").is_ok());
        assert!(validate_name("Mary O'Connor").is_ok());
        assert!(validate_name("Jean-Pierre").is_ok());
        assert!(validate_name("<script>alert('xss')</script>").is_err());
        assert!(validate_name("").is_err());

        // Test length limit
        let long_name = "a".repeat(101);
        assert!(validate_name(&long_name).is_err());
    }

    #[test]
    fn test_log_level_validation() {
        assert_eq!(validate_log_level("ERROR").unwrap(), "error");
        assert_eq!(validate_log_level("invalid").unwrap(), "info");
        assert_eq!(validate_log_level("debug").unwrap(), "debug");
    }

    #[test]
    fn test_log_message_validation() {
        assert!(validate_log_message("Normal log message").is_ok());

        // Test length truncation
        let long_message = "a".repeat(1001);
        let result = validate_log_message(&long_message).unwrap();
        assert_eq!(result.len(), 1000);

        assert!(validate_log_message("<script>alert('xss')</script>").is_err());
        assert!(validate_log_message("").is_err());
    }

    #[test]
    fn test_dangerous_content_detection() {
        let dangerous_inputs = vec![
            "<script>alert('xss')</script>",
            "javascript:alert('xss')",
            "onclick=alert('xss')",
            "<iframe src='evil.com'></iframe>",
            "data:text/html,<script>alert('xss')</script>",
        ];

        for input in dangerous_inputs {
            assert!(check_dangerous_content(input).is_err());
        }

        let safe_inputs = vec![
            "Normal text",
            "user@example.com",
            "Some numbers: 123",
            "Special chars: !@#$%^&*()",
        ];

        for input in safe_inputs {
            assert!(check_dangerous_content(input).is_ok());
        }
    }
}