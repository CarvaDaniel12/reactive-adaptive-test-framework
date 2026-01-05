//! Jira integration error types.
//!
//! Provides structured error handling for OAuth flows and API operations.

use thiserror::Error;

/// Errors that can occur during Jira authentication.
#[derive(Debug, Error)]
pub enum JiraAuthError {
    /// Invalid OAuth state parameter - possible CSRF attack
    #[error("Invalid OAuth state parameter - possible CSRF attack")]
    InvalidState,

    /// Authorization code expired or invalid
    #[error("Authorization code expired or invalid")]
    InvalidCode,

    /// User denied authorization on Jira
    #[error("User denied authorization")]
    UserDenied,

    /// Token refresh failed
    #[error("Token refresh failed: {0}")]
    RefreshFailed(String),

    /// Token not found in storage
    #[error("No tokens found for integration: {0}")]
    TokenNotFound(String),

    /// Token has expired and refresh is not possible
    #[error("Token expired and cannot be refreshed")]
    TokenExpired,

    /// Missing required OAuth configuration
    #[error("Missing OAuth configuration: {0}")]
    MissingConfig(String),

    /// Network error during OAuth flow
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// Failed to parse OAuth response
    #[error("Failed to parse OAuth response: {0}")]
    ParseError(String),

    /// Token storage error
    #[error("Token storage error: {0}")]
    StorageError(String),

    /// Encryption/decryption error
    #[error("Encryption error: {0}")]
    EncryptionError(String),
}

/// Errors that can occur during Jira API operations.
#[derive(Debug, Error)]
pub enum JiraApiError {
    /// Authentication required - no valid token
    #[error("Authentication required")]
    Unauthorized,

    /// Forbidden - insufficient permissions
    #[error("Forbidden: {0}")]
    Forbidden(String),

    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Rate limited by Jira
    #[error("Rate limited - retry after {0} seconds")]
    RateLimited(u64),

    /// Jira API returned an error
    #[error("Jira API error: {status} - {message}")]
    ApiError { status: u16, message: String },

    /// Network error
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// Failed to parse API response
    #[error("Failed to parse response: {0}")]
    ParseError(String),

    /// Authentication error
    #[error("Authentication error: {0}")]
    Auth(#[from] JiraAuthError),
}

impl JiraAuthError {
    /// Get a user-friendly message for display.
    #[must_use]
    pub const fn user_message(&self) -> &str {
        match self {
            Self::InvalidState => "Security validation failed. Please try again.",
            Self::InvalidCode => "Authorization expired. Please try connecting again.",
            Self::UserDenied => "Authorization was denied. Please try again and approve access.",
            Self::RefreshFailed(_) | Self::TokenExpired => {
                "Session expired. Please reconnect to Jira."
            }
            Self::TokenNotFound(_) => "Not connected to Jira. Please connect your account.",
            Self::MissingConfig(_) => "Jira is not configured. Please complete setup first.",
            Self::Network(_) => "Network error. Please check your connection and try again.",
            Self::ParseError(_) => "Unexpected response from Jira. Please try again.",
            Self::StorageError(_) => "Failed to save credentials. Please try again.",
            Self::EncryptionError(_) => "Security error. Please contact support.",
        }
    }

    /// Check if the error requires re-authentication.
    #[must_use]
    pub const fn requires_reauth(&self) -> bool {
        matches!(
            self,
            Self::InvalidState
                | Self::InvalidCode
                | Self::UserDenied
                | Self::RefreshFailed(_)
                | Self::TokenNotFound(_)
                | Self::TokenExpired
        )
    }
}

impl JiraApiError {
    /// Check if the error requires re-authentication.
    #[must_use]
    pub const fn requires_reauth(&self) -> bool {
        matches!(self, Self::Unauthorized | Self::Auth(_))
    }
}
