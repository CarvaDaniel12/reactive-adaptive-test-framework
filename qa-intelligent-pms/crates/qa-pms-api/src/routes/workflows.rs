//! Workflow API endpoints.
//!
//! Provides endpoints for managing workflow templates and instances.
//! Refactored to use unified `ApiError` for cleaner error handling.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::info;
use utoipa::ToSchema;
use uuid::Uuid;

use qa_pms_workflow::{
    cancel_workflow as db_cancel_workflow, complete_step as db_complete_step,
    complete_workflow as db_complete_workflow, create_instance, get_active_workflow,
    get_all_templates, get_all_user_active_workflows, get_instance, get_step_results, get_template,
    pause_workflow as db_pause_workflow, resume_workflow as db_resume_workflow,
    skip_step as db_skip_step, start_step, StepLink, TemplateSummary, WorkflowStep,
};

use crate::app::AppState;
use qa_pms_core::error::ApiError;

/// Result type alias for API handlers.
type ApiResult<T> = Result<T, ApiError>;

/// Helper trait to convert sqlx errors to ApiError.
trait SqlxResultExt<T> {
    fn map_db_err(self) -> Result<T, ApiError>;
}

impl<T> SqlxResultExt<T> for Result<T, sqlx::Error> {
    fn map_db_err(self) -> Result<T, ApiError> {
        self.map_err(|e| ApiError::Internal(e.into()))
    }
}

/// Create the workflows router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/workflows/templates", get(list_templates))
        .route("/api/v1/workflows/templates/:id", get(get_template_by_id))
        .route("/api/v1/workflows", post(create_workflow))
        .route("/api/v1/workflows/:id", get(get_workflow))
        .route("/api/v1/workflows/active/:ticket_id", get(get_active_workflow_for_ticket))
        .route("/api/v1/workflows/:id/steps/:step_index/complete", post(complete_step))
        .route("/api/v1/workflows/:id/steps/:step_index/skip", post(skip_step))
        .route("/api/v1/workflows/:id/pause", post(pause_workflow))
        .route("/api/v1/workflows/:id/resume", post(resume_workflow))
        .route("/api/v1/workflows/:id/complete", post(complete_workflow))
        .route("/api/v1/workflows/:id/summary", get(get_workflow_summary))
        .route("/api/v1/workflows/:id/cancel", post(cancel_workflow))
        .route("/api/v1/workflows/user/active", get(get_user_active_workflows))
}

// ============================================================================
// Response Types
// ============================================================================

/// Template list response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TemplatesListResponse {
    pub templates: Vec<TemplateResponse>,
}

/// Single template response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TemplateResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub ticket_type: String,
    pub step_count: usize,
    pub estimated_minutes: i32,
    pub is_default: bool,
}

impl From<TemplateSummary> for TemplateResponse {
    fn from(t: TemplateSummary) -> Self {
        Self {
            id: t.id,
            name: t.name,
            description: t.description,
            ticket_type: t.ticket_type,
            step_count: t.step_count,
            estimated_minutes: t.estimated_minutes,
            is_default: t.is_default,
        }
    }
}

/// Template detail response with steps.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TemplateDetailResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub ticket_type: String,
    pub steps: Vec<StepResponse>,
    pub estimated_minutes: i32,
    pub is_default: bool,
}

/// Step response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StepResponse {
    pub index: usize,
    pub name: String,
    pub description: String,
    pub estimated_minutes: i32,
}

impl From<(usize, &WorkflowStep)> for StepResponse {
    fn from((index, step): (usize, &WorkflowStep)) -> Self {
        Self {
            index,
            name: step.name.clone(),
            description: step.description.clone(),
            estimated_minutes: step.estimated_minutes,
        }
    }
}

// ============================================================================
// Workflow Instance Types
// ============================================================================

/// Request to create a new workflow instance.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateWorkflowRequest {
    pub template_id: Uuid,
    pub ticket_id: String,
    pub ticket_title: String,
    pub user_id: String,
}

/// Response after creating a workflow.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateWorkflowResponse {
    pub id: Uuid,
    pub template_name: String,
    pub current_step: StepResponse,
    pub total_steps: usize,
}

