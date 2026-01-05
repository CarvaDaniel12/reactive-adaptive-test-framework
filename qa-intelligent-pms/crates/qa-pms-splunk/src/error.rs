//! Splunk module error types.

use thiserror::Error;

/// Errors that can occur in the Splunk module.
#[derive(Debug, Error)]
pub enum SplunkError {
    /// Template not found.
    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    /// Invalid query template.
    #[error("Invalid query template: {0}")]
    InvalidTemplate(String),

    /// Database error.
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Placeholder error.
    #[error("Missing placeholder value: {0}")]
    MissingPlaceholder(String),
}
