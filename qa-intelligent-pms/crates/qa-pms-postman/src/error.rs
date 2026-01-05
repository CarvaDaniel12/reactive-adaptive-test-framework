//! Postman API error types.
//!
//! Domain-specific error types for Postman API operations.

use reqwest::StatusCode;
use thiserror::Error;

/// Postman API errors.
#[derive(Debug, Error)]
pub enum PostmanError {
    /// Invalid or missing API key.
    #[error("Unauthorized - invalid API key")]
    Unauthorized,

    /// Resource not found.
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Rate limit exceeded.
    #[error("Rate limited - too many requests")]
    RateLimited,

    /// Generic API error with status code.
    #[error("API error (HTTP {status}): {body}")]
    ApiError {
        /// HTTP status code.
        status: StatusCode,
        /// Response body.
        body: String,
    },

    /// Network or connection error.
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// JSON parsing error.
    #[error("Failed to parse response: {0}")]
    Parse(String),
}

impl PostmanError {
    /// Check if the error is retryable.
    ///
    /// Returns `true` for rate limiting, network errors, and server errors (5xx).
    #[must_use]
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::RateLimited | Self::Network(_) => true,
            Self::ApiError { status, .. } => status.is_server_error(),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unauthorized_not_retryable() {
        let err = PostmanError::Unauthorized;
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_not_found_not_retryable() {
        let err = PostmanError::NotFound("/collections/123".to_string());
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_rate_limited_is_retryable() {
        let err = PostmanError::RateLimited;
        assert!(err.is_retryable());
    }

    #[test]
    fn test_server_error_is_retryable() {
        let err = PostmanError::ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: "Internal error".to_string(),
        };
        assert!(err.is_retryable());
    }

    #[test]
    fn test_client_error_not_retryable() {
        let err = PostmanError::ApiError {
            status: StatusCode::BAD_REQUEST,
            body: "Bad request".to_string(),
        };
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_error_display() {
        let err = PostmanError::NotFound("/workspaces/abc".to_string());
        assert_eq!(err.to_string(), "Resource not found: /workspaces/abc");
    }
}