/// Workflow detail response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowDetailResponse {
    pub id: Uuid,
    pub template_id: Uuid,
    pub template_name: String,
    pub ticket_id: String,
    pub status: String,
    pub current_step: i32,
    pub steps: Vec<WorkflowStepWithStatus>,
    pub estimated_minutes: i32,
    pub started_at: String,
}

/// Step with completion status.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStepWithStatus {
    pub index: usize,
    pub name: String,
    pub description: String,
    pub estimated_minutes: i32,
    pub status: String,
    pub notes: Option<String>,
}

/// Response for checking active workflow.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ActiveWorkflowResponse {
    pub exists: bool,
    pub workflow: Option<WorkflowSummary>,
}

/// Brief workflow summary.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowSummary {
    pub id: Uuid,
    pub template_name: String,
    pub status: String,
    pub current_step: i32,
    pub total_steps: usize,
    pub started_at: String,
}

// ============================================================================
// Step Completion Types
// ============================================================================

/// Request to complete a step.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CompleteStepRequest {
    pub notes: Option<String>,
    #[serde(default)]
    pub links: Vec<StepLinkRequest>,
}

/// Link to attach to a step.
#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StepLinkRequest {
    pub label: String,
    pub url: String,
}

/// Response after completing/skipping a step.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StepActionResponse {
    pub workflow_completed: bool,
    pub next_step: Option<StepResponse>,
    pub current_step_index: i32,
}

/// Response for pause/resume operations.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStatusResponse {
    pub status: String,
    pub message: String,
}

/// Workflow summary response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowSummaryResponse {
    pub id: Uuid,
    pub template_name: String,
    pub ticket_id: String,
    pub status: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub total_steps: usize,
    pub completed_steps: usize,
    pub skipped_steps: usize,
    pub steps: Vec<StepSummary>,
    pub all_notes: Vec<String>,
}

/// Step summary for completed workflow.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StepSummary {
    pub index: usize,
    pub name: String,
    pub status: String,
    pub notes: Option<String>,
}

/// User active workflows response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserActiveWorkflowsResponse {
    pub workflows: Vec<WorkflowSummary>,
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Fetch template or return NotFound error.
async fn fetch_template(state: &AppState, id: Uuid) -> ApiResult<qa_pms_workflow::WorkflowTemplate> {
    get_template(&state.db, id)
        .await
        .map_err(|e| ApiError::Internal(e.into()))?
        .ok_or_else(|| ApiError::NotFound("Template not found".to_string()))
}

/// Fetch workflow instance or return NotFound error.
async fn fetch_instance(state: &AppState, id: Uuid) -> ApiResult<qa_pms_workflow::WorkflowInstance> {
    get_instance(&state.db, id)
        .await
        .map_err(|e| ApiError::Internal(e.into()))?
        .ok_or_else(|| ApiError::NotFound("Workflow not found".to_string()))
}

// ============================================================================
// Handlers - Simplified with ApiError
// ============================================================================

/// List all workflow templates.
#[utoipa::path(
    get,
    path = "/api/v1/workflows/templates",
    responses(
        (status = 200, description = "List of workflow templates", body = TemplatesListResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Workflows"
)]
pub async fn list_templates(State(state): State<AppState>) -> ApiResult<Json<TemplatesListResponse>> {
    let templates = get_all_templates(&state.db).await.map_db_err()?;
    let responses: Vec<TemplateResponse> = templates
        .iter()
        .map(|t| TemplateSummary::from(t).into())
        .collect();

    info!(count = responses.len(), "Listed workflow templates");

    Ok(Json(TemplatesListResponse { templates: responses }))
}

/// Get a workflow template by ID.
#[utoipa::path(
    get,
    path = "/api/v1/workflows/templates/{id}",
    params(("id" = Uuid, Path, description = "Template ID")),
    responses(
        (status = 200, description = "Template details", body = TemplateDetailResponse),
        (status = 404, description = "Template not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Workflows"
)]
pub async fn get_template_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<TemplateDetailResponse>> {
    let template = fetch_template(&state, id).await?;
    let estimated_minutes = template.total_estimated_minutes();
    let steps: Vec<StepResponse> = template
        .steps()
        .iter()
        .enumerate()
        .map(|(i, s)| (i, s).into())
        .collect();

    info!(template_id = %id, "Retrieved workflow template");

    Ok(Json(TemplateDetailResponse {
        id: template.id,
        name: template.name,
        description: template.description,
        ticket_type: template.ticket_type,
        steps,
        estimated_minutes,
        is_default: template.is_default,
    }))
}

