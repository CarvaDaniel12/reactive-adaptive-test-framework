//! Time tracking type definitions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// A time tracking session for a workflow step.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TimeSession {
    pub id: Uuid,
    pub workflow_instance_id: Uuid,
    pub step_index: i32,
    pub started_at: DateTime<Utc>,
    pub paused_at: Option<DateTime<Utc>>,
    pub resumed_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub total_seconds: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A pause event within a time session.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TimePauseEvent {
    pub id: Uuid,
    pub session_id: Uuid,
    pub paused_at: DateTime<Utc>,
    pub resumed_at: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i32>,
    pub created_at: DateTime<Utc>,
}

/// Time estimate for a template step.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TimeEstimate {
    pub id: Uuid,
    pub template_id: Uuid,
    pub step_index: i32,
    pub estimated_seconds: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Summary of time spent on a workflow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSummary {
    pub workflow_instance_id: Uuid,
    pub total_seconds: i32,
    pub step_times: Vec<StepTime>,
}

/// Time spent on a single step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepTime {
    pub step_index: i32,
    pub actual_seconds: i32,
    pub estimated_seconds: Option<i32>,
    pub gap_percentage: Option<f32>,
}
