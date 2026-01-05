//! Workflow type definitions.
//!
//! Database models and domain types for the workflow engine.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ============================================================================
// Enums
// ============================================================================

/// Workflow instance status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowStatus {
    /// Workflow is actively being worked on
    Active,
    /// Workflow is paused (user stepped away)
    Paused,
    /// Workflow completed successfully
    Completed,
    /// Workflow was cancelled/abandoned
    Cancelled,
}

impl WorkflowStatus {
    /// Convert from database string.
    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s {
            "active" => Self::Active,
            "paused" => Self::Paused,
            "completed" => Self::Completed,
            "cancelled" => Self::Cancelled,
            _ => Self::Active,
        }
    }

    /// Convert to database string.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Paused => "paused",
            Self::Completed => "completed",
            Self::Cancelled => "cancelled",
        }
    }
}

/// Step completion status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepStatus {
    /// Step not yet started
    Pending,
    /// Step currently being worked on
    InProgress,
    /// Step completed successfully
    Completed,
    /// Step was skipped
    Skipped,
}

impl StepStatus {
    /// Convert from database string.
    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s {
            "pending" => Self::Pending,
            "in_progress" => Self::InProgress,
            "completed" => Self::Completed,
            "skipped" => Self::Skipped,
            _ => Self::Pending,
        }
    }

    /// Convert to database string.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::Skipped => "skipped",
        }
    }
}

// ============================================================================
// Step Definition
// ============================================================================

/// Individual step definition within a workflow template.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStep {
    /// Step name (e.g., "Reproduce Bug")
    pub name: String,
    /// Detailed description of what to do
    pub description: String,
    /// Estimated time in minutes
    pub estimated_minutes: i32,
}

/// Link attached to a step result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepLink {
    /// Link title
    pub title: String,
    /// Link URL
    pub url: String,
}

// ============================================================================
// Database Models
// ============================================================================

/// Workflow template stored in database.
#[derive(Debug, Clone, FromRow)]
pub struct WorkflowTemplate {
    /// Unique identifier
    pub id: Uuid,
    /// Template name
    pub name: String,
    /// Template description
    pub description: Option<String>,
    /// Ticket type this template is for (bug, feature, regression, custom)
    pub ticket_type: String,
    /// Steps as JSON array
    pub steps_json: sqlx::types::Json<Vec<WorkflowStep>>,
    /// Whether this is a default template
    pub is_default: bool,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl WorkflowTemplate {
    /// Get the workflow steps.
    #[must_use]
    pub fn steps(&self) -> &[WorkflowStep] {
        &self.steps_json.0
    }

    /// Get the total estimated time in minutes.
    #[must_use]
    pub fn total_estimated_minutes(&self) -> i32 {
        self.steps().iter().map(|s| s.estimated_minutes).sum()
    }
}

/// Workflow instance for a specific ticket.
#[derive(Debug, Clone, FromRow)]
pub struct WorkflowInstance {
    /// Unique identifier
    pub id: Uuid,
    /// Template this instance is based on
    pub template_id: Uuid,
    /// Jira ticket key (e.g., "PROJ-123")
    pub ticket_id: String,
    /// User who started the workflow
    pub user_id: String,
    /// Current status (stored as string in DB)
    pub status: String,
    /// Current step index (0-based)
    pub current_step: i32,
    /// When the workflow was started
    pub started_at: DateTime<Utc>,
    /// When the workflow was paused (if paused)
    pub paused_at: Option<DateTime<Utc>>,
    /// When the workflow was resumed (if resumed after pause)
    pub resumed_at: Option<DateTime<Utc>>,
    /// When the workflow was completed (if completed)
    pub completed_at: Option<DateTime<Utc>>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl WorkflowInstance {
    /// Get the workflow status as enum.
    #[must_use]
    pub fn status_enum(&self) -> WorkflowStatus {
        WorkflowStatus::from_str(&self.status)
    }

    /// Check if workflow is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.status_enum() == WorkflowStatus::Active
    }