/// Create a new workflow instance.
#[utoipa::path(
    post,
    path = "/api/v1/workflows",
    request_body = CreateWorkflowRequest,
    responses(
        (status = 201, description = "Workflow created", body = CreateWorkflowResponse),
        (status = 400, description = "Invalid request"),
        (status = 404, description = "Template not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Workflows"
)]
pub async fn create_workflow(
    State(state): State<AppState>,
    Json(request): Json<CreateWorkflowRequest>,
) -> ApiResult<(StatusCode, Json<CreateWorkflowResponse>)> {
    let template = fetch_template(&state, request.template_id).await?;
    
    let instance = create_instance(
        &state.db,
        request.template_id,
        &request.ticket_id,
        &request.user_id,
    )
    .await
    .map_db_err()?;

    // Start the first step (non-critical if fails)
    if let Err(e) = start_step(&state.db, instance.id, 0).await {
        tracing::warn!(error = %e, "Failed to start first step");
    }

    let steps = template.steps();
    let total_steps = steps.len();
    let template_name = template.name.clone();
    
    let first_step = steps.first().map(|s| StepResponse {
        index: 0,
        name: s.name.clone(),
        description: s.description.clone(),
        estimated_minutes: s.estimated_minutes,
    }).unwrap_or(StepResponse {
        index: 0,
        name: "No steps".to_string(),
        description: String::new(),
        estimated_minutes: 0,
    });

    info!(
        workflow_id = %instance.id,
        ticket_id = %request.ticket_id,
        template = %template_name,
        "Created workflow instance"
    );

    Ok((StatusCode::CREATED, Json(CreateWorkflowResponse {
        id: instance.id,
        template_name,
        current_step: first_step,
        total_steps,
    })))
}

