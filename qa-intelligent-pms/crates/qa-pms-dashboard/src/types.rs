//! Shared types for dashboard responses.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Individual KPI metric with value, change percentage, and trend direction.
///
/// Used across QA and PM dashboards for consistent metric presentation.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct KPIMetric {
    /// The current metric value
    pub value: f64,
    /// Percentage change from previous period
    pub change: f64,
    /// Trend direction: "up", "down", or "neutral"
    pub trend: String,
}

impl KPIMetric {
    /// Create a new KPI metric.
    #[must_use]
    pub fn new(value: f64, change: f64, trend: impl Into<String>) -> Self {
        Self {
            value,
            change,
            trend: trend.into(),
        }
    }

    /// Create a KPI metric with automatic change and trend calculation.
    #[must_use]
    pub fn from_values(current: f64, previous: f64) -> Self {
        Self {
            value: current,
            change: crate::calculate_change(current, previous),
            trend: crate::calculate_trend(current, previous).to_string(),
        }
    }

    /// Create a KPI metric with inverted trend (lower is better).
    ///
    /// Useful for metrics like "time per task" where lower values are better.
    #[must_use]
    pub fn from_values_inverted(current: f64, previous: f64) -> Self {
        Self {
            value: current,
            change: crate::calculate_change(current, previous),
            // Inverted: if current is lower than previous, trend is "up" (good)
            trend: crate::calculate_trend(previous, current).to_string(),
        }
    }
}

/// Trend data point for time-series charts.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TrendDataPoint {
    /// Formatted date string (e.g., "Jan 15")
    pub date: String,
    /// Number of tickets completed
    pub tickets: i32,
    /// Hours worked
    pub hours: f64,
}

impl TrendDataPoint {
    /// Create a new trend data point.
    #[must_use]
    pub fn new(date: impl Into<String>, tickets: i32, hours: f64) -> Self {
        Self {
            date: date.into(),
            tickets,
            hours,
        }
    }
}

/// Recent activity item for activity feeds.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ActivityItem {
    /// Unique identifier
    pub id: String,
    /// Activity type (e.g., "workflow_completed")
    #[serde(rename = "type")]
    pub activity_type: String,
    /// Display title
    pub title: String,
    /// Associated ticket key (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_key: Option<String>,
    /// ISO 8601 timestamp
    pub timestamp: String,
    /// Duration in seconds (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}

impl ActivityItem {
    /// Create a new activity item for a completed workflow.
    #[must_use]
    pub fn workflow_completed(
        id: impl Into<String>,
        title: impl Into<String>,
        ticket_key: Option<String>,
        timestamp: impl Into<String>,
        duration: Option<i64>,
    ) -> Self {
        Self {
            id: id.into(),
            activity_type: "workflow_completed".to_string(),
            title: title.into(),
            ticket_key,
            timestamp: timestamp.into(),
            duration,
        }
    }
}

/// Breakdown of tickets by type for KPI cards.
///
/// Story 8.2 AC #4: Breakdown by ticket type (hover for details)
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TypeBreakdown {
    /// Ticket type (e.g., "bug", "feature", "regression")
    pub ticket_type: String,
    /// Count of tickets of this type
    pub count: i64,
    /// Percentage of total tickets
    pub percentage: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kpi_metric_from_values() {
        let kpi = KPIMetric::from_values(110.0, 100.0);
        assert_eq!(kpi.value, 110.0);
        assert_eq!(kpi.change, 10.0);
        assert_eq!(kpi.trend, "up");
    }

    #[test]
    fn test_kpi_metric_inverted() {
        // For time metrics: lower is better
        let kpi = KPIMetric::from_values_inverted(90.0, 100.0);
        assert_eq!(kpi.value, 90.0);
        assert_eq!(kpi.change, -10.0);
        // Trend is "up" because lower time is good
        assert_eq!(kpi.trend, "up");
    }

    #[test]
    fn test_trend_data_point() {
        let point = TrendDataPoint::new("Jan 15", 5, 8.5);
        assert_eq!(point.date, "Jan 15");
        assert_eq!(point.tickets, 5);
        assert_eq!(point.hours, 8.5);
    }

    #[test]
    fn test_activity_item() {
        let item = ActivityItem::workflow_completed(
            "abc123",
            "Bug Fix Workflow",
            Some("PROJ-123".to_string()),
            "2026-01-06T10:00:00Z",
            Some(3600),
        );
        assert_eq!(item.activity_type, "workflow_completed");
        assert_eq!(item.ticket_key, Some("PROJ-123".to_string()));
    }
}
