//! Workflow repository functions.
//!
//! Database operations for workflow templates, instances, and step results.

use sqlx::PgPool;
use uuid::Uuid;

use crate::types::{
    StepLink, WorkflowInstance, WorkflowStep, WorkflowStepResult, WorkflowTemplate,
};

// ============================================================================
// Template Operations
// ============================================================================

/// Get all default templates.
///
/// # Errors
/// Returns error if database query fails.
pub async fn get_default_templates(pool: &PgPool) -> Result<Vec<WorkflowTemplate>, sqlx::Error> {
    sqlx::query_as::<_, WorkflowTemplate>(
        r"
        SELECT id, name, description, ticket_type, 
               steps_json, is_default, created_at, updated_at
        FROM workflow_templates
        WHERE is_default = true
        ORDER BY name
        ",
    )
    .fetch_all(pool)
    .await
}

/// Get template by ID.
///
/// # Errors
/// Returns error if database query fails.
pub async fn get_template(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<WorkflowTemplate>, sqlx::Error> {
    sqlx::query_as::<_, WorkflowTemplate>(
        r"
        SELECT id, name, description, ticket_type,
               steps_json, is_default, created_at, updated_at
        FROM workflow_templates
        WHERE id = $1
        ",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

/// Get templates by ticket type.
///
/// # Errors
/// Returns error if database query fails.
pub async fn get_templates_by_type(
    pool: &PgPool,
    ticket_type: &str,
) -> Result<Vec<WorkflowTemplate>, sqlx::Error> {
    sqlx::query_as::<_, WorkflowTemplate>(
        r"
        SELECT id, name, description, ticket_type,
               steps_json, is_default, created_at, updated_at
        FROM workflow_templates
        WHERE ticket_type = $1
        ORDER BY is_default DESC, name
        ",
    )
    .bind(ticket_type)
    .fetch_all(pool)
    .await
}

/// Get all templates.
///
/// # Errors
/// Returns error if database query fails.
pub async fn get_all_templates(pool: &PgPool) -> Result<Vec<WorkflowTemplate>, sqlx::Error> {
    sqlx::query_as::<_, WorkflowTemplate>(
        r"
        SELECT id, name, description, ticket_type,
               steps_json, is_default, created_at, updated_at
        FROM workflow_templates
        ORDER BY is_default DESC, ticket_type, name
        ",
    )
    .fetch_all(pool)
    .await
}

/// Create a new workflow template.
///
/// # Errors
/// Returns error if database insert fails.
pub async fn create_template(
    pool: &PgPool,
    name: &str,
    description: Option<&str>,
    ticket_type: &str,
    steps: &[WorkflowStep],
    is_default: bool,
) -> Result<WorkflowTemplate, sqlx::Error> {
    let steps_json = sqlx::types::Json(steps.to_vec());

    sqlx::query_as::<_, WorkflowTemplate>(
        r"
        INSERT INTO workflow_templates (name, description, ticket_type, steps_json, is_default)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, description, ticket_type, steps_json, is_default, created_at, updated_at
        ",
    )
    .bind(name)
    .bind(description)
    .bind(ticket_type)
    .bind(steps_json)
    .bind(is_default)
    .fetch_one(pool)
    .await
}

// ============================================================================
// Instance Operations
// ============================================================================

/// Get active workflow for a ticket.
///
/// # Errors
/// Returns error if database query fails.
pub async fn get_active_workflow(
    pool: &PgPool,
    ticket_id: &str,
) -> Result<Option<WorkflowInstance>, sqlx::Error> {
    sqlx::query_as::<_, WorkflowInstance>(
        r"
        SELECT id, template_id, ticket_id, user_id, status,
               current_step, started_at, paused_at, resumed_at, completed_at,
               created_at, updated_at
        FROM workflow_instances
        WHERE ticket_id = $1 AND status IN ('active', 'paused')
        ORDER BY created_at DESC
        LIMIT 1
        ",
    )
    .bind(ticket_id)
    .fetch_optional(pool)
    .await
}

/// Get workflow instance by ID.
///
/// # Errors
/// Returns error if database query fails.
pub async fn get_instance(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<WorkflowInstance>, sqlx::Error> {
    sqlx::query_as::<_, WorkflowInstance>(
        r"
        SELECT id, template_id, ticket_id, user_id, status,
               current_step, started_at, paused_at, resumed_at, completed_at,
               created_at, updated_at
        FROM workflow_instances
        WHERE id = $1
        ",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

/// Get all workflows for a user.
///
/// # Errors
/// Returns error if database query fails.
pub async fn get_user_workflows(
    pool: &PgPool,
    user_id: &str,
) -> Result<Vec<WorkflowInstance>, sqlx::Error> {
    sqlx::query_as::<_, WorkflowInstance>(
        r"
        SELECT id, template_id, ticket_id, user_id, status,
               current_step, started_at, paused_at, resumed_at, completed_at,
               created_at, updated_at
        FROM workflow_instances
        WHERE user_id = $1
        ORDER BY created_at DESC
        ",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

/// Create a new workflow instance.
///
/// # Errors
/// Returns error if database insert fails.
pub async fn create_instance(
    pool: &PgPool,
    template_id: Uuid,
    ticket_id: &str,
    user_id: &str,
) -> Result<WorkflowInstance, sqlx::Error> {
    sqlx::query_as::<_, WorkflowInstance>(
        r"
        INSERT INTO workflow_instances (template_id, ticket_id, user_id)
        VALUES ($1, $2, $3)
        RETURNING id, template_id, ticket_id, user_id, status,
                  current_step, started_at, paused_at, resumed_at, completed_at,
                  created_at, updated_at
        ",
    )
    .bind(template_id)
    .bind(ticket_id)
    .bind(user_id)
    .fetch_one(pool)
    .await
}

/// Update workflow instance status.
///
/// # Errors
/// Returns error if database update fails.
pub async fn update_instance_status(
    pool: &PgPool,
    id: Uuid,
    status: &str,
) -> Result<WorkflowInstance, sqlx::Error> {
    let paused_at = if status == "paused" {
        Some(chrono::Utc::now())
    } else {
        None
    };
    let completed_at = if status == "completed" || status == "cancelled" {
        Some(chrono::Utc::now())
    } else {
        None
    };

    sqlx::query_as::<_, WorkflowInstance>(
        r"
        UPDATE workflow_instances
        SET status = $2, paused_at = COALESCE($3, paused_at), 
            completed_at = COALESCE($4, completed_at)
        WHERE id = $1
        RETURNING id, template_id, ticket_id, user_id, status,
                  current_step, started_at, paused_at, resumed_at, completed_at,
                  created_at, updated_at
        ",
    )
    .bind(id)
    .bind(status)
    .bind(paused_at)
    .bind(completed_at)
    .fetch_one(pool)
    .await
}

/// Update workflow instance current step.
///
/// # Errors
/// Returns error if database update fails.
pub async fn update_instance_step(
    pool: &PgPool,
    id: Uuid,
    current_step: i32,
) -> Result<WorkflowInstance, sqlx::Error> {
    sqlx::query_as::<_, WorkflowInstance>(
        r"
        UPDATE workflow_instances
        SET current_step = $2
        WHERE id = $1
        RETURNING id, template_id, ticket_id, user_id, status,
                  current_step, started_at, paused_at, resumed_at, completed_at,
                  created_at, updated_at
        ",
    )
    .bind(id)
    .bind(current_step)
    .fetch_one(pool)
    .await
}

// ============================================================================
// Step Result Operations
// ============================================================================

/// Get step results for a workflow instance.
///
/// # Errors
/// Returns error if database query fails.
pub async fn get_step_results(
    pool: &PgPool,
    instance_id: Uuid,
) -> Result<Vec<WorkflowStepResult>, sqlx::Error> {
    sqlx::query_as::<_, WorkflowStepResult>(
        r"
        SELECT id, instance_id, step_index, status, notes,
               links, started_at, completed_at, created_at, updated_at
        FROM workflow_step_results
        WHERE instance_id = $1
        ORDER BY step_index
        ",
    )
    .bind(instance_id)
    .fetch_all(pool)
    .await
}

/// Get step result by instance and step index.
///
/// # Errors
/// Returns error if database query fails.
pub async fn get_step_result(
    pool: &PgPool,
    instance_id: Uuid,
    step_index: i32,
) -> Result<Option<WorkflowStepResult>, sqlx::Error> {
    sqlx::query_as::<_, WorkflowStepResult>(
        r"
        SELECT id, instance_id, step_index, status, notes,
               links, started_at, completed_at, created_at, updated_at
        FROM workflow_step_results
        WHERE instance_id = $1 AND step_index = $2
        ",
    )
    .bind(instance_id)
    .bind(step_index)
    .fetch_optional(pool)
    .await
}

/// Create or update step result.
///
/// # Errors
/// Returns error if database upsert fails.
pub async fn upsert_step_result(
    pool: &PgPool,
    instance_id: Uuid,
    step_index: i32,
    status: &str,
    notes: Option<&str>,
    links: Option<&[StepLink]>,
) -> Result<WorkflowStepResult, sqlx::Error> {
    let links_json = links.map(|l| sqlx::types::Json(l.to_vec()));

    let started_at = if status == "in_progress" {
        Some(chrono::Utc::now())
    } else {
        None
    };
    let completed_at = if status == "completed" || status == "skipped" {
        Some(chrono::Utc::now())
    } else {
        None
    };

    sqlx::query_as::<_, WorkflowStepResult>(
        r"
        INSERT INTO workflow_step_results (instance_id, step_index, status, notes, links, started_at, completed_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (instance_id, step_index) 
        DO UPDATE SET 
            status = EXCLUDED.status,
            notes = COALESCE(EXCLUDED.notes, workflow_step_results.notes),
            links = COALESCE(EXCLUDED.links, workflow_step_results.links),
            started_at = COALESCE(workflow_step_results.started_at, EXCLUDED.started_at),
            completed_at = COALESCE(EXCLUDED.completed_at, workflow_step_results.completed_at)
        RETURNING id, instance_id, step_index, status, notes,
                  links, started_at, completed_at, created_at, updated_at
        ",
    )
    .bind(instance_id)
    .bind(step_index)
    .bind(status)
    .bind(notes)
    .bind(links_json)
    .bind(started_at)
    .bind(completed_at)
    .fetch_one(pool)
    .await
}

/// Complete a step with notes and links.
///
/// # Errors
/// Returns error if database update fails.
pub async fn complete_step(
    pool: &PgPool,
    instance_id: Uuid,
    step_index: i32,
    notes: Option<&str>,
    links: Option<&[StepLink]>,
) -> Result<WorkflowStepResult, sqlx::Error> {
    upsert_step_result(pool, instance_id, step_index, "completed", notes, links).await
}

/// Start a step.
///
/// # Errors
/// Returns error if database update fails.
pub async fn start_step(
    pool: &PgPool,
    instance_id: Uuid,
    step_index: i32,
) -> Result<WorkflowStepResult, sqlx::Error> {
    upsert_step_result(pool, instance_id, step_index, "in_progress", None, None).await
}

/// Skip a step.
///
/// # Errors
/// Returns error if database update fails.
pub async fn skip_step(
    pool: &PgPool,
    instance_id: Uuid,
    step_index: i32,
) -> Result<WorkflowStepResult, sqlx::Error> {
    upsert_step_result(pool, instance_id, step_index, "skipped", None, None).await
}

/// Pause a workflow.
///
/// # Errors
/// Returns error if database update fails.
pub async fn pause_workflow(pool: &PgPool, instance_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r"
        UPDATE workflow_instances
        SET status = 'paused', paused_at = NOW(), updated_at = NOW()
        WHERE id = $1 AND status = 'active'
        ",
    )
    .bind(instance_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Resume a paused workflow.
///
/// # Errors
/// Returns error if database update fails.
pub async fn resume_workflow(pool: &PgPool, instance_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r"
        UPDATE workflow_instances
        SET status = 'active', resumed_at = NOW(), updated_at = NOW()
        WHERE id = $1 AND status = 'paused'
        ",
    )
    .bind(instance_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Complete a workflow.
///
/// # Errors
/// Returns error if database update fails.
pub async fn complete_workflow(pool: &PgPool, instance_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r"
        UPDATE workflow_instances
        SET status = 'completed', completed_at = NOW(), updated_at = NOW()
        WHERE id = $1
        ",
    )
    .bind(instance_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Cancel a workflow.
///
/// # Errors
/// Returns error if database update fails.
pub async fn cancel_workflow(pool: &PgPool, instance_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r"
        UPDATE workflow_instances
        SET status = 'cancelled', updated_at = NOW()
        WHERE id = $1
        ",
    )
    .bind(instance_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Get all active workflows for a user.
///
/// # Errors
/// Returns error if database query fails.
pub async fn get_all_user_active_workflows(
    pool: &PgPool,
    user_id: &str,
) -> Result<Vec<WorkflowInstance>, sqlx::Error> {
    sqlx::query_as::<_, WorkflowInstance>(
        r"
        SELECT id, template_id, ticket_id, user_id, status,
               current_step, started_at, completed_at, paused_at, resumed_at,
               created_at, updated_at
        FROM workflow_instances
        WHERE user_id = $1 AND status IN ('active', 'paused')
        ORDER BY updated_at DESC
        ",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}
