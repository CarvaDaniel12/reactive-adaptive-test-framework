//! Service for integration health business logic.

use crate::repository::IntegrationHealthRepository;
use crate::types::{IntegrationId, HealthStatus, IntegrationHealth, IntegrationEvent};
use crate::error::IntegrationHealthError;
use qa_pms_dashboard::{period_boundaries, Period};

/// Service for integration health business logic.
pub struct IntegrationHealthService {
    repository: IntegrationHealthRepository,
}

impl IntegrationHealthService {
    /// Create a new service.
    #[must_use]
    pub const fn new(repository: IntegrationHealthRepository) -> Self {
        Self { repository }
    }

    /// Get health status for all integrations.
    ///
    /// # Errors
    /// Returns error if repository query fails.
    pub async fn get_health_status(&self) -> Result<Vec<IntegrationHealth>, IntegrationHealthError> {
        // Get latest health for each integration
        let integrations = vec![
            IntegrationId::BookingCom,
            IntegrationId::Airbnb,
            IntegrationId::Vrbo,
            IntegrationId::Hmbn,
        ];

        let mut results = Vec::new();
        for integration_id in integrations {
            if let Some(health) = self.repository.get_latest_health(integration_id).await? {
                results.push(health);
            }
        }
        Ok(results)
    }

    /// Get health history for an integration within a period.
    ///
    /// # Errors
    /// Returns error if repository query fails.
    pub async fn get_health_history(
        &self,
        integration_id: IntegrationId,
        period: Period,
    ) -> Result<Vec<IntegrationHealth>, IntegrationHealthError> {
        let (start, end, _prev_start) = period_boundaries(period.days());
        let history = self.repository.get_health_history(integration_id, start, end).await?;
        
        // Calculate trends (future: use qa_pms_dashboard::calculate_trend)
        // For now, return history as-is
        Ok(history)
    }

    /// Update health status.
    ///
    /// # Errors
    /// Returns error if repository store fails.
    pub async fn update_health_status(
        &self,
        health: &IntegrationHealth,
    ) -> Result<(), IntegrationHealthError> {
        self.repository.store_health_status(health).await
    }

    /// Add event and recalculate health status.
    ///
    /// # Errors
    /// Returns error if repository store fails.
    pub async fn add_event(
        &self,
        event: &IntegrationEvent,
    ) -> Result<(), IntegrationHealthError> {
        // Store event
        self.repository.store_event(event).await?;
        
        // Recalculate health status (future: calculate based on recent events)
        // For now, just store the event
        Ok(())
    }

    /// Calculate health status from error rate.
    ///
    /// - **Healthy**: error_rate < 0.02 (2%)
    /// - **Warning**: 0.02 ≤ error_rate < 0.05 (5%)
    /// - **Critical**: error_rate ≥ 0.05 (5%)
    #[must_use]
    pub fn calculate_status_from_error_rate(error_rate: f64) -> HealthStatus {
        if error_rate < 0.02 {
            HealthStatus::Healthy
        } else if error_rate < 0.05 {
            HealthStatus::Warning
        } else {
            HealthStatus::Critical
        }
    }

