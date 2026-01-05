//! Health check types and traits for integration monitoring.
//!
//! Provides:
//! - `HealthStatus` enum for integration states
//! - `HealthCheckResult` for individual check results
//! - `IntegrationHealth` for aggregated health state
//! - `HealthCheck` trait for implementing health checks

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Health status of an integration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum HealthStatus {
    /// Integration is fully operational
    Online,
    /// Integration is working but with degraded performance
    Degraded,
    /// Integration is not responding
    #[default]
    Offline,
}


/// Result of a single health check.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckResult {
    /// Integration name (e.g., "jira", "postman", "testmo")
    pub integration: String,
    /// Current status
    pub status: HealthStatus,
    /// Response time in milliseconds (if available)
    pub response_time_ms: Option<u64>,
    /// Error message (if offline or degraded)
    pub error_message: Option<String>,
    /// When the check was performed
    pub checked_at: DateTime<Utc>,
}

impl HealthCheckResult {
    /// Create an online result with response time.
    #[must_use] 
    pub fn online(integration: &str, response_time: Duration) -> Self {
        Self {
            integration: integration.to_string(),
            status: HealthStatus::Online,
            response_time_ms: Some(response_time.as_millis() as u64),
            error_message: None,
            checked_at: Utc::now(),
        }
    }

    /// Create a degraded result (working but slow or with warnings).
    #[must_use] 
    pub fn degraded(integration: &str, response_time: Duration, message: &str) -> Self {
        Self {
            integration: integration.to_string(),
            status: HealthStatus::Degraded,
            response_time_ms: Some(response_time.as_millis() as u64),
            error_message: Some(message.to_string()),
            checked_at: Utc::now(),
        }
    }

    /// Create an offline result with error message.
    #[must_use] 
    pub fn offline(integration: &str, error: &str) -> Self {
        Self {
            integration: integration.to_string(),
            status: HealthStatus::Offline,
            response_time_ms: None,
            error_message: Some(error.to_string()),
            checked_at: Utc::now(),
        }
    }
}

/// Aggregated health state for an integration.
///
/// Tracks historical health information including downtime tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationHealth {
    /// Integration name
    pub integration: String,
    /// Current status
    pub status: HealthStatus,
    /// Last time the integration was successfully checked
    pub last_successful_check: Option<DateTime<Utc>>,
    /// Last check timestamp (success or failure)
    pub last_check: DateTime<Utc>,
    /// Last response time in milliseconds
    pub response_time_ms: Option<u64>,
    /// Last error message (if any)
    pub error_message: Option<String>,
    /// Number of consecutive check failures
    pub consecutive_failures: u32,
    /// When downtime started (if currently offline)
    pub downtime_start: Option<DateTime<Utc>>,
}

impl IntegrationHealth {
    /// Create a new health state for an integration.
    #[must_use] 
    pub fn new(integration: &str) -> Self {
        Self {
            integration: integration.to_string(),
            status: HealthStatus::Offline,
            last_successful_check: None,
            last_check: Utc::now(),
            response_time_ms: None,
            error_message: None,
            consecutive_failures: 0,
            downtime_start: None,
        }
    }

    /// Check if the integration is currently down.
    #[must_use] 
    pub fn is_offline(&self) -> bool {
        self.status == HealthStatus::Offline
    }

    /// Get downtime duration if currently offline.
    #[must_use] 
    pub fn downtime_duration(&self) -> Option<chrono::Duration> {
        self.downtime_start.map(|start| Utc::now() - start)
    }
}

/// Trait for implementing health checks.
///
/// Each integration should implement this trait to provide health monitoring.
#[async_trait]
pub trait HealthCheck: Send + Sync {
    /// Get the integration name (e.g., "jira", "postman").
    fn integration_name(&self) -> &str;

    /// Perform the health check.
    ///
    /// Should check connectivity and authentication status.
    async fn check(&self) -> HealthCheckResult;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_serialization() {
        assert_eq!(
            serde_json::to_string(&HealthStatus::Online).unwrap(),
            "\"online\""
        );
        assert_eq!(
            serde_json::to_string(&HealthStatus::Degraded).unwrap(),
            "\"degraded\""
        );
        assert_eq!(
            serde_json::to_string(&HealthStatus::Offline).unwrap(),
            "\"offline\""
        );
    }

    #[test]
    fn test_health_check_result_online() {
        let result = HealthCheckResult::online("jira", Duration::from_millis(150));
        assert_eq!(result.integration, "jira");
        assert_eq!(result.status, HealthStatus::Online);
        assert_eq!(result.response_time_ms, Some(150));
        assert!(result.error_message.is_none());
    }

    #[test]
    fn test_health_check_result_degraded() {
        let result = HealthCheckResult::degraded("postman", Duration::from_secs(3), "Slow response");
        assert_eq!(result.status, HealthStatus::Degraded);
        assert_eq!(result.response_time_ms, Some(3000));
        assert_eq!(result.error_message, Some("Slow response".to_string()));
    }

    #[test]
    fn test_health_check_result_offline() {
        let result = HealthCheckResult::offline("testmo", "Connection refused");
        assert_eq!(result.status, HealthStatus::Offline);
        assert!(result.response_time_ms.is_none());
        assert_eq!(result.error_message, Some("Connection refused".to_string()));
    }

    #[test]
    fn test_integration_health_new() {
        let health = IntegrationHealth::new("jira");
        assert_eq!(health.integration, "jira");
        assert_eq!(health.status, HealthStatus::Offline);
        assert_eq!(health.consecutive_failures, 0);
        assert!(health.downtime_start.is_none());
    }

    #[test]
    fn test_integration_health_is_offline() {
        let mut health = IntegrationHealth::new("jira");
        assert!(health.is_offline());

        health.status = HealthStatus::Online;
        assert!(!health.is_offline());
    }
}
