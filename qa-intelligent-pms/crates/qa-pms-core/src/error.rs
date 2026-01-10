//! Error types for API boundaries.
//!
//! Uses `thiserror` for structured error types that can be converted to HTTP responses.

use serde::Serialize;

/// API error types for HTTP responses.
///
/// These are used at API boundaries to provide structured error responses.
/// Internal code should use `anyhow::Result` for error propagation.
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Validation error
    #[error("Validation failed: {0}")]
    Validation(String),

    /// Authentication required
    #[error("Authentication required: {0}")]
    Unauthorized(String),

    /// Permission denied
    #[error("Permission denied: {0}")]
    Forbidden(String),

    /// Conflict with existing resource
    #[error("Conflict: {0}")]
    Conflict(String),

    /// External service error
    #[error("External service error: {0}")]
    ExternalService(String),

    /// Service unavailable
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded")]
    RateLimited,

    /// Internal server error (wraps anyhow errors)
    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

impl ApiError {
    /// Get the error code for this error type.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::NotFound(_) => "NOT_FOUND",
            Self::Validation(_) => "VALIDATION_ERROR",
            Self::Unauthorized(_) => "UNAUTHORIZED",
            Self::Forbidden(_) => "FORBIDDEN",
            Self::Conflict(_) => "CONFLICT",
            Self::ExternalService(_) => "EXTERNAL_SERVICE_ERROR",
            Self::ServiceUnavailable(_) => "SERVICE_UNAVAILABLE",
            Self::RateLimited => "RATE_LIMITED",
            Self::Internal(_) => "INTERNAL_ERROR",
        }
    }

    /// Get the HTTP status code for this error type.
    #[must_use]
    pub const fn status_code(&self) -> u16 {
        match self {
            Self::NotFound(_) => 404,
            Self::Validation(_) => 400,
            Self::Unauthorized(_) => 401,
            Self::Forbidden(_) => 403,
            Self::Conflict(_) => 409,
            Self::ExternalService(_) => 502,
            Self::ServiceUnavailable(_) => 503,
            Self::RateLimited => 429,
            Self::Internal(_) => 500,
        }
    }
}

/// Standardized error response format for API.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "axum", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    /// Human-readable error message
    pub error: String,
    /// Error code for programmatic handling
    pub code: String,
    /// Additional error details (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl ErrorResponse {
    /// Create a new error response.
    #[must_use]
    pub fn new(error: impl Into<String>, code: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            code: code.into(),
            details: None,
        }
    }

    /// Create an error response with additional details.
    #[must_use]
    pub fn with_details(
        error: impl Into<String>,
        code: impl Into<String>,
        details: serde_json::Value,
    ) -> Self {
        Self {
            error: error.into(),
            code: code.into(),
            details: Some(details),
        }
    }
}

impl From<&ApiError> for ErrorResponse {
    fn from(err: &ApiError) -> Self {
        Self::new(err.to_string(), err.code())
    }
}

// Axum integration: IntoResponse for ApiError
#[cfg(feature = "axum")]
mod axum_impl {
    use super::{ApiError, ErrorResponse};
    use axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
        Json,
    };

    impl IntoResponse for ApiError {
        fn into_response(self) -> Response {
            let status = match self.status_code() {
                400 => StatusCode::BAD_REQUEST,
                401 => StatusCode::UNAUTHORIZED,
                403 => StatusCode::FORBIDDEN,
                404 => StatusCode::NOT_FOUND,
                409 => StatusCode::CONFLICT,
                429 => StatusCode::TOO_MANY_REQUESTS,
                502 => StatusCode::BAD_GATEWAY,
                503 => StatusCode::SERVICE_UNAVAILABLE,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };

            let body = ErrorResponse::from(&self);
            (status, Json(body)).into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_codes() {
        assert_eq!(ApiError::NotFound("test".into()).code(), "NOT_FOUND");
        assert_eq!(
            ApiError::Validation("test".into()).code(),
            "VALIDATION_ERROR"
        );
        assert_eq!(ApiError::Unauthorized("test".into()).code(), "UNAUTHORIZED");
        assert_eq!(
            ApiError::ServiceUnavailable("test".into()).code(),
            "SERVICE_UNAVAILABLE"
        );
    }

    #[test]
    fn test_api_error_status_codes() {
        assert_eq!(ApiError::NotFound("test".into()).status_code(), 404);
        assert_eq!(ApiError::Validation("test".into()).status_code(), 400);
        assert_eq!(ApiError::Unauthorized("test".into()).status_code(), 401);
        assert_eq!(
            ApiError::ServiceUnavailable("test".into()).status_code(),
            503
        );
    }

    #[test]
    fn test_error_response_serialization() {
        let response = ErrorResponse::new("User not found", "NOT_FOUND");
        let json = serde_json::to_string(&response).expect("Failed to serialize");
        assert!(json.contains("\"error\":\"User not found\""));
        assert!(json.contains("\"code\":\"NOT_FOUND\""));
    }
}
