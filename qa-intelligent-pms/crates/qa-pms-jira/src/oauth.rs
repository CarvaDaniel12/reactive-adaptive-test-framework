//! Jira OAuth 2.0 client implementation.
//!
//! Supports OAuth 2.0 Authorization Code flow with optional PKCE.
//! Note: Jira Cloud requires `client_secret` and does not support PKCE as of 2024.
//! PKCE is included for Jira Data Center/Server support.

use crate::error::JiraAuthError;
use crate::pkce::{generate_state, PkceChallenge};
use anyhow::Result;
use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

/// Jira OAuth configuration.
#[derive(Debug, Clone)]
pub struct JiraOAuthConfig {
    /// OAuth Client ID from Atlassian Developer Console
    pub client_id: String,
    /// OAuth Client Secret (encrypted at rest)
    pub client_secret: SecretString,
    /// Redirect URI configured in Atlassian Developer Console
    pub redirect_uri: String,
    /// OAuth scopes to request
    pub scopes: Vec<String>,
}

impl JiraOAuthConfig {
    /// Default scopes required for QA PMS functionality.
    #[must_use]
    pub fn default_scopes() -> Vec<String> {
        vec![
            "read:jira-user".to_string(),
            "read:jira-work".to_string(),
            "write:jira-work".to_string(),
            "offline_access".to_string(),
        ]
    }
}

/// OAuth token response from Atlassian.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    /// Access token for API calls
    pub access_token: String,
    /// Refresh token for obtaining new access tokens
    pub refresh_token: Option<String>,
    /// Token lifetime in seconds
    pub expires_in: u64,
    /// Token type (usually "Bearer")
    pub token_type: String,
    /// Granted scopes
    pub scope: String,
}

/// Authorization state stored between redirect and callback.
#[derive(Debug, Clone)]
pub struct AuthorizationState {
    /// PKCE code verifier (needed to exchange code for tokens)
    pub code_verifier: String,
    /// Random state for CSRF protection
    pub state: String,
}

/// Jira OAuth 2.0 client.
pub struct JiraOAuthClient {
    config: JiraOAuthConfig,
    http_client: Client,
}

impl JiraOAuthClient {
    /// Atlassian authorization endpoint.
    const AUTH_URL: &'static str = "https://auth.atlassian.com/authorize";
    /// Atlassian token endpoint.
    const TOKEN_URL: &'static str = "https://auth.atlassian.com/oauth/token";
    /// Atlassian API audience.
    const AUDIENCE: &'static str = "api.atlassian.com";

    /// Create a new OAuth client with the given configuration.
    #[must_use]
    pub fn new(config: JiraOAuthConfig) -> Self {
        Self {
            config,
            http_client: Client::new(),
        }
    }

    /// Build the authorization URL for redirecting the user.
    ///
    /// Returns the URL and the authorization state that must be stored
    /// for validating the callback.
    #[must_use]
    pub fn build_authorization_url(&self) -> (String, AuthorizationState) {
        let pkce = PkceChallenge::new();
        let state = generate_state();

        let scopes = self.config.scopes.join(" ");

        // Build URL with all required parameters
        let url = format!(
            "{}?audience={}&client_id={}&scope={}&redirect_uri={}&state={}&response_type=code&prompt=consent&code_challenge={}&code_challenge_method={}",
            Self::AUTH_URL,
            Self::AUDIENCE,
            urlencoding::encode(&self.config.client_id),
            urlencoding::encode(&scopes),
            urlencoding::encode(&self.config.redirect_uri),
            urlencoding::encode(&state),
            urlencoding::encode(&pkce.challenge),
            PkceChallenge::method(),
        );

        debug!(
            client_id = %self.config.client_id,
            redirect_uri = %self.config.redirect_uri,
            "Built authorization URL"
        );

        let auth_state = AuthorizationState {
            code_verifier: pkce.verifier,
            state,
        };

        (url, auth_state)
    }

