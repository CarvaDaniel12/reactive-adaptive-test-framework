//! Error types for the AI module.

use thiserror::Error;

/// Errors that can occur in the AI module.
#[derive(Debug, Error)]
pub enum AIError {
    /// AI not configured
    #[error("AI not configured. Please add your API key in Settings.")]
    NotConfigured,

    /// Invalid API key
    #[error("Invalid API key: {0}")]
    InvalidApiKey(String),

    /// Provider not supported
    #[error("Provider not supported: {0}")]
    UnsupportedProvider(String),

    /// API request failed
    #[error("API request failed: {0}")]
    RequestFailed(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded. Please wait and try again.")]
    RateLimited,

    /// Model not available
    #[error("Model not available: {0}")]
    ModelNotAvailable(String),

    /// Context too long
    #[error("Context too long for model. Please reduce input size.")]
    ContextTooLong,

    /// Parsing error
    #[error("Failed to parse AI response: {0}")]
    ParseError(String),

    /// Network error
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl AIError {
    /// Check if this error should trigger a fallback to non-AI behavior.
    #[must_use] 
    pub const fn should_fallback(&self) -> bool {
        matches!(
            self,
            Self::NotConfigured
                | Self::InvalidApiKey(_)
                | Self::RateLimited
                | Self::NetworkError(_)
        )
    }
}
