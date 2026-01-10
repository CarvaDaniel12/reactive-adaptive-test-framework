//! Alert service for generating and managing alerts.

use crate::repository::PatternRepository;
use crate::types::{Alert, DetectedPattern, NewAlert};

/// Alert service for generating alerts from patterns.
pub struct AlertService {
    repo: PatternRepository,
}

impl AlertService {
    /// Create a new alert service.
    pub const fn new(repo: PatternRepository) -> Self {
        Self { repo }
    }

    /// Generate an alert from a detected pattern.
    pub async fn generate_alert(&self, pattern: &DetectedPattern) -> anyhow::Result<Alert> {
        let alert = NewAlert {
            pattern_id: Some(pattern.id),
            alert_type: pattern.pattern_type,
            severity: pattern.severity,
            title: pattern.title.clone(),
            message: pattern.description.clone(),
            affected_tickets: pattern.affected_tickets.clone(),
            suggested_actions: pattern.suggested_actions.clone(),
        };

        self.repo.create_alert(alert).await
    }

    /// Get all unread alerts.
    pub async fn get_unread_alerts(&self) -> anyhow::Result<Vec<Alert>> {
        self.repo.get_unread_alerts().await
    }

    /// Get unread alert count for badge display.
    pub async fn get_unread_count(&self) -> anyhow::Result<i64> {
        self.repo.get_unread_count().await
    }

    /// Mark an alert as read.
    pub async fn mark_read(&self, alert_id: uuid::Uuid) -> anyhow::Result<()> {
        self.repo.mark_alert_read(alert_id).await
    }

    /// Dismiss an alert.
    pub async fn dismiss(&self, alert_id: uuid::Uuid, user: Option<&str>) -> anyhow::Result<()> {
        self.repo.dismiss_alert(alert_id, user).await
    }
}