    /// Check if workflow can be resumed.
    #[must_use]
    pub fn can_resume(&self) -> bool {
        matches!(
            self.status_enum(),
            WorkflowStatus::Active | WorkflowStatus::Paused
        )
    }
}

/// Result of a completed workflow step.
#[derive(Debug, Clone, FromRow)]
pub struct WorkflowStepResult {
    /// Unique identifier
    pub id: Uuid,
    /// Parent workflow instance
    pub instance_id: Uuid,
    /// Step index (0-based)
    pub step_index: i32,
    /// Step status (stored as string in DB)
    pub status: String,
    /// User notes for this step
    pub notes: Option<String>,
    /// Links attached to this step
    pub links: Option<sqlx::types::Json<Vec<StepLink>>>,
    /// When the step was started
    pub started_at: Option<DateTime<Utc>>,
    /// When the step was completed
    pub completed_at: Option<DateTime<Utc>>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl WorkflowStepResult {
    /// Get the step status as enum.
    #[must_use]
    pub fn status_enum(&self) -> StepStatus {
        StepStatus::from_str(&self.status)
    }

    /// Get the links if any.
    #[must_use]
    pub fn links(&self) -> &[StepLink] {
        self.links.as_ref().map_or(&[], |l| &l.0)
    }

    /// Calculate duration in seconds.
    #[must_use]
    pub fn duration_seconds(&self) -> Option<i64> {
        match (self.started_at, self.completed_at) {
            (Some(start), Some(end)) => Some((end - start).num_seconds()),
            _ => None,
        }
    }
}

// ============================================================================
// API Types
// ============================================================================

/// Template summary for listing.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateSummary {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub ticket_type: String,
    pub step_count: usize,
    pub estimated_minutes: i32,
    pub is_default: bool,
}

impl From<&WorkflowTemplate> for TemplateSummary {
    fn from(t: &WorkflowTemplate) -> Self {
        Self {
            id: t.id,
            name: t.name.clone(),
            description: t.description.clone(),
            ticket_type: t.ticket_type.clone(),
            step_count: t.steps().len(),
            estimated_minutes: t.total_estimated_minutes(),
            is_default: t.is_default,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_status_conversion() {
        assert_eq!(WorkflowStatus::from_str("active"), WorkflowStatus::Active);
        assert_eq!(WorkflowStatus::from_str("paused"), WorkflowStatus::Paused);
        assert_eq!(
            WorkflowStatus::from_str("completed"),
            WorkflowStatus::Completed
        );
        assert_eq!(
            WorkflowStatus::from_str("cancelled"),
            WorkflowStatus::Cancelled
        );
        assert_eq!(WorkflowStatus::from_str("unknown"), WorkflowStatus::Active);
    }

    #[test]
    fn test_step_status_conversion() {
        assert_eq!(StepStatus::from_str("pending"), StepStatus::Pending);
        assert_eq!(StepStatus::from_str("in_progress"), StepStatus::InProgress);
        assert_eq!(StepStatus::from_str("completed"), StepStatus::Completed);
        assert_eq!(StepStatus::from_str("skipped"), StepStatus::Skipped);
        assert_eq!(StepStatus::from_str("unknown"), StepStatus::Pending);
    }

    #[test]
    fn test_workflow_step_serialization() {
        let step = WorkflowStep {
            name: "Test Step".to_string(),
            description: "Do something".to_string(),
            estimated_minutes: 15,
        };

        let json = serde_json::to_string(&step).unwrap();
        assert!(json.contains("\"name\":\"Test Step\""));
        assert!(json.contains("\"estimatedMinutes\":15"));
    }

    #[test]
    fn test_step_link_serialization() {
        let link = StepLink {
            title: "Bug Report".to_string(),
            url: "https://jira.example.com/PROJ-123".to_string(),
        };

        let json = serde_json::to_string(&link).unwrap();
        assert!(json.contains("\"title\":\"Bug Report\""));
    }
}
