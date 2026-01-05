//! Time tracking service providing high-level operations.
//!
//! This service wraps the low-level `qa-pms-time` repository functions
//! with a convenient API for workflow time tracking.

use sqlx::PgPool;
use tracing::{debug, info, instrument, warn};
use uuid::Uuid;

use crate::error::TrackingError;
use qa_pms_time::{
    end_session, get_active_session, get_session, get_session_for_step, get_workflow_sessions,
    pause_session, resume_session, start_session, StepTime, TimeSession, TimeSummary,
};

/// Service for managing workflow time tracking.
///
/// Provides high-level operations for starting, pausing, resuming,
/// and ending time tracking sessions for workflow steps.
///
/// # Example
///
/// ```ignore
/// use qa_pms_tracking::TrackingService;
///
/// let service = TrackingService::new(pool);
///
/// // Start tracking a step
/// let session = service.start_step(workflow_id, 0).await?;
///
/// // Pause if needed
/// service.pause_current(workflow_id).await?;
///
/// // Resume tracking
/// service.resume_current(workflow_id).await?;
///
/// // End the step
/// let session = service.end_step(workflow_id, 0).await?;
/// ```
#[derive(Clone)]
pub struct TrackingService {
    pool: PgPool,
}

impl TrackingService {
    /// Create a new tracking service.
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Start tracking time for a workflow step.
    ///
    /// If a session already exists for this step, it will be restarted.
    #[instrument(skip(self), fields(workflow_id = %workflow_instance_id, step = step_index))]
    pub async fn start_step(
        &self,
        workflow_instance_id: Uuid,
        step_index: i32,
    ) -> Result<TimeSession, TrackingError> {
        info!("Starting time tracking for step");

        let session = start_session(&self.pool, workflow_instance_id, step_index).await?;

        debug!(session_id = %session.id, "Time session started");
        Ok(session)
    }

    /// End tracking for a specific step.
    #[instrument(skip(self), fields(workflow_id = %workflow_instance_id, step = step_index))]
    pub async fn end_step(
        &self,
        workflow_instance_id: Uuid,
        step_index: i32,
    ) -> Result<TimeSession, TrackingError> {
        let session = get_session_for_step(&self.pool, workflow_instance_id, step_index)
            .await?
            .ok_or(TrackingError::NoActiveSession(workflow_instance_id))?;

        if session.ended_at.is_some() {
            return Err(TrackingError::SessionAlreadyEnded(session.id));
        }

        let ended = end_session(&self.pool, session.id).await?;

        info!(
            session_id = %ended.id,
            total_seconds = ended.total_seconds,
            "Time session ended"
        );

        Ok(ended)
    }

    /// Pause the currently active session for a workflow.
    #[instrument(skip(self), fields(workflow_id = %workflow_instance_id))]
    pub async fn pause_current(&self, workflow_instance_id: Uuid) -> Result<(), TrackingError> {
        let session = get_active_session(&self.pool, workflow_instance_id)
            .await?
            .ok_or(TrackingError::NoActiveSession(workflow_instance_id))?;

        if session.paused_at.is_some() {
            warn!(session_id = %session.id, "Session already paused");
            return Ok(());
        }

        pause_session(&self.pool, session.id).await?;
        debug!(session_id = %session.id, "Time session paused");

        Ok(())
    }

    /// Resume the currently paused session for a workflow.
    #[instrument(skip(self), fields(workflow_id = %workflow_instance_id))]
    pub async fn resume_current(&self, workflow_instance_id: Uuid) -> Result<(), TrackingError> {
        let session = get_active_session(&self.pool, workflow_instance_id)
            .await?
            .ok_or(TrackingError::NoActiveSession(workflow_instance_id))?;

        if session.paused_at.is_none() {
            warn!(session_id = %session.id, "Session not paused");
            return Ok(());
        }

        resume_session(&self.pool, session.id).await?;
        debug!(session_id = %session.id, "Time session resumed");

        Ok(())
    }

    /// Get the currently active session for a workflow.
    #[instrument(skip(self), fields(workflow_id = %workflow_instance_id))]
    pub async fn get_active(
        &self,
        workflow_instance_id: Uuid,
    ) -> Result<Option<TimeSession>, TrackingError> {
        let session = get_active_session(&self.pool, workflow_instance_id).await?;
        Ok(session)
    }

    /// Get a session by ID.
    pub async fn get_session(&self, session_id: Uuid) -> Result<TimeSession, TrackingError> {
        get_session(&self.pool, session_id)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => TrackingError::SessionNotFound(session_id),
                other => TrackingError::Database(other),
            })
    }

    /// Get all sessions for a workflow.
    pub async fn get_workflow_sessions(
        &self,
        workflow_instance_id: Uuid,
    ) -> Result<Vec<TimeSession>, TrackingError> {
        let sessions = get_workflow_sessions(&self.pool, workflow_instance_id).await?;
        Ok(sessions)
    }

    /// Calculate time summary for a workflow.
    ///
    /// Returns total time and per-step breakdown with gap analysis.
    #[instrument(skip(self), fields(workflow_id = %workflow_instance_id))]
    pub async fn calculate_summary(
        &self,
        workflow_instance_id: Uuid,
    ) -> Result<TimeSummary, TrackingError> {
        let sessions = get_workflow_sessions(&self.pool, workflow_instance_id).await?;

        let total_seconds: i32 = sessions.iter().map(|s| s.total_seconds).sum();

        let step_times: Vec<StepTime> = sessions
            .into_iter()
            .map(|s| StepTime {
                step_index: s.step_index,
                actual_seconds: s.total_seconds,
                estimated_seconds: None, // Would need to fetch estimates
                gap_percentage: None,
            })
            .collect();

        Ok(TimeSummary {
            workflow_instance_id,
            total_seconds,
            step_times,
        })
    }

    /// Check if tracking is currently active for a workflow.
    pub async fn is_tracking_active(
        &self,
        workflow_instance_id: Uuid,
    ) -> Result<bool, TrackingError> {
        let session = get_active_session(&self.pool, workflow_instance_id).await?;
        Ok(session.is_some())
    }

    /// Check if tracking is currently paused for a workflow.
    pub async fn is_tracking_paused(
        &self,
        workflow_instance_id: Uuid,
    ) -> Result<bool, TrackingError> {
        let session = get_active_session(&self.pool, workflow_instance_id).await?;
        Ok(session.map_or(false, |s| s.paused_at.is_some()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Integration tests would require a test database
    // These are placeholder tests demonstrating the API

    #[test]
    fn test_tracking_service_creation() {
        // This test verifies the type system only - actual creation requires a pool
        fn _accepts_pool(_pool: PgPool) {
            let _service = TrackingService::new(_pool);
        }
    }
}