    /// Exchange an authorization code for access and refresh tokens.
    ///
    /// # Errors
    ///
    /// Returns an error if the token exchange fails.
    pub async fn exchange_code_for_tokens(
        &self,
        code: &str,
        code_verifier: &str,
    ) -> Result<TokenResponse, JiraAuthError> {
        info!("Exchanging authorization code for tokens");

        let params = [
            ("grant_type", "authorization_code"),
            ("client_id", &self.config.client_id),
            ("client_secret", self.config.client_secret.expose_secret()),
            ("code", code),
            ("redirect_uri", &self.config.redirect_uri),
            ("code_verifier", code_verifier),
        ];

        let response = self
            .http_client
            .post(Self::TOKEN_URL)
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %body, "Token exchange failed");

            // Parse error response
            if body.contains("invalid_grant") {
                return Err(JiraAuthError::InvalidCode);
            }
            if body.contains("access_denied") {
                return Err(JiraAuthError::UserDenied);
            }

            return Err(JiraAuthError::ParseError(format!(
                "Token exchange failed: {status} - {body}"
            )));
        }

        let tokens: TokenResponse = response
            .json()
            .await
            .map_err(|e| JiraAuthError::ParseError(e.to_string()))?;

        info!(
            expires_in = tokens.expires_in,
            has_refresh = tokens.refresh_token.is_some(),
            "Successfully obtained tokens"
        );

        Ok(tokens)
    }

    /// Refresh an access token using a refresh token.
    ///
    /// # Errors
    ///
    /// Returns an error if the refresh fails.
    pub async fn refresh_access_token(
        &self,
        refresh_token: &str,
    ) -> Result<TokenResponse, JiraAuthError> {
        info!("Refreshing access token");

        let params = [
            ("grant_type", "refresh_token"),
            ("client_id", &self.config.client_id),
            ("client_secret", self.config.client_secret.expose_secret()),
            ("refresh_token", refresh_token),
        ];

        let response = self
            .http_client
            .post(Self::TOKEN_URL)
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(status = %status, "Token refresh failed");

            return Err(JiraAuthError::RefreshFailed(format!("{status}: {body}")));
        }

        let tokens: TokenResponse = response
            .json()
            .await
            .map_err(|e| JiraAuthError::ParseError(e.to_string()))?;

        info!(
            expires_in = tokens.expires_in,
            "Successfully refreshed token"
        );

        Ok(tokens)
    }

    /// Get the configured redirect URI.
    #[must_use]
    pub fn redirect_uri(&self) -> &str {
        &self.config.redirect_uri
    }

    /// Get the configured client ID.
    #[must_use]
    pub fn client_id(&self) -> &str {
        &self.config.client_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> JiraOAuthConfig {
        JiraOAuthConfig {
            client_id: "test-client-id".to_string(),
            client_secret: SecretString::from("test-secret".to_string()),
            redirect_uri: "http://localhost:3000/api/v1/auth/jira/callback".to_string(),
            scopes: JiraOAuthConfig::default_scopes(),
        }
    }

    #[test]
    fn test_build_authorization_url() {
        let client = JiraOAuthClient::new(test_config());
        let (url, state) = client.build_authorization_url();

        assert!(url.starts_with("https://auth.atlassian.com/authorize"));
        assert!(url.contains("client_id=test-client-id"));
        assert!(url.contains("response_type=code"));
        assert!(url.contains("code_challenge="));
        assert!(url.contains("code_challenge_method=S256"));
        assert!(url.contains("offline_access"));
        assert!(!state.code_verifier.is_empty());
        assert!(!state.state.is_empty());
    }

    #[test]
    fn test_default_scopes() {
        let scopes = JiraOAuthConfig::default_scopes();
        assert!(scopes.contains(&"read:jira-user".to_string()));
        assert!(scopes.contains(&"read:jira-work".to_string()));
        assert!(scopes.contains(&"write:jira-work".to_string()));
        assert!(scopes.contains(&"offline_access".to_string()));
    }
}
