//! Testmo integration health check.
//!
//! Validates Testmo API key and connectivity.

use async_trait::async_trait;
use qa_pms_core::health::{HealthCheck, HealthCheckResult};
use reqwest::Client;
use std::time::{Duration, Instant};
use tracing::debug;

/// Request timeout (10 seconds).
const REQUEST_TIMEOUT_SECS: u64 = 10;

/// Health check for Testmo integration.
///
/// Validates the API key by calling the `/api/v1/user` endpoint.
pub struct TestmoHealthCheck {
    http_client: Client,
    base_url: String,
    api_key: String,
}

impl TestmoHealthCheck {
    /// Create a new Testmo health check.
    ///
    /// # Arguments
    /// * `base_url` - Testmo instance URL (e.g., "<https://company.testmo.net>")
    /// * `api_key` - Testmo API key
    #[must_use]
    pub fn new(base_url: String, api_key: String) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .build()
            .expect("Failed to build HTTP client");

        // Ensure base_url doesn't have trailing slash
        let base_url = base_url.trim_end_matches('/').to_string();

        Self {
            http_client,
            base_url,
            api_key,
        }
    }

    /// Get the health check URL.
    fn health_url(&self) -> String {
        format!("{}/api/v1/user", self.base_url)
    }
}

#[async_trait]
impl HealthCheck for TestmoHealthCheck {
    fn integration_name(&self) -> &'static str {
        "testmo"
    }

    async fn check(&self) -> HealthCheckResult {
        let start = Instant::now();
        let url = self.health_url();

        debug!(url = %url, "Performing Testmo health check");

        match self
            .http_client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await
        {
            Ok(response) => {
                let duration = start.elapsed();

                if response.status().is_success() {
                    debug!(
                        response_time_ms = duration.as_millis(),
                        "Testmo health check passed"
                    );
                    HealthCheckResult::online("testmo", duration)
                } else if response.status() == reqwest::StatusCode::UNAUTHORIZED {
                    HealthCheckResult::offline("testmo", "Invalid API key")
                } else if response.status() == reqwest::StatusCode::FORBIDDEN {
                    HealthCheckResult::offline("testmo", "API key lacks required permissions")
                } else if response.status() == reqwest::StatusCode::NOT_FOUND {
                    HealthCheckResult::offline("testmo", "Invalid Testmo URL - endpoint not found")
                } else {
                    HealthCheckResult::offline("testmo", &format!("HTTP {}", response.status()))
                }
            }
            Err(e) => {
                if e.is_timeout() {
                    HealthCheckResult::offline(
                        "testmo",
                        &format!("Request timeout (>{REQUEST_TIMEOUT_SECS}s)"),
                    )
                } else if e.is_connect() {
                    HealthCheckResult::offline(
                        "testmo",
                        "Connection failed - check URL and network",
                    )
                } else {
                    HealthCheckResult::offline("testmo", &e.to_string())
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
    fn test_integration_name() {
        let check = TestmoHealthCheck::new(
            "https://company.testmo.net".to_string(),
            "test-api-key".to_string(),
        );
        assert_eq!(check.integration_name(), "testmo");
    }

    #[test]
    fn test_health_url() {
        let check = TestmoHealthCheck::new(
            "https://company.testmo.net".to_string(),
            "api-key".to_string(),
        );
        assert_eq!(check.health_url(), "https://company.testmo.net/api/v1/user");
    }

    #[test]
    fn test_health_url_strips_trailing_slash() {
        let check = TestmoHealthCheck::new(
            "https://company.testmo.net/".to_string(),
            "api-key".to_string(),
        );
        assert_eq!(check.health_url(), "https://company.testmo.net/api/v1/user");
    }

    #[tokio::test]
    async fn test_health_check_invalid_url() {
        let check = TestmoHealthCheck::new(
            "https://invalid-testmo-url.example.com".to_string(),
            "invalid-key".to_string(),
        );
        let result = check.check().await;

        assert_eq!(result.integration, "testmo");
        // Should be offline due to connection error
        assert_eq!(result.status, HealthStatus::Offline);
        assert!(result.error_message.is_some());
    }

    #[test]
    fn test_health_check_result_online() {
        let result = HealthCheckResult::online("testmo", Duration::from_millis(180));
        assert_eq!(result.status, HealthStatus::Online);
        assert_eq!(result.response_time_ms, Some(180));
    }

    #[test]
    fn test_health_check_result_offline() {
        let result = HealthCheckResult::offline("testmo", "Invalid API key");
        assert_eq!(result.status, HealthStatus::Offline);
        assert_eq!(result.error_message, Some("Invalid API key".to_string()));
    }
}
