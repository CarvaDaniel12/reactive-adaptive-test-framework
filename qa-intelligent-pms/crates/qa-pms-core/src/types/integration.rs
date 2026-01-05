//! Integration-related types and traits.
//!
//! Defines the common interface for all external integrations (Jira, Postman, Testmo, etc.)

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::Result;

/// Status of an integration connection.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IntegrationStatus {
    /// Integration is online and responding normally
    Online,
    /// Integration is responding but with degraded performance
    Degraded,
    /// Integration is not responding
    Offline,
    /// Integration status is unknown (not yet checked)
    #[default]
    Unknown,
}

/// Health information for an integration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationHealth {
    /// Name of the integration
    pub name: String,
    /// Current status
    pub status: IntegrationStatus,
    /// Last successful health check timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_check: Option<DateTime<Utc>>,
    /// Response time in milliseconds (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time_ms: Option<u64>,
    /// Error message if offline or degraded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl IntegrationHealth {
    /// Create a new health status for an online integration.
    #[must_use]
    pub fn online(name: impl Into<String>, response_time_ms: u64) -> Self {
        Self {
            name: name.into(),
            status: IntegrationStatus::Online,
            last_check: Some(Utc::now()),
            response_time_ms: Some(response_time_ms),
            error: None,
        }
    }

    /// Create a new health status for an offline integration.
    #[must_use]
    pub fn offline(name: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: IntegrationStatus::Offline,
            last_check: Some(Utc::now()),
            response_time_ms: None,
            error: Some(error.into()),
        }
    }

    /// Create a new health status for a degraded integration.
    #[must_use]
    pub fn degraded(
        name: impl Into<String>,
        response_time_ms: u64,
        error: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            status: IntegrationStatus::Degraded,
            last_check: Some(Utc::now()),
            response_time_ms: Some(response_time_ms),
            error: Some(error.into()),
        }
    }

    /// Create a new health status for an unknown integration.
    #[must_use]
    pub fn unknown(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: IntegrationStatus::Unknown,
            last_check: None,
            response_time_ms: None,
            error: None,
        }
    }
}

/// Trait for external integrations.
///
/// All integration crates should implement this trait to provide
/// a consistent interface for health checks and validation.
#[async_trait]
pub trait Integration: Send + Sync {
    /// Get the name of this integration (e.g., "Jira", "Postman").
    fn name(&self) -> &str;

    /// Check if the integration is healthy and responding.
    async fn health_check(&self) -> Result<IntegrationHealth>;

    /// Validate the credentials for this integration.
    async fn validate_credentials(&self) -> Result<bool>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_health_serialization() {
        let health = IntegrationHealth::online("Jira", 150);
        let json = serde_json::to_string(&health).expect("Failed to serialize");
        assert!(json.contains("\"status\":\"online\""));
        assert!(json.contains("\"responseTimeMs\":150"));
    }

    #[test]
    fn test_integration_status_default() {
        assert_eq!(IntegrationStatus::default(), IntegrationStatus::Unknown);
    }
}
