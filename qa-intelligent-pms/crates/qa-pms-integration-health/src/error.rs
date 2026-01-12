//! Error types for integration health operations.

use qa_pms_core::error::ApiError;
use thiserror::Error;

/// Error types for integration health operations.
#[derive(Error, Debug)]
pub enum IntegrationHealthError {
    /// Integration not found
    #[error("Integration not found: {0}")]
    NotFound(String),

    /// Database error
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

// Conversion to ApiError for API boundaries
impl From<IntegrationHealthError> for ApiError {
    fn from(err: IntegrationHealthError) -> Self {
        match err {
            IntegrationHealthError::NotFound(msg) => ApiError::NotFound(msg),
            IntegrationHealthError::Database(e) => ApiError::Internal(e.into()),
            IntegrationHealthError::Internal(e) => ApiError::Internal(e),
        }
    }
}

// Conversion from ApiError for repository layer (needed when using SqlxResultExt)
impl From<ApiError> for IntegrationHealthError {
    fn from(err: ApiError) -> Self {
        match err {
            ApiError::Internal(e) => IntegrationHealthError::Internal(e),
            ApiError::NotFound(msg) => IntegrationHealthError::NotFound(msg),
            _ => IntegrationHealthError::Internal(anyhow::anyhow!("{}", err)),
        }
    }
}