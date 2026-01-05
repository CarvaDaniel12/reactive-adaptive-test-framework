//! Time tracking repository functions.

use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::types::{TimeEstimate, TimePauseEvent, TimeSession};

/// Start a new time session for a workflow step.
pub async fn start_session(
    pool: &PgPool,
    workflow_instance_id: Uuid,
    step_index: i32,
) -> Result<TimeSession, sqlx::Error> {
    sqlx::query_as::<_, TimeSession>(
        r"
        INSERT INTO time_sessions (workflow_instance_id, step_index, started_at, is_active)
        VALUES ($1, $2, NOW(), true)
        ON CONFLICT (workflow_instance_id, step_index) 
        DO UPDATE SET started_at = NOW(), is_active = true, updated_at = NOW()
        RETURNING *
        ",
    )
    .bind(workflow_instance_id)
    .bind(step_index)
    .fetch_one(pool)
    .await
}

/// End a time session.
pub async fn end_session(pool: &PgPool, session_id: Uuid) -> Result<TimeSession, sqlx::Error> {
    // First calculate total seconds
    let session = get_session(pool, session_id).await?;
    let elapsed = Utc::now()
        .signed_duration_since(session.started_at)
        .num_seconds() as i32;

    // Get total paused time
    let paused_seconds: i32 = get_total_paused_time(pool, session_id).await.unwrap_or(0);
    let total_seconds = elapsed - paused_seconds;

    sqlx::query_as::<_, TimeSession>(
        r"
        UPDATE time_sessions
        SET ended_at = NOW(), is_active = false, total_seconds = $2, updated_at = NOW()
        WHERE id = $1
        RETURNING *
        ",
    )
    .bind(session_id)
    .bind(total_seconds.max(0))
    .fetch_one(pool)
    .await
}

/// Pause a time session.
pub async fn pause_session(pool: &PgPool, session_id: Uuid) -> Result<TimePauseEvent, sqlx::Error> {
    // Update session paused_at
    sqlx::query(
        r"
        UPDATE time_sessions SET paused_at = NOW(), updated_at = NOW()
        WHERE id = $1
        ",
    )
    .bind(session_id)
    .execute(pool)
    .await?;

    // Create pause event
    sqlx::query_as::<_, TimePauseEvent>(
        r"
        INSERT INTO time_pause_events (session_id, paused_at)
        VALUES ($1, NOW())
        RETURNING *
        ",
    )
    .bind(session_id)
    .fetch_one(pool)
    .await
}

/// Resume a paused time session.
pub async fn resume_session(pool: &PgPool, session_id: Uuid) -> Result<(), sqlx::Error> {
    // Update session resumed_at
    sqlx::query(
        r"
        UPDATE time_sessions SET resumed_at = NOW(), paused_at = NULL, updated_at = NOW()
        WHERE id = $1
        ",
    )
    .bind(session_id)
    .execute(pool)
    .await?;

    // Update the latest pause event
    sqlx::query(
        r"
        UPDATE time_pause_events 
        SET resumed_at = NOW(), 
            duration_seconds = EXTRACT(EPOCH FROM (NOW() - paused_at))::INT
        WHERE session_id = $1 AND resumed_at IS NULL
        ",
    )
    .bind(session_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Get a time session by ID.
pub async fn get_session(pool: &PgPool, session_id: Uuid) -> Result<TimeSession, sqlx::Error> {
    sqlx::query_as::<_, TimeSession>(
        r"SELECT * FROM time_sessions WHERE id = $1",
    )
    .bind(session_id)
    .fetch_one(pool)
    .await
}

/// Get active session for a workflow.
pub async fn get_active_session(
    pool: &PgPool,
    workflow_instance_id: Uuid,
) -> Result<Option<TimeSession>, sqlx::Error> {
    sqlx::query_as::<_, TimeSession>(
        r"
        SELECT * FROM time_sessions 
        WHERE workflow_instance_id = $1 AND is_active = true
        ORDER BY started_at DESC
        LIMIT 1
        ",
    )
    .bind(workflow_instance_id)
    .fetch_optional(pool)
    .await
}

/// Get session for a specific step.
pub async fn get_session_for_step(
    pool: &PgPool,
    workflow_instance_id: Uuid,
    step_index: i32,
) -> Result<Option<TimeSession>, sqlx::Error> {
    sqlx::query_as::<_, TimeSession>(
        r"
        SELECT * FROM time_sessions 
        WHERE workflow_instance_id = $1 AND step_index = $2
        ",
    )
    .bind(workflow_instance_id)
    .bind(step_index)
    .fetch_optional(pool)
    .await
}

/// Get all sessions for a workflow.
pub async fn get_workflow_sessions(
    pool: &PgPool,
    workflow_instance_id: Uuid,
) -> Result<Vec<TimeSession>, sqlx::Error> {
    sqlx::query_as::<_, TimeSession>(
        r"
        SELECT * FROM time_sessions 
        WHERE workflow_instance_id = $1
        ORDER BY step_index
        ",
    )
    .bind(workflow_instance_id)
    .fetch_all(pool)
    .await
}

/// Get total paused time for a session.
pub async fn get_total_paused_time(pool: &PgPool, session_id: Uuid) -> Result<i32, sqlx::Error> {
    let result: (Option<i64>,) = sqlx::query_as(
        r"
        SELECT COALESCE(SUM(duration_seconds), 0) as total
        FROM time_pause_events
        WHERE session_id = $1 AND duration_seconds IS NOT NULL
        ",
    )
    .bind(session_id)
    .fetch_one(pool)
    .await?;

    Ok(result.0.unwrap_or(0) as i32)
}

/// Get time estimate for a template step.
pub async fn get_estimate(
    pool: &PgPool,
    template_id: Uuid,
    step_index: i32,
) -> Result<Option<TimeEstimate>, sqlx::Error> {
    sqlx::query_as::<_, TimeEstimate>(
        r"
        SELECT * FROM time_estimates 
        WHERE template_id = $1 AND step_index = $2
        ",
    )
    .bind(template_id)
    .bind(step_index)
    .fetch_optional(pool)
    .await
}

/// Set time estimate for a template step.
pub async fn set_estimate(
    pool: &PgPool,
    template_id: Uuid,
    step_index: i32,
    estimated_seconds: i32,
) -> Result<TimeEstimate, sqlx::Error> {
    sqlx::query_as::<_, TimeEstimate>(
        r"
        INSERT INTO time_estimates (template_id, step_index, estimated_seconds)
        VALUES ($1, $2, $3)
        ON CONFLICT (template_id, step_index)
        DO UPDATE SET estimated_seconds = $3, updated_at = NOW()
        RETURNING *
        ",
    )
    .bind(template_id)
    .bind(step_index)
    .bind(estimated_seconds)
    .fetch_one(pool)
    .await
}
