use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ErrorCode {
    // Database errors
    DatabaseConnection,
    DatabaseQuery,
    DatabaseMigration,
    DatabaseTimeout,

    // Validation errors
    ValidationError,
    InvalidInput,
    MissingField,
    InvalidFormat,

    // Authentication/Authorization errors
    AuthenticationFailed,
    Unauthorized,
    Forbidden,
    TokenExpired,

    // File system errors
    FileNotFound,
    FilePermission,
    FileRead,
    FileWrite,
    DirectoryCreate,

    // Network/External service errors
    NetworkError,
    ExternalServiceUnavailable,
    RequestTimeout,

    // Cache errors
    CacheConnection,
    CacheOperation,

    // Configuration errors
    ConfigurationError,
    EnvironmentError,

    // System errors
    SystemError,
    ResourceExhausted,
    PermissionDenied,

    // Generic errors
    InternalError,
    NotImplemented,
    Unknown,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code_str = match self {
            ErrorCode::DatabaseConnection => "DATABASE_CONNECTION",
            ErrorCode::DatabaseQuery => "DATABASE_QUERY",
            ErrorCode::DatabaseMigration => "DATABASE_MIGRATION",
            ErrorCode::DatabaseTimeout => "DATABASE_TIMEOUT",
            ErrorCode::ValidationError => "VALIDATION_ERROR",
            ErrorCode::InvalidInput => "INVALID_INPUT",
            ErrorCode::MissingField => "MISSING_FIELD",
            ErrorCode::InvalidFormat => "INVALID_FORMAT",
            ErrorCode::AuthenticationFailed => "AUTHENTICATION_FAILED",
            ErrorCode::Unauthorized => "UNAUTHORIZED",
            ErrorCode::Forbidden => "FORBIDDEN",
            ErrorCode::TokenExpired => "TOKEN_EXPIRED",
            ErrorCode::FileNotFound => "FILE_NOT_FOUND",
            ErrorCode::FilePermission => "FILE_PERMISSION",
            ErrorCode::FileRead => "FILE_READ",
            ErrorCode::FileWrite => "FILE_WRITE",
            ErrorCode::DirectoryCreate => "DIRECTORY_CREATE",
            ErrorCode::NetworkError => "NETWORK_ERROR",
            ErrorCode::ExternalServiceUnavailable => "EXTERNAL_SERVICE_UNAVAILABLE",
            ErrorCode::RequestTimeout => "REQUEST_TIMEOUT",
            ErrorCode::CacheConnection => "CACHE_CONNECTION",
            ErrorCode::CacheOperation => "CACHE_OPERATION",
            ErrorCode::ConfigurationError => "CONFIGURATION_ERROR",
            ErrorCode::EnvironmentError => "ENVIRONMENT_ERROR",
            ErrorCode::SystemError => "SYSTEM_ERROR",
            ErrorCode::ResourceExhausted => "RESOURCE_EXHAUSTED",
            ErrorCode::PermissionDenied => "PERMISSION_DENIED",
            ErrorCode::InternalError => "INTERNAL_ERROR",
            ErrorCode::NotImplemented => "NOT_IMPLEMENTED",
            ErrorCode::Unknown => "UNKNOWN",
        };
        write!(f, "{}", code_str)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppError {
    pub code: ErrorCode,
    pub message: String,
    pub details: Option<String>,
    pub context: Option<serde_json::Value>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub request_id: Option<String>,
}

impl AppError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            details: None,
            context: None,
            timestamp: chrono::Utc::now(),
            request_id: None,
        }
    }

    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    pub fn with_context<T: Serialize>(mut self, context: T) -> Self {
        self.context = serde_json::to_value(context).ok();
        self
    }

    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    // Convenience constructors for common error types
    pub fn database_error(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::DatabaseQuery, message)
    }

    pub fn validation_error(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::ValidationError, message)
    }

    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::new(ErrorCode::FileNotFound, format!("{} not found", resource.into()))
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Unauthorized, message)
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Forbidden, message)
    }

    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::InternalError, message)
    }

    pub fn invalid_input(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(ErrorCode::InvalidInput, message)
            .with_context(serde_json::json!({ "field": field.into() }))
    }

    pub fn file_error(operation: impl Into<String>, path: impl Into<String>, message: impl Into<String>) -> Self {
        let op = operation.into();
        let error_code = match op.as_str() {
            "read" => ErrorCode::FileRead,
            "write" => ErrorCode::FileWrite,
            "create" => ErrorCode::DirectoryCreate,
            "permission" => ErrorCode::FilePermission,
            _ => ErrorCode::SystemError,
        };

        Self::new(error_code, message)
            .with_context(serde_json::json!({
                "operation": op,
                "path": path.into()
            }))
    }

    pub fn cache_error(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::CacheOperation, message)
    }

    /// Convert to a user-friendly error message
    pub fn user_message(&self) -> String {
        match &self.code {
            ErrorCode::DatabaseConnection | ErrorCode::DatabaseTimeout => {
                "Database is temporarily unavailable. Please try again later.".to_string()
            }
            ErrorCode::ValidationError | ErrorCode::InvalidInput | ErrorCode::InvalidFormat => {
                "Invalid input provided. Please check your data and try again.".to_string()
            }
            ErrorCode::MissingField => {
                "Required information is missing. Please fill in all required fields.".to_string()
            }
            ErrorCode::AuthenticationFailed => {
                "Authentication failed. Please check your credentials.".to_string()
            }
            ErrorCode::Unauthorized => {
                "You are not authorized to perform this action. Please log in.".to_string()
            }
            ErrorCode::Forbidden => {
                "You don't have permission to access this resource.".to_string()
            }
            ErrorCode::TokenExpired => {
                "Your session has expired. Please log in again.".to_string()
            }
            ErrorCode::FileNotFound => {
                "The requested file or resource was not found.".to_string()
            }
            ErrorCode::FilePermission => {
                "Permission denied. Unable to access the file or directory.".to_string()
            }
            ErrorCode::NetworkError | ErrorCode::ExternalServiceUnavailable => {
                "Network error occurred. Please check your connection and try again.".to_string()
            }
            ErrorCode::RequestTimeout => {
                "The request timed out. Please try again.".to_string()
            }
            ErrorCode::CacheConnection | ErrorCode::CacheOperation => {
                "Cache service is temporarily unavailable. Some features may be slower.".to_string()
            }
            ErrorCode::ResourceExhausted => {
                "System resources are currently exhausted. Please try again later.".to_string()
            }
            ErrorCode::NotImplemented => {
                "This feature is not yet implemented.".to_string()
            }
            _ => {
                "An unexpected error occurred. Please try again later.".to_string()
            }
        }
    }

    /// Check if this error should be retried
    pub fn is_retryable(&self) -> bool {
        matches!(
            self.code,
            ErrorCode::DatabaseTimeout
                | ErrorCode::NetworkError
                | ErrorCode::ExternalServiceUnavailable
                | ErrorCode::RequestTimeout
                | ErrorCode::CacheConnection
                | ErrorCode::ResourceExhausted
        )
    }

    /// Check if this error should be logged
    pub fn should_log(&self) -> bool {
        !matches!(
            self.code,
            ErrorCode::ValidationError
                | ErrorCode::InvalidInput
                | ErrorCode::MissingField
                | ErrorCode::InvalidFormat
                | ErrorCode::Unauthorized
                | ErrorCode::Forbidden
        )
    }

    /// Get the appropriate log level for this error
    pub fn log_level(&self) -> tracing::Level {
        match self.code {
            ErrorCode::ValidationError
            | ErrorCode::InvalidInput
            | ErrorCode::MissingField
            | ErrorCode::InvalidFormat
            | ErrorCode::Unauthorized
            | ErrorCode::Forbidden => tracing::Level::WARN,

            ErrorCode::DatabaseTimeout
            | ErrorCode::NetworkError
            | ErrorCode::RequestTimeout
            | ErrorCode::CacheConnection => tracing::Level::WARN,

            ErrorCode::InternalError
            | ErrorCode::SystemError
            | ErrorCode::ResourceExhausted
            | ErrorCode::DatabaseConnection
            | ErrorCode::ConfigurationError => tracing::Level::ERROR,

            _ => tracing::Level::ERROR,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for AppError {}

// Trait for converting errors to AppError
pub trait IntoAppError<T> {
    fn into_app_error(self, code: ErrorCode) -> Result<T, AppError>;
    fn with_app_context<C: Serialize>(self, code: ErrorCode, context: C) -> Result<T, AppError>;
}

impl<T, E: fmt::Display> IntoAppError<T> for Result<T, E> {
    fn into_app_error(self, code: ErrorCode) -> Result<T, AppError> {
        self.map_err(|e| AppError::new(code, e.to_string()))
    }

    fn with_app_context<C: Serialize>(self, code: ErrorCode, context: C) -> Result<T, AppError> {
        self.map_err(|e| {
            AppError::new(code, e.to_string())
                .with_context(context)
        })
    }
}

// Convenient result type alias
pub type AppResult<T> = Result<T, AppError>;

// Macros for error handling
#[macro_export]
macro_rules! app_error {
    ($code:expr, $msg:literal) => {
        $crate::errors::AppError::new($code, $msg)
    };
    ($code:expr, $msg:literal, $($arg:tt)*) => {
        $crate::errors::AppError::new($code, format!($msg, $($arg)*))
    };
}

#[macro_export]
macro_rules! bail {
    ($code:expr, $msg:literal) => {
        return Err($crate::errors::AppError::new($code, $msg))
    };
    ($code:expr, $msg:literal, $($arg:tt)*) => {
        return Err($crate::errors::AppError::new($code, format!($msg, $($arg)*)))
    };
}

#[macro_export]
macro_rules! ensure {
    ($cond:expr, $code:expr, $msg:literal) => {
        if !$cond {
            return Err($crate::errors::AppError::new($code, $msg));
        }
    };
    ($cond:expr, $code:expr, $msg:literal, $($arg:tt)*) => {
        if !$cond {
            return Err($crate::errors::AppError::new($code, format!($msg, $($arg)*)));
        }
    };
}