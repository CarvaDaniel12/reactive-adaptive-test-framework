//! Authentication types and traits.
//!
//! Provides token storage abstraction for OAuth integrations.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Stored OAuth tokens for an integration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredTokens {
    /// Access token for API calls
    pub access_token: String,
    /// Refresh token for obtaining new access tokens
    pub refresh_token: String,
    /// When the access token expires
    pub expires_at: DateTime<Utc>,
    /// Integration identifier (e.g., "jira", "postman")
    pub integration: String,
    /// When the tokens were last updated
    pub updated_at: DateTime<Utc>,
}

impl StoredTokens {
    /// Create new stored tokens from OAuth response.
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    pub fn new(
        integration: impl Into<String>,
        access_token: impl Into<String>,
        refresh_token: impl Into<String>,
        expires_in_seconds: u64,
    ) -> Self {
        let now = Utc::now();
        Self {
            access_token: access_token.into(),
            refresh_token: refresh_token.into(),
            expires_at: now + chrono::Duration::seconds(expires_in_seconds as i64),
            integration: integration.into(),
            updated_at: now,
        }
    }

    /// Check if the access token is expired.
    #[must_use]
    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }

    /// Check if the access token will expire within the given duration.
    #[must_use]
    pub fn expires_within(&self, seconds: i64) -> bool {
        Utc::now() + chrono::Duration::seconds(seconds) >= self.expires_at
    }

    /// Get seconds until expiration (0 if already expired).
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    pub fn seconds_until_expiry(&self) -> u64 {
        let diff = self.expires_at - Utc::now();
        if diff.num_seconds() < 0 {
            0
        } else {
            diff.num_seconds() as u64
        }
    }
}

/// Token storage abstraction for OAuth integrations.
///
/// Implementations should encrypt tokens at rest.
#[async_trait]
pub trait TokenStore: Send + Sync {
    /// Store tokens for an integration.
    ///
    /// # Errors
    ///
    /// Returns an error if storage fails.
    async fn store_tokens(&self, tokens: StoredTokens) -> anyhow::Result<()>;

    /// Get tokens for an integration.
    ///
    /// # Errors
    ///
    /// Returns an error if retrieval fails.
    async fn get_tokens(&self, integration: &str) -> anyhow::Result<Option<StoredTokens>>;

    /// Delete tokens for an integration.
    ///
    /// # Errors
    ///
    /// Returns an error if deletion fails.
    async fn delete_tokens(&self, integration: &str) -> anyhow::Result<()>;

    /// Check if tokens exist for an integration.
    ///
    /// # Errors
    ///
    /// Returns an error if the check fails.
    async fn has_tokens(&self, integration: &str) -> anyhow::Result<bool> {
        Ok(self.get_tokens(integration).await?.is_some())
    }

    /// Check if tokens are expired for an integration.
    ///
    /// Returns `true` if tokens don't exist or are expired.
    ///
    /// # Errors
    ///
    /// Returns an error if the check fails.
    async fn is_token_expired(&self, integration: &str) -> anyhow::Result<bool> {
        Ok(self
            .get_tokens(integration)
            .await?
            .map_or(true, |tokens| tokens.is_expired()))
    }

    /// Check if tokens will expire soon (within 5 minutes).
    ///
    /// # Errors
    ///
    /// Returns an error if the check fails.
    async fn needs_refresh(&self, integration: &str) -> anyhow::Result<bool> {
        Ok(self
            .get_tokens(integration)
            .await?
            .is_some_and(|tokens| tokens.expires_within(300)))
    }
}

/// Authorization state storage for OAuth flows.
///
/// Stores the PKCE code verifier between authorization redirect and callback.
#[async_trait]
pub trait AuthStateStore: Send + Sync {
    /// Store authorization state.
    ///
    /// # Arguments
    ///
    /// * `state` - The random state parameter
    /// * `code_verifier` - The PKCE code verifier
    ///
    /// # Errors
    ///
    /// Returns an error if storage fails.
    async fn store(&self, state: &str, code_verifier: &str) -> anyhow::Result<()>;

    /// Get and remove authorization state.
    ///
    /// This should be called once during the callback to retrieve and consume
    /// the state, preventing replay attacks.
    ///
    /// # Errors
    ///
    /// Returns an error if retrieval fails.
    async fn get_and_remove(&self, state: &str) -> anyhow::Result<Option<String>>;

    /// Clean up expired states.
    ///
    /// # Errors
    ///
    /// Returns an error if cleanup fails.
    async fn cleanup_expired(&self) -> anyhow::Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stored_tokens_new() {
        let tokens = StoredTokens::new("jira", "access123", "refresh456", 3600);
        
        assert_eq!(tokens.integration, "jira");
        assert_eq!(tokens.access_token, "access123");
        assert_eq!(tokens.refresh_token, "refresh456");
        assert!(!tokens.is_expired());
    }

    #[test]
    fn test_stored_tokens_expiry() {
        let mut tokens = StoredTokens::new("jira", "access", "refresh", 3600);
        
        // Not expired
        assert!(!tokens.is_expired());
        assert!(tokens.seconds_until_expiry() > 3500);
        
        // Simulate expiration
        tokens.expires_at = Utc::now() - chrono::Duration::seconds(1);
        assert!(tokens.is_expired());
        assert_eq!(tokens.seconds_until_expiry(), 0);
    }

    #[test]
    fn test_expires_within() {
        let tokens = StoredTokens::new("jira", "access", "refresh", 300);
        
        // Should expire within 5 minutes
        assert!(tokens.expires_within(300));
        // Should not expire within 1 second
        assert!(!tokens.expires_within(1));
    }
}