    /// Calculate overall health status from multiple metrics.
    ///
    /// Returns the highest severity across all metrics (Critical > Warning > Healthy).
    #[must_use]
    pub fn calculate_overall_status(
        pricing_sync_status: Option<HealthStatus>,
        fees_sync_status: Option<HealthStatus>,
        error_rate: Option<f64>,
    ) -> HealthStatus {
        let mut statuses = vec![];
        
        if let Some(status) = pricing_sync_status {
            statuses.push(status);
        }
        
        if let Some(status) = fees_sync_status {
            statuses.push(status);
        }
        
        if let Some(rate) = error_rate {
            statuses.push(Self::calculate_status_from_error_rate(rate));
        }
        
        // Return highest severity (Critical > Warning > Healthy)
        statuses.iter().max_by_key(|s| match s {
            HealthStatus::Critical => 2,
            HealthStatus::Warning => 1,
            HealthStatus::Healthy => 0,
        }).copied().unwrap_or(HealthStatus::Healthy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_status_from_error_rate_healthy() {
        // Test healthy status (< 2%)
        assert_eq!(
            IntegrationHealthService::calculate_status_from_error_rate(0.0),
            HealthStatus::Healthy
        );
        assert_eq!(
            IntegrationHealthService::calculate_status_from_error_rate(0.01),
            HealthStatus::Healthy
        );
        assert_eq!(
            IntegrationHealthService::calculate_status_from_error_rate(0.019),
            HealthStatus::Healthy
        );
    }

    #[test]
    fn test_calculate_status_from_error_rate_warning() {
        // Test warning status (2% to < 5%)
        assert_eq!(
            IntegrationHealthService::calculate_status_from_error_rate(0.02),
            HealthStatus::Warning
        );
        assert_eq!(
            IntegrationHealthService::calculate_status_from_error_rate(0.03),
            HealthStatus::Warning
        );
        assert_eq!(
            IntegrationHealthService::calculate_status_from_error_rate(0.049),
            HealthStatus::Warning
        );
    }

    #[test]
    fn test_calculate_status_from_error_rate_critical() {
        // Test critical status (≥ 5%)
        assert_eq!(
            IntegrationHealthService::calculate_status_from_error_rate(0.05),
            HealthStatus::Critical
        );
        assert_eq!(
            IntegrationHealthService::calculate_status_from_error_rate(0.1),
            HealthStatus::Critical
        );
        assert_eq!(
            IntegrationHealthService::calculate_status_from_error_rate(1.0),
            HealthStatus::Critical
        );
    }

    #[test]
    fn test_calculate_overall_status_single_metric() {
        // Test with single metric
        assert_eq!(
            IntegrationHealthService::calculate_overall_status(
                Some(HealthStatus::Healthy),
                None,
                None
            ),
            HealthStatus::Healthy
        );
        assert_eq!(
            IntegrationHealthService::calculate_overall_status(
                None,
                Some(HealthStatus::Warning),
                None
            ),
            HealthStatus::Warning
        );
        assert_eq!(
            IntegrationHealthService::calculate_overall_status(
                None,
                None,
                Some(0.06) // 6% error rate = Critical
            ),
            HealthStatus::Critical
        );
    }

    #[test]
    fn test_calculate_overall_status_multiple_metrics() {
        // Test with multiple metrics - should return highest severity
        assert_eq!(
            IntegrationHealthService::calculate_overall_status(
                Some(HealthStatus::Healthy),
                Some(HealthStatus::Warning),
                None
            ),
            HealthStatus::Warning
        );
        assert_eq!(
            IntegrationHealthService::calculate_overall_status(
                Some(HealthStatus::Healthy),
                Some(HealthStatus::Healthy),
                Some(0.03) // 3% error rate = Warning
            ),
            HealthStatus::Warning
        );
        assert_eq!(
            IntegrationHealthService::calculate_overall_status(
                Some(HealthStatus::Warning),
                Some(HealthStatus::Healthy),
                Some(0.06) // 6% error rate = Critical
            ),
            HealthStatus::Critical
        );
        assert_eq!(
            IntegrationHealthService::calculate_overall_status(
                Some(HealthStatus::Critical),
                Some(HealthStatus::Warning),
                Some(0.01) // 1% error rate = Healthy
            ),
            HealthStatus::Critical
        );
    }

    #[test]
    fn test_calculate_overall_status_empty_metrics() {
        // Test with no metrics - should default to Healthy
        assert_eq!(
            IntegrationHealthService::calculate_overall_status(None, None, None),
            HealthStatus::Healthy
        );
    }

    #[test]
    fn test_calculate_overall_status_error_rate_conversion() {
        // Test that error_rate is correctly converted to status
        assert_eq!(
            IntegrationHealthService::calculate_overall_status(
                None,
                None,
                Some(0.01) // 1% = Healthy
            ),
            HealthStatus::Healthy
        );
        assert_eq!(
            IntegrationHealthService::calculate_overall_status(
                None,
                None,
                Some(0.03) // 3% = Warning
            ),
            HealthStatus::Warning
        );
        assert_eq!(
            IntegrationHealthService::calculate_overall_status(
                None,
                None,
                Some(0.10) // 10% = Critical
            ),
            HealthStatus::Critical
        );
    }
}
