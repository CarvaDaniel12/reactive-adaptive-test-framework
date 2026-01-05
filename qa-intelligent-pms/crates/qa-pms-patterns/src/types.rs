//! Types for pattern detection.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Pattern type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PatternType {
    /// Steps/tickets taking >50% longer than estimated
    TimeExcess,
    /// 3+ consecutive tickets with same component/issue
    ConsecutiveProblem,
    /// Sudden increase in tickets for an area
    Spike,
}

impl std::fmt::Display for PatternType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TimeExcess => write!(f, "time_excess"),
            Self::ConsecutiveProblem => write!(f, "consecutive_problem"),
            Self::Spike => write!(f, "spike"),
        }
    }
}

/// Alert severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Info,
    Warning,
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "info"),
            Self::Warning => write!(f, "warning"),
            Self::Critical => write!(f, "critical"),
        }
    }
}

/// A detected pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectedPattern {
    pub id: Uuid,
    pub pattern_type: PatternType,
    pub severity: Severity,
    pub title: String,
    pub description: Option<String>,
    pub affected_tickets: Vec<String>,
    pub common_factor: Option<String>,
    pub average_excess_percent: Option<f64>,
    pub confidence_score: f64,
    pub suggested_actions: Vec<String>,
    pub metadata: serde_json::Value,
    pub detected_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Input for creating a new pattern.
#[derive(Debug, Clone)]
pub struct NewPattern {
    pub pattern_type: PatternType,
    pub severity: Severity,
    pub title: String,
    pub description: Option<String>,
    pub affected_tickets: Vec<String>,
    pub common_factor: Option<String>,
    pub average_excess_percent: Option<f64>,
    pub confidence_score: f64,
    pub suggested_actions: Vec<String>,
    pub metadata: serde_json::Value,
}

/// An alert generated from a pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    pub id: Uuid,
    pub pattern_id: Option<Uuid>,
    pub alert_type: PatternType,
    pub severity: Severity,
    pub title: String,
    pub message: Option<String>,
    pub affected_tickets: Vec<String>,
    pub suggested_actions: Vec<String>,
    pub is_read: bool,
    pub is_dismissed: bool,
    pub dismissed_at: Option<DateTime<Utc>>,
    pub dismissed_by: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Input for creating a new alert.
#[derive(Debug, Clone)]
pub struct NewAlert {
    pub pattern_id: Option<Uuid>,
    pub alert_type: PatternType,
    pub severity: Severity,
    pub title: String,
    pub message: Option<String>,
    pub affected_tickets: Vec<String>,
    pub suggested_actions: Vec<String>,
}

/// Resolution status for patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionStatus {
    Addressed,
    Ignored,
    Recurring,
}

/// Pattern resolution record.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatternResolution {
    pub id: Uuid,
    pub pattern_id: Uuid,
    pub resolution_status: ResolutionStatus,
    pub resolution_notes: Option<String>,
    pub resolved_by: Option<String>,
    pub resolved_at: DateTime<Utc>,
}

/// Workflow data for pattern analysis.
#[derive(Debug, Clone)]
pub struct WorkflowAnalysisData {
    pub workflow_id: Uuid,
    pub ticket_key: String,
    pub template_name: String,
    pub actual_duration_seconds: i64,
    pub estimated_duration_seconds: Option<i64>,
    pub step_notes: Vec<String>,
    pub component: Option<String>,
    pub completed_at: DateTime<Utc>,
}

/// Time excess detection result.
#[derive(Debug, Clone)]
pub struct TimeExcessResult {
    pub ticket_key: String,
    pub actual_seconds: i64,
    pub estimated_seconds: i64,
    pub excess_percent: f64,
    pub step_name: Option<String>,
}

/// Consecutive problem detection result.
#[derive(Debug, Clone)]
pub struct ConsecutiveProblemResult {
    pub ticket_keys: Vec<String>,
    pub common_factor: String,
    pub factor_type: String, // "component", "keyword", "step"
    pub confidence: f64,
}
