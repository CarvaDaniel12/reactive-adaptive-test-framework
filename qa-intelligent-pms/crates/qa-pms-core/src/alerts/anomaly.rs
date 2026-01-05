//! Anomaly alerting module.
//!
//! This module provides alert notification for detected anomalies,
//! integrating with the existing alert system.

use chrono::Utc;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::Result;

/// Anomaly alert configuration.
#[derive(Debug, Clone)]
pub struct AnomalyAlertConfig {
    /// Minimum severity threshold for alerts
    pub min_severity: AnomalySeverityThreshold,
    /// Enable in-app notifications
    pub in_app_enabled: bool,
    /// Enable email notifications
    pub email_enabled: bool,
    /// Enable Slack webhook notifications
    pub slack_enabled: bool,
    /// Email recipient
    pub email_recipient: Option<String>,
    /// Slack webhook URL
    pub slack_webhook_url: Option<String>,
    /// Rate limiting window in seconds
    pub rate_limit_window_seconds: u64,
    /// Maximum alerts per window
    pub max_alerts_per_window: usize,
}

/// Minimum severity threshold for alerts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnomalySeverityThreshold {
    /// Show all anomalies (Info, Warning, Critical)
    Info,
    /// Show Warning and Critical only
    Warning,
    /// Show Critical only
    Critical,
}

impl Default for AnomalyAlertConfig {
    fn default() -> Self {
        Self {
            min_severity: AnomalySeverityThreshold::Warning,
            in_app_enabled: true,
            email_enabled: false,
            slack_enabled: false,
            email_recipient: None,
            slack_webhook_url: None,
            rate_limit_window_seconds: 300, // 5 minutes
            max_alerts_per_window: 10,
        }
    }
}

/// Anomaly severity (mirrored from qa-pms-ai for convenience).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnomalySeverity {
    /// Informational anomaly
    Info,
    /// Warning-level anomaly
    Warning,
    /// Critical anomaly
    Critical,
}

impl AnomalySeverity {
    /// Convert from string.
    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s {
            "info" => Self::Info,
            "warning" => Self::Warning,
            "critical" => Self::Critical,
            _ => Self::Info,
        }
    }
}

/// Anomaly alert data for conversion to existing alert format.
#[derive(Debug, Clone)]
pub struct AnomalyAlert {
    /// Anomaly ID
    pub id: Uuid,
    /// Anomaly type
    pub anomaly_type: String,
    /// Severity
    pub severity: AnomalySeverity,
    /// Description
    pub description: String,
    /// Affected entities
    pub affected_entities: Vec<String>,
    /// Investigation steps
    pub investigation_steps: Vec<String>,
    /// Metrics as JSON
    pub metrics_json: String,
}

/// Helper function to create an AnomalyAlert from anomaly data.
/// This is typically called from qa-pms-ai crate after detecting an anomaly.
#[must_use]
pub fn create_anomaly_alert(
    id: Uuid,
    anomaly_type: String,
    severity: AnomalySeverity,
    description: String,
    affected_entities: Vec<String>,
    investigation_steps: Vec<String>,
    metrics_json: String,
) -> AnomalyAlert {
    AnomalyAlert {
        id,
        anomaly_type,
        severity,
        description,
        affected_entities,
        investigation_steps,
        metrics_json,
    }
}

/// Alert rate limiter to prevent alert spam.
#[derive(Debug, Clone)]
pub struct AlertRateLimiter {
    /// Window size in seconds
    window_size: u64,
    /// Max alerts per window
    max_alerts: usize,
    /// Alert timestamps
    alerts: Vec<chrono::DateTime<chrono::Utc>>,
}

impl AlertRateLimiter {
    /// Create a new rate limiter.
    #[must_use]
    pub fn new(window_size_seconds: u64, max_alerts: usize) -> Self {
        Self {
            window_size: window_size_seconds,
            max_alerts,
            alerts: Vec::new(),
        }
    }

    /// Check if alert should be allowed (not rate limited).
    pub fn should_allow(&mut self) -> bool {
        let now = Utc::now();
        let cutoff = now - chrono::Duration::seconds(self.window_size as i64);

        // Remove old alerts outside the window
        self.alerts.retain(|&t| t > cutoff);

        // Check if we're under the limit
        if self.alerts.len() >= self.max_alerts {
            return false;
        }

        // Record this alert
        self.alerts.push(now);
        true
    }

    /// Reset the rate limiter.
    pub fn reset(&mut self) {
        self.alerts.clear();
    }
}

/// Anomaly alert notification service.
pub struct AnomalyAlertService {
    /// Alert configuration
    config: AnomalyAlertConfig,
    /// Rate limiter
    rate_limiter: AlertRateLimiter,
}

impl AnomalyAlertService {
    /// Create a new anomaly alert service.
    #[must_use]
    pub fn new(config: AnomalyAlertConfig) -> Self {
        let rate_limiter = AlertRateLimiter::new(
            config.rate_limit_window_seconds,
            config.max_alerts_per_window,
        );
        Self {
            config,
            rate_limiter,
        }
    }

