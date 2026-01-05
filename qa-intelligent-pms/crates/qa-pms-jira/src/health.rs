//! Jira integration health check.
//!
//! Validates Jira connectivity and authentication status.
//! Supports both API Token (Basic Auth) and OAuth authentication.

use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use qa_pms_core::health::{HealthCheck, HealthCheckResult};
use reqwest::Client;
use std::time::{Duration, Instant};
use tracing::debug;

/// Response time threshold for degraded status (2 seconds).
const DEGRADED_THRESHOLD_SECS: u64 = 2;

/// Request timeout (10 seconds).
const REQUEST_TIMEOUT_SECS: u64 = 10;

/// Jira authentication credentials.
#[derive(Clone)]
pub enum JiraAuth {
    /// API Token authentication (Basic Auth with email:token)
    ApiToken { email: String, api_token: String },
    /// OAuth 2.0 authentication (Bearer token)
    OAuth { access_token: String },
}

/// Health check for Jira integration.
///
/// Pings the `/rest/api/3/myself` endpoint to verify:
/// - Network connectivity
/// - Valid authentication
/// - Response time within acceptable limits
pub struct JiraHealthCheck {
    http_client: Client,
    /// Jira instance URL (e.g., "<https://company.atlassian.net>")
    instance_url: String,
    /// Authentication credentials
    auth: JiraAuth,
}

impl JiraHealthCheck {
    /// Create a new Jira health check with API Token authentication.
    ///
    /// This is the recommended method for most use cases.
    ///
    /// # Arguments
    /// * `instance_url` - Jira Cloud URL (e.g., "<https://company.atlassian.net>")
    /// * `email` - User email address
    /// * `api_token` - API token from <https://id.atlassian.com/manage-profile/security/api-tokens>
    #[must_use]
    pub fn with_api_token(instance_url: String, email: String, api_token: String) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            http_client,
            instance_url: instance_url.trim_end_matches('/').to_string(),
            auth: JiraAuth::ApiToken { email, api_token },
        }
    }

    /// Create a new Jira health check with OAuth authentication.
    ///
    /// # Arguments
    /// * `cloud_id` - Jira Cloud ID (from OAuth flow)
    /// * `access_token` - OAuth access token
    #[must_use]
    pub fn with_oauth(cloud_id: String, access_token: String) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .build()
            .expect("Failed to build HTTP client");

        // OAuth uses the Atlassian API gateway
        let instance_url = format!("https://api.atlassian.com/ex/jira/{cloud_id}");

        Self {
            http_client,
            instance_url,
            auth: JiraAuth::OAuth { access_token },
        }
    }

    /// Legacy constructor for OAuth (kept for compatibility).
    #[must_use]
    #[deprecated(since = "0.2.0", note = "Use with_api_token or with_oauth instead")]
    pub fn new(cloud_id: String, access_token: String) -> Self {
        Self::with_oauth(cloud_id, access_token)
    }

    /// Get the health check URL.
    fn health_url(&self) -> String {
        format!("{}/rest/api/3/myself", self.instance_url)
    }

    /// Build the authorization header value.
    fn auth_header(&self) -> String {
        match &self.auth {
            JiraAuth::ApiToken { email, api_token } => {
                let credentials = format!("{email}:{api_token}");
                format!("Basic {}", BASE64.encode(credentials.as_bytes()))
            }
            JiraAuth::OAuth { access_token } => {
                format!("Bearer {access_token}")
            }
        }
    }
}

