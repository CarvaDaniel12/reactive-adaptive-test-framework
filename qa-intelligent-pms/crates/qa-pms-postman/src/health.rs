//! Postman integration health check.
//!
//! Validates Postman API key and connectivity.

use async_trait::async_trait;
use qa_pms_core::health::{HealthCheck, HealthCheckResult};
use reqwest::Client;
use std::time::{Duration, Instant};
use tracing::debug;

/// Postman API base URL.
const POSTMAN_API_URL: &str = "https://api.getpostman.com/me";

/// Request timeout (10 seconds).
const REQUEST_TIMEOUT_SECS: u64 = 10;

/// Health check for Postman integration.
///
/// Validates the API key by calling the `/me` endpoint.
pub struct PostmanHealthCheck {
    http_client: Client,
    api_key: String,
}

impl PostmanHealthCheck {
    /// Create a new Postman health check.
    ///
    /// # Arguments
    /// * `api_key` - Postman API key
    #[must_use]
    pub fn new(api_key: String) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            http_client,
            api_key,
        }
    }
}

#[async_trait]
impl HealthCheck for PostmanHealthCheck {
    fn integration_name(&self) -> &'static str {
        "postman"
    }

    async fn check(&self) -> HealthCheckResult {
        let start = Instant::now();

        debug!("Performing Postman health check");

        match self
            .http_client
            .get(POSTMAN_API_URL)
            .header("X-Api-Key", &self.api_key)
            .send()
            .await
        {
            Ok(response) => {
                let duration = start.elapsed();

                if response.status().is_success() {
                    debug!(
                        response_time_ms = duration.as_millis(),
                        "Postman health check passed"
                    );
                    HealthCheckResult::online("postman", duration)
                } else if response.status() == reqwest::StatusCode::UNAUTHORIZED {
                    HealthCheckResult::offline("postman", "Invalid API key")
                } else if response.status() == reqwest::StatusCode::FORBIDDEN {
                    HealthCheckResult::offline("postman", "API key lacks required permissions")
                } else if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                    HealthCheckResult::degraded(
                        "postman",
                        duration,
                        "Rate limit exceeded - requests may be throttled",
                    )
                } else {
                    HealthCheckResult::offline("postman", &format!("HTTP {}", response.status()))
                }
            }
            Err(e) => {
                if e.is_timeout() {
                    HealthCheckResult::offline(
                        "postman",
                        &format!("Request timeout (>{REQUEST_TIMEOUT_SECS}s)"),
                    )
                } else if e.is_connect() {
                    HealthCheckResult::offline("postman", "Connection failed - check network")
                } else {
                    HealthCheckResult::offline("postman", &e.to_string())
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
        let check = PostmanHealthCheck::new("test-api-key".to_string());
        assert_eq!(check.integration_name(), "postman");
    }

    #[tokio::test]
    async fn test_health_check_invalid_key() {
        let check = PostmanHealthCheck::new("invalid-api-key".to_string());
        let result = check.check().await;

        assert_eq!(result.integration, "postman");
        // Should be offline due to invalid API key
        assert_eq!(result.status, HealthStatus::Offline);
        assert!(result.error_message.is_some());
    }

    #[test]
    fn test_health_check_result_online() {
        let result = HealthCheckResult::online("postman", Duration::from_millis(200));
        assert_eq!(result.status, HealthStatus::Online);
        assert_eq!(result.response_time_ms, Some(200));
    }

    #[test]
    fn test_health_check_result_offline() {
        let result = HealthCheckResult::offline("postman", "Invalid API key");
        assert_eq!(result.status, HealthStatus::Offline);
        assert_eq!(result.error_message, Some("Invalid API key".to_string()));
    }
}
