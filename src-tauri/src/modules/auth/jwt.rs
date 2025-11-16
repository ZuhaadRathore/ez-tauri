//! JWT token generation and validation

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// JWT claims structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// User email
    pub email: String,
    /// Issued at (timestamp)
    pub iat: i64,
    /// Expiration time (timestamp)
    pub exp: i64,
    /// Token type (access or refresh)
    pub token_type: TokenType,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    Access,
    Refresh,
}

/// JWT service for token operations
pub struct JwtService {
    secret: String,
    access_token_expiry_hours: i64,
    refresh_token_expiry_days: i64,
}

impl JwtService {
    /// Create a new JWT service with the given secret
    pub fn new(secret: String) -> Self {
        Self {
            secret,
            access_token_expiry_hours: 1,       // 1 hour for access tokens (improved security)
            refresh_token_expiry_days: 30,      // 30 days for refresh tokens
        }
    }

    /// Create with custom expiry times
    pub fn with_expiry(secret: String, access_hours: i64, refresh_days: i64) -> Self {
        Self {
            secret,
            access_token_expiry_hours: access_hours,
            refresh_token_expiry_days: refresh_days,
        }
    }

    /// Generate an access token for a user
    pub fn generate_access_token(
        &self,
        user_id: Uuid,
        email: &str,
    ) -> Result<String, JwtError> {
        let now = Utc::now();
        let exp = now + Duration::hours(self.access_token_expiry_hours);

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            iat: now.timestamp(),
            exp: exp.timestamp(),
            token_type: TokenType::Access,
        };

        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| JwtError::TokenGeneration(e.to_string()))
    }

    /// Generate a refresh token for a user
    pub fn generate_refresh_token(
        &self,
        user_id: Uuid,
        email: &str,
    ) -> Result<String, JwtError> {
        let now = Utc::now();
        let exp = now + Duration::days(self.refresh_token_expiry_days);

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            iat: now.timestamp(),
            exp: exp.timestamp(),
            token_type: TokenType::Refresh,
        };

        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| JwtError::TokenGeneration(e.to_string()))
    }

    /// Generate both access and refresh tokens
    pub fn generate_token_pair(
        &self,
        user_id: Uuid,
        email: &str,
    ) -> Result<TokenPair, JwtError> {
        Ok(TokenPair {
            access_token: self.generate_access_token(user_id, email)?,
            refresh_token: self.generate_refresh_token(user_id, email)?,
        })
    }

    /// Validate and decode a token
    pub fn validate_token(&self, token: &str) -> Result<Claims, JwtError> {
        let validation = Validation::new(Algorithm::HS256);

        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map(|data| data.claims)
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => JwtError::TokenExpired,
            jsonwebtoken::errors::ErrorKind::InvalidToken => JwtError::InvalidToken,
            jsonwebtoken::errors::ErrorKind::InvalidSignature => JwtError::InvalidSignature,
            _ => JwtError::ValidationError(e.to_string()),
        })
    }

    /// Validate that the token is an access token
    pub fn validate_access_token(&self, token: &str) -> Result<Claims, JwtError> {
        let claims = self.validate_token(token)?;

        if claims.token_type != TokenType::Access {
            return Err(JwtError::InvalidTokenType);
        }

        Ok(claims)
    }

    /// Validate that the token is a refresh token
    pub fn validate_refresh_token(&self, token: &str) -> Result<Claims, JwtError> {
        let claims = self.validate_token(token)?;

        if claims.token_type != TokenType::Refresh {
            return Err(JwtError::InvalidTokenType);
        }

        Ok(claims)
    }

    /// Extract user ID from token without full validation (for logging, etc.)
    pub fn extract_user_id(&self, token: &str) -> Option<String> {
        self.validate_token(token)
            .ok()
            .map(|claims| claims.sub)
    }
}

/// Token pair containing access and refresh tokens
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

/// JWT-related errors
#[derive(Debug, thiserror::Error)]
pub enum JwtError {
    #[error("Failed to generate token: {0}")]
    TokenGeneration(String),

    #[error("Token has expired")]
    TokenExpired,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Invalid token signature")]
    InvalidSignature,

    #[error("Invalid token type")]
    InvalidTokenType,

    #[error("Token validation error: {0}")]
    ValidationError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_service() -> JwtService {
        JwtService::new("test-secret-key-min-32-chars!!".to_string())
    }

    #[test]
    fn test_generate_access_token() {
        let service = create_test_service();
        let user_id = Uuid::new_v4();
        let email = "test@example.com";

        let token = service.generate_access_token(user_id, email);
        assert!(token.is_ok());
    }

    #[test]
    fn test_generate_refresh_token() {
        let service = create_test_service();
        let user_id = Uuid::new_v4();
        let email = "test@example.com";

        let token = service.generate_refresh_token(user_id, email);
        assert!(token.is_ok());
    }

    #[test]
    fn test_generate_token_pair() {
        let service = create_test_service();
        let user_id = Uuid::new_v4();
        let email = "test@example.com";

        let result = service.generate_token_pair(user_id, email);
        assert!(result.is_ok());

        let pair = result.unwrap();
        assert!(!pair.access_token.is_empty());
        assert!(!pair.refresh_token.is_empty());
    }

    #[test]
    fn test_validate_access_token() {
        let service = create_test_service();
        let user_id = Uuid::new_v4();
        let email = "test@example.com";

        let token = service.generate_access_token(user_id, email).unwrap();
        let claims = service.validate_access_token(&token);

        assert!(claims.is_ok());
        let claims = claims.unwrap();
        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.email, email);
        assert_eq!(claims.token_type, TokenType::Access);
    }

    #[test]
    fn test_validate_refresh_token() {
        let service = create_test_service();
        let user_id = Uuid::new_v4();
        let email = "test@example.com";

        let token = service.generate_refresh_token(user_id, email).unwrap();
        let claims = service.validate_refresh_token(&token);

        assert!(claims.is_ok());
        let claims = claims.unwrap();
        assert_eq!(claims.token_type, TokenType::Refresh);
    }

    #[test]
    fn test_wrong_token_type_validation() {
        let service = create_test_service();
        let user_id = Uuid::new_v4();
        let email = "test@example.com";

        // Try to validate access token as refresh token
        let access_token = service.generate_access_token(user_id, email).unwrap();
        let result = service.validate_refresh_token(&access_token);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), JwtError::InvalidTokenType));
    }

    #[test]
    fn test_invalid_token() {
        let service = create_test_service();
        let result = service.validate_token("invalid-token");
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_user_id() {
        let service = create_test_service();
        let user_id = Uuid::new_v4();
        let email = "test@example.com";

        let token = service.generate_access_token(user_id, email).unwrap();
        let extracted_id = service.extract_user_id(&token);

        assert!(extracted_id.is_some());
        assert_eq!(extracted_id.unwrap(), user_id.to_string());
    }
}