#[async_trait]
impl HealthCheck for JiraHealthCheck {
    fn integration_name(&self) -> &'static str {
        "jira"
    }

    async fn check(&self) -> HealthCheckResult {
        let start = Instant::now();
        let url = self.health_url();

        debug!(url = %url, "Performing Jira health check");

        match self
            .http_client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await
        {
            Ok(response) => {
                let duration = start.elapsed();

                if response.status().is_success() {
                    if duration.as_secs() >= DEGRADED_THRESHOLD_SECS {
                        HealthCheckResult::degraded(
                            "jira",
                            duration,
                            &format!(
                                "Response time ({:.1}s) exceeds {}s threshold",
                                duration.as_secs_f64(),
                                DEGRADED_THRESHOLD_SECS
                            ),
                        )
                    } else {
                        debug!(
                            response_time_ms = duration.as_millis(),
                            "Jira health check passed"
                        );
                        HealthCheckResult::online("jira", duration)
                    }
                } else if response.status() == reqwest::StatusCode::UNAUTHORIZED {
                    HealthCheckResult::offline(
                        "jira",
                        "Authentication failed - check email/API token",
                    )
                } else if response.status() == reqwest::StatusCode::FORBIDDEN {
                    HealthCheckResult::offline("jira", "Access denied - check permissions")
                } else {
                    HealthCheckResult::offline("jira", &format!("HTTP {}", response.status()))
                }
            }
            Err(e) => {
                if e.is_timeout() {
                    HealthCheckResult::offline(
                        "jira",
                        &format!("Request timeout (>{REQUEST_TIMEOUT_SECS}s)"),
                    )
                } else if e.is_connect() {
                    HealthCheckResult::offline("jira", "Connection failed - check URL and network")
                } else {
                    HealthCheckResult::offline("jira", &e.to_string())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qa_pms_core::health::HealthStatus;

    #[test]
    fn test_health_url_api_token() {
        let check = JiraHealthCheck::with_api_token(
            "https://company.atlassian.net".to_string(),
            "user@example.com".to_string(),
            "token123".to_string(),
        );
        assert_eq!(
            check.health_url(),
            "https://company.atlassian.net/rest/api/3/myself"
        );
    }

    #[test]
    fn test_health_url_oauth() {
        let check = JiraHealthCheck::with_oauth("cloud-id-123".to_string(), "token".to_string());
        assert_eq!(
            check.health_url(),
            "https://api.atlassian.com/ex/jira/cloud-id-123/rest/api/3/myself"
        );
    }

    #[test]
    fn test_auth_header_api_token() {
        let check = JiraHealthCheck::with_api_token(
            "https://test.atlassian.net".to_string(),
            "user@test.com".to_string(),
            "api-token-123".to_string(),
        );
        let header = check.auth_header();
        assert!(header.starts_with("Basic "));
        // Decode and verify
        let encoded = header.strip_prefix("Basic ").unwrap();
        let decoded = String::from_utf8(BASE64.decode(encoded).unwrap()).unwrap();
        assert_eq!(decoded, "user@test.com:api-token-123");
    }

    #[test]
    fn test_auth_header_oauth() {
        let check = JiraHealthCheck::with_oauth("cloud".to_string(), "oauth-token-456".to_string());
        assert_eq!(check.auth_header(), "Bearer oauth-token-456");
    }

    #[test]
    fn test_integration_name() {
        let check = JiraHealthCheck::with_api_token(
            "https://test.atlassian.net".to_string(),
            "user@test.com".to_string(),
            "token".to_string(),
        );
        assert_eq!(check.integration_name(), "jira");
    }

    #[test]
    fn test_strips_trailing_slash() {
        let check = JiraHealthCheck::with_api_token(
            "https://company.atlassian.net/".to_string(),
            "user@example.com".to_string(),
            "token".to_string(),
        );
        assert_eq!(
            check.health_url(),
            "https://company.atlassian.net/rest/api/3/myself"
        );
    }

    #[test]
    fn test_health_check_result_online() {
        let result = HealthCheckResult::online("jira", Duration::from_millis(150));
        assert_eq!(result.status, HealthStatus::Online);
        assert_eq!(result.response_time_ms, Some(150));
    }

    #[test]
    fn test_health_check_result_degraded() {
        let result = HealthCheckResult::degraded("jira", Duration::from_secs(3), "Slow");
        assert_eq!(result.status, HealthStatus::Degraded);
        assert_eq!(result.response_time_ms, Some(3000));
    }

    #[test]
    fn test_health_check_result_offline() {
        let result = HealthCheckResult::offline("jira", "Auth expired");
        assert_eq!(result.status, HealthStatus::Offline);
        assert!(result.response_time_ms.is_none());
    }
}
