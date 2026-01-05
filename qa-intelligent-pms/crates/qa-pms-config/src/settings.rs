//! Application settings loaded from environment variables.
//!
//! Uses `dotenvy` to load `.env` files and provides typed configuration.

use anyhow::{Context, Result};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

/// Application settings loaded from environment.
#[derive(Debug, Clone)]
pub struct Settings {
    /// Server configuration
    pub server: ServerSettings,
    /// Database configuration
    pub database: DatabaseSettings,
    /// Encryption key for secrets
    pub encryption_key: SecretString,
    /// Jira integration settings (optional)
    pub jira: Option<JiraSettings>,
    /// Postman integration settings (optional)
    pub postman: Option<PostmanSettings>,
    /// Testmo integration settings (optional)
    pub testmo: Option<TestmoSettings>,
}

/// Server configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerSettings {
    /// Host address to bind to
    pub host: String,
    /// Port to listen on
    pub port: u16,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
        }
    }
}

/// Database configuration.
#[derive(Debug, Clone)]
pub struct DatabaseSettings {
    /// Full database connection URL
    pub url: SecretString,
    /// Maximum number of connections in the pool
    pub max_connections: u32,
    /// Minimum number of connections in the pool
    pub min_connections: u32,
}

impl DatabaseSettings {
    /// Get the connection URL for display (masked).
    #[must_use]
    pub fn url_masked(&self) -> String {
        let url = self.url.expose_secret();
        // Mask password in URL for logging
        if let Some(at_pos) = url.find('@') {
            if let Some(colon_pos) = url[..at_pos].rfind(':') {
                let before_password = &url[..=colon_pos];
                let after_password = &url[at_pos..];
                return format!("{before_password}****{after_password}");
            }
        }
        "****".to_string()
    }
}

/// Jira authentication method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JiraAuthMethod {
    /// API Token with email (Basic Auth)
    ApiToken,
    /// OAuth 2.0 flow
    OAuth,
}

/// Jira integration settings.
///
/// Supports two authentication methods:
/// 1. **API Token** (recommended): Set `email` and `api_token` - simpler setup
/// 2. **OAuth 2.0**: Set `client_id`, `client_secret`, `redirect_uri` - for advanced apps
#[derive(Debug, Clone)]
pub struct JiraSettings {
    /// Jira Cloud instance URL (e.g., "<https://company.atlassian.net>")
    pub instance_url: String,
    /// User email for API Token auth
    pub email: Option<String>,
    /// API Token for Basic Auth (generated at <https://id.atlassian.com/manage-profile/security/api-tokens>)
    pub api_token: Option<SecretString>,
    /// OAuth 2.0 Client ID (alternative to API Token)
    pub client_id: Option<String>,
    /// OAuth 2.0 Client Secret (alternative to API Token)
    pub client_secret: Option<SecretString>,
    /// OAuth redirect URI
    pub redirect_uri: Option<String>,
}

impl JiraSettings {
    /// Get the configured authentication method.
    #[must_use]
    pub const fn auth_method(&self) -> JiraAuthMethod {
        if self.email.is_some() && self.api_token.is_some() {
            JiraAuthMethod::ApiToken
        } else {
            JiraAuthMethod::OAuth
        }
    }

    /// Check if API Token auth is configured.
    #[must_use]
    pub const fn has_api_token(&self) -> bool {
        self.email.is_some() && self.api_token.is_some()
    }
}

/// Postman integration settings.
#[derive(Debug, Clone)]
pub struct PostmanSettings {
    /// API key (encrypted)
    pub api_key: SecretString,
}

/// Testmo integration settings.
#[derive(Debug, Clone)]
pub struct TestmoSettings {
    /// Testmo instance URL
    pub base_url: String,
    /// API key (encrypted)
    pub api_key: SecretString,
    /// Default project ID for searches
    pub project_id: Option<i64>,
}

impl Settings {
    /// Load settings from environment variables.
    ///
    /// Loads `.env` file if present, then reads from environment.
    ///
    /// # Errors
    ///
    /// Returns an error if required environment variables are missing.
    pub fn from_env() -> Result<Self> {
        // Load .env file (ignore if not present)
        let _ = dotenvy::dotenv();

        let server = ServerSettings {
            host: std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .context("PORT must be a valid number")?,
        };

        let database = DatabaseSettings {
            url: SecretString::from(
                std::env::var("DATABASE_URL").context("DATABASE_URL is required")?,
            ),
            max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .context("DATABASE_MAX_CONNECTIONS must be a valid number")?,
            min_connections: std::env::var("DATABASE_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "2".to_string())
                .parse()
                .context("DATABASE_MIN_CONNECTIONS must be a valid number")?,
        };

        let encryption_key = SecretString::from(
            std::env::var("ENCRYPTION_KEY").context("ENCRYPTION_KEY is required")?,
        );

        // Optional integrations
        let jira = Self::load_jira_settings();
        let postman = Self::load_postman_settings();
        let testmo = Self::load_testmo_settings();

        Ok(Self {
            server,
            database,
            encryption_key,
            jira,
            postman,
            testmo,
        })
    }

    fn load_jira_settings() -> Option<JiraSettings> {
        // Instance URL is required for any Jira integration
        let instance_url = std::env::var("JIRA_URL").ok()?;

        // API Token auth (preferred - simpler)
        let email = std::env::var("JIRA_EMAIL").ok();
        let api_token = std::env::var("JIRA_API_TOKEN")
            .ok()
            .map(SecretString::from);

        // OAuth auth (alternative)
        let client_id = std::env::var("JIRA_CLIENT_ID").ok();
        let client_secret = std::env::var("JIRA_CLIENT_SECRET")
            .ok()
            .map(SecretString::from);
        let redirect_uri = std::env::var("JIRA_REDIRECT_URI").ok();

        // Need either API Token or OAuth credentials
        let has_api_token = email.is_some() && api_token.is_some();
        let has_oauth = client_id.is_some() && client_secret.is_some();

        if !has_api_token && !has_oauth {
            return None;
        }

        Some(JiraSettings {
            instance_url,
            email,
            api_token,
            client_id,
            client_secret,
            redirect_uri,
        })
    }

    fn load_postman_settings() -> Option<PostmanSettings> {
        let api_key = std::env::var("POSTMAN_API_KEY").ok()?;
        Some(PostmanSettings {
            api_key: SecretString::from(api_key),
        })
    }

    fn load_testmo_settings() -> Option<TestmoSettings> {
        let base_url = std::env::var("TESTMO_URL").ok()?;
        let api_key = std::env::var("TESTMO_API_KEY").ok()?;
        let project_id = std::env::var("TESTMO_PROJECT_ID")
            .ok()
            .and_then(|s| s.parse().ok());

        Some(TestmoSettings {
            base_url,
            api_key: SecretString::from(api_key),
            project_id,
        })
    }

    /// Get the server address string (host:port).
    #[must_use]
    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_settings_default() {
        let settings = ServerSettings::default();
        assert_eq!(settings.host, "127.0.0.1");
        assert_eq!(settings.port, 3000);
    }

    #[test]
    fn test_database_url_masked() {
        let db = DatabaseSettings {
            url: SecretString::from("postgres://user:secret123@host:5432/db".to_string()),
            max_connections: 10,
            min_connections: 2,
        };
        let masked = db.url_masked();
        assert!(!masked.contains("secret123"));
        assert!(masked.contains("****"));
    }
}
