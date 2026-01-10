//! In-memory health state storage.
//!
//! Thread-safe storage for integration health states with downtime alerting.

use crate::health::{HealthCheckResult, HealthStatus, IntegrationHealth};
use chrono::{Duration, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Thread-safe in-memory store for integration health states.
///
/// Tracks health check results and alerts when integrations are down for extended periods.
#[derive(Clone)]
pub struct HealthStore {
    state: Arc<RwLock<HashMap<String, IntegrationHealth>>>,
    /// Downtime threshold in minutes before alerting (default: 2)
    alert_threshold_minutes: i64,
}

impl Default for HealthStore {
    fn default() -> Self {
        Self::new()
    }
}

impl HealthStore {
    /// Create a new health store with default 2-minute alert threshold.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(HashMap::new())),
            alert_threshold_minutes: 2,
        }
    }

    /// Create a health store with custom alert threshold.
    #[must_use]
    pub fn with_alert_threshold(minutes: i64) -> Self {
        Self {
            state: Arc::new(RwLock::new(HashMap::new())),
            alert_threshold_minutes: minutes,
        }
    }

    /// Update health state with a new check result.
    ///
    /// Handles state transitions and alerts for extended downtime (NFR-REL-02).
    pub async fn update(&self, result: HealthCheckResult) {
        let mut state = self.state.write().await;

        let entry = state
            .entry(result.integration.clone())
            .or_insert_with(|| IntegrationHealth::new(&result.integration));

        let previous_status = entry.status;
        entry.last_check = result.checked_at;
        entry.status = result.status;
        entry.response_time_ms = result.response_time_ms;
        entry.error_message = result.error_message.clone();

        match result.status {
            HealthStatus::Online => {
                entry.last_successful_check = Some(result.checked_at);
                entry.consecutive_failures = 0;

                // Log recovery if was previously down
                if let Some(start) = entry.downtime_start {
                    let downtime: Duration = Utc::now() - start;
                    info!(
                        integration = %entry.integration,
                        downtime_seconds = downtime.num_seconds(),
                        "Integration recovered"
                    );
                }
                entry.downtime_start = None;
            }
            HealthStatus::Degraded => {
                entry.last_successful_check = Some(result.checked_at);
                entry.consecutive_failures = 0;
                entry.downtime_start = None;

                info!(
                    integration = %entry.integration,
                    response_time_ms = ?entry.response_time_ms,
                    message = ?entry.error_message,
                    "Integration degraded"
                );
            }
            HealthStatus::Offline => {
                entry.consecutive_failures += 1;

                // Start downtime tracking on first failure
                if entry.downtime_start.is_none() {
                    entry.downtime_start = Some(Utc::now());
                }

                // Alert if down > threshold (NFR-REL-02)
                if let Some(start) = entry.downtime_start {
                    let downtime: Duration = Utc::now() - start;
                    if downtime > Duration::minutes(self.alert_threshold_minutes) {
                        warn!(
                            integration = %entry.integration,
                            downtime_minutes = downtime.num_minutes(),
                            consecutive_failures = entry.consecutive_failures,
                            error = ?entry.error_message,
                            "⚠️ Integration has been offline for more than {} minutes",
                            self.alert_threshold_minutes
                        );
                    }
                }

                // Log state transition to offline
                if previous_status != HealthStatus::Offline {
                    warn!(
                        integration = %entry.integration,
                        error = ?entry.error_message,
                        "Integration went offline"
                    );
                }
            }
        }
    }

    /// Get all integration health states.
    pub async fn get_all(&self) -> Vec<IntegrationHealth> {
        self.state.read().await.values().cloned().collect()
    }

    /// Get health state for a specific integration.
    pub async fn get(&self, integration: &str) -> Option<IntegrationHealth> {
        self.state.read().await.get(integration).cloned()
    }

    /// Check if any integration is currently offline.
    pub async fn has_offline(&self) -> bool {
        self.state
            .read()
            .await
            .values()
            .any(|h| h.status == HealthStatus::Offline)
    }

    /// Get count of integrations by status.
    pub async fn status_counts(&self) -> (usize, usize, usize) {
        let state = self.state.read().await;
        let online = state
            .values()
            .filter(|h| h.status == HealthStatus::Online)
            .count();
        let degraded = state
            .values()
            .filter(|h| h.status == HealthStatus::Degraded)
            .count();
        let offline = state
            .values()
            .filter(|h| h.status == HealthStatus::Offline)
            .count();
        (online, degraded, offline)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration as StdDuration;

    #[tokio::test]
    async fn test_store_update_online() {
        let store = HealthStore::new();
        let result = HealthCheckResult::online("jira", StdDuration::from_millis(100));

        store.update(result).await;

        let health = store.get("jira").await.unwrap();
        assert_eq!(health.status, HealthStatus::Online);
        assert_eq!(health.consecutive_failures, 0);
        assert!(health.downtime_start.is_none());
    }

    #[tokio::test]
    async fn test_store_update_offline_increments_failures() {
        let store = HealthStore::new();

        store
            .update(HealthCheckResult::offline("jira", "Error 1"))
            .await;
        store
            .update(HealthCheckResult::offline("jira", "Error 2"))
            .await;
        store
            .update(HealthCheckResult::offline("jira", "Error 3"))
            .await;

        let health = store.get("jira").await.unwrap();
        assert_eq!(health.status, HealthStatus::Offline);
        assert_eq!(health.consecutive_failures, 3);
        assert!(health.downtime_start.is_some());
    }

    #[tokio::test]
    async fn test_store_recovery_clears_downtime() {
        let store = HealthStore::new();

        // Go offline
        store
            .update(HealthCheckResult::offline("jira", "Error"))
            .await;
        let health = store.get("jira").await.unwrap();
        assert!(health.downtime_start.is_some());

        // Recover
        store
            .update(HealthCheckResult::online(
                "jira",
                StdDuration::from_millis(50),
            ))
            .await;
        let health = store.get("jira").await.unwrap();
        assert!(health.downtime_start.is_none());
        assert_eq!(health.consecutive_failures, 0);
    }

    #[tokio::test]
    async fn test_store_get_all() {
        let store = HealthStore::new();

        store
            .update(HealthCheckResult::online(
                "jira",
                StdDuration::from_millis(100),
            ))
            .await;
        store
            .update(HealthCheckResult::online(
                "postman",
                StdDuration::from_millis(200),
            ))
            .await;
        store
            .update(HealthCheckResult::offline("testmo", "Down"))
            .await;

        let all = store.get_all().await;
        assert_eq!(all.len(), 3);
    }

    #[tokio::test]
    async fn test_store_has_offline() {
        let store = HealthStore::new();

        store
            .update(HealthCheckResult::online(
                "jira",
                StdDuration::from_millis(100),
            ))
            .await;
        assert!(!store.has_offline().await);

        store
            .update(HealthCheckResult::offline("postman", "Error"))
            .await;
        assert!(store.has_offline().await);
    }

    #[tokio::test]
    async fn test_store_status_counts() {
        let store = HealthStore::new();

        store
            .update(HealthCheckResult::online(
                "jira",
                StdDuration::from_millis(100),
            ))
            .await;
        store
            .update(HealthCheckResult::degraded(
                "postman",
                StdDuration::from_secs(3),
                "Slow",
            ))
            .await;
        store
            .update(HealthCheckResult::offline("testmo", "Down"))
            .await;

        let (online, degraded, offline) = store.status_counts().await;
        assert_eq!(online, 1);
        assert_eq!(degraded, 1);
        assert_eq!(offline, 1);
    }
}