    /// Send alert notification for an anomaly.
    pub async fn notify(&mut self, anomaly: AnomalyAlert) -> Result<()> {
        // Check severity threshold
        if !self.should_send_for_severity(anomaly.severity) {
            info!(
                anomaly_id = %anomaly.id,
                severity = ?anomaly.severity,
                "Alert skipped: below severity threshold"
            );
            return Ok(());
        }

        // Check rate limiting
        if !self.rate_limiter.should_allow() {
            warn!(
                anomaly_id = %anomaly.id,
                "Alert rate limited: too many alerts in time window"
            );
            return Ok(());
        }

        // Send in-app notification
        if self.config.in_app_enabled {
            if let Err(e) = self.send_in_app_notification(&anomaly).await {
                error!(
                    anomaly_id = %anomaly.id,
                    error = %e,
                    "Failed to send in-app notification"
                );
            }
        }

        // Send email notification (if configured)
        if self.config.email_enabled {
            if let Some(ref recipient) = self.config.email_recipient {
                if let Err(e) = self.send_email_notification(&anomaly, recipient).await {
                    error!(
                        anomaly_id = %anomaly.id,
                        recipient,
                        error = %e,
                        "Failed to send email notification"
                    );
                }
            }
        }

        // Send Slack notification (if configured)
        if self.config.slack_enabled {
            if let Some(ref webhook_url) = self.config.slack_webhook_url {
                if let Err(e) = self.send_slack_notification(&anomaly, webhook_url).await {
                    error!(
                        anomaly_id = %anomaly.id,
                        error = %e,
                        "Failed to send Slack notification"
                    );
                }
            }
        }

        info!(
            anomaly_id = %anomaly.id,
            anomaly_type = %anomaly.anomaly_type,
            severity = ?anomaly.severity,
            "Anomaly alert sent"
        );

        Ok(())
    }

    /// Check if alert should be sent for given severity.
    fn should_send_for_severity(&self, severity: AnomalySeverity) -> bool {
        match (self.config.min_severity, severity) {
            (AnomalySeverityThreshold::Info, _) => true,
            (AnomalySeverityThreshold::Warning, AnomalySeverity::Warning | AnomalySeverity::Critical) => {
                true
            }
            (AnomalySeverityThreshold::Critical, AnomalySeverity::Critical) => true,
            _ => false,
        }
    }

    /// Send in-app notification.
    async fn send_in_app_notification(&self, _anomaly: &AnomalyAlert) -> Result<()> {
        // In-app notifications are handled by storing alerts in the database
        // The existing alert system will pick them up and display them
        // This is a placeholder for future in-app notification logic
        Ok(())
    }

    /// Send email notification.
    async fn send_email_notification(
        &self,
        anomaly: &AnomalyAlert,
        _recipient: &str,
    ) -> Result<()> {
        // Email notification is not implemented yet
        // This would use an email service (e.g., SMTP, SendGrid, SES)
        info!(
            anomaly_id = %anomaly.id,
            "Email notification not implemented yet"
        );
        Ok(())
    }

    /// Send Slack webhook notification.
    async fn send_slack_notification(
        &self,
        anomaly: &AnomalyAlert,
        webhook_url: &str,
    ) -> Result<()> {
        // Slack notification is not implemented yet
        // This would POST to the Slack webhook URL
        info!(
            anomaly_id = %anomaly.id,
            webhook_url,
            "Slack notification not implemented yet"
        );
        Ok(())
    }

    /// Create alert message from anomaly.
    #[must_use]
    pub fn create_alert_message(anomaly: &AnomalyAlert) -> String {
        format!(
            "Anomaly Detected: {}\n\nType: {}\nSeverity: {:?}\n\nDescription: {}\n\nAffected: {}\n\nInvestigation Steps:\n{}",
            anomaly.anomaly_type,
            anomaly.anomaly_type,
            anomaly.severity,
            anomaly.description,
            anomaly.affected_entities.join(", "),
            anomaly.investigation_steps.iter()
                .map(|s| format!("  - {}", s))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl Default for AnomalyAlertService {
    fn default() -> Self {
        Self::new(AnomalyAlertConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_threshold() {
        let config = AnomalyAlertConfig {
            min_severity: AnomalySeverityThreshold::Warning,
            ..Default::default()
        };
        let mut service = AnomalyAlertService::new(config);

        let info_anomaly = AnomalyAlert {
            id: Uuid::new_v4(),
            anomaly_type: "test".to_string(),
            severity: AnomalySeverity::Info,
            description: "test".to_string(),
            affected_entities: vec![],
            investigation_steps: vec![],
            metrics_json: "{}".to_string(),
        };

        assert!(!service.should_send_for_severity(info_anomaly.severity));

        let warning_anomaly = AnomalyAlert {
            severity: AnomalySeverity::Warning,
            ..info_anomaly.clone()
        };
        assert!(service.should_send_for_severity(warning_anomaly.severity));
    }

    #[test]
    fn test_rate_limiter() {
        let mut limiter = AlertRateLimiter::new(60, 5);

        // Should allow first 5
        for _ in 0..5 {
            assert!(limiter.should_allow());
        }

        // Should block 6th
        assert!(!limiter.should_allow());
    }

    #[test]
    fn test_alert_message_creation() {
        let anomaly = AnomalyAlert {
            id: Uuid::new_v4(),
            anomaly_type: "performance_degradation".to_string(),
            severity: AnomalySeverity::Warning,
            description: "Test anomaly".to_string(),
            affected_entities: vec!["TICKET-1".to_string(), "TICKET-2".to_string()],
            investigation_steps: vec![
                "Step 1".to_string(),
                "Step 2".to_string(),
            ],
            metrics_json: "{}".to_string(),
        };

        let message = AnomalyAlertService::create_alert_message(&anomaly);
        assert!(message.contains("performance_degradation"));
        assert!(message.contains("Test anomaly"));
        assert!(message.contains("TICKET-1"));
        assert!(message.contains("Step 1"));
    }
}