/// Get workflow instance by ID.
#[utoipa::path(
    get,
    path = "/api/v1/workflows/{id}",
    params(("id" = Uuid, Path, description = "Workflow instance ID")),
    responses(
        (status = 200, description = "Workflow details", body = WorkflowDetailResponse),
        (status = 404, description = "Workflow not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Workflows"
)]
pub async fn get_workflow(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<WorkflowDetailResponse>> {
    let instance = fetch_instance(&state, id).await?;
    let template = fetch_template(&state, instance.template_id).await?;
    let step_results = get_step_results(&state.db, id).await.unwrap_or_default();

    let estimated_minutes = template.total_estimated_minutes();
    let template_name = template.name.clone();
    
    let steps: Vec<WorkflowStepWithStatus> = template
        .steps()
        .iter()
        .enumerate()
        .map(|(i, step)| {
            let result = step_results.iter().find(|r| r.step_index == i as i32);
            WorkflowStepWithStatus {
                index: i,
                name: step.name.clone(),
                description: step.description.clone(),
                estimated_minutes: step.estimated_minutes,
                status: result.map_or("pending".to_string(), |r| r.status.clone()),
                notes: result.and_then(|r| r.notes.clone()),
            }
        })
        .collect();

    info!(workflow_id = %id, "Retrieved workflow details");

    Ok(Json(WorkflowDetailResponse {
        id: instance.id,
        template_id: instance.template_id,
        template_name,
        ticket_id: instance.ticket_id,
        status: instance.status,
        current_step: instance.current_step,
        steps,
        estimated_minutes,
        started_at: instance.started_at.to_rfc3339(),
    }))
}

/// Check for active workflow on a ticket.
#[utoipa::path(
    get,
    path = "/api/v1/workflows/active/{ticket_id}",
    params(("ticket_id" = String, Path, description = "Jira ticket key")),
    responses(
        (status = 200, description = "Active workflow check", body = ActiveWorkflowResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Workflows"
)]
pub async fn get_active_workflow_for_ticket(
    State(state): State<AppState>,
    Path(ticket_id): Path<String>,
) -> ApiResult<Json<ActiveWorkflowResponse>> {
    let instance = get_active_workflow(&state.db, &ticket_id).await.map_db_err()?;

    let response = match instance {
        Some(inst) => {
            let template = get_template(&state.db, inst.template_id).await.map_db_err()?.unwrap_or_else(|| {
                panic!("Template not found for instance")
            });
            let total_steps = template.steps().len();
            
            info!(ticket_id = %ticket_id, workflow_id = %inst.id, "Found active workflow");

            ActiveWorkflowResponse {
                exists: true,
                workflow: Some(WorkflowSummary {
                    id: inst.id,
                    template_name: template.name,
                    status: inst.status,
                    current_step: inst.current_step,
                    total_steps,
                    started_at: inst.started_at.to_rfc3339(),
                }),
            }
        }
        None => {
            info!(ticket_id = %ticket_id, "No active workflow found");
            ActiveWorkflowResponse {
                exists: false,
                workflow: None,
            }
        }
    };

    Ok(Json(response))
}

/// Path parameters for step actions.
#[derive(Debug, Deserialize)]
pub struct StepActionPath {
    id: Uuid,
    step_index: i32,
}

/// Complete a workflow step.
#[utoipa::path(
    post,
    path = "/api/v1/workflows/{id}/steps/{step_index}/complete",
    params(
        ("id" = Uuid, Path, description = "Workflow instance ID"),
        ("step_index" = i32, Path, description = "Step index (0-based)")
    ),
    request_body = CompleteStepRequest,
    responses(
        (status = 200, description = "Step completed", body = StepActionResponse),
        (status = 404, description = "Workflow not found"),
        (status = 400, description = "Invalid step index"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Workflows"
)]
pub async fn complete_step(
    State(state): State<AppState>,
    Path(path): Path<StepActionPath>,
    Json(request): Json<CompleteStepRequest>,
) -> ApiResult<Json<StepActionResponse>> {
    let instance = fetch_instance(&state, path.id).await?;
    let template = fetch_template(&state, instance.template_id).await?;
    let total_steps = template.steps().len() as i32;

    if path.step_index < 0 || path.step_index >= total_steps {
        return Err(ApiError::Validation("Invalid step index".to_string()));
    }

    let links: Vec<StepLink> = request
        .links
        .iter()
        .map(|l| StepLink {
            title: l.label.clone(),
            url: l.url.clone(),
        })
        .collect();

    let notes_ref = request.notes.as_deref();
    let links_ref = if links.is_empty() { None } else { Some(links.as_slice()) };
    
    db_complete_step(&state.db, path.id, path.step_index, notes_ref, links_ref).await.map_db_err()?;

    let next_step_index = path.step_index + 1;
    let workflow_completed = next_step_index >= total_steps;

    let next_step = if !workflow_completed {
        template.steps().get(next_step_index as usize).map(|s| StepResponse {
            index: next_step_index as usize,
            name: s.name.clone(),
            description: s.description.clone(),
            estimated_minutes: s.estimated_minutes,
        })
    } else {
        None
    };

    info!(workflow_id = %path.id, step_index = path.step_index, workflow_completed, "Completed workflow step");

    Ok(Json(StepActionResponse {
        workflow_completed,
        next_step,
        current_step_index: if workflow_completed { path.step_index } else { next_step_index },
    }))
}

/// Skip a workflow step.
#[utoipa::path(
    post,
    path = "/api/v1/workflows/{id}/steps/{step_index}/skip",
    params(
        ("id" = Uuid, Path, description = "Workflow instance ID"),
        ("step_index" = i32, Path, description = "Step index (0-based)")
    ),
    responses(
        (status = 200, description = "Step skipped", body = StepActionResponse),
        (status = 404, description = "Workflow not found"),
        (status = 400, description = "Invalid step index"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Workflows"
)]
pub async fn skip_step(
    State(state): State<AppState>,
    Path(path): Path<StepActionPath>,
) -> ApiResult<Json<StepActionResponse>> {
    let instance = fetch_instance(&state, path.id).await?;
    let template = fetch_template(&state, instance.template_id).await?;
    let total_steps = template.steps().len() as i32;

    if path.step_index < 0 || path.step_index >= total_steps {
        return Err(ApiError::Validation("Invalid step index".to_string()));
    }

    db_skip_step(&state.db, path.id, path.step_index).await.map_db_err()?;

    let next_step_index = path.step_index + 1;
    let workflow_completed = next_step_index >= total_steps;

    let next_step = if !workflow_completed {
        template.steps().get(next_step_index as usize).map(|s| StepResponse {
            index: next_step_index as usize,
            name: s.name.clone(),
            description: s.description.clone(),
            estimated_minutes: s.estimated_minutes,
        })
    } else {
        None
    };

    info!(workflow_id = %path.id, step_index = path.step_index, workflow_completed, "Skipped workflow step");

    Ok(Json(StepActionResponse {
        workflow_completed,
        next_step,
        current_step_index: if workflow_completed { path.step_index } else { next_step_index },
    }))
}

/// Pause a workflow.
#[utoipa::path(
    post,
    path = "/api/v1/workflows/{id}/pause",
    params(("id" = Uuid, Path, description = "Workflow instance ID")),
    responses(
        (status = 200, description = "Workflow paused", body = WorkflowStatusResponse),
        (status = 404, description = "Workflow not found"),
        (status = 400, description = "Workflow cannot be paused"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Workflows"
)]
pub async fn pause_workflow(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<WorkflowStatusResponse>> {
    let instance = fetch_instance(&state, id).await?;

    if instance.status != "active" {
        return Err(ApiError::Validation("Workflow is not active".to_string()));
    }

    db_pause_workflow(&state.db, id).await.map_db_err()?;

    info!(workflow_id = %id, "Paused workflow");

    Ok(Json(WorkflowStatusResponse {
        status: "paused".to_string(),
        message: "Workflow paused successfully".to_string(),
    }))
}

/// Resume a paused workflow.
#[utoipa::path(
    post,
    path = "/api/v1/workflows/{id}/resume",
    params(("id" = Uuid, Path, description = "Workflow instance ID")),
    responses(
        (status = 200, description = "Workflow resumed", body = WorkflowStatusResponse),
        (status = 404, description = "Workflow not found"),
        (status = 400, description = "Workflow cannot be resumed"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Workflows"
)]
pub async fn resume_workflow(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<WorkflowStatusResponse>> {
    let instance = fetch_instance(&state, id).await?;

    if instance.status != "paused" {
        return Err(ApiError::Validation("Workflow is not paused".to_string()));
    }

    db_resume_workflow(&state.db, id).await.map_db_err()?;

    info!(workflow_id = %id, "Resumed workflow");

    Ok(Json(WorkflowStatusResponse {
        status: "active".to_string(),
        message: "Workflow resumed successfully".to_string(),
    }))
}

/// Complete a workflow.
#[utoipa::path(
    post,
    path = "/api/v1/workflows/{id}/complete",
    params(("id" = Uuid, Path, description = "Workflow instance ID")),
    responses(
        (status = 200, description = "Workflow completed", body = WorkflowStatusResponse),
        (status = 404, description = "Workflow not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Workflows"
)]
pub async fn complete_workflow(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<WorkflowStatusResponse>> {
    let _ = fetch_instance(&state, id).await?;
    
    db_complete_workflow(&state.db, id).await.map_db_err()?;

    info!(workflow_id = %id, "Completed workflow");

    // Trigger pattern detection in background (Story 9.1, 9.2, 9.3)
    let pool = state.db.clone();
    tokio::spawn(async move {
        let detector = qa_pms_patterns::PatternDetector::new(pool.clone());
        match detector.analyze_workflow(id).await {
            Ok(patterns) => {
                if !patterns.is_empty() {
                    info!(
                        workflow_id = %id,
                        patterns_detected = patterns.len(),
                        "Pattern detection completed"
                    );
                    // Generate alerts for detected patterns
                    let repo = qa_pms_patterns::PatternRepository::new(pool);
                    let alert_service = qa_pms_patterns::AlertService::new(repo);
                    for pattern in patterns {
                        if let Err(e) = alert_service.generate_alert(&pattern).await {
                            tracing::warn!(error = %e, "Failed to generate alert for pattern");
                        }
                    }
                }
            }
            Err(e) => {
                tracing::warn!(workflow_id = %id, error = %e, "Pattern detection failed");
            }
        }
    });

    Ok(Json(WorkflowStatusResponse {
        status: "completed".to_string(),
        message: "Workflow completed successfully".to_string(),
    }))
}

/// Get workflow summary.
#[utoipa::path(
    get,
    path = "/api/v1/workflows/{id}/summary",
    params(("id" = Uuid, Path, description = "Workflow instance ID")),
    responses(
        (status = 200, description = "Workflow summary", body = WorkflowSummaryResponse),
        (status = 404, description = "Workflow not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Workflows"
)]
pub async fn get_workflow_summary(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<WorkflowSummaryResponse>> {
    let instance = fetch_instance(&state, id).await?;
    let template = fetch_template(&state, instance.template_id).await?;
    let step_results = get_step_results(&state.db, id).await.unwrap_or_default();

    let steps: Vec<StepSummary> = template
        .steps()
        .iter()
        .enumerate()
        .map(|(i, step)| {
            let result = step_results.iter().find(|r| r.step_index == i as i32);
            StepSummary {
                index: i,
                name: step.name.clone(),
                status: result.map_or("pending".to_string(), |r| r.status.clone()),
                notes: result.and_then(|r| r.notes.clone()),
            }
        })
        .collect();

    let completed_steps = steps.iter().filter(|s| s.status == "completed").count();
    let skipped_steps = steps.iter().filter(|s| s.status == "skipped").count();
    let all_notes: Vec<String> = steps.iter().filter_map(|s| s.notes.clone()).collect();

    Ok(Json(WorkflowSummaryResponse {
        id: instance.id,
        template_name: template.name,
        ticket_id: instance.ticket_id,
        status: instance.status,
        started_at: instance.started_at.to_rfc3339(),
        completed_at: instance.completed_at.map(|t| t.to_rfc3339()),
        total_steps: steps.len(),
        completed_steps,
        skipped_steps,
        steps,
        all_notes,
    }))
}

/// Cancel a workflow.
#[utoipa::path(
    post,
    path = "/api/v1/workflows/{id}/cancel",
    params(("id" = Uuid, Path, description = "Workflow instance ID")),
    responses(
        (status = 200, description = "Workflow cancelled", body = WorkflowStatusResponse),
        (status = 404, description = "Workflow not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Workflows"
)]
pub async fn cancel_workflow(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<WorkflowStatusResponse>> {
    let _ = fetch_instance(&state, id).await?;

    db_cancel_workflow(&state.db, id).await.map_db_err()?;

    info!(workflow_id = %id, "Cancelled workflow");

    Ok(Json(WorkflowStatusResponse {
        status: "cancelled".to_string(),
        message: "Workflow cancelled".to_string(),
    }))
}

/// Get all active workflows for current user.
#[utoipa::path(
    get,
    path = "/api/v1/workflows/user/active",
    responses(
        (status = 200, description = "User active workflows", body = UserActiveWorkflowsResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Workflows"
)]
pub async fn get_user_active_workflows(State(state): State<AppState>) -> ApiResult<Json<UserActiveWorkflowsResponse>> {
    // TODO: Get user_id from auth context
    let user_id = "current-user@example.com";

    let instances = get_all_user_active_workflows(&state.db, user_id).await.map_db_err()?;

    let mut workflows = Vec::with_capacity(instances.len());
    for inst in instances {
        if let Ok(Some(template)) = get_template(&state.db, inst.template_id).await {
            let total_steps = template.steps().len();
            workflows.push(WorkflowSummary {
                id: inst.id,
                template_name: template.name,
                status: inst.status,
                current_step: inst.current_step,
                total_steps,
                started_at: inst.started_at.to_rfc3339(),
            });
        }
    }

    Ok(Json(UserActiveWorkflowsResponse { workflows }))
}
