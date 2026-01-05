//! Tracking service error types.

use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur during time tracking operations.
#[derive(Debug, Error)]
pub enum TrackingError {
    /// Session not found
    #[error("Session not found: {0}")]
    SessionNotFound(Uuid),

    /// No active session for workflow
    #[error("No active session for workflow: {0}")]
    NoActiveSession(Uuid),

    /// Session already active
    #[error("Session already active for workflow: {0}, step: {1}")]
    SessionAlreadyActive(Uuid, i32),

    /// Session not paused
    #[error("Session {0} is not paused")]
    SessionNotPaused(Uuid),

    /// Session already ended
    #[error("Session {0} has already ended")]
    SessionAlreadyEnded(Uuid),

    /// Database error
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl TrackingError {
    /// Check if this is a "not found" error.
    #[must_use]
    pub const fn is_not_found(&self) -> bool {
        matches!(self, Self::SessionNotFound(_) | Self::NoActiveSession(_))
    }

    /// Check if this is a database error.
    #[must_use]
    pub const fn is_database_error(&self) -> bool {
        matches!(self, Self::Database(_))
    }
}
