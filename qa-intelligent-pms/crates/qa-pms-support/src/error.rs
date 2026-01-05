//! Error types for the support module.

use thiserror::Error;

/// Errors that can occur in the support module.
#[derive(Debug, Error)]
pub enum SupportError {
    /// Database error
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),

    /// Error log not found
    #[error("Error log not found: {0}")]
    ErrorLogNotFound(uuid::Uuid),

    /// Knowledge base entry not found
    #[error("Knowledge base entry not found: {0}")]
    KbEntryNotFound(uuid::Uuid),

    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Diagnostic failed
    #[error("Diagnostic failed: {0}")]
    DiagnosticFailed(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}
